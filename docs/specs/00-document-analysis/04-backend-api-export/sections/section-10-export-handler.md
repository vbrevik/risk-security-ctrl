Now I have all the context needed. Let me write the section.

# Section 10: Export Handler

## Overview

This section implements the `export_analysis` route handler in `backend/src/features/analysis/routes.rs`. The handler accepts a format parameter ("pdf" or "docx"), loads the analysis and its findings from the database, delegates to the appropriate generator (from sections 08 and 09), and returns the rendered document bytes with correct HTTP headers. An audit log entry is recorded for each export.

**Dependencies:** This section depends on:
- **Section 02** (route scaffold) -- the handler stub and route `GET /:id/export/:format` must exist
- **Section 06** (findings endpoint) -- the database query logic for loading `AnalysisFindingWithConcept` records must be available (or can be extracted into a shared helper)
- **Section 08** (PDF export) -- `export_pdf::generate_pdf` must be implemented
- **Section 09** (DOCX export) -- `export_docx::generate_docx` must be implemented

## Tests

All tests go in `backend/src/features/analysis/routes.rs` (or a separate test module imported there), following the existing integration test pattern of `create_test_app()` with `.oneshot()`.

### test_export_pdf_returns_pdf_content_type

```rust
/// Create an analysis with findings, then GET /api/analyses/:id/export/pdf.
/// Assert 200 status, Content-Type is "application/pdf", and
/// Content-Disposition header contains 'attachment; filename="<name>.pdf"'.
#[tokio::test]
async fn test_export_pdf_returns_pdf_content_type() {
    // Setup: create_test_app, insert analysis + findings into DB
    // GET /api/analyses/{id}/export/pdf
    // Assert status 200
    // Assert content-type header == "application/pdf"
    // Assert content-disposition contains ".pdf"
    // Assert body is non-empty
}
```

### test_export_docx_returns_docx_content_type

```rust
/// Same as above but for DOCX format.
/// Content-Type should be "application/vnd.openxmlformats-officedocument.wordprocessingml.document".
#[tokio::test]
async fn test_export_docx_returns_docx_content_type() {
    // Setup: create_test_app, insert analysis + findings
    // GET /api/analyses/{id}/export/docx
    // Assert status 200
    // Assert correct DOCX content-type
    // Assert content-disposition contains ".docx"
}
```

### test_export_invalid_format_returns_400

```rust
/// GET /api/analyses/:id/export/csv should return 400 Bad Request.
#[tokio::test]
async fn test_export_invalid_format_returns_400() {
    // GET /api/analyses/{id}/export/csv
    // Assert status 400
}
```

### test_export_nonexistent_analysis_returns_404

```rust
/// GET /api/analyses/fake-id/export/pdf should return 404.
#[tokio::test]
async fn test_export_nonexistent_analysis_returns_404() {
    // GET /api/analyses/nonexistent-uuid/export/pdf
    // Assert status 404
}
```

### test_export_creates_audit_entry

```rust
/// After a successful export, verify an audit_log row exists
/// with action = "analysis_exported".
#[tokio::test]
async fn test_export_creates_audit_entry() {
    // Setup: create analysis with findings
    // GET /api/analyses/{id}/export/pdf
    // Query audit_log table for action = 'analysis_exported', entity_id = {id}
    // Assert row exists
}
```

## Implementation Details

### File: `backend/src/features/analysis/routes.rs`

The `export_analysis` handler is registered on the route `GET /:id/export/:format` (set up in section 02). The implementation follows this flow:

### Handler Signature

```rust
/// Export an analysis report in the specified format (pdf or docx).
#[utoipa::path(
    get,
    path = "/api/analyses/{id}/export/{format}",
    params(
        ("id" = String, Path, description = "Analysis ID"),
        ("format" = String, Path, description = "Export format: pdf or docx"),
    ),
    responses(
        (status = 200, description = "Exported document bytes"),
        (status = 400, description = "Invalid format"),
        (status = 404, description = "Analysis not found"),
        (status = 500, description = "Export generation failed"),
    ),
    tag = "analyses"
)]
async fn export_analysis(
    State(state): State<AppState>,
    Path((id, format)): Path<(String, String)>,
) -> AppResult<impl IntoResponse> {
    // ...
}
```

