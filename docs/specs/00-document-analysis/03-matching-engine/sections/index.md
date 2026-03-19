<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-config-types
section-02-framework-detection
section-03-fts5-retrieval
section-04-scoring
section-05-classification
section-06-matcher-impl
section-07-wiring
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-config-types | - | 02, 03, 04, 05, 06 | Yes |
| section-02-framework-detection | 01 | 06 | Yes |
| section-03-fts5-retrieval | 01 | 04, 06 | Yes |
| section-04-scoring | 01, 03 | 05, 06 | No |
| section-05-classification | 04 | 06 | No |
| section-06-matcher-impl | 02, 03, 04, 05 | 07 | No |
| section-07-wiring | 06 | - | No |

## Execution Order

1. section-01-config-types (no deps)
2. section-02-framework-detection, section-03-fts5-retrieval (parallel after 01)
3. section-04-scoring (after 03)
4. section-05-classification (after 04)
5. section-06-matcher-impl (after all)
6. section-07-wiring (after 06)

## Section Summaries

### section-01-config-types
MatcherConfig struct with JSON parsing, defaults, and Topic type. ConceptCandidate and ScoredCandidate structs.

### section-02-framework-detection
detect_frameworks() using keyword→topic→framework matching with direct name matching.

### section-03-fts5-retrieval
retrieve_candidates() using FTS5 MATCH queries, exact matches, and gap candidate loading.

### section-04-scoring
score_candidates() implementing TF-IDF-like scoring with boost terms and normalization.

### section-05-classification
classify_findings() with threshold-based classification, priority ranking, recommendation generation, evidence extraction.

### section-06-matcher-impl
DeterministicMatcher struct implementing MatchingEngine trait, orchestrating all stages.

### section-07-wiring
Update analysis/mod.rs to export matcher module.
