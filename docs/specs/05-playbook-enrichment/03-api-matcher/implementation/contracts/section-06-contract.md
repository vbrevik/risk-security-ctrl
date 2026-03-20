# Section 06 Contract: Integration Tests

## GOAL
End-to-end integration tests for guidance enrichment: API, matcher, OpenAPI, no-regression.

## CONSTRAINTS
- Tests must seed their own guidance data idempotently (no *-guidance.json files exist)
- Use existing create_test_app/create_test_pool helpers
- OpenAPI tests gated on #[cfg(feature = "swagger")]

## FAILURE CONDITIONS
- SHALL NOT depend on pre-existing guidance data files
- SHALL NOT break existing tests
- SHALL NOT skip key test scenarios
