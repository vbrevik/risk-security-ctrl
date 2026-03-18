Now I have all the context needed. Let me also check if there's an `include_deleted` field already in `AnalysisListQuery`.

Looking at the models, `AnalysisListQuery` does NOT have `include_deleted` yet (it only has `page`, `limit`, `status`). The plan says to add it.

Now I have enough context to write the section.

# Section 05: List, Get, and Delete Handlers

## Overview

This section implements three handlers in `backend/src/features/analysis/routes.rs`:

- **`list_analyses`** -- paginated listing of analyses, excluding soft-deleted by default
- **`get_analysis`** -- single analysis with summary statistics (finding counts per framework)
- **`delete_analysis`** -- soft-delete (sets status to "deleted") with audit logging

These handlers follow the same patterns used in `backend/src/features/compliance/routes.rs`: extract `State<AppState>`, use `AppResult<T>` return types, build dynamic SQL WHERE clauses, and return `PaginatedResponse<T>` for list endpoints.

## Dependencies

- **section-02** (route scaffold): The router in `routes.rs` must exist with stub handlers wired to `GET /`, `GET /:id`, and `DELETE /:id`.
- **section-03** (create analysis): At least one analysis must be insertable into the database for these handlers to return meaningful results. The `analyses` and `analysis_findings` tables must exist (from migrations in earlier splits).

## Files to Modify

| File | Action |
|------|--------|
| `backend/src/features/analysis/models.rs` | Add `include_deleted` field to `AnalysisListQuery` |
| `backend/src/features/analysis/routes.rs` | Implement `list_analyses`, `get_analysis`, `delete_analysis` handlers |

## Tests (Write First)

All tests go in `backend/src/features/analysis/routes.rs` (or a dedicated test file if the project uses `tests/`). They follow the integration test pattern: `create_test_app()` then `.oneshot(Request::builder()...)` and assert responses.

### test_list_analyses_paginated

Create 3 analyses in the DB, then `GET /api/analyses?limit=2`. Assert the response is 200, the `data` array has 2 items, `total` is 3, and `total_pages` is 2.

### test_list_analyses_excludes_deleted

Create an analysis, then `DELETE /api/analyses/:id`. Afterwards `GET /api/analyses` and assert the deleted analysis is NOT in the results.

### test_list_analyses_include_deleted

Delete an analysis, then `GET /api/analyses?include_deleted=true`. Assert the deleted analysis IS present and has `status: "deleted"`.

### test_list_analyses_filter_by_status

Create analyses with different statuses. `GET /api/analyses?status=completed`. Assert only completed analyses are returned.

### test_get_analysis_returns_summary

Create an analysis that has findings in the DB. `GET /api/analyses/:id`. Assert the response is 200 and includes `total_findings`, `gap_count`, `addressed_count`, `partially_addressed_count`, and a non-empty `frameworks_matched` array.

### test_get_analysis_not_found

`GET /api/analyses/nonexistent-uuid`. Assert 404.

### test_delete_analysis_soft_delete

`DELETE /api/analyses/:id`. Assert 204. Then `GET /api/analyses/:id` returns 404. Then `GET /api/analyses?include_deleted=true` shows the analysis with `status: "deleted"`.

### test_delete_nonexistent_returns_404

`DELETE /api/analyses/nonexistent-uuid`. Assert 404.

## Implementation Details

### 1. Add `include_deleted` to `AnalysisListQuery`

In `backend/src/features/analysis/models.rs`, add the field to the existing struct:

```rust
#[derive(Debug, Deserialize, IntoParams)]
pub struct AnalysisListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub status: Option<AnalysisStatus>,
    #[serde(default)]
    pub include_deleted: bool,
}
```

The `#[serde(default)]` makes it default to `false` when the query parameter is omitted.

### 2. Handler: `list_analyses`

Signature:

```rust
pub async fn list_analyses(
    State(state): State<AppState>,
    Query(query): Query<AnalysisListQuery>,
) -> AppResult<Json<PaginatedResponse<Analysis>>>
```

Logic:

1. Compute `offset = (query.page - 1) * query.limit`.
2. Build a dynamic WHERE clause:
   - If `include_deleted` is false (default), add `status != 'deleted'` condition.
   - If `query.status` is `Some(s)`, add `status = ?` condition with the string value of `s`.
