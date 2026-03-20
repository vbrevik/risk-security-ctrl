I now have all the context needed. Let me produce the section content.

# Section 3: ConceptCandidate Enrichment

## Overview

This section extends the `ConceptCandidate` struct in the analysis matcher with optional guidance fields (`about_en` and `actions_text`), modifies the SQL queries in `retrieve_candidates()` to LEFT JOIN guidance tables for FTS and exact-match candidates, and adds a second FTS query against `concept_guidance_fts` to broaden recall.

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs`

**Dependencies:** None (this section is parallelizable with Section 1).

**Blocks:** Section 4 (TF-IDF Scoring), Section 5 (Actionable Recommendations), Section 6 (Integration Tests).

---

## Background

The analysis matcher currently retrieves concept candidates in three stages:

1. **FTS5 MATCH** against `concepts_fts` (name, definition columns)
2. **Exact match** on `name_en` and `code` via LIKE
3. **Gap candidates** (all remaining concepts in detected frameworks, scoring 0.0)

Each stage queries the `concepts` table and maps rows through a `ConceptRow` intermediate struct into `ConceptCandidate`. The existing `ConceptCandidate` contains only core concept fields: `id`, `framework_id`, `parent_id`, `name_en`, `definition_en`, `code`, `source_reference`, `concept_type`.

A prior phase (02-schema-import) added four guidance tables to the database:

- `concept_guidance` -- one row per concept with `about_en`, `about_nb`, `source_pdf`, `source_page`
- `concept_actions` -- ordered action items per concept with `action_text_en`
- `concept_transparency_questions` -- ordered questions per concept
- `concept_references` -- academic/transparency references per concept

It also added `concept_guidance_fts`, an FTS5 virtual table indexing `name_en`, `definition_en`, and `about_en` from the guidance view. These tables are populated only for NIST AI RMF action-level concepts.

---

## Tests (Write First)

Add these tests to the `#[cfg(test)] mod tests` block in `matcher.rs`. All async tests use `#[tokio::test]` and the existing `setup_test_db()` helper (which must be extended -- see below).

```rust
// Test: candidate retrieved for concept WITH guidance has about_en populated
#[tokio::test]
async fn test_candidate_with_guidance_has_about_en() {
    // Setup DB with guidance data for a concept.
    // Retrieve candidates via FTS keywords matching that concept.
    // Assert the matching candidate has about_en = Some(...).
}

// Test: candidate retrieved for concept WITH guidance has actions_text populated (newline-separated)
#[tokio::test]
async fn test_candidate_with_guidance_has_actions_text() {
    // Setup DB with two actions for a concept (sort_order 1 and 2).
    // Retrieve candidates. Assert actions_text = Some("action 1\naction 2").
}

// Test: candidate retrieved for concept WITHOUT guidance has None for about_en and actions_text
#[tokio::test]
async fn test_candidate_without_guidance_has_none_fields() {
    // Retrieve candidates for a concept that has no rows in concept_guidance.
    // Assert about_en is None and actions_text is None.
}

// Test: guidance FTS query with custom weights returns results matching about_en content
#[tokio::test]
async fn test_guidance_fts_returns_about_en_matches() {
    // Insert a concept whose name_en/definition_en do NOT contain "measurement".
    // Insert concept_guidance row whose about_en contains "measurement approaches".
    // Rebuild concept_guidance_fts.
    // Retrieve candidates with keyword "measurement".
    // Assert the concept appears in results (found via guidance FTS, not concepts_fts).
}

// Test: union of both FTS tables returns broader candidate set than concepts_fts alone
#[tokio::test]
async fn test_fts_union_broader_recall() {
    // Insert concept A matching only via concepts_fts.
    // Insert concept B matching only via concept_guidance_fts (about_en content).
    // Retrieve candidates. Assert both A and B are present.
}

// Test: gap candidates do NOT have guidance fields populated (None)
#[tokio::test]
async fn test_gap_candidates_no_guidance_fields() {
    // Retrieve candidates where some concepts are gap candidates.
    // Assert gap candidates have about_en = None, actions_text = None.
}

// Test: deduplication across FTS tables keeps first occurrence
#[tokio::test]
async fn test_dedup_across_fts_tables() {
    // Insert concept that matches BOTH concepts_fts and concept_guidance_fts.
    // Retrieve candidates. Assert concept appears exactly once.
}

// Test: existing ConceptCandidate test fixtures updated with new optional fields
#[test]
fn test_make_candidate_includes_guidance_fields() {
    // Verify the make_candidate() helper produces about_en: None, actions_text: None.
}
```

