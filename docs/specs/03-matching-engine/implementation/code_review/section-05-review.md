# Section 05 Code Review: Classification

## High
1. **UTF-8 slicing panic** — `definition_en[..100]` panics on multi-byte chars.
2. **Gap recommendation trailing period inconsistency** — with-ref vs without-ref formatting.

## Medium
3. Cap only prioritizes zero-score gaps, not all gaps.
4. NotApplicable handled but never produced (dead code).
5. HashMap grouping gives non-deterministic output order.

## Low
6. Score 0.1 test on exact boundary without comment.
7. No test for evidence extraction.
8. Test helper named make_scored vs plan's make_scored_candidate.
