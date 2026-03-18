Now I have all the context needed. Let me produce the section content.

# Section 5: Gap Classification and Findings

## Overview

This section implements the `classify_findings()` function in `backend/src/features/analysis/matcher.rs`. It takes scored candidates from the scoring stage (section 04) and produces classified findings with threshold-based types, priority rankings, recommendation text, and evidence extraction.

**Depends on:** section-01-config-types (for `MatcherConfig`, `ScoredCandidate`), section-04-scoring (produces the `Vec<ScoredCandidate>` input)

**Blocks:** section-06-matcher-impl (consumes `classify_findings()` as stage 4 of the pipeline)

## Key Types (from dependencies)

These types are defined in earlier sections and consumed here. They are listed for reference only.

`MatcherConfig` (section 01) provides the threshold fields used by classification:
- `min_confidence_threshold: f64` -- default 0.1, minimum score to include a finding
- `addressed_threshold: f64` -- default 0.6, score at or above means "addressed"
- `partial_threshold: f64` -- default 0.3, score at or above means "partially addressed"
- `max_findings_per_framework: usize` -- default 50, cap per framework
- `include_addressed_findings: bool` -- default true

`ScoredCandidate` (section 01/04) carries all `ConceptCandidate` fields plus `confidence_score: f64`. The relevant fields consumed here are: `id`, `framework_id`, `parent_id` (Option), `name_en`, `definition_en`, `source_reference` (Option), `confidence_score`.

`NewFinding` is defined in `backend/src/features/analysis/engine.rs` with fields: `concept_id`, `framework_id`, `finding_type` (FindingType), `confidence_score`, `evidence_text` (Option), `recommendation` (Option), `priority` (i32).

`FindingType` is defined in `backend/src/features/analysis/models.rs` with variants: `Addressed`, `PartiallyAddressed`, `Gap`, `NotApplicable`.

The tokenizer utility `sentence_split()` from `backend/src/features/analysis/tokenizer.rs` splits text into sentences. The `extract_keywords()` function extracts lowercase, stopword-filtered, deduplicated keywords from text.

## File to Modify

`backend/src/features/analysis/matcher.rs`

## Tests (write first)

All tests go in the `#[cfg(test)] mod tests` block within `matcher.rs`. These are pure unit tests with no database dependency.

### Test: classify with score 0.8 produces Addressed

Create a single `ScoredCandidate` with `confidence_score: 0.8`, `parent_id: None`. Call `classify_findings()` with default config and an empty document text string. Assert the resulting `NewFinding` has `finding_type == FindingType::Addressed`.

### Test: classify with score 0.4 produces PartiallyAddressed

Same setup but `confidence_score: 0.4`. Assert `finding_type == FindingType::PartiallyAddressed`.

### Test: classify with score 0.1 produces Gap

Same setup but `confidence_score: 0.1`. Assert `finding_type == FindingType::Gap`.

### Test: classify with score 0.0 produces Gap (zero-match gap candidate)

Same setup but `confidence_score: 0.0`. This represents a concept from a detected framework that had zero keyword overlap. Assert it is still included as a Gap finding (zero-score gaps are always included even though 0.0 is below `min_confidence_threshold`).

### Test: priority P1 for root concept gap, P2 for child gap

Create two `ScoredCandidate` entries both with `confidence_score: 0.0` (Gap):
- One with `parent_id: None` (root concept)
- One with `parent_id: Some("parent-1".to_string())` (child concept)

Assert the root gap gets `priority: 1` and the child gap gets `priority: 2`.

### Test: recommendation text contains concept name and source_reference

Create a `ScoredCandidate` with `name_en: "Access Control"`, `source_reference: Some("NIST SP 800-53 AC-1")`, `confidence_score: 0.0` (Gap). Assert the recommendation string contains both "Access Control" and "NIST SP 800-53 AC-1".

### Test: max_findings_per_framework caps output

Create 60 `ScoredCandidate` entries all in the same framework. Use default config (`max_findings_per_framework: 50`). Assert the output vector has at most 50 entries.

### Test: include_addressed_findings=false excludes Addressed findings

Create candidates with scores spanning all three classification ranges (e.g., 0.8, 0.4, 0.1). Set `config.include_addressed_findings = false`. Assert no findings have `finding_type == FindingType::Addressed`.

### Helper: make_scored_candidate

Write a helper function `make_scored_candidate(id, framework_id, parent_id, name, definition, source_ref, score)` that constructs a `ScoredCandidate` for use in tests. This avoids boilerplate in each test.

## Implementation: classify_findings function

### Signature

```rust
pub fn classify_findings(
    scored_candidates: Vec<ScoredCandidate>,
    config: &MatcherConfig,
    document_text: &str,
) -> Vec<NewFinding>
```

The `document_text` parameter is needed for evidence extraction (finding the best-matching sentence).

### Algorithm

