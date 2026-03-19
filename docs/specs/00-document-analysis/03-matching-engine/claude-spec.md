# Combined Spec: 03-matching-engine

## What We're Building
`DeterministicMatcher` implementing the `MatchingEngine` trait. Two-stage pipeline: FTS5 candidate retrieval → keyword overlap scoring. Plus framework detection, gap classification, priority ranking, and recommendation generation.

## Key Decisions from Interview
- **All concepts** from detected frameworks get findings (comprehensive, not just top-level)
- **Topics passed as parameter** to matcher (not read from disk) for testability
- **FTS5 MATCH** for candidate retrieval (first usage in codebase — existing search uses LIKE)

## Architecture
```
DocumentParser output (text) → DeterministicMatcher.analyze()
  1. Framework detection (keywords → topics → frameworks)
  2. FTS5 candidate retrieval per framework
  3. TF-IDF scoring of candidates
  4. Gap classification (thresholds from prompt template)
  5. Reference validation (concept_id exists in DB)
  6. Priority ranking (concept depth: root=P1, child=P2, etc.)
  7. Recommendation generation
```
