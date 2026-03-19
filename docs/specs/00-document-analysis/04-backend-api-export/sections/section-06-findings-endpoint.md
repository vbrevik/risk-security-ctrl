Now I have everything I need.

# Section 06: Findings Endpoint

## Overview

This section implements the `get_findings` handler at `GET /api/analyses/:id/findings`. The endpoint returns a paginated list of `AnalysisFindingWithConcept` records (findings JOINed with concept metadata) for a given analysis, with support for filtering by framework, finding type, and priority, plus validated sort ordering.

**Dependencies:** This section depends on section-02 (route scaffold with the handler stub wired in) and section-05 (list/get/delete handlers, which establish the analysis query patterns). The `get_findings` stub must already exist in `routes.rs` and be wired to `GET /:id/findings`.

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs`

---

## Existing Types (from models.rs -- do not modify)

The following types in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/models.rs` are used directly. Do not create new types.

**`FindingsListQuery`** -- query parameters for filtering and pagination:

```rust
pub struct FindingsListQuery {
    pub framework_id: Option<String>,
    pub finding_type: Option<FindingType>,
    pub priority: Option<i32>,
    pub sort_by: Option<String>,
    pub page: i64,       // default: 1
    pub limit: i64,      // default: 50
}
```

**`AnalysisFindingWithConcept`** -- the response item type containing finding fields plus concept metadata from a JOIN:

```rust
pub struct AnalysisFindingWithConcept {
    pub id: String,
    pub analysis_id: String,
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
    pub sort_order: i32,
    pub created_at: String,
    // Concept metadata (from JOIN)
    pub concept_code: Option<String>,
    pub concept_name_en: String,
    pub concept_name_nb: String,
    pub concept_definition_en: String,
    pub concept_definition_nb: Option<String>,
    pub source_reference: Option<String>,
}
```

**`PaginatedResponse<T>`** -- the standard pagination wrapper used across the codebase (defined in `compliance/models.rs`):

```rust
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
```

---

## Database Schema Context

The query JOINs two tables:

**`analysis_findings`** table:
- `id`, `analysis_id`, `concept_id`, `framework_id`, `finding_type`, `confidence_score`, `evidence_text`, `recommendation`, `priority` (1-4), `sort_order`, `created_at`
- Indexes exist on `analysis_id`, `framework_id`, `finding_type`, `priority`, and a composite `(analysis_id, finding_type, priority)`

**`concepts`** table:
- `id`, `framework_id`, `code` (nullable), `name_en`, `name_nb` (nullable), `definition_en` (nullable), `definition_nb` (nullable), `source_reference` (nullable)

The JOIN is on `analysis_findings.concept_id = concepts.id`.

---

## Tests (write first)

Add these tests to the integration test file for analysis routes (or as `#[cfg(test)]` module in `routes.rs`, following the existing pattern). Tests use `create_test_app()` and `.oneshot(Request::builder()...)` like the compliance module.

### test_get_findings_default_sort

Create an analysis with findings pre-inserted into the database. GET `/api/analyses/:id/findings` with no query params. Assert response is 200 with `PaginatedResponse` shape. Assert items are ordered by `priority ASC, confidence_score DESC` (the default sort order).

### test_get_findings_filter_by_framework

Insert findings with two different `framework_id` values. GET with `?framework_id=X`. Assert only findings matching that framework are returned.

### test_get_findings_filter_by_type

Insert findings with different `finding_type` values. GET with `?finding_type=gap`. Assert only gap findings are returned.

### test_get_findings_returns_concept_metadata

GET findings. Assert response items have populated `concept_name_en` and `concept_code` fields from the JOINed concepts table.

### test_get_findings_invalid_sort_rejected

GET with `?sort_by=malicious_column`. Assert response is 400 (bad request), not a SQL error.

### test_get_findings_nonexistent_analysis

GET `/api/analyses/nonexistent-id/findings`. Assert 404.

### test_get_findings_pagination

Insert enough findings to span multiple pages. GET with `?limit=2`. Assert `total_pages` is correct and `data` length is 2.

---

## Implementation Details

### Handler Signature

