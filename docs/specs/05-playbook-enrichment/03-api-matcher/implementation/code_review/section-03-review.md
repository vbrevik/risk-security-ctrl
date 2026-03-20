# Section 03 Code Review

## All failure conditions pass.

## Medium
**M3:** BM25 weight order needs inline comment mapping to columns.
**M4:** test_gap_candidates_no_guidance_fields could silently pass if filter yields zero.
**M5:** Dedup test relies on FTS5 stemming — may not work with unicode61 tokenizer.

## Low
**L6:** Unused transparency_questions/references tables in test setup — schema completeness.
**L7:** No test for guidance FTS failure graceful fallback.
**L8:** GROUP_CONCAT edge cases — non-issue due to NOT NULL constraint.