### Step-by-step Logic

1. **Validate format** -- Check that `format` is `"pdf"` or `"docx"`. If not, return `AppError::BadRequest("Invalid export format. Must be 'pdf' or 'docx'")`.

2. **Load analysis** -- Query the `analyses` table by `id`. If not found or status is `"deleted"`, return `AppError::NotFound`. Convert the `AnalysisRow` to an `Analysis` struct.

3. **Load findings with concept metadata** -- Query `analysis_findings` JOINed with `concepts` to get `AnalysisFindingWithConcept` records for this analysis. This is the same query used by the `get_findings` handler (section 06). Consider extracting the JOIN query into a shared helper function to avoid duplication, or simply inline the query here. The query should select all findings for the analysis ordered by `sort_order ASC` (no pagination needed for export -- all findings are included).

4. **Build framework list** -- Derive `frameworks: Vec<(String, String)>` (framework_id, framework_name) from the matched_framework_ids on the analysis. Query the `frameworks` table (or `concepts` table, depending on schema) to resolve IDs to human-readable names. If framework names are not available in a separate table, use the framework_id as both ID and name.

5. **Call the appropriate generator** -- Based on format:
   - `"pdf"` -> `export_pdf::generate_pdf(&analysis, &findings, &frameworks)`
   - `"docx"` -> `export_docx::generate_docx(&analysis, &findings, &frameworks)`

   Both generators return `Result<Vec<u8>, ExportError>`. Map `ExportError` to `AppError::Internal`.

6. **Build response with headers** -- Construct the HTTP response with:
   - **Content-Type**:
     - PDF: `"application/pdf"`
     - DOCX: `"application/vnd.openxmlformats-officedocument.wordprocessingml.document"`
   - **Content-Disposition**: `attachment; filename="{analysis_name}_{date}.{ext}"` where `{date}` is the current date formatted as `YYYY-MM-DD` and `{ext}` is `pdf` or `docx`. Sanitize the analysis name to remove characters unsafe for filenames (replace non-alphanumeric except hyphens/underscores with underscores).

   Use Axum's response builder pattern:
   ```rust
   Ok((
       [(header::CONTENT_TYPE, content_type),
        (header::CONTENT_DISPOSITION, content_disposition)],
       bytes,
   ))
   ```

7. **Audit log** -- Insert an `audit_log` entry with:
   - `action`: `"analysis_exported"`
   - `entity_type`: `"analysis"`
   - `entity_id`: the analysis ID
   - `new_value`: the export format (e.g., `"pdf"`)

   Follow the existing pattern from `compliance/routes.rs`:
   ```rust
   sqlx::query(
       r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
          VALUES (?, NULL, 'analysis_exported', 'analysis', ?, ?, datetime('now'))"#
   )
   .bind(uuid::Uuid::new_v4().to_string())
   .bind(&id)
   .bind(&format)
   .execute(&state.db)
   .await?;
   ```

### Error Handling

- If the analysis is not found or is deleted, return `AppError::NotFound`.
- If the format is not "pdf" or "docx", return `AppError::BadRequest`.
- If the generator fails (font missing, chart rendering error, etc.), map the error to `AppError::Internal` with a descriptive message. Do not expose internal error details to the client beyond "Export generation failed".

### ExportError Type

The `ExportError` type should be defined in a shared location (e.g., `backend/src/features/analysis/mod.rs` or a dedicated `errors.rs`) if not already present from sections 08/09. It needs an `impl From<ExportError> for AppError` conversion:

```rust
impl From<ExportError> for AppError {
    fn from(e: ExportError) -> Self {
        AppError::Internal(format!("Export generation failed: {}", e))
    }
}
```

### Content-Type Constants

Define these as constants near the handler or in a shared location to avoid typos:

```rust
const PDF_CONTENT_TYPE: &str = "application/pdf";
const DOCX_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
```

### Filename Sanitization

The analysis name used in the Content-Disposition filename must be sanitized. A simple helper:

```rust
fn sanitize_filename(name: &str) -> String {
    /// Replace any character that is not alphanumeric, hyphen, or underscore
    /// with an underscore. Truncate to 100 characters.
}
```