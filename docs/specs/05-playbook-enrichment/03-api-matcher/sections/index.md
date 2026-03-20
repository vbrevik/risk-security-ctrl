<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-guidance-response-types
section-02-concept-detail-handler
section-03-candidate-enrichment
section-04-tfidf-scoring
section-05-actionable-recommendations
section-06-integration-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-guidance-response-types | - | 02, 06 | Yes |
| section-02-concept-detail-handler | 01 | 06 | No |
| section-03-candidate-enrichment | - | 04, 05 | Yes |
| section-04-tfidf-scoring | 03 | 05, 06 | No |
| section-05-actionable-recommendations | 03, 04 | 06 | No |
| section-06-integration-tests | 01, 02, 03, 04, 05 | - | No |

## Execution Order

1. section-01-guidance-response-types, section-03-candidate-enrichment (parallel, no dependencies)
2. section-02-concept-detail-handler (after 01)
3. section-04-tfidf-scoring (after 03)
4. section-05-actionable-recommendations (after 03, 04)
5. section-06-integration-tests (final, after all)

## Section Summaries

### section-01-guidance-response-types
Add ConceptGuidanceResponse, ActionResponse, QuestionResponse, ReferenceResponse structs to models.rs. Extend ConceptWithRelationships with optional guidance field.

### section-02-concept-detail-handler
Modify get_concept_relationships handler to query guidance tables and assemble nested response. Update OpenAPI annotations.

### section-03-candidate-enrichment
Add about_en and actions_text fields to ConceptCandidate. Add guidance FTS union query with custom BM25 weights. Update retrieval to LEFT JOIN guidance tables.

### section-04-tfidf-scoring
Modify score_candidates to include about_en and actions_text in keyword extraction for TF-IDF computation.

### section-05-actionable-recommendations
Modify recommendation generation to include suggested actions from pre-fetched actions_text in finding text.

### section-06-integration-tests
End-to-end API tests, matcher integration tests, OpenAPI schema verification, no-regression checks.
