# Implementation Plan: 03-matching-engine

## Overview

This plan implements `DeterministicMatcher` — a struct that implements the `MatchingEngine` async trait defined in split 01. It analyzes document text against the ontology's ~1000 concepts across 24 frameworks using a deterministic two-stage pipeline: FTS5 full-text search for candidate retrieval, followed by keyword overlap scoring for ranking.

The matcher is a single file (`backend/src/features/analysis/matcher.rs`) plus a prompt template configuration type. It consumes the tokenizer utilities from split 02 and the trait/types from split 01. The route handlers (split 04) will call it.

## Why This Matters

This is the analytical core. Without it, documents can be parsed (split 02) but not analyzed against frameworks. The matcher turns "here's some text" into "here are 47 findings across 3 frameworks, with 12 gaps prioritized by severity."

## Architecture

```
Input: text + optional prompt_template + topics + db pool
  │
  ├─ Stage 1: Framework Detection
  │   keywords → topic matching → framework identification
  │
  ├─ Stage 2: FTS5 Candidate Retrieval
  │   FTS5 MATCH query per framework → candidate concept IDs
  │
  ├─ Stage 3: TF-IDF Scoring
  │   concept definition keywords × document keywords → confidence_score
  │
  ├─ Stage 4: Gap Classification
  │   score thresholds → addressed / partially_addressed / gap
  │
  ├─ Stage 5: Reference Validation
  │   verify concept_id exists in DB → drop invalid
  │
  ├─ Stage 6: Priority Ranking
  │   concept depth (root=P1, child=P2, grandchild+=P3/P4)
  │
  └─ Stage 7: Recommendation Generation
      finding_type → templated recommendation text
```

---

## Section 1: Prompt Template Configuration

### File: `backend/src/features/analysis/matcher.rs`

Define the `MatcherConfig` struct that deserializes from the prompt template JSON. This configures all tunable parameters of the matching pipeline.

#### MatcherConfig struct

Derives `Debug, Clone, Serialize, Deserialize`. Fields:

- `version: u32` — schema version (currently 1)
- `min_confidence_threshold: f64` — minimum score to include a finding (default 0.1)
- `addressed_threshold: f64` — score >= this means "addressed" (default 0.6)
- `partial_threshold: f64` — score >= this means "partially addressed" (default 0.3)
- `max_findings_per_framework: usize` — cap findings per framework (default 50)
- `include_addressed_findings: bool` — whether to include high-confidence matches (default true)
- `boost_terms: HashMap<String, f64>` — domain terms that get score multipliers (default: security=1.5, risk=1.5, compliance=1.3, control=1.2)

Provide a `Default` impl that returns sensible defaults matching the spec's JSON example.

Provide a `from_json` associated function that takes `Option<&str>` and returns `Self` — parses JSON if provided, falls back to default if None or parse failure (log warning on failure).

#### Topic struct (local)

A minimal struct for deserialized topic tags passed in from the route handler. Fields: `id: String`, `name_en: String`, `concept_ids: Vec<String>`. Derive `Debug, Clone, Deserialize`.

---

## Section 2: Framework Detection

### File: `backend/src/features/analysis/matcher.rs` (continued)

#### detect_frameworks function

Takes: document keywords (`&[String]`), topics (`&[Topic]`), frameworks from DB (`Vec<(String, String)>` — id, name pairs), config (`&MatcherConfig`).

Returns: `Vec<String>` — framework IDs ordered by match strength.

Algorithm:

1. **Topic matching:** For each topic, compute overlap between topic `name_en`/keyword tokens and document keywords. Score = overlap count. Map matched topics to their `concept_ids`.

2. **Framework scoring:** For each framework, count how many of its concepts appear in matched topic concept_ids. Also check if document keywords contain the framework name or common abbreviations (e.g., "nist", "iso", "gdpr"). Combine topic-based score + direct name match score.

3. **Filter:** Only include frameworks with score > 0.

