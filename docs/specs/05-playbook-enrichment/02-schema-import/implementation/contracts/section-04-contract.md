# Section 04 Contract: Integration Tests

## GOAL
Add integration tests that exercise the full guidance pipeline with real NIST AI RMF concept IDs and verify no regressions on existing API endpoints.

## CONTEXT
Sections 01-03 already have 29 unit tests with in-memory SQLite. This section adds integration-level tests using the real application setup (`create_test_app()` / `setup_pool()`) with the full ontology imported, plus API regression checks.

## CONSTRAINTS
- Use real concept IDs from NIST AI RMF ontology (nist-ai-gv-1-1, nist-ai-gv-1-2)
- Reuse `common::create_test_app()` pattern for API tests
- Tests must tolerate pre-existing data (no total row counts, use specific-value assertions)
- Clean up test guidance data after import tests

## FORMAT
- Modify: `backend/tests/guidance_tests.rs` (add integration tests)

## FAILURE CONDITIONS
- SHALL NOT duplicate tests already covered in sections 01-03
- SHALL NOT break existing tests
- SHALL NOT leave test data that pollutes other test runs
