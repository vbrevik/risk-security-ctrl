I have all the context I need. Here is the section content:

# Section 2: Framework Detection

## Overview

This section implements the `detect_frameworks` function in `backend/src/features/analysis/matcher.rs`. This function takes document keywords, topic tags, and a list of frameworks from the database, and returns framework IDs ordered by match strength. It is the first analytical stage of the matching pipeline -- determining which security/compliance frameworks are relevant to a given document before candidate retrieval begins.

## Dependencies

- **Section 01 (config-types):** Requires `MatcherConfig`, `Topic`, and `ConceptCandidate` structs to be defined in `matcher.rs`. The `Topic` struct must have fields `id: String`, `name_en: String`, `concept_ids: Vec<String>`.
- **Tokenizer module** at `backend/src/features/analysis/tokenizer.rs` provides `extract_keywords()` which is used to tokenize topic names for matching.

## Database Context

The `frameworks` table schema (from `backend/migrations/001_initial_schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS frameworks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT,
    description TEXT,
    source_url TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);
```

The `concepts` table has a `framework_id` foreign key referencing `frameworks(id)`.

## Topic Tags Data

Topics come from `ontology-data/topic-tags.json`. Each topic has:
- `id` -- e.g., `"identity-access"`
- `name_en` -- e.g., `"Identity & Access Management"`
- `concept_ids` -- array of concept IDs that belong to this topic, spanning multiple frameworks (e.g., `"nist-800-53-ac"`, `"nist-csf-pr-aa"`, `"cisa-ztmm-identity"`)

Topics bridge the gap between document keywords and framework identification: a document about "authentication" matches the "Identity & Access Management" topic, which maps to concepts in NIST 800-53, NIST CSF, and CISA ZTMM frameworks.

## Tests

All tests go in a `#[cfg(test)] mod tests` block within `backend/src/features/analysis/matcher.rs`. These are pure unit tests -- no database required because `detect_frameworks` receives framework pairs as a parameter rather than querying the DB itself.

### Test: detect_frameworks with risk-related keywords matches ISO 31000

```rust
/// Given document keywords containing "risk" and "assessment", and topics/frameworks
/// that include ISO 31000, detect_frameworks should return the ISO 31000 framework ID.
#[test]
fn test_detect_frameworks_risk_keywords_match_iso31000() {
    // Setup: keywords from a risk management document
    // Topics: at least one topic whose name_en tokenizes to overlap with "risk"/"assessment"
    // Frameworks: include ("iso31000", "ISO 31000") pair
    // Assert: returned vec contains "iso31000"
}
```

### Test: detect_frameworks with "NIST" keyword matches by direct name

```rust
/// Given document keywords containing "nist", detect_frameworks should match
/// NIST frameworks by direct name matching even without topic overlap.
#[test]
fn test_detect_frameworks_direct_name_match_nist() {
    // Setup: keywords = ["nist", "cybersecurity", "framework"]
    // Topics: can be empty or minimal
    // Frameworks: include ("nist-csf", "NIST Cybersecurity Framework")
    // Assert: "nist-csf" is in the returned vec
}
```

### Test: detect_frameworks with unrelated keywords returns empty

```rust
/// Given document keywords that have no overlap with any topic or framework name,
/// detect_frameworks should return an empty vec.
#[test]
fn test_detect_frameworks_unrelated_keywords_empty() {
    // Setup: keywords = ["banana", "tropical", "fruit"]
    // Topics/frameworks: standard security frameworks
    // Assert: returned vec is empty
}
```

### Test: detect_frameworks orders by match strength

```rust
/// When multiple frameworks match, they should be ordered by score (highest first).
#[test]
fn test_detect_frameworks_ordered_by_strength() {
    // Setup: keywords that strongly match one framework and weakly match another
    // Assert: strongly-matched framework appears first in the returned vec
}
```

## Implementation Details

### File: `backend/src/features/analysis/matcher.rs`

#### Function signature

```rust
/// Detect which frameworks are relevant to a document based on keyword matching.
///
/// Uses a two-pronged approach:
/// 1. Topic matching: overlap between document keywords and topic name tokens,
///    then map matched topics to their concept_ids to identify frameworks.
/// 2. Direct name matching: check if document keywords contain framework names
///    or common abbreviations (e.g., "nist", "iso", "gdpr").
///
/// Returns framework IDs ordered by match strength (highest first).
pub fn detect_frameworks(
    doc_keywords: &[String],
    topics: &[Topic],
    frameworks: &[(String, String)],  // (id, name) pairs (changed to borrow per code review)
    config: &MatcherConfig,
) -> Vec<String>
```

Note: This is a synchronous, pure function. The caller (in Section 06) is responsible for querying `SELECT id, name FROM frameworks` and passing the results in.

#### Algorithm in detail

**Step 1 -- Topic matching:**

For each topic in `topics`:
1. Tokenize `topic.name_en` using `extract_keywords()` from the tokenizer module.
2. Compute overlap count: how many of these topic keyword tokens appear in `doc_keywords`.
3. If overlap > 0, the topic is "matched." Record the overlap count as the topic's score.
4. Collect all `concept_ids` from matched topics into a `HashSet` (deduplicated per code review to prevent score inflation from overlapping topics).

**Step 2 -- Framework scoring:**

Build a `HashMap<String, f64>` mapping framework_id to score. For each framework `(id, name)`:

1. **Topic-based score:** Count how many concept_ids from matched topics have a prefix matching this framework's ID (since concept IDs typically follow the pattern `{framework-id}-{suffix}`). Alternatively, iterate framework concepts and check membership. The simpler approach: for each concept_id in the matched topic concept_ids, check if any framework's ID is a prefix of the concept_id (e.g., concept `"nist-csf-pr-aa"` has prefix `"nist-csf"`). Accumulate count per framework.

2. **Direct name match score:** Tokenize the framework `name` with `extract_keywords()`. Check overlap between these tokens and `doc_keywords`. Also check common abbreviations -- extract the first word or known abbreviations from the framework name (e.g., "nist" from "NIST Cybersecurity Framework", "iso" from "ISO 31000"). Each direct name match adds a fixed bonus (e.g., 2.0 points).

3. **Combine:** `total_score = topic_concept_count + direct_name_bonus`.

**Step 3 -- Filter and sort:**

1. Remove any framework with `total_score <= 0.0`.
2. Sort descending by `total_score`.
3. Return the framework IDs as `Vec<String>`.

#### Key considerations

- The `doc_keywords` are already lowercased and stopword-filtered (from `extract_keywords()` in the tokenizer). Topic name tokens and framework name tokens must also be lowercased for comparison. Using `extract_keywords()` on topic/framework names ensures consistent tokenization.
- Create a `HashSet<&str>` from `doc_keywords` for O(1) lookups during overlap computation.
- Framework ID prefix matching for concept_ids: use `concept_id.starts_with(&format!("{}-", framework_id)) || concept_id == framework_id` with delimiter guard to prevent prefix collisions (e.g., "nist" matching "nist-csf-*"). Changed per code review.
- The `config` parameter is accepted for future extensibility (e.g., configurable minimum topic overlap threshold) but is not used in the initial implementation.