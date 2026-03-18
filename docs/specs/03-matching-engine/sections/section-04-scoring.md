Now I have all the context needed. Let me generate the section content.

# Section 4: TF-IDF Scoring

## Overview

This section implements the `score_candidates()` function in `backend/src/features/analysis/matcher.rs`. It takes candidate concepts (retrieved by FTS5 in section 03) and scores them against the document using a TF-IDF-like algorithm with configurable boost terms. The output is a list of `ScoredCandidate` structs with `confidence_score` values normalized to the 0.0--1.0 range.

## Dependencies

- **Section 01 (Config Types):** Provides `MatcherConfig` (with `boost_terms: HashMap<String, f64>`) and the `ConceptCandidate` struct.
- **Section 03 (FTS5 Retrieval):** Produces the `Vec<ConceptCandidate>` input consumed by this function.
- **Tokenizer module** (`backend/src/features/analysis/tokenizer.rs`): Provides `extract_keywords()` which returns deduplicated, lowercased, stopword-filtered keywords from text.

## File to Modify

`backend/src/features/analysis/matcher.rs`

## Data Structures

### ScoredCandidate struct

All fields from `ConceptCandidate` plus a confidence score. Define it alongside `ConceptCandidate` (from section 01):

```rust
#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub name_en: String,
    pub definition_en: String,
    pub code: Option<String>,
    pub source_reference: Option<String>,
    pub concept_type: String,
    pub confidence_score: f64,
}
```

## Function Signature

```rust
pub fn score_candidates(
    candidates: &[ConceptCandidate],
    doc_keywords: &[String],
    doc_term_freq: &HashMap<String, usize>,
    config: &MatcherConfig,
) -> Vec<ScoredCandidate>
```

Parameters:

- `candidates` -- concept candidates from section 03's `retrieve_candidates()`.
- `doc_keywords` -- keywords extracted from the document via `extract_keywords()`.
- `doc_term_freq` -- term frequency map from the document via `term_frequency()`.
- `config` -- matcher configuration providing `boost_terms`.

Returns a `Vec<ScoredCandidate>` with every input candidate scored. Gap candidates (zero overlap) receive a score of 0.0.

## Algorithm

For each candidate in `candidates`:

1. **Extract concept keywords.** Call `extract_keywords()` (from `super::tokenizer`) on a concatenation of `candidate.name_en` and `candidate.definition_en`. This yields the concept's keyword set.

2. **Compute keyword overlap.** Build a `HashSet` from `doc_keywords` for O(1) lookup. For each concept keyword, check membership in the document keyword set. Collect the overlapping keywords.

3. **Compute IDF values.** Before the per-candidate loop, precompute a document-frequency map across ALL candidates: for each unique keyword that appears in any candidate's extracted keywords, count how many candidates contain it. Then IDF for a term = `max(ln(total_candidates as f64 / df as f64), 0.1)` where `df` is the number of candidates containing that term. The floor of 0.1 ensures single-candidate sets still produce nonzero scores (per code review: without this, `ln(1/1) = 0` makes all scores zero). If `total_candidates` is 0, IDF is 0.0.

4. **Compute TF-IDF with boost for each overlapping keyword:**
   - `tf` = the keyword's count in `doc_term_freq`, defaulting to 1 if the keyword is present in `doc_keywords` but missing from `doc_term_freq` (this can happen because `extract_keywords` strips stopwords/short words but `term_frequency` operates on raw whitespace-split tokens).
   - `idf` = precomputed IDF for this keyword.
   - `boost` = if the keyword appears as a key in `config.boost_terms`, use that value; otherwise 1.0.
   - Contribution = `tf as f64 * idf * boost`.

5. **Sum contributions** for all overlapping keywords to get `raw_score`.

6. **Normalize to 0.0--1.0.** Compute `max_possible_score` as the sum over ALL concept keywords (not just overlapping ones) of `(max_tf * idf * max_boost)` where `max_tf` is the maximum term frequency in `doc_term_freq` and `max_boost` is the maximum boost value in `config.boost_terms` (or 1.0 if boost_terms is empty). Then `normalized = raw_score / max_possible_score`. If `max_possible_score` is 0.0, the normalized score is 0.0.

7. **Clamp** the result to the 0.0--1.0 range using `f64::clamp(0.0, 1.0)`.

8. **Build ScoredCandidate** copying all fields from the `ConceptCandidate` plus the computed `confidence_score`.

Return all scored candidates (do not filter here; filtering happens in section 05).

## Tests

All tests go in a `#[cfg(test)] mod tests` block within `matcher.rs`. These tests are pure unit tests with no database dependency.

### Test: score_candidates with high keyword overlap produces score near 1.0

Create a `ConceptCandidate` whose `name_en` and `definition_en` consist of words that are all present in the document keywords. Provide a `doc_term_freq` where those words have high frequency. Verify the resulting `confidence_score` is above 0.7 (exact value depends on IDF, which varies with candidate count -- using a single candidate means IDF = ln(1/1) = 0, so use at least 2 candidates where one has different keywords to make IDF nonzero).

```rust
#[test]
fn score_candidates_high_overlap_scores_high() {
    // Set up two candidates: one matching doc keywords, one not.
    // Verify the matching candidate scores significantly higher.
}
```

### Test: score_candidates with no keyword overlap produces score 0.0

Create a `ConceptCandidate` with keywords entirely disjoint from the document keywords. Verify `confidence_score == 0.0`.

```rust
#[test]
fn score_candidates_no_overlap_scores_zero() {
    // Candidate with definition "quantum entanglement photon"
    // Document keywords: ["risk", "security", "compliance"]
    // Assert confidence_score == 0.0
}
```

### Test: score_candidates applies boost_terms correctly

Create two identical-overlap scenarios but one uses a keyword present in `boost_terms` and the other does not. The boosted candidate should score higher.

```rust
#[test]
fn score_candidates_boost_terms_increase_score() {
    // Two candidates, same overlap count, but one overlaps on "security" (boosted 1.5x)
    // and the other overlaps on "process" (no boost).
    // Assert the boosted candidate scores higher.
}
```

### Test: score_candidates normalizes all scores to 0.0--1.0

Score a batch of candidates and assert every score is within the valid range.

```rust
#[test]
fn score_candidates_all_scores_in_valid_range() {
    // Create several candidates with varying overlap.
    // Assert all scores satisfy 0.0 <= score <= 1.0.
}
```

## Implementation Notes

- The `extract_keywords` function from `backend/src/features/analysis/tokenizer.rs` is already implemented. It lowercases, removes stopwords, filters tokens shorter than 3 characters, and deduplicates. Import it as `use super::tokenizer::extract_keywords;`.
- The `term_frequency` function from the same module counts whitespace-delimited lowercase tokens. Note that `term_frequency` does NOT filter stopwords, so there may be keywords in the overlap set that are absent from `doc_term_freq`. Default to `tf = 1` in such cases.
- The IDF precomputation should happen once before the per-candidate scoring loop to avoid redundant work. Build a `HashMap<String, usize>` mapping each keyword to the number of candidates whose extracted keywords contain it.
- The `boost_terms` defaults in `MatcherConfig` are: `security=1.5, risk=1.5, compliance=1.3, control=1.2`. These are domain terms that should carry more weight when they overlap.
- This function is synchronous (no async, no database access). All data it needs is passed in as parameters.