4. **Sort:** Descending by score. Return framework IDs.

This function queries the DB to get framework list: `SELECT id, name FROM frameworks`.

---

## Section 3: FTS5 Candidate Retrieval

### File: `backend/src/features/analysis/matcher.rs` (continued)

#### retrieve_candidates function

Takes: document keywords (`&[String]`), framework_ids (`&[String]`), db (`&SqlitePool`).

Returns: `Vec<ConceptCandidate>` — deduplicated candidate concepts with their metadata.

#### ConceptCandidate struct

Fields: `id: String`, `framework_id: String`, `parent_id: Option<String>`, `name_en: String`, `definition_en: String`, `code: Option<String>`, `source_reference: Option<String>`, `concept_type: String`.

Algorithm:

1. **Build FTS5 query:** Join top-N keywords (up to 20) with OR: `"keyword1 OR keyword2 OR keyword3"`. Use FTS5 MATCH syntax.

2. **Query FTS5:**
   ```sql
   SELECT c.id, c.framework_id, c.parent_id, c.name_en, c.definition_en, c.code, c.source_reference, c.concept_type
   FROM concepts c
   JOIN concepts_fts ON concepts_fts.rowid = c.rowid
   WHERE concepts_fts MATCH ?
   AND c.framework_id IN (?, ?, ...)
   ```
   Note: FTS5 content table is `concepts`, so `concepts_fts.rowid` maps to `concepts.rowid`.

3. **Also query exact matches** on concept names/codes for keywords not well-served by FTS5:
   ```sql
   SELECT ... FROM concepts WHERE framework_id IN (...) AND (LOWER(name_en) LIKE ? OR LOWER(code) LIKE ?)
   ```

4. **Deduplicate** by concept ID.

5. **Also load ALL concepts** from detected frameworks that had NO FTS5 match — these are gap candidates. Query: `SELECT ... FROM concepts WHERE framework_id IN (...) AND id NOT IN (matched_ids)`.

Important: The FTS5 MATCH query syntax uses OR between terms. Individual terms should not contain special FTS5 operators (no quotes, no AND). Sanitize keywords before building the query string.

---

## Section 4: TF-IDF Scoring

### File: `backend/src/features/analysis/matcher.rs` (continued)

#### score_candidates function

Takes: candidates (`&[ConceptCandidate]`), document keywords (`&[String]`), document term_freq (`&HashMap<String, usize>`), config (`&MatcherConfig`).

Returns: `Vec<ScoredCandidate>` — candidates with confidence scores.

#### ScoredCandidate struct

Fields: all ConceptCandidate fields + `confidence_score: f64`.

Algorithm for each candidate:

1. **Extract concept keywords** from `name_en` + `definition_en` using `extract_keywords()` from the tokenizer.

2. **Compute overlap:** Count how many concept keywords appear in the document keywords set. Apply boost_terms multipliers from config.

3. **Compute TF component:** For each overlapping keyword, look up its frequency in the document term_freq map. Sum weighted frequencies.

4. **Compute IDF component:** Keywords that appear in fewer concept definitions across ALL candidates are more discriminating. IDF = log(total_candidates / candidates_containing_term).

5. **Combine:** `raw_score = sum(TF * IDF * boost)` for all overlapping keywords. Normalize to 0.0-1.0 by dividing by the maximum possible score (if all concept keywords matched with max boost).

6. **Clamp** to 0.0-1.0 range.

For gap candidates (concepts with zero keyword overlap), score is 0.0.

---

## Section 5: Gap Classification and Findings

### File: `backend/src/features/analysis/matcher.rs` (continued)

#### classify_findings function

Takes: scored candidates (`Vec<ScoredCandidate>`), config (`&MatcherConfig`).

Returns: `Vec<NewFinding>` — classified findings with priorities and recommendations.

Algorithm:

