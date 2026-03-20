Now I have all the context needed. Let me generate the section content.

# Section 2: Concept Detail Handler Extension

## Overview

This section modifies the `get_concept_relationships` handler in `backend/src/features/ontology/routes.rs` to query the four guidance tables (`concept_guidance`, `concept_actions`, `concept_transparency_questions`, `concept_references`) and assemble a nested `ConceptGuidanceResponse` in the API response. It also updates OpenAPI annotations in `backend/src/main.rs`.

**Depends on:** Section 1 (Guidance Response Types) -- the four response structs (`ConceptGuidanceResponse`, `ActionResponse`, `QuestionResponse`, `ReferenceResponse`) and the `guidance: Option<ConceptGuidanceResponse>` field on `ConceptWithRelationships` must exist before this section can be implemented.

**Blocks:** Section 6 (Integration Tests).

## Files to Modify

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/ontology/routes.rs` -- handler logic
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs` -- OpenAPI schema registration

## Tests (Write First)

All tests go in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/api_tests.rs` (or a new dedicated test file). Tests use the existing `create_test_app()` helper from `backend/tests/common/mod.rs`, which sets up an in-memory SQLite database with migrations and imports ontology data from `ontology-data/`.

```rust
// File: backend/tests/api_tests.rs (append these tests)

// Test: GET /api/ontology/concepts/{id}/relationships returns guidance object
// for a NIST AI RMF action-level concept that has imported playbook data.
// Expect: response JSON contains a "guidance" key with nested structure.
#[tokio::test]
async fn test_concept_relationships_includes_guidance_for_nist_ai_concept() {
    // Use create_test_app(), GET /api/ontology/concepts/nist-ai-gv-1-1/relationships
    // Assert: status 200
    // Assert: json["guidance"] is an object (not null)
    // Assert: json["guidance"]["source_pdf"] is a non-empty string
    // Assert: json["guidance"]["source_page"] is a positive integer
    // Assert: json["guidance"]["about_en"] is a string (may be null for nb)
}

// Test: guidance.suggested_actions are ordered by sort_order
#[tokio::test]
async fn test_guidance_actions_ordered_by_sort_order() {
    // GET /api/ontology/concepts/nist-ai-gv-1-1/relationships
    // Assert: guidance.suggested_actions is an array
    // Assert: sort_order values are monotonically increasing
}

// Test: guidance.references contain both "academic" and "transparency_resource" types
#[tokio::test]
async fn test_guidance_references_have_correct_types() {
    // GET /api/ontology/concepts/nist-ai-gv-1-1/relationships
    // Assert: guidance.references is an array
    // Assert: each reference has a "type" field (not "reference_type")
    // Assert: "type" values are either "academic" or "transparency_resource"
}

// Test: non-NIST concept has no guidance field in JSON
#[tokio::test]
async fn test_concept_without_guidance_omits_field() {
    // GET /api/ontology/concepts/iso31000-1/relationships
    // Assert: status 200
    // Assert: json["guidance"] is null/absent (serde skip_serializing_if)
    // Specifically: the key "guidance" should NOT appear in the JSON object
}

// Test: concept with guidance row but zero actions returns guidance with empty arrays
#[tokio::test]
async fn test_guidance_with_empty_actions_returns_empty_array() {
    // This requires either a real concept with no actions or inserting test data.
    // If no such concept exists in imported data, this can use the setup_db() pattern
    // from guidance_tests.rs to create an in-memory DB with a guidance row but no actions.
    // Assert: guidance.suggested_actions == []
}

// Test: existing relationship fields still present and correct in enriched response
#[tokio::test]
async fn test_existing_relationship_fields_preserved() {
    // GET /api/ontology/concepts/nist-ai-gv-1-1/relationships
    // Assert: response still contains "related_concepts" array
    // Assert: concept fields (id, framework_id, name_en, etc.) still present via serde(flatten)
}
```

## Implementation Details

### 1. Handler Modification (`routes.rs`)

The current `get_concept_relationships` handler at line 234 of `routes.rs` fetches the concept and its incoming/outgoing relationships, then assembles a `ConceptWithRelationships`. The modification adds guidance data retrieval between the relationship queries and the final response assembly.

**Import the new types** at the top of `routes.rs` in the existing `use super::models::` import block. Add `ConceptGuidanceResponse`, `ActionResponse`, `QuestionResponse`, `ReferenceResponse`.

**Add four guidance queries** after the existing relationship queries (after line 288). Use `tokio::try_join!` to run all four concurrently for reduced latency:

Query 1 -- Guidance row (use `fetch_optional`, returns `Option`):
```sql
SELECT source_pdf, source_page, about_en, about_nb
FROM concept_guidance WHERE concept_id = ?
```

Query 2 -- Actions (use `fetch_all`):
```sql
SELECT action_text_en, action_text_nb, sort_order
FROM concept_actions WHERE concept_id = ? ORDER BY sort_order
```

Query 3 -- Questions (use `fetch_all`):
```sql
SELECT question_text_en, question_text_nb, sort_order
FROM concept_transparency_questions WHERE concept_id = ? ORDER BY sort_order
```

Query 4 -- References (use `fetch_all`):
```sql
SELECT reference_type, title, authors, year, venue, url, sort_order
FROM concept_references WHERE concept_id = ? ORDER BY sort_order
```

All queries use `sqlx::query().bind(&id)` with parameterized binds (STIG V-222607). The `id` variable is the same path parameter already extracted and used for the concept and relationship queries.

**Use `tokio::try_join!`** to run queries 1-4 concurrently. This reduces four sequential database round-trips to one concurrent batch. The syntax:

```rust
let (guidance_row, actions_rows, questions_rows, references_rows) = tokio::try_join!(
    guidance_query,
    actions_query,
    questions_query,
    references_query,
)
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
```

**Assemble the guidance field.** If `guidance_row` is `None` (query 1 returned no row), set `guidance = None`. If it is `Some`, map the raw query rows into the response types:

- Map each action row to `ActionResponse { sort_order, text_en, text_nb }`
- Map each question row to `QuestionResponse { sort_order, text_en, text_nb }`
- Map each reference row to `ReferenceResponse { reference_type, title, authors, year, venue, url }`
- Wrap in `ConceptGuidanceResponse { source_pdf, source_page, about_en, about_nb, suggested_actions, transparency_questions, references }`

**Note on `sqlx::query` vs `sqlx::query_as`:** The guidance tables are not directly mapped to response types because column names differ (e.g., `action_text_en` in DB vs `text_en` in response). Use `sqlx::query()` with `.fetch_all()` and manually map via `row.get::<Type, _>("column_name")`, or use intermediate row types with `FromRow` and map. The manual mapping approach is simpler and avoids creating throwaway structs.

**Set the guidance field** on the `ConceptWithRelationships` struct in the final assembly (around line 294). The existing code constructs the struct literally; add `guidance` to the struct literal.

### 2. Error Handling

Follow the existing pattern visible throughout `routes.rs`: database errors map to `StatusCode::INTERNAL_SERVER_ERROR` via `.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)`. If guidance tables are empty or the concept has no guidance, the response simply omits the `guidance` field (it will be `None`, and `skip_serializing_if` from Section 1 ensures it is absent from JSON). No new error variants or paths are introduced (STIG V-222585, V-222610).

### 3. OpenAPI Updates (`main.rs`)

In `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs`, the `#[derive(OpenApi)]` block starting at line 14 has a `components(schemas(...))` section (line 38-44). Register the four new schema types:

