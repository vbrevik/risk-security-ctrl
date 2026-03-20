# Section 01 Contract: Guidance Response Types

## GOAL
Add four response structs (ActionResponse, QuestionResponse, ReferenceResponse, ConceptGuidanceResponse) to models.rs and extend ConceptWithRelationships with an optional guidance field.

## CONTEXT
Section 1 of 6 in the API matcher enhancement. These types define the JSON API contract for guidance data. Required by Section 2 (handler) and Section 6 (integration tests).

## CONSTRAINTS
- All structs derive Debug, Serialize, ToSchema (no Deserialize/FromRow — response-only)
- ReferenceResponse.reference_type must use `#[serde(rename = "type")]`
- ConceptWithRelationships.guidance uses `#[serde(skip_serializing_if = "Option::is_none")]`
- Place new structs after RelatedConcept, before PaginatedResponse
- No new imports needed (serde::Serialize and utoipa::ToSchema already imported)
- STIG V-222585: None/empty guidance is omitted, not errored

## FORMAT
Modify: `backend/src/features/ontology/models.rs`

## FAILURE CONDITIONS
- SHALL NOT add Deserialize or FromRow to new structs
- SHALL NOT break existing ConceptWithRelationships serialization
- SHALL NOT use "reference_type" as the JSON key (must be "type")
- SHALL NOT include guidance key in JSON when value is None
- SHALL NOT skip tests