1. **Classify by threshold:**
   - score >= `addressed_threshold` (0.6) → `FindingType::Addressed`
   - score >= `partial_threshold` (0.3) → `FindingType::PartiallyAddressed`
   - score < `partial_threshold` → `FindingType::Gap`

2. **Filter:** Remove findings below `min_confidence_threshold` (0.1) UNLESS they are gaps (score 0.0 gaps are always included — they represent missing coverage).

3. **If `include_addressed_findings` is false**, drop `Addressed` findings.

4. **Cap** per framework at `max_findings_per_framework`.

5. **Assign priority** based on concept depth:
   - `parent_id IS NULL` (root concept): gaps → P1, partial → P2
   - One level deep: gaps → P2, partial → P3
   - Deeper: gaps → P3, partial → P4
   - Addressed: always P4

6. **Generate recommendation** text:
   - Addressed: `"Document adequately covers {name_en}. Reference: {source_reference}"`
   - Partial: `"Document partially addresses {name_en}. Consider expanding coverage of {definition excerpt (first 100 chars)}. Reference: {source_reference}"`
   - Gap: `"Document does not address {name_en}: {definition excerpt}. Recommended action: review and implement controls per {source_reference}"`

7. **Extract evidence_text** for non-gap findings: find the sentence from the document that best matches the concept (highest keyword overlap with any sentence from the tokenizer's sentence_split output). Store that sentence as evidence.

---

## Section 6: DeterministicMatcher Implementation

### File: `backend/src/features/analysis/matcher.rs` (continued)

#### DeterministicMatcher struct

A unit struct: `pub struct DeterministicMatcher;`

Implements `MatchingEngine` trait. The `analyze` method orchestrates all stages:

```rust
#[async_trait]
impl MatchingEngine for DeterministicMatcher {
    async fn analyze(&self, text, prompt_template, db) -> Result<MatchingResult, AnalysisError> {
        // 1. Parse config from prompt_template
        // 2. Extract keywords and term frequency from text
        // 3. Load frameworks from DB
        // 4. Detect relevant frameworks (needs topics passed somehow)
        // 5. Retrieve candidates via FTS5
        // 6. Score candidates
        // 7. Classify into findings
        // 8. Validate references
        // 9. Build MatchingResult
    }
}
```

**Topics parameter problem:** The `MatchingEngine` trait takes `(text, prompt_template, db)` — no topics parameter. Two options:
- Store topics in `DeterministicMatcher` as a field (loaded once at construction)
- Load topics from disk inside `analyze()` (matching what the API endpoint does)

Since the user chose "pass via parameter" but the trait is fixed, the pragmatic solution is to **store topics in the struct** at construction time. The route handler loads topics and passes them when constructing the matcher:

```rust
pub struct DeterministicMatcher {
    topics: Vec<Topic>,
}

impl DeterministicMatcher {
    pub fn new(topics: Vec<Topic>) -> Self { ... }
}
```

#### Reference validation

After classification, verify each finding's `concept_id` exists:
```sql
SELECT COUNT(*) FROM concepts WHERE id = ?
```
Drop any finding with a non-existent concept_id and log a warning.

#### Timing and token count

Record `Instant::now()` at start, compute `elapsed.as_millis()` for `processing_time_ms`. Token count is the document's word count * 1.33 (same formula as the parser).

---

## Section 7: Module Wiring

### Files to modify

1. **`backend/src/features/analysis/mod.rs`** — Add `pub mod matcher;`

No Cargo.toml changes needed — all dependencies (sqlx, serde, async-trait) are already present.

---

## Decision Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Gap scope | All concepts | User wants comprehensive analysis |
| Topics storage | Struct field | Trait signature is fixed, topics needed at detection time |
| FTS5 query | OR-joined keywords | Simple, effective for candidate retrieval |
| Score normalization | 0.0-1.0 clamped | Consistent with DB schema CHECK constraint |
| Evidence extraction | Best-matching sentence | Gives users specific document context for each finding |
| Priority algorithm | Concept depth-based | Root concepts are more important than nested ones |
