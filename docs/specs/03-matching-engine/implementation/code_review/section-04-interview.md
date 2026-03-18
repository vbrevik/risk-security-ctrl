# Section 04 Code Review Interview

## Fixes Applied

### 1. IDF floor for single-candidate edge case [ASKED USER]
**Change:** Added `raw_idf.max(0.1)` to IDF computation, ensuring single-candidate sets produce nonzero scores.
**Rationale:** Without this, a document matching only 1 concept would always score 0.0 and be silently filtered.

### 2. Fix no_overlap test [AUTO-FIX]
**Change:** Added a second candidate to the no_overlap test so IDF is nonzero. Test now validates true keyword disjointness.
**Rationale:** Original test passed because IDF=0, not because keywords were disjoint.

## Let Go

- Normalization max_boost dilution — per plan specification
- Edge case tests for empty/all-zero — not required by plan
- extract_keywords stopword rebuild perf — not this section's scope