---

## Implementation Details

### 1. Extend `ConceptCandidate` struct

Add two optional fields after the existing fields at line 129:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptCandidate {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub name_en: String,
    pub definition_en: String,
    pub code: Option<String>,
    pub source_reference: Option<String>,
    pub concept_type: String,
    // New guidance fields
    pub about_en: Option<String>,
    pub actions_text: Option<String>,
}
```

### 2. Extend `ConceptRow` and its `From` impl

Add `about_en: Option<String>` and `actions_text: Option<String>` fields to the `ConceptRow` struct (line 253). Update the `From<ConceptRow> for ConceptCandidate` impl (line 264) to map these fields through.

Create a second row type for gap candidates that does NOT include guidance fields, to avoid the unnecessary LEFT JOIN:

```rust
/// Row type for gap candidates (no guidance data needed).
#[derive(sqlx::FromRow)]
struct GapConceptRow {
    id: String,
    framework_id: String,
    parent_id: Option<String>,
    name_en: String,
    definition_en: String,
    code: Option<String>,
    source_reference: Option<String>,
    concept_type: String,
}
```

The `From<GapConceptRow> for ConceptCandidate` impl sets `about_en: None` and `actions_text: None`.

### 3. Modify FTS5 query (Step 1) in `retrieve_candidates()`

Change the Step 1 SQL to LEFT JOIN guidance data. The query at line 303 becomes:

```sql
SELECT c.id, c.framework_id, c.parent_id, c.name_en,
       COALESCE(c.definition_en, '') as definition_en,
       c.code, c.source_reference, c.concept_type,
       cg.about_en,
       (SELECT GROUP_CONCAT(action_text_en, char(10))
        FROM (SELECT action_text_en FROM concept_actions
              WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
FROM concepts c
JOIN concepts_fts ON concepts_fts.rowid = c.rowid
LEFT JOIN concept_guidance cg ON cg.concept_id = c.id
WHERE concepts_fts MATCH ?1
AND c.framework_id IN (SELECT value FROM json_each(?2))
```

Key points:
- `LEFT JOIN concept_guidance` ensures concepts without guidance still appear (fields are NULL/None)
- The subquery for `actions_text` uses `GROUP_CONCAT` with `char(10)` (newline) separator inside a nested SELECT to guarantee `ORDER BY sort_order`
- The outer query uses `ConceptRow` (now with the two extra fields)

### 4. Add guidance FTS query (new Step 1b) in `retrieve_candidates()`

After the existing FTS5 query block, add a second FTS query against `concept_guidance_fts`:

```sql
SELECT c.id, c.framework_id, c.parent_id, c.name_en,
       COALESCE(c.definition_en, '') as definition_en,
       c.code, c.source_reference, c.concept_type,
       cg.about_en,
       (SELECT GROUP_CONCAT(action_text_en, char(10))
        FROM (SELECT action_text_en FROM concept_actions
              WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
FROM concept_guidance_fts gf
JOIN concept_guidance cg ON cg.rowid = gf.rowid
JOIN concepts c ON c.id = cg.concept_id
WHERE concept_guidance_fts MATCH ?1
AND c.framework_id IN (SELECT value FROM json_each(?2))
ORDER BY bm25(concept_guidance_fts, 10.0, 3.0, 5.0)
LIMIT ?3
```

BM25 weights map to the FTS5 column order: `name_en=10.0`, `definition_en=3.0`, `about_en=5.0`. These prioritize concept name matches while boosting guidance `about_en` over definitions.

The LIMIT parameter should use the same cap as the existing FTS query (use `capped.len()` or a reasonable constant like 50).

Use the same `match_expr` (OR-joined sanitized keywords) and `fw_json` binds. The `sanitize_fts_keywords()` function is already applied to keywords (STIG V-222602).

Results are merged into the same `candidates` vec using the existing `seen_ids` HashSet for deduplication. First-occurrence wins -- no score comparison needed because the final ranking is done by TF-IDF re-scoring in Section 4.

Wrap this query in `unwrap_or_else` with a warning log, matching the existing FTS query's error handling pattern. If `concept_guidance_fts` fails (e.g., table not yet created), the matcher gracefully falls back to concepts_fts results only.

### 5. Modify exact-match query (Step 2)

Add the same LEFT JOIN and actions subquery to the exact-match query at line 332:

```sql
SELECT c.id, c.framework_id, c.parent_id, c.name_en,
       COALESCE(c.definition_en, '') as definition_en,
       c.code, c.source_reference, c.concept_type,
       cg.about_en,
       (SELECT GROUP_CONCAT(action_text_en, char(10))
        FROM (SELECT action_text_en FROM concept_actions
              WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
FROM concepts c
LEFT JOIN concept_guidance cg ON cg.concept_id = c.id
WHERE c.framework_id IN (SELECT value FROM json_each(?1))
AND (LOWER(c.name_en) LIKE '%' || ?2 || '%' ESCAPE '\'
     OR LOWER(c.code) LIKE '%' || ?2 || '%' ESCAPE '\')
```

### 6. Keep gap query (Step 3) unchanged

The gap candidate query at line 355 must NOT LEFT JOIN guidance tables. Gap candidates always score 0.0 and guidance fields are never used for scoring or recommendations on gap concepts. Use `GapConceptRow` for this query to produce `ConceptCandidate` with `about_en: None, actions_text: None`.

### 7. Update `make_candidate()` test helper

The `make_candidate()` helper at line 1037 must be updated to include the new fields:

```rust
fn make_candidate(id: &str, name: &str, definition: &str) -> ConceptCandidate {
    ConceptCandidate {
        id: id.into(),
        framework_id: "fw-1".into(),
        parent_id: None,
        name_en: name.into(),
        definition_en: definition.into(),
        code: None,
        source_reference: None,
        concept_type: "concept".into(),
        about_en: None,
        actions_text: None,
    }
}
```

Any other test code that constructs `ConceptCandidate` literals must also be updated with the two new fields set to `None`.

### 8. Extend `setup_test_db()` for guidance tables

The test helper `setup_test_db()` at line 937 must create the guidance tables and the FTS virtual table so the new queries work. Add to the `raw_sql` block:

```sql
CREATE TABLE concept_guidance (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL UNIQUE REFERENCES concepts(id) ON DELETE CASCADE,
    source_pdf TEXT NOT NULL,
    source_page INTEGER NOT NULL,
    about_en TEXT,
    about_nb TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE concept_actions (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    action_text_en TEXT NOT NULL,
    action_text_nb TEXT,
    sort_order INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(concept_id, sort_order)
);

CREATE VIEW concept_guidance_search_v AS
SELECT
    cg.rowid AS rowid,
    c.name_en,
    c.definition_en,
    cg.about_en
FROM concept_guidance cg
JOIN concepts c ON c.id = cg.concept_id;

CREATE VIRTUAL TABLE concept_guidance_fts USING fts5(
    name_en, definition_en, about_en,
    content='concept_guidance_search_v',
    content_rowid='rowid'
);
```

For tests that need guidance data, insert rows into `concept_guidance` and `concept_actions`, then rebuild the FTS index:

```sql
INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
```

---

## Compliance Notes

- **V-222607 (CAT I):** All new queries use `sqlx::query_as().bind()` with parameterized binds. No string interpolation of user input into SQL.
- **V-222602 (CAT I):** FTS5 MATCH input for `concept_guidance_fts` uses the same `sanitize_fts_keywords()` output as `concepts_fts`. The `match_expr` is pre-sanitized.
- **V-222585 (CAT I):** Fail secure. If guidance tables are empty or a concept has no guidance rows, the LEFT JOIN produces NULL which maps to `None`. The guidance FTS query failure is caught and logged, not propagated.
- **V-222610 (CAT II):** The `unwrap_or_else` on the guidance FTS query logs the error internally and returns an empty vec. No database error details are exposed to the caller.

## Implementation Notes (Post-Build)

**Files modified:**
- `backend/src/features/analysis/matcher.rs` — ConceptCandidate + ConceptRow extended, GapConceptRow added, 3 SQL queries modified (FTS with LEFT JOIN, exact-match with LEFT JOIN, guidance FTS new), gap query uses GapConceptRow, setup_test_db extended with guidance tables, 8 new tests + seed_guidance helper

**Code review fixes:**
- Added inline SQL comment mapping BM25 weights to FTS5 column order
- Added explicit count assertion in gap_candidates test to prevent silent pass

**All 42 matcher tests pass. No deviations from plan.**