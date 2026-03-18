Now I have all the context needed. Let me generate the section content.

# Section 3: FTS5 Candidate Retrieval

## Overview

This section implements the `retrieve_candidates` function in `backend/src/features/analysis/matcher.rs`. It uses SQLite FTS5 full-text search to find ontology concepts that match document keywords, supplements those results with exact name/code matches, deduplicates, and then loads all remaining concepts from detected frameworks as gap candidates (concepts the document does not mention).

This function is the bridge between framework detection (section 02) and scoring (section 04). Its output is a list of `ConceptCandidate` structs that the scorer will rank.

## Dependencies

- **Section 01 (config-types):** Provides `MatcherConfig` and the `ConceptCandidate` struct. That section must define `ConceptCandidate` before this section can be implemented.
- **Existing code:** The `concepts_fts` FTS5 virtual table already exists in the database schema (migration `001_initial_schema.sql`). It indexes `name_en`, `name_nb`, `definition_en`, `definition_nb` with `content='concepts'` and `content_rowid='rowid'`. Triggers keep it in sync.

## Database Schema Context

The `concepts` table has these columns relevant to candidate retrieval:

- `id TEXT PRIMARY KEY`
- `framework_id TEXT NOT NULL`
- `parent_id TEXT` (NULL for root concepts)
- `concept_type TEXT NOT NULL`
- `code TEXT` (e.g., "ID.AM-1")
- `name_en TEXT NOT NULL`
- `definition_en TEXT`
- `source_reference TEXT`

The FTS5 virtual table `concepts_fts` mirrors `name_en`, `name_nb`, `definition_en`, `definition_nb` and uses `content_rowid='rowid'` to join back to `concepts`.

## ConceptCandidate Struct

This struct is defined in section 01 but is documented here for clarity. It must have at least these fields:

```rust
pub struct ConceptCandidate {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub name_en: String,
    pub definition_en: String,
    pub code: Option<String>,
    pub source_reference: Option<String>,
    pub concept_type: String,
}
```

Derive `Debug, Clone, sqlx::FromRow` (or map manually from query results).

## Tests

All tests in this section are integration tests that require a populated SQLite database. They should be placed in a `#[cfg(test)] mod tests` block inside `backend/src/features/analysis/matcher.rs` (or a dedicated integration test file).

### Test: retrieve_candidates returns concepts matching keywords

Set up an in-memory SQLite database with the FTS5 table and a few concepts. Call `retrieve_candidates` with keywords that match one of the concepts. Assert the returned vec contains the expected concept ID.

```rust
/// Integration test: retrieve_candidates finds FTS5-matched concepts.
/// Requires an in-memory SQLite DB with concepts + concepts_fts populated.
#[sqlx::test]
async fn test_retrieve_candidates_returns_fts_matches(pool: SqlitePool) {
    // Setup: run migrations, insert a framework and concept with "risk assessment" in name_en
    // Act: call retrieve_candidates with keywords ["risk", "assessment"] and the framework ID
    // Assert: returned candidates include the inserted concept
}
```

### Test: retrieve_candidates includes gap candidates

After FTS5 matching, all concepts from detected frameworks that were NOT matched should still appear in the output (as gap candidates for later scoring at 0.0).

```rust
/// Integration test: unmatched concepts from detected frameworks appear as gap candidates.
#[sqlx::test]
async fn test_retrieve_candidates_includes_gap_candidates(pool: SqlitePool) {
    // Setup: insert framework with 3 concepts, only 1 matches keywords
    // Act: call retrieve_candidates
    // Assert: all 3 concepts appear in output (1 FTS match + 2 gap candidates)
}
```

### Test: retrieve_candidates deduplicates results

If a concept matches both the FTS5 query and the exact name/code query, it should appear only once.

```rust
/// Integration test: duplicate concepts from FTS5 + exact match are deduplicated.
#[sqlx::test]
async fn test_retrieve_candidates_deduplicates(pool: SqlitePool) {
    // Setup: insert concept whose name_en exactly matches a keyword AND matches FTS5
    // Act: call retrieve_candidates
    // Assert: concept appears exactly once in output
}
```

### Test: FTS5 query sanitizes special characters

Keywords containing FTS5 operators (quotes, asterisks, parentheses, etc.) must be stripped before building the MATCH query to avoid syntax errors.

```rust
#[test]
fn test_sanitize_fts_query_removes_special_chars() {
    // Act: call sanitize_fts_keywords with vec!["risk*", "\"assessment\"", "(control)"]
    // Assert: result is ["risk", "assessment", "control"] with operators stripped
}
```

## Implementation Details

### File: `backend/src/features/analysis/matcher.rs`

#### Helper: sanitize_fts_keywords

