Now I have all the context I need. Let me produce the section content.

# Section 1: Guidance Response Types

## Overview

This section adds four new response structs to the ontology models module and extends the existing `ConceptWithRelationships` struct with an optional guidance field. These types define the API contract for guidance data serialization and are required by Section 2 (concept detail handler) and Section 6 (integration tests).

**No dependencies** -- this section can be implemented first, in parallel with Section 3.

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/ontology/models.rs`

## Background

The database already contains four guidance tables (added in the 02-schema-import phase):
- `concept_guidance` -- top-level guidance metadata (source PDF, page, about text)
- `concept_actions` -- suggested actions with sort order
- `concept_transparency_questions` -- transparency questions with sort order
- `concept_references` -- academic and transparency resource references

These tables are populated for NIST AI RMF action-level concepts. The response types defined here mirror the table structure for JSON serialization. All types must derive `Serialize` and `ToSchema` (for utoipa OpenAPI documentation).

## Tests (Write First)

Add these tests in a `#[cfg(test)] mod tests` block at the bottom of `models.rs` (or in a dedicated test file if the project convention differs). Each test constructs a struct, serializes it to JSON with `serde_json`, and asserts on the output structure.

```rust
#[cfg(test)]
mod guidance_response_tests {
    use super::*;
    use serde_json;

    // Test: ConceptGuidanceResponse serializes to expected JSON structure
    // - Construct a ConceptGuidanceResponse with all fields populated
    // - Serialize to serde_json::Value
    // - Assert top-level keys: source_pdf, source_page, about_en, about_nb,
    //   suggested_actions, transparency_questions, references

    // Test: ActionResponse serializes sort_order and text_en correctly
    // - Construct ActionResponse { sort_order: 1, text_en: "Do X".into(), text_nb: None }
    // - Serialize, assert JSON has "sort_order": 1, "text_en": "Do X"

    // Test: QuestionResponse serializes sort_order and text_en correctly
    // - Same pattern as ActionResponse test

    // Test: ReferenceResponse uses "type" (not "reference_type") in JSON via serde(rename)
    // - Construct ReferenceResponse with reference_type: "academic".into()
    // - Serialize to JSON, assert the key is "type" not "reference_type"
    // - Assert "reference_type" key does NOT exist in the JSON output

    // Test: ConceptWithRelationships with guidance=None omits guidance field from JSON
    // - Construct ConceptWithRelationships with guidance: None
    // - Serialize to JSON
    // - Assert the "guidance" key is absent from the JSON object

    // Test: ConceptWithRelationships with guidance=Some includes the nested object
    // - Construct ConceptWithRelationships with guidance: Some(...)
    // - Serialize to JSON
    // - Assert "guidance" key is present and contains expected nested structure
}
```

## Implementation Details

### New Structs

Add the following four structs to `models.rs`. All must derive `Debug, Serialize, ToSchema`. Place them after the existing `RelatedConcept` struct and before `PaginatedResponse`.

**`ActionResponse`** -- represents one suggested action:
- `sort_order: i64`
- `text_en: String`
- `text_nb: Option<String>`

**`QuestionResponse`** -- represents one transparency question:
- `sort_order: i64`
- `text_en: String`
- `text_nb: Option<String>`

**`ReferenceResponse`** -- represents one reference entry. The `reference_type` field must be renamed to `"type"` in the serialized JSON:
```rust
#[derive(Debug, Serialize, ToSchema)]
pub struct ReferenceResponse {
    #[serde(rename = "type")]
    pub reference_type: String,  // "academic" or "transparency_resource"
    pub title: String,
    pub authors: Option<String>,
    pub year: Option<i64>,
    pub venue: Option<String>,
    pub url: Option<String>,
}
```

**`ConceptGuidanceResponse`** -- top-level container assembling all guidance data:
- `source_pdf: String`
- `source_page: i64`
- `about_en: Option<String>`
- `about_nb: Option<String>`
- `suggested_actions: Vec<ActionResponse>`
- `transparency_questions: Vec<QuestionResponse>`
- `references: Vec<ReferenceResponse>`

### Extending `ConceptWithRelationships`

Add one new field to the existing `ConceptWithRelationships` struct:

```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub guidance: Option<ConceptGuidanceResponse>,
```

This is backward-compatible. When guidance is `None` (which is the case for all non-NIST AI RMF concepts), the `guidance` key is omitted entirely from the JSON response thanks to `skip_serializing_if`. Existing API clients that do not expect this field will see no change in behavior.

### Derive Trait Requirements

All four new structs need:
- `Debug` -- for logging and test output
- `Serialize` -- for JSON serialization in API responses
- `ToSchema` -- for OpenAPI/utoipa schema generation

They do **not** need `Deserialize` or `FromRow` because they are response-only types assembled manually from query results (not deserialized from incoming requests or mapped directly from SQL rows).

### Imports

The existing imports at the top of `models.rs` already include `serde::Serialize` and `utoipa::ToSchema`, so no new imports are needed.

## Acceptance Criteria

1. All six test stubs pass after implementation.
2. `cargo clippy` reports no warnings on the new types.
3. Serializing `ConceptWithRelationships` with `guidance: None` produces JSON without a `"guidance"` key.
4. Serializing `ReferenceResponse` produces `"type"` (not `"reference_type"`) in the JSON output.
5. All new structs appear in the OpenAPI schema when registered (registration happens in Section 2).

## Implementation Notes (Post-Build)

**Files modified:**
- `backend/src/features/ontology/models.rs` — 4 new structs + guidance field + 6 tests
- `backend/src/features/ontology/routes.rs` — added `guidance: None` to existing construction

**Code review decisions:**
- Nested Option fields keep null serialization (not omitted) — explicit for API consumers
- Added empty vec assertions (`[]`) to prevent regression from accidental skip_serializing_if on Vec fields
- No Clone derive added — deferred until needed by downstream sections

**All 6 tests pass. No deviations from plan.**