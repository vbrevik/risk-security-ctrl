I now have all the context needed. Let me produce the section content.

# Section 4: TF-IDF Scoring Enhancement

## Overview

This section modifies the `score_candidates()` function in `backend/src/features/analysis/matcher.rs` to include guidance text (`about_en` and `actions_text`) in the keyword extraction and TF-IDF computation. Currently, keyword profiles are built from only `name_en + definition_en`. After this change, concepts with guidance data will have richer keyword profiles that improve matching accuracy.

**Depends on:** Section 3 (ConceptCandidate Enrichment) -- the `about_en: Option<String>` and `actions_text: Option<String>` fields must already exist on `ConceptCandidate`.

**Blocks:** Section 5 (Actionable Recommendations), Section 6 (Integration Tests).

---

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs`

---

## Tests (Write First)

All tests go in the existing `#[cfg(test)] mod tests` block in `matcher.rs`. The existing `make_candidate` helper must be updated in Section 3 to include `about_en` and `actions_text` fields. These tests assume that update is already in place.

```rust
// Helper for Section 4 tests: creates a candidate with guidance fields populated
// fn make_candidate_with_guidance(id, name, definition, about_en, actions_text) -> ConceptCandidate

// Test: concept with guidance text scores higher when query keywords match actions
// Create two candidates with identical name_en and definition_en.
// One has actions_text containing keywords that overlap with doc_keywords; the other has None.
// The candidate with actions_text should score higher because it has more overlapping keywords.
#[test]
fn score_candidates_guidance_text_boosts_score() { ... }

// Test: concept without guidance scores same as before (no regression)
// Create a candidate with about_en=None and actions_text=None.
// Verify it produces the same score as the pre-enrichment scoring would have.
#[test]
fn score_candidates_no_guidance_no_regression() { ... }

// Test: scoring uses all four text fields: name_en + definition_en + about_en + actions_text
// Create a candidate where a query keyword appears ONLY in about_en, and another where it
// appears ONLY in actions_text. Both should produce non-zero scores for those keywords.
#[test]
fn score_candidates_uses_all_four_text_fields() { ... }

// Test: normalization still produces scores in [0.0, 1.0] range with enriched text
// Create candidates with very long actions_text (many keywords). Verify all scores clamp
// within the valid range.
#[test]
fn score_candidates_enriched_scores_in_valid_range() { ... }
```

### Test Design Rationale

The key insight is that the change is purely additive to keyword extraction. The core test (`score_candidates_guidance_text_boosts_score`) demonstrates this by using two otherwise-identical candidates where the only difference is guidance text containing query-matching keywords. The no-regression test ensures that `None` guidance fields produce identical behavior to the original implementation.

---

## Implementation Details

### Change Location

The modification is in the `score_candidates()` function, specifically the keyword extraction loop at lines 396-402 of the current file. This is the `candidate_keywords` precomputation step.

### Current Code (lines 396-402)

```rust
let candidate_keywords: Vec<Vec<String>> = candidates
    .iter()
    .map(|c| {
        let text = format!("{} {}", c.name_en, c.definition_en);
        extract_keywords(&text)
    })
    .collect();
```

### New Code

Change the text construction inside the `.map()` closure to concatenate all four text sources:

```rust
let candidate_keywords: Vec<Vec<String>> = candidates
    .iter()
    .map(|c| {
        let text = format!(
            "{} {} {} {}",
            c.name_en,
            c.definition_en,
            c.about_en.as_deref().unwrap_or(""),
            c.actions_text.as_deref().unwrap_or(""),
        );
        extract_keywords(&text)
    })
    .collect();
```

### What Does NOT Change

- The IDF computation loop (lines 404-412) -- unchanged, it operates on the already-expanded `candidate_keywords` vec.
- The boost terms map and max_boost calculation -- unchanged.
- The per-candidate scoring loop (lines 440-481) -- unchanged, it uses `candidate_keywords` which now contains the enriched keywords.
- The normalization formula (`raw_score / max_possible`, clamped to [0.0, 1.0]) -- unchanged.
- The function signature -- unchanged.

### Why This Works

The `extract_keywords()` function (in `tokenizer.rs`) lowercases text, splits on whitespace/punctuation, removes stopwords, filters short tokens, and deduplicates. Passing in additional text simply produces more unique keywords for each candidate. The TF-IDF math naturally handles this:

- **More keywords per candidate** means more potential overlap with document keywords, increasing `raw_score`.
- **IDF adjusts automatically** because `df` counts are computed from the expanded keyword sets. If a guidance keyword appears in many candidates, its IDF drops, preventing it from dominating scores.
- **Normalization adjusts automatically** because `max_possible` sums over ALL concept keywords (including the new ones). A candidate with many guidance keywords has a proportionally higher denominator, preventing inflated scores.

Concepts without guidance data (`about_en=None`, `actions_text=None`) concatenate empty strings, producing zero additional keywords. Their scoring behavior is identical to the pre-change implementation.

---

## Updating Existing Test Fixtures

The existing test helper `make_candidate` (line 1037) will have been updated in Section 3 to include the two new `Option` fields defaulting to `None`. The four existing scoring tests (`score_candidates_high_overlap_scores_high`, `score_candidates_no_overlap_scores_zero`, `score_candidates_boost_terms_increase_score`, `score_candidates_all_scores_in_valid_range`) should continue to pass without modification because their candidates have `None` for both guidance fields, which produces identical keyword extraction to the original code.

---

## Compliance Notes

No new SQL queries are introduced in this section. The change is purely in-memory keyword extraction logic. No user input is processed beyond what already flows through `score_candidates()`. Relevant STIG controls are satisfied by the existing architecture:

- **V-222609 (CAT I)**: No `unwrap()` on user input -- `unwrap_or("")` handles `None` safely.
- **V-222585 (CAT I)**: Missing guidance data degrades gracefully to empty string, not an error.

## Implementation Notes (Post-Build)

**Files modified:** `backend/src/features/analysis/matcher.rs` — 1 line changed in score_candidates() keyword extraction, 4 new tests + make_candidate_with_guidance helper.

**All 46 matcher tests pass. No deviations from plan.**