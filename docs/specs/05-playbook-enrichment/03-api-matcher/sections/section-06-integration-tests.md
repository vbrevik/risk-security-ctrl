I now have comprehensive context. Let me generate the section content.

# Section 6: Integration Tests and OpenAPI Verification

## Overview

This section adds end-to-end integration tests that validate the complete guidance enrichment pipeline: API responses with guidance data, matcher scoring with guidance-enriched candidates, actionable recommendations in findings, and OpenAPI schema registration. It depends on all prior sections (1 through 5) being complete.

**Dependencies:** Sections 1, 2, 3, 4, and 5 must be implemented before these tests can pass.

## File to Create

**`/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/integration_guidance_tests.rs`**

This file contains all integration tests for the guidance enrichment feature. It is separate from existing test files (`api_tests.rs`, `guidance_tests.rs`, `analysis_tests.rs`) to keep the integration-level concerns distinct from unit and schema-level tests.

## Test Infrastructure

All tests use the shared test helpers in `backend/tests/common/mod.rs`:

- **`create_test_app()`** returns an Axum `Router` backed by a real SQLite database with migrations run and ontology data imported (including NIST AI RMF concepts and guidance data from the 02-schema-import phase).
- **`create_test_pool()`** returns a raw `SqlitePool` for direct database queries without the HTTP layer.
- HTTP tests use `tower::ServiceExt::oneshot()` to send a single request and get a response.
- Response bodies are read with `axum::body::to_bytes()` and parsed as `serde_json::Value`.

The test database is seeded from `ontology-data/` which includes NIST AI RMF concepts. After the 02-schema-import phase, concepts like `nist-ai-gv-1-1` and `nist-ai-gv-1-2` have rows in `concept_guidance`, `concept_actions`, `concept_transparency_questions`, and `concept_references`.

## Tests to Write First

```rust
// File: backend/tests/integration_guidance_tests.rs

// === API Integration Tests ===

// Test: GET /api/ontology/concepts/nist-ai-gv-1-1/relationships returns guidance object
//   - Hit the endpoint via create_test_app + oneshot
//   - Assert 200 status
//   - Parse JSON, assert json["guidance"] is an object (not null)
//   - Assert json["guidance"]["source_pdf"] is a string
//   - Assert json["guidance"]["suggested_actions"] is a non-empty array
//   - Assert json["guidance"]["transparency_questions"] is a non-empty array
//   - Assert json["guidance"]["references"] is an array

// Test: guidance response matches expected schema shape
//   - Same endpoint as above
//   - Verify each action in suggested_actions has "sort_order" (number) and "text_en" (string)
//   - Verify each question in transparency_questions has "sort_order" and "text_en"
//   - Verify each reference has "type" (not "reference_type"), "title"
//   - Verify at least one reference has type "academic" or "transparency_resource"

// Test: non-guidance concept omits guidance field entirely
//   - GET /api/ontology/concepts/iso31000-principles/relationships
//   - Assert 200 status
//   - Assert json["guidance"] is Value::Null (field absent in JSON parsed as null by serde_json)
//   - Assert json["related_concepts"] is still present (existing fields unchanged)

// Test: list endpoints do not include guidance
//   - GET /api/ontology/concepts?limit=10
//   - For each concept in the response array, assert no "guidance" key present

// === Matcher Integration Tests ===

// Test: full analysis pipeline produces guidance-enriched scores for NIST AI RMF
//   - Use create_test_pool() to get a DB handle
//   - Construct a DeterministicMatcher with topics loaded from ontology-data/topic-tags.json
//   - Call matcher.analyze() with text containing NIST AI RMF governance keywords
//     (e.g., "organizational policies for AI risk management and governance transparency")
//   - Assert result contains findings with framework_id matching NIST AI RMF
//   - Assert at least one finding has a non-empty recommendation field

// Test: analysis recommendations include suggested action text for matched concepts
//   - Same setup as above, use text that targets GV-1 action keywords
//   - Find the finding for a concept with known guidance (e.g., nist-ai-gv-1-1 or similar)
//   - Assert the recommendation text contains "Suggested Actions:" heading
//   - Assert the recommendation contains at least one action line

// Test: non-guidance frameworks produce unchanged analysis output
//   - Run analysis with text targeting ISO 31000 concepts (e.g., "risk assessment process")
//   - Assert findings exist for ISO 31000 framework
//   - Assert NO finding recommendation contains "Suggested Actions:" heading
//   - This confirms guidance enrichment only applies to concepts that have guidance data

// === OpenAPI Verification Tests ===

// Test: OpenAPI JSON includes ConceptGuidanceResponse schema definition
//   - Build the app with create_test_app()
//   - GET /api-docs/openapi.json
//   - Parse the JSON response
//   - Navigate to json["components"]["schemas"]
//   - Assert "ConceptGuidanceResponse" key exists
//   - Assert "ActionResponse" key exists
//   - Assert "QuestionResponse" key exists
//   - Assert "ReferenceResponse" key exists

// Test: concept detail endpoint documents guidance in response schema
//   - From the same OpenAPI JSON, find the path for the concept relationships endpoint
//   - Verify the response schema references ConceptGuidanceResponse or includes guidance property

// === No-Regression Tests ===

// Test: existing concept relationship fields still present in enriched response
//   - GET /api/ontology/concepts/nist-ai-gv-1-1/relationships
//   - Assert json["id"], json["name_en"], json["related_concepts"] all present
//   - Guidance is additive; nothing removed

// Test: existing matcher tests still pass (no regression)
//   - This is implicit: running `cargo test` should pass all existing tests
//   - But add one explicit test: run analysis with a document that has no AI/governance terms
//   - Assert the analysis completes without error and returns zero or low-confidence findings
```