A pure function that strips FTS5 special characters and reserved words from keywords before building the MATCH query. FTS5 operators removed: `"`, `*`, `(`, `)`, `+`, `-`, `^`, `{`, `}`, `:`, `~`, `\`. FTS5 reserved words filtered: AND, OR, NOT, NEAR (case-insensitive). Also discards any keyword that becomes empty after sanitization. Added `escape_like()` helper for LIKE wildcard escaping (`%`, `_`). Per code review: keywords lowercased before LIKE binding, ESCAPE clause added to LIKE patterns.

Signature:

```rust
/// Strip FTS5 special operators from keywords to prevent MATCH syntax errors.
fn sanitize_fts_keywords(keywords: &[String]) -> Vec<String>
```

#### Main function: retrieve_candidates

Signature:

```rust
/// Retrieve candidate concepts from the database using FTS5 and exact matching.
///
/// Returns all matched concepts plus gap candidates (unmatched concepts from
/// detected frameworks) for comprehensive gap analysis.
pub async fn retrieve_candidates(
    keywords: &[String],
    framework_ids: &[String],
    db: &SqlitePool,
) -> Result<Vec<ConceptCandidate>, sqlx::Error>
```

Algorithm, step by step:

1. **Sanitize and cap keywords.** Call `sanitize_fts_keywords` on the input. Take the first 20 keywords (cap at 20 to keep FTS5 queries reasonable).

2. **Build FTS5 MATCH string.** Join sanitized keywords with `" OR "` to form a query like `"risk OR assessment OR security OR control"`. If no keywords survive sanitization, skip the FTS5 query entirely.

3. **Execute FTS5 query.** Use a query that joins `concepts_fts` back to `concepts` and filters by framework:

   ```sql
   SELECT c.id, c.framework_id, c.parent_id, c.name_en,
          COALESCE(c.definition_en, '') as definition_en,
          c.code, c.source_reference, c.concept_type
   FROM concepts c
   JOIN concepts_fts ON concepts_fts.rowid = c.rowid
   WHERE concepts_fts MATCH ?1
   AND c.framework_id IN (select value from json_each(?2))
   ```

   The `framework_ids` filter uses `json_each()` with a JSON array string to avoid dynamic SQL for the IN clause. Pass `serde_json::to_string(&framework_ids)` as the parameter.

   **Important:** SQLite FTS5 `MATCH` requires the query string as a single parameter. Do not use `query_as!` (compile-time checked) because the dynamic nature of MATCH strings and `json_each` makes `sqlx::query_as` (runtime) more practical here.

4. **Execute exact match query.** For each keyword, check against `LOWER(name_en)` and `LOWER(code)`:

   ```sql
   SELECT c.id, c.framework_id, c.parent_id, c.name_en,
          COALESCE(c.definition_en, '') as definition_en,
          c.code, c.source_reference, c.concept_type
   FROM concepts c
   WHERE c.framework_id IN (select value from json_each(?1))
   AND (LOWER(c.name_en) LIKE '%' || ?2 || '%' OR LOWER(c.code) LIKE '%' || ?2 || '%')
   ```

   Run this for a subset of keywords (e.g., keywords longer than 4 characters) to avoid overly broad LIKE matches. Collect results into the same candidate set.

5. **Deduplicate.** Use a `HashSet<String>` of concept IDs. Insert FTS5 results first, then exact match results, skipping any ID already seen.

6. **Load gap candidates.** Query all concepts from the detected frameworks that are NOT in the matched set:

   ```sql
   SELECT c.id, c.framework_id, c.parent_id, c.name_en,
          COALESCE(c.definition_en, '') as definition_en,
          c.code, c.source_reference, c.concept_type
   FROM concepts c
   WHERE c.framework_id IN (select value from json_each(?1))
   AND c.id NOT IN (select value from json_each(?2))
   ```

   Pass the matched IDs as a JSON array via `json_each` to avoid dynamic SQL.

7. **Combine and return.** Append gap candidates to the matched candidates vec. Return the full list.

### Error Handling

- If the FTS5 MATCH query fails (e.g., all keywords were stripped), log a warning and continue with only exact matches and gap candidates.
- Database errors from `sqlx` propagate as `sqlx::Error`. The caller (section 06, `DeterministicMatcher`) converts these to `AnalysisError::DatabaseError`.

### Performance Considerations

- The keyword cap of 20 prevents FTS5 queries from becoming too large.
- The `json_each()` approach for IN clauses avoids SQL injection and dynamic query string construction.
- Gap candidate loading is a single query, not N+1.
- For a typical ontology of ~1000 concepts across ~24 frameworks, with 2-4 detected frameworks, the gap candidate query returns at most a few hundred rows -- well within SQLite's comfort zone.