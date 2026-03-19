I now have all the context needed. Let me produce the section content.

# Section 3: Route Handler Completion

## Overview

This section completes the stub route handlers in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs` and wires the analysis feature into the main application router. Currently, all handlers return empty JSON or placeholder responses. This section fills them in with real database queries, file upload processing, and proper error handling.

**Dependencies:** This section depends on section-01 (upload handler) and section-02 (parser refinements) being completed first. Specifically, it uses:
- `upload::validate_upload()` and `upload::save_upload()` from section-01
- `parser::parse_async()` and the `From<ParsingError> for AppError` impl from section-02

## Current State

The file at `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs` already has:
- A `router()` function that defines all route paths correctly
- `DefaultBodyLimit::max(20 * 1024 * 1024)` on the upload route
- Stub handlers for: `create_analysis`, `upload_analysis`, `list_analyses`, `get_analysis`, `get_findings`, `delete_analysis`, `export_analysis`, `get_prompt_template`, `update_prompt_template`
- `#[utoipa::path]` annotations on all handlers (paths, tags, params, responses)

The router is already wired into `lib.rs` at line 89: `.nest("/analyses", features::analysis::routes::router())`.

All handlers currently return empty JSON objects or empty lists. The task is to replace these stubs with real implementations.

## Tests First

Add these tests to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs` inside a `#[cfg(test)] mod tests` block. These are handler-level tests that use `axum_test` or the existing `create_test_app()` helper from `backend/tests/common/mod.rs`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// POST /api/analyses/upload with a valid PDF multipart body should:
    /// - return 201
    /// - return JSON with an `id` field (UUID)
    /// - create a row in the `analyses` table with status "processing" or "completed"
    /// - store extracted text in `analyses.extracted_text`
    #[tokio::test]
    async fn upload_pdf_creates_analysis() {}

    /// POST /api/analyses/upload with a file exceeding 20MB should return 400
    /// with an error message mentioning file size.
    #[tokio::test]
    async fn upload_oversized_returns_400() {}

    /// POST /api/analyses/upload with an unsupported file extension (e.g., .exe)
    /// should return 400 with an error message about unsupported format.
    #[tokio::test]
    async fn upload_unsupported_format_returns_400() {}
}
```

## Implementation Details

### File to modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs`

### Additional imports needed

The handler implementations will need these additional imports added to the top of `routes.rs`:

- `uuid::Uuid` for generating analysis IDs
- `super::upload::{validate_upload, save_upload}` for file validation and saving
- `super::parser::parse_async` for async document parsing
- `super::matcher::DeterministicMatcher` for running the matching engine
- `super::engine::MatchingEngine` for the trait method
- `super::models::{Analysis, AnalysisRow, AnalysisFinding, AnalysisFindingRow, AnalysisFindingWithConcept}` for DB row types and response types
- `sqlx` for database queries

### Handler: `upload_analysis`

This is the primary handler to implement. It should:

1. Extract the multipart fields: iterate `multipart.next_field().await` looking for a field named `"file"`. Extract the filename and read bytes via `field.bytes().await`.
2. Call `validate_upload(filename, size, &header_bytes)` to check extension, size, and magic bytes. Map errors to `AppError::BadRequest` via the `From<ParsingError> for AppError` conversion from section-02.
3. Generate a new UUID for the analysis ID.
4. Call `save_upload(analysis_id, filename, &bytes)` to persist the file to disk.
5. Call `parse_async(file_path)` to extract text from the uploaded document.
6. Insert a row into the `analyses` table with status `"processing"`, the `input_type`, `original_filename`, `file_path`, and `extracted_text`.
7. Create a `DeterministicMatcher` from `state.topics` and call `matcher.analyze(&extracted_text, None, &state.db).await`.
8. Update the analysis row: set `status = "completed"`, `matched_framework_ids` (JSON array), `processing_time_ms`, `token_count`.
9. Insert each finding from `MatchingResult.findings` into the `analysis_findings` table.
10. Return `(StatusCode::CREATED, Json(analysis_response))` where the response includes at minimum `{ "id": "<uuid>", "status": "completed" }`.

If parsing or matching fails, update the analysis row with `status = "failed"` and `error_message`, then return the error.

### Handler: `create_analysis`

Implement the text-based analysis creation (no file upload):

1. Validate that `body.input_text` is non-empty and under a reasonable length.
2. Generate a UUID for the analysis ID.
3. Insert into `analyses` with `input_type = "text"`, `input_text = body.input_text`, `extracted_text = body.input_text`.
4. Run `DeterministicMatcher::analyze()` on the text.
5. Store findings, update status, return 201 with the analysis JSON.

### Handler: `list_analyses`