## Implementation Details

### File Structure

The test file follows the established pattern from `api_tests.rs` and `analysis_tests.rs`:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

mod common;
use common::{create_test_app, create_test_pool};
```

For matcher integration tests that call `DeterministicMatcher` directly, also import:

```rust
use ontology_backend::features::analysis::matcher::DeterministicMatcher;
```

For loading topics (needed by the matcher), follow the same pattern as `common/mod.rs`:

```rust
let topics = ontology_backend::load_topics(
    std::path::Path::new("../ontology-data/topic-tags.json"),
);
let matcher = DeterministicMatcher::new(topics);
```

### API Test Pattern

Every API test follows this sequence:

1. `let app = create_test_app().await;`
2. Build a `Request` with the target URI
3. `let response = app.oneshot(request).await.unwrap();`
4. Assert status code
5. `let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();`
6. `let json: Value = serde_json::from_slice(&body).unwrap();`
7. Assert JSON structure

### OpenAPI Test Notes

The OpenAPI endpoint is only available when the `swagger` feature is enabled. The test should be gated:

```rust
#[cfg(feature = "swagger")]
#[tokio::test]
async fn test_openapi_includes_guidance_schemas() {
    // GET /api-docs/openapi.json and verify schema registration
}
```

If the test environment does not enable the `swagger` feature by default, annotate with `#[cfg(feature = "swagger")]` so the test is skipped in minimal builds rather than failing.

### Matcher Test: Choosing Input Text

The matcher uses keyword detection and TF-IDF scoring. For reliable test results:

- Use text that contains known keywords from NIST AI RMF GV-1 actions (governance, organizational policies, risk management, AI systems, transparency).
- Keep the text short but keyword-dense to ensure high confidence matching.
- For the non-guidance framework test, use ISO 31000 keywords (risk assessment, risk treatment, monitoring, review) without any AI/governance terms.

### Verifying "Suggested Actions:" in Recommendations

Section 5 appends a "Suggested Actions:" block to recommendations for concepts that have `actions_text`. The integration test should:

1. Find any finding whose `concept_id` starts with `nist-ai-` (NIST AI RMF concepts are the ones with guidance data).
2. Check `finding.recommendation` contains the substring `"Suggested Actions:"`.
3. Optionally verify action lines are formatted as `"- {action text} ({concept_code}, Action {n})"`.

### Test Data Assumptions

These tests rely on the ontology import having populated guidance data. The `create_test_pool()` function in `common/mod.rs` already imports from `ontology-data/` when the concepts table is empty. The 02-schema-import phase added guidance import to this pipeline. If guidance data is imported separately (not via `import_all_ontologies`), the test setup may need to explicitly call the guidance import function. Verify by checking that `create_test_pool()` results in non-empty `concept_guidance` table rows for `nist-ai-gv-*` concepts.

### STIG Compliance

These tests exercise the security controls from prior sections:

- **V-222607**: All queries in the tested code paths use parameterized `.bind()` calls (verified by Section 2 and 3 implementations).
- **V-222602**: FTS5 MATCH inputs are sanitized via `sanitize_keyword()` (verified by Section 3).
- **V-222585**: Missing guidance data returns `None`/empty arrays, not errors (tested by the non-guidance concept test).
- **V-222610**: The tests verify that database errors do not leak details (the error handling is tested implicitly by requesting non-existent concepts and verifying generic 404 responses).

## Relevant Files

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/integration_guidance_tests.rs` -- new file to create
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/common/mod.rs` -- shared test helpers (read-only, no changes needed)
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/api_tests.rs` -- existing API tests for reference on patterns
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/analysis_tests.rs` -- existing analysis tests for matcher invocation pattern
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/guidance_tests.rs` -- existing guidance schema tests (unit level)
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs` -- OpenAPI `ApiDoc` struct where new schemas are registered (lines 38-44)
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs` -- `DeterministicMatcher::analyze()` entry point
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/ontology/routes.rs` -- `get_concept_relationships` handler under test

## Implementation Notes (Post-Build)

**Files created:** `backend/tests/integration_guidance_tests.rs` — 8 integration tests

**Deviations from plan:**
- OpenAPI tests omitted: the swagger feature is not enabled in test builds, and OpenAPI schemas were verified via registration in Section 2
- Matcher actions test uses `retrieve_candidates` directly instead of full `analyze()` pipeline, since `max_findings_per_framework=50` caps gap candidates. Actions-in-recommendation formatting is covered by Section 05 unit tests.
- Tests seed guidance data idempotently (INSERT OR IGNORE) since no `*-guidance.json` files exist in `ontology-data/`
- Used `nist-ai-gv-4-1` and `nist-ai-gv-6-2` as test concepts to avoid collision with guidance_tests.rs which cleans `nist-ai-gv-1-*`

**All tests pass (full suite). 8 integration tests covering API, matcher, and no-regression.**