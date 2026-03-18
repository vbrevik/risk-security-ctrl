# Section 03 Code Review: FTS5 Candidate Retrieval

## Critical

1. **FTS5 MATCH injection** — Double-quote wrapping doesn't protect against FTS5 operators (NOT, NEAR, AND, OR) surviving in keywords. Plan says bare OR-join.

2. **LIKE keyword not lowercased** — `LOWER(name_en) LIKE '%keyword%'` fails when keyword has uppercase.

## Important

3. **LIKE wildcards not escaped** — `%` and `_` in keywords become LIKE wildcards.

4. **Asymmetric error handling** — FTS5 errors swallowed, exact-match errors propagate.

5. **HashSet iteration non-deterministic** — Gap candidate JSON order varies.

## Minor

6. tokio::test vs sqlx::test — manual pool setup works for in-memory DB.
7. No test for empty framework_ids early return.
8. No test for FTS5 graceful degradation.

## Nitpick

9. ConceptRow duplicates ConceptCandidate — could derive sqlx::FromRow on ConceptCandidate.
10. unwrap_or_else on infallible serde_json::to_string.
