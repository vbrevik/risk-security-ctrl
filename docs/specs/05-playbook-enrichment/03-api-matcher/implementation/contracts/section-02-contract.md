# Section 02 Contract: Concept Detail Handler Extension

## GOAL
Extend get_concept_relationships handler to query guidance tables and return nested guidance data. Register new schemas in OpenAPI.

## CONTEXT
Section 2 of 6. Depends on Section 1 (response types). The handler already fetches concept + relationships; we add 4 guidance queries run concurrently with tokio::try_join!.

## CONSTRAINTS
- All SQL uses parameterized binds via sqlx::query().bind() (STIG V-222607)
- Use tokio::try_join! for concurrent guidance queries
- Missing guidance returns None, not error (STIG V-222585)
- Error responses are generic 500 (STIG V-222610)
- No .unwrap() on user input paths (STIG V-222609)
- Register 4 new schemas in main.rs OpenAPI components

## FORMAT
Modify: `backend/src/features/ontology/routes.rs`, `backend/src/main.rs`
Tests in: `backend/tests/api_tests.rs`

## FAILURE CONDITIONS
- SHALL NOT use string interpolation in SQL
- SHALL NOT panic on missing guidance data
- SHALL NOT leak database error details in responses
- SHALL NOT break existing relationship data in response
- SHALL NOT skip tests