3. Run a `SELECT COUNT(*)` query with the WHERE clause and bind params to get `total`.
4. Run a `SELECT * FROM analyses {where_clause} ORDER BY created_at DESC LIMIT ? OFFSET ?` query, mapping rows via `sqlx::query_as::<_, AnalysisRow>` then converting each to `Analysis`.
5. Return `PaginatedResponse::new(data, total, query.page, query.limit)`.

Follow the same dynamic WHERE clause + bind parameter pattern from `compliance/routes.rs::list_assessments` (building a `Vec<String>` of conditions and a `Vec<String>` of params, then matching on `params.len()` for binding).

Add the `#[utoipa::path]` annotation for OpenAPI documentation:
- path: `/api/analyses`
- tag: `"analysis"`
- params: `AnalysisListQuery`
- response 200 with body `PaginatedResponse<Analysis>`

### 3. Handler: `get_analysis`

Signature:

```rust
pub async fn get_analysis(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<AnalysisSummary>>
```

Logic:

1. Fetch the analysis row: `SELECT * FROM analyses WHERE id = ? AND status != 'deleted'`. If not found, return `AppError::NotFound`.
2. Convert `AnalysisRow` to `Analysis`.
3. Query aggregate finding counts for the analysis:
   ```sql
   SELECT
       framework_id,
       COUNT(*) as total_findings,
       SUM(CASE WHEN finding_type = 'addressed' THEN 1 ELSE 0 END) as addressed_count,
       SUM(CASE WHEN finding_type = 'partially_addressed' THEN 1 ELSE 0 END) as partially_addressed_count,
       SUM(CASE WHEN finding_type = 'gap' THEN 1 ELSE 0 END) as gap_count,
       SUM(CASE WHEN finding_type = 'not_applicable' THEN 1 ELSE 0 END) as not_applicable_count
   FROM analysis_findings
   WHERE analysis_id = ?
   GROUP BY framework_id
   ```
4. For each framework row, look up the framework name: `SELECT name FROM frameworks WHERE id = ?`. Build a `FrameworkFindingSummary` for each.
5. Sum the totals across all frameworks to populate the top-level `total_findings`, `gap_count`, `addressed_count`, `partially_addressed_count`.
6. Return `AnalysisSummary { analysis, total_findings, gap_count, addressed_count, partially_addressed_count, frameworks_matched }`.

Add `#[utoipa::path]` with path `/api/analyses/{id}`, tag `"analysis"`, response 200 body `AnalysisSummary`, response 404.

### 4. Handler: `delete_analysis`

Signature:

```rust
pub async fn delete_analysis(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode>
```

Logic:

1. Check the analysis exists and is not already deleted: `SELECT id FROM analyses WHERE id = ? AND status != 'deleted'`. If not found, return `AppError::NotFound`.
2. Soft-delete: `UPDATE analyses SET status = 'deleted', updated_at = ? WHERE id = ?`.
3. Insert an audit log entry:
   ```sql
   INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
   VALUES (?, NULL, 'analysis_deleted', 'analysis', ?, ?, ?)
   ```
   The `new_value` should be a JSON object: `{"id": "<id>", "status": "deleted"}`.
4. Return `StatusCode::NO_CONTENT` (204).

Add `#[utoipa::path]` with path `/api/analyses/{id}`, tag `"analysis"`, response 204, response 404.

### 5. Imports needed in `routes.rs`

The handler implementations will need these imports (some may already exist from the stub in section-02):

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::AppState;

use super::models::{
    Analysis, AnalysisListQuery, AnalysisRow, AnalysisSummary,
    FrameworkFindingSummary,
};
// Use PaginatedResponse from compliance (or create a shared one)
use crate::features::compliance::models::PaginatedResponse;
```

Note: `PaginatedResponse` is defined in both `compliance::models` and `ontology::models`. Import from whichever is the established convention, or if a shared version exists at crate root, use that instead.

### 6. Aggregate query helper struct

You will need a temporary `FromRow` struct for the framework aggregation query result:

```rust
#[derive(Debug, sqlx::FromRow)]
struct FrameworkAggregateRow {
    framework_id: String,
    total_findings: i64,
    addressed_count: i64,
    partially_addressed_count: i64,
    gap_count: i64,
    not_applicable_count: i64,
}
```

This can be defined privately inside `routes.rs` since it is only used by `get_analysis`.