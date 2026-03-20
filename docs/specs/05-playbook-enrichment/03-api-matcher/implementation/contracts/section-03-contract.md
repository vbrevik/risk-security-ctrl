# Section 03 Contract: ConceptCandidate Enrichment

## GOAL
Extend ConceptCandidate with about_en and actions_text fields, add guidance FTS query, modify retrieval queries with LEFT JOIN guidance data.

## CONSTRAINTS
- All SQL uses parameterized binds (STIG V-222607)
- FTS5 MATCH uses sanitize_fts_keywords output (STIG V-222602)
- Gap candidates must NOT LEFT JOIN guidance tables (optimization)
- Guidance FTS failure must be caught and logged, not propagated (V-222585)
- GROUP_CONCAT uses nested SELECT for ORDER BY guarantee
- First-occurrence dedup via seen_ids HashSet

## FORMAT
Modify: `backend/src/features/analysis/matcher.rs`

## FAILURE CONDITIONS
- SHALL NOT break existing ConceptCandidate construction sites
- SHALL NOT use string interpolation in SQL
- SHALL NOT propagate guidance FTS errors to caller
- SHALL NOT skip tests
- SHALL NOT LEFT JOIN guidance on gap candidates