1. Query `analyses` table with pagination from `AnalysisListQuery` (page, limit).
2. Apply optional `status` filter if provided in query params.
3. Calculate offset as `(page - 1) * limit`.
4. Also run a `COUNT(*)` query for total count.
5. Return paginated JSON: `{ items: Analysis[], total, page, limit, total_pages }`.

### Handler: `get_analysis`

1. Query `analyses` table by `id`.
2. If no row found, return `AppError::NotFound`.
3. Convert `AnalysisRow` to `Analysis` via the existing `From` impl and return as JSON.

### Handler: `get_findings`

1. Query `analysis_findings` joined with `concepts` table for concept metadata.
2. Filter by `analysis_id` from path, plus optional `framework_id`, `finding_type`, `priority` from `FindingsListQuery`.
3. Apply pagination. Return paginated list of `AnalysisFindingWithConcept`.

### Handler: `delete_analysis`

1. Run `DELETE FROM analyses WHERE id = ?` (CASCADE will handle findings).
2. If no rows affected, return `AppError::NotFound`.
3. Optionally delete the uploaded file from disk.
4. Return `StatusCode::NO_CONTENT`.

### Handlers: `get_prompt_template` and `update_prompt_template`

These can remain as basic stubs for now (returning/accepting a JSON object with a `template` field), or implement a simple key-value store in a `settings` table. For MVP, store the default `MatcherConfig` as JSON and allow overriding it.

### Handler: `export_analysis`

Leave as `StatusCode::NOT_IMPLEMENTED` -- this is out of scope for the document parsing pipeline.

### Router-level body size limit

The current router already has `DefaultBodyLimit::max(20 * 1024 * 1024)` on the upload route. Per the plan, increase this to `25 * 1024 * 1024` to account for multipart overhead (the multipart framing and headers add bytes beyond the raw file size). Additionally, import and apply `tower_http::limit::RequestBodyLimitLayer` as a second layer for defense-in-depth:

```rust
use tower_http::limit::RequestBodyLimitLayer;

let upload_routes = Router::new()
    .route("/upload", post(upload_analysis))
    .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
    .layer(RequestBodyLimitLayer::new(25 * 1024 * 1024));
```

### OpenAPI annotations

The existing `#[utoipa::path]` annotations are already in place on all handlers. When completing the handler implementations, update the `responses` attributes to include the actual response schema types (e.g., `body = Analysis` for `get_analysis`, `body = Vec<AnalysisFinding>` for `get_findings`). Also add `request_body(content_type = "multipart/form-data")` to the `upload_analysis` annotation.

### Wiring into main router

Already done. The line `.nest("/analyses", features::analysis::routes::router())` exists in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs` at line 89 inside the `api_routes()` function. No changes needed here.

## Database Queries Reference

The `analyses` table schema (from migration `003_analysis_schema.sql`):

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PRIMARY KEY | UUID |
| name | TEXT NOT NULL | User-provided or derived from filename |
| description | TEXT | Optional |
| input_type | TEXT NOT NULL | CHECK: text, pdf, docx |
| input_text | TEXT | Raw input for text-based analysis |
| original_filename | TEXT | Original uploaded filename |
| file_path | TEXT | Path to saved file on disk |
| extracted_text | TEXT | Parsed text content |
| status | TEXT NOT NULL DEFAULT 'pending' | CHECK: pending, processing, completed, failed, deleted |
| error_message | TEXT | Error details if failed |
| prompt_template | TEXT | JSON config for matcher |
| matched_framework_ids | TEXT | JSON array of framework IDs |
| processing_time_ms | INTEGER | Processing duration |
| token_count | INTEGER | Estimated token count |
| created_by | TEXT | User ID |
| created_at | TEXT | ISO datetime |
| updated_at | TEXT | ISO datetime |

The `analysis_findings` table has a foreign key to `analyses(id)` with `ON DELETE CASCADE`, so deleting an analysis automatically removes its findings.

## Key Patterns to Follow

- Use `sqlx::query!` or `sqlx::query_as!` macros for compile-time checked queries where possible. Fall back to `sqlx::query_as::<_, AnalysisRow>()` with string queries if the `sqlx prepare` step has not been run.
- All handlers receive `State(state): State<AppState>` where `state.db` is the `SqlitePool` and `state.topics` is `Vec<Topic>`.
- Return `AppResult<T>` (alias for `Result<T, AppError>`) from all handlers.
- Use the existing `From<AnalysisRow> for Analysis` and `From<AnalysisFindingRow> for AnalysisFinding` conversions to map DB rows to API responses.
- Errors from `ParsingError` convert to `AppError` via the `From` impl added in section-02.