```rust
#[utoipa::path(
    get,
    path = "/api/analyses/{id}/findings",
    tag = "analysis",
    params(("id" = String, Path, description = "Analysis ID"), FindingsListQuery),
    responses(
        (status = 200, body = PaginatedResponse<AnalysisFindingWithConcept>),
        (status = 404, description = "Analysis not found"),
        (status = 400, description = "Invalid sort_by parameter")
    )
)]
pub async fn get_findings(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<FindingsListQuery>,
) -> AppResult<Json<PaginatedResponse<AnalysisFindingWithConcept>>>
```

### Implementation Steps

1. **Verify analysis exists and is not deleted.** Query `analyses` by id, check `status != 'deleted'`. Return `AppError::NotFound` if missing or deleted.

2. **Validate `sort_by` against allowlist.** If `query.sort_by` is `Some(value)`, check it is one of `["priority", "confidence_score", "framework_id"]`. If not, return `AppError::BadRequest` with a clear message. This is critical because `sort_by` is interpolated into the SQL ORDER BY clause (cannot be parameterized). If `sort_by` is None, use the default: `priority ASC, confidence_score DESC`.

3. **Build the dynamic SQL query.** The base query is:

    ```sql
    SELECT
        af.id, af.analysis_id, af.concept_id, af.framework_id,
        af.finding_type, af.confidence_score, af.evidence_text,
        af.recommendation, af.priority, af.sort_order, af.created_at,
        c.code AS concept_code,
        c.name_en AS concept_name_en,
        COALESCE(c.name_nb, '') AS concept_name_nb,
        COALESCE(c.definition_en, '') AS concept_definition_en,
        c.definition_nb AS concept_definition_nb,
        c.source_reference
    FROM analysis_findings af
    JOIN concepts c ON af.concept_id = c.id
    WHERE af.analysis_id = ?
    ```

    Append optional WHERE clauses dynamically:
    - If `framework_id` is Some: `AND af.framework_id = ?`
    - If `finding_type` is Some: `AND af.finding_type = ?`
    - If `priority` is Some: `AND af.priority = ?`

4. **Count query.** Run a parallel count query with the same WHERE clauses (replacing SELECT columns with `COUNT(*)`).

5. **Apply ORDER BY.** Use the validated `sort_by` value or the default `af.priority ASC, af.confidence_score DESC`. The `sort_by` values map directly to column names with aliases:
    - `"priority"` maps to `af.priority ASC`
    - `"confidence_score"` maps to `af.confidence_score DESC`
    - `"framework_id"` maps to `af.framework_id ASC`

6. **Apply LIMIT/OFFSET.** `LIMIT ? OFFSET ?` with `query.limit` and `(query.page - 1) * query.limit`.

7. **Map rows to `AnalysisFindingWithConcept`.** The struct does not have a `FromRow` derive, so you will need either a custom `FromRow` implementation or manual row mapping using `sqlx::query_as` with a row struct. The `finding_type` field needs conversion from `String` to `FindingType` enum.

8. **Return `PaginatedResponse::new(findings, total, query.page, query.limit)`.** Use the `PaginatedResponse` from `compliance::models` (or add a re-export / local equivalent if the analysis module does not already have access -- check imports).

### Dynamic SQL Pattern

Follow the same dynamic query building pattern used in `compliance/routes.rs` `list_assessments`. Build the WHERE clause as a string, then use `sqlx::query_as` with sequential `.bind()` calls in the same order as the `?` placeholders.

Since SQLx does not support dynamic queries with compile-time checking, use `sqlx::query_as::<_, RowType>(&sql_string)` with runtime string interpolation for the validated ORDER BY clause. All filter values are bound as parameters (never interpolated).

### Sort Validation Detail

The allowlist validation must happen before any SQL construction. The `sort_by` field comes from user input via query string, and ORDER BY columns cannot be parameterized in SQL. The allowlist is:

```rust
const ALLOWED_SORT_FIELDS: &[&str] = &["priority", "confidence_score", "framework_id"];
```

If the user provides a value not in this list, return immediately with `AppError::BadRequest`.

---

## Import Requirements

The handler needs these imports in `routes.rs`:

```rust
use super::models::{
    AnalysisFindingWithConcept, FindingsListQuery, FindingType,
};
use crate::features::compliance::models::PaginatedResponse;
// Or if a PaginatedResponse is added to analysis models, use that instead
```

---

## OpenAPI Registration

Register the `get_findings` handler path in the OpenAPI doc builder (in `lib.rs` or wherever `utoipa` paths are aggregated), so it appears in `/swagger-ui`.