# Section 03 Code Review Interview

## Auto-fixes Applied

### 1. FTS5 MATCH injection prevention [AUTO-FIX]
**Change:** Removed double-quote wrapping from FTS5 MATCH keywords. Added backslash to special char filter. Added FTS5_RESERVED word filter (AND, OR, NOT, NEAR). Updated sanitize test.
**Rationale:** Double-quoting introduced injection risk; bare OR-join per plan is safer.

### 2. Lowercase LIKE keywords [AUTO-FIX]
**Change:** Added `.to_lowercase()` on keyword before binding to LIKE query.
**Rationale:** `LOWER(name_en) LIKE '%Keyword%'` never matches; must be `'%keyword%'`.

### 3. Escape LIKE wildcards [AUTO-FIX]
**Change:** Added `escape_like()` helper replacing `%` and `_` with escaped versions. Added `ESCAPE '\'` to SQL LIKE clauses.
**Rationale:** `%` and `_` in keywords would become LIKE wildcards causing overly broad matches.

## Let Go

- Asymmetric error handling (FTS5 swallowed, exact-match propagated) — intentional
- HashSet iteration order — irrelevant for correctness
- ConceptRow vs deriving FromRow on ConceptCandidate — clean separation of DB/domain types
- Tests use tokio::test instead of sqlx::test — works fine for in-memory DBs
- No test for empty framework_ids — defensive guard, low risk