The function proceeds through seven steps in order:

**Step 1 -- Classify by threshold.** For each `ScoredCandidate`, determine its `FindingType`:
- `score >= config.addressed_threshold` (default 0.6) produces `FindingType::Addressed`
- `score >= config.partial_threshold` (default 0.3) produces `FindingType::PartiallyAddressed`
- `score < config.partial_threshold` produces `FindingType::Gap`

**Step 2 -- Filter by minimum confidence.** Remove candidates with `confidence_score < config.min_confidence_threshold` (default 0.1), with one exception: candidates with `confidence_score == 0.0` that are classified as Gap are always retained. These represent concepts from detected frameworks that had zero keyword overlap and represent genuine coverage gaps.

**Step 3 -- Filter addressed findings.** If `config.include_addressed_findings` is `false`, drop all findings classified as `Addressed`.

**Step 4 -- Cap per framework.** Group remaining findings by `framework_id`. For each group, sort by `confidence_score` descending and take at most `config.max_findings_per_framework` entries. Gap findings (score 0.0) should be prioritized in the kept set -- sort gaps first within the cap logic so they are not dropped in favor of higher-scoring partial matches.

**Step 5 -- Assign priority.** Priority is based on concept depth (determined by `parent_id`) and finding type:

| parent_id | FindingType | Priority |
|-----------|-------------|----------|
| None (root) | Gap | 1 |
| None (root) | PartiallyAddressed | 2 |
| Some (child, one level) | Gap | 2 |
| Some (child, one level) | PartiallyAddressed | 3 |
| deeper (grandchild+) | Gap | 3 |
| deeper (grandchild+) | PartiallyAddressed | 4 |
| any | Addressed | 4 |

Since concept depth beyond one level is not directly available from `parent_id` alone (the field is just an Option), treat all candidates with `parent_id: Some(_)` as "one level deep" for this implementation. To determine grandchild+ depth would require a DB lookup which is not available in this pure function. The practical result is that root concepts (parent_id=None) get higher priority than child concepts (parent_id=Some).

Simplified priority assignment:
- Addressed findings: always priority 4
- Root concept (`parent_id: None`): Gap=1, Partial=2
- Child concept (`parent_id: Some(_)`): Gap=2, Partial=3

**Step 6 -- Generate recommendation text.** Based on `finding_type`:

- **Addressed:** `"Document adequately covers {name_en}. Reference: {source_reference}"` -- if `source_reference` is None, omit the reference clause.
- **PartiallyAddressed:** `"Document partially addresses {name_en}. Consider expanding coverage of {definition_excerpt}. Reference: {source_reference}"` -- `definition_excerpt` is the first 100 characters of `definition_en`. If `source_reference` is None, omit.
- **Gap:** `"Document does not address {name_en}: {definition_excerpt}. Recommended action: review and implement controls per {source_reference}"` -- if `source_reference` is None, end with "review and implement appropriate controls."

**Step 7 -- Extract evidence_text.** For non-Gap findings only (Addressed and PartiallyAddressed), find the sentence from the document that best matches the concept. Use `sentence_split(document_text)` from the tokenizer to get sentences, then `extract_keywords()` on each sentence and on the concept's `name_en + definition_en`. The sentence with the highest keyword overlap count is selected as `evidence_text`. If no sentence has any overlap, `evidence_text` is None. For Gap findings, `evidence_text` is always None (there is no matching content in the document).

### Constructing NewFinding

For each surviving candidate, construct a `NewFinding`:

```rust
NewFinding {
    concept_id: candidate.id.clone(),
    framework_id: candidate.framework_id.clone(),
    finding_type,    // from step 1
    confidence_score: candidate.confidence_score,
    evidence_text,   // from step 7
    recommendation,  // from step 6
    priority,        // from step 5
}
```

### Performance Consideration

Evidence extraction (step 7) calls `sentence_split` and `extract_keywords` per candidate, which could be expensive for large documents with many candidates. To optimize, split the document into sentences once before the loop and pre-compute keywords for each sentence. The concept keyword extraction is per-candidate and cannot be cached.

```rust
// Pre-compute once before the loop
let sentences = sentence_split(document_text);
let sentence_keyword_sets: Vec<(String, HashSet<String>)> = sentences
    .iter()
    .map(|s| {
        let kws: HashSet<String> = extract_keywords(s).into_iter().collect();
        (s.clone(), kws)
    })
    .collect();
```

Then for each non-gap candidate, compute concept keywords and find the sentence with maximum intersection size.

## Summary Checklist

1. Write the test helper `make_scored_candidate()`
2. Write all 8 test stubs listed above
3. Implement `classify_findings()` with the 7-step algorithm
4. Run `cargo test` from `/Users/vidarbrevik/projects/risk-security-ctrl/backend/` to verify all tests pass
5. Verify `cargo clippy` has no warnings in the new code