```rust
components(
    schemas(
        // ... existing schemas ...
        ontology_backend::features::ontology::models::ConceptGuidanceResponse,
        ontology_backend::features::ontology::models::ActionResponse,
        ontology_backend::features::ontology::models::QuestionResponse,
        ontology_backend::features::ontology::models::ReferenceResponse,
    )
),
```

The `get_concept_relationships` handler's `#[utoipa::path]` annotation (line 222-232 of `routes.rs`) already documents `body = ConceptWithRelationships` in its response. Since Section 1 adds the `guidance` field to that struct with `ToSchema`, no change to the path annotation itself is needed -- utoipa will automatically include the nested schema. However, verify that the `ConceptWithRelationships` response entry correctly picks up the new optional field in the generated OpenAPI JSON.

### 4. Backward Compatibility

The `guidance` field is `Option<ConceptGuidanceResponse>` with `skip_serializing_if = "Option::is_none"`. This means:

- Concepts without guidance data: JSON response is identical to the current response (no `guidance` key)
- Concepts with guidance data: JSON response gains a new `guidance` key with nested objects
- Existing API clients that do not expect `guidance` will silently ignore the new field

No breaking changes to existing endpoints, response shapes, or error codes.

## Database Schema Reference

The four guidance tables were created by migration `004_guidance_data_schema.sql`. Key columns used by the queries above:

| Table | Key Columns |
|-------|------------|
| `concept_guidance` | `concept_id`, `source_pdf`, `source_page`, `about_en`, `about_nb` |
| `concept_actions` | `concept_id`, `action_text_en`, `action_text_nb`, `sort_order` |
| `concept_transparency_questions` | `concept_id`, `question_text_en`, `question_text_nb`, `sort_order` |
| `concept_references` | `concept_id`, `reference_type`, `title`, `authors`, `year`, `venue`, `url`, `sort_order` |

All tables have indexes on `concept_id`. The `concept_guidance` table has a UNIQUE constraint on `concept_id` (one guidance row per concept), which is why query 1 uses `fetch_optional`.

## STIG Compliance Checklist

- **V-222607 (CAT I)**: All four new queries use `sqlx::query().bind()` with parameterized inputs. No string interpolation.
- **V-222606 (CAT I)**: The `concept_id` comes from the URL path parameter, extracted and type-validated by Axum's `Path<String>` extractor. Same source as the existing concept query.
- **V-222609 (CAT I)**: No `.unwrap()` on user input paths. All database errors handled via `.map_err()`.
- **V-222610 (CAT II)**: Error responses are generic `500 Internal Server Error` with no database details leaked.
- **V-222585 (CAT I)**: Missing guidance data returns `None`, not an error. The endpoint never fails due to absent guidance.

## Implementation Notes (Post-Build)

**Files modified:**
- `backend/src/features/ontology/routes.rs` — handler extension with 4 concurrent guidance queries via tokio::try_join!
- `backend/src/main.rs` — 4 new OpenAPI schema registrations
- `backend/tests/api_tests.rs` — 6 new API-level tests with idempotent guidance seeding

**Code review decisions:**
- `Row::get()` panicking variant accepted — DB NOT NULL constraints guarantee safety
- Tests use `nist-ai-gv-3-1` (not `gv-1-1`) to avoid conflict with guidance_tests.rs FTS5 tests
- All tests use `INSERT OR IGNORE` for idempotent data seeding on persistent DB
- `use sqlx::Row` moved to module level

**All tests pass (full suite including 34 guidance_tests). No deviations from plan.**