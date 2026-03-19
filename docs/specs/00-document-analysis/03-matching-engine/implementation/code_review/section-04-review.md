# Section 04 Code Review: TF-IDF Scoring

## High

1. **Normalization max_boost dilution** — denominator uses max_boost for all keywords, penalizing candidates with many unboosted keywords. Per plan specification, not a bug.

2. **Single-candidate IDF=0** — When only 1 candidate exists, all IDFs are ln(1/1)=0, making all scores 0.0 regardless of overlap. Real production concern.

## Medium

3. No test for single-candidate IDF=0 edge case.
4. No test for empty candidates input.
5. No test for all-zero scores scenario.

## Low

6. `no_overlap` test passes for wrong reason (IDF=0, not disjointness).
7. extract_keywords rebuilds stopword set per call (perf, not correctness).
8. Composition deviation from plan is clean.
