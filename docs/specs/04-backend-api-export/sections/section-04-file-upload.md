Now I have all the context I need. Let me produce the section content.

# Section 04: File Upload Handler

## Overview

This section implements the `upload_analysis` handler in `backend/src/features/analysis/routes.rs`. The handler accepts multipart form data containing a file (PDF or DOCX), validates it, stores it with a UUID-based filename, parses the file content via `DocumentParser::parse()` in a blocking task, and then runs the same matching pipeline as the text-input handler from section 03.

## Dependencies

- **Section 01** (Dependencies and AppState): `topics: Vec<Topic>` must be available on `AppState`
- **Section 02** (Route Scaffold): The route `POST /upload` with `DefaultBodyLimit::max(20MB)` must be wired in the router
- **Section 03** (Create Analysis Text): The analysis orchestration flow (DB insert, matching, findings storage, audit logging) is reused. The `upload_analysis` handler calls the same internal pipeline after extracting text from the uploaded file

## Key Files

- **Modify:** `backend/src/features/analysis/routes.rs` -- implement the `upload_analysis` handler
- **Read (existing):** `backend/src/features/analysis/parser.rs` -- `DocumentParser::parse(&file_path)` and `DocumentParser::parse_text(&text)`
- **Read (existing):** `backend/src/features/analysis/matcher.rs` -- `DeterministicMatcher`
- **Read (existing):** `backend/src/features/analysis/engine.rs` -- `NewFinding`, `MatchingEngine` trait, `MatchingResult`
- **Read (existing):** `backend/src/features/analysis/models.rs` -- `Analysis`, `CreateAnalysisRequest`, `AnalysisSummary`, `InputType`
- **Read (existing):** `backend/src/error.rs` -- `AppError`, `AppResult`
- **Read (existing):** `backend/tests/common/mod.rs` -- `create_test_app()` test helper

## Tests First

Tests go in `backend/tests/api_tests.rs` (or a new `backend/tests/analysis_upload_tests.rs` file). They follow the existing integration test pattern: `create_test_app()` then `.oneshot(Request)` assertions.

### test_upload_pdf_file

POST multipart to `/api/analyses/upload` with a valid PDF file and a `name` field. Assert 201 status and response body contains `input_type: "pdf"`. The file need not be a real PDF with parseable content -- the handler should still return 201 with `status: "completed"` or `status: "failed"` depending on whether the parser can extract text.

For a minimal test, create a small valid PDF byte sequence or use an actual tiny PDF fixture file stored in `backend/tests/fixtures/`. The key assertion is that the multipart parsing and file storage work correctly.

```rust
#[tokio::test]
async fn test_upload_pdf_file() {
    /// POST multipart with a PDF file field and name field
    /// Assert: 201, response body has input_type = "pdf"
    /// Assert: file is saved to backend/uploads/ directory
}
```

### test_upload_docx_file

Same as above but with a `.docx` extension. Assert `input_type: "docx"`.

```rust
#[tokio::test]
async fn test_upload_docx_file() {
    /// POST multipart with a DOCX file field and name field
    /// Assert: 201, response body has input_type = "docx"
}
```

### test_upload_invalid_extension_rejected

POST multipart with a `.txt` file. Assert 400 status.

```rust
#[tokio::test]
async fn test_upload_invalid_extension_rejected() {
    /// POST multipart with a .txt file
    /// Assert: 400 status code
}
```

### test_upload_missing_file_rejected

POST multipart with only a `name` field, no `file` field. Assert 400 status.

```rust
#[tokio::test]
async fn test_upload_missing_file_rejected() {
    /// POST multipart without a file field
    /// Assert: 400 status code
}
```

### test_upload_missing_name_rejected

POST multipart with a `file` field but no `name` field. Assert 400 status.

```rust
#[tokio::test]
async fn test_upload_missing_name_rejected() {
    /// POST multipart with file but no name field
    /// Assert: 400 status code
}
```

## Implementation Details

### Handler Signature

```rust
pub async fn upload_analysis(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<(StatusCode, Json<AnalysisSummary>)>
```

The handler uses `axum::extract::Multipart` following the same pattern as the existing `upload_evidence` handler in `backend/src/features/compliance/routes.rs` (line 1203).

### Multipart Field Processing

Iterate over multipart fields using `multipart.next_field().await`. Expected fields:

| Field Name | Type | Required | Description |
|---|---|---|---|
| `file` | file | Yes | The PDF or DOCX file |
| `name` | text | Yes | Analysis name |
| `description` | text | No | Analysis description |
| `prompt_template` | text | No | Custom prompt template JSON |

Use a `while let Some(field)` loop with a `match` on `field.name()`, collecting values into local `Option<T>` variables. After the loop, validate that `file` and `name` were provided, returning `AppError::BadRequest` if missing.

### File Validation

Before saving the file:

1. **Extension check**: Extract extension from the original filename via `Path::new(&original_name).extension()`. Only `.pdf` and `.docx` are allowed. Return `AppError::BadRequest` for other extensions.

2. **Null byte check**: Reject filenames containing null bytes (`\0`) to prevent path traversal attacks. Check with `original_name.contains('\0')`.

3. **Size enforcement**: The 20MB limit is enforced at the router level via `DefaultBodyLimit::max(20 * 1024 * 1024)` (configured in section 02). No additional size check needed in the handler, but the multipart `field.bytes().await` call will fail if the limit is exceeded.

### File Storage

Store files using UUID-based filenames to prevent path traversal:

1. Generate analysis ID: `let analysis_id = Uuid::new_v4().to_string();`
2. Determine extension from the original filename
3. Construct stored filename: `{analysis_id}.{extension}` (e.g., `abc-123.pdf`)
4. Create upload directory: `tokio::fs::create_dir_all("uploads").await`
5. Write file: `tokio::fs::write(&dest_path, &data).await`
6. Store the original filename in the `original_filename` DB column

The upload directory is `backend/uploads/` (relative to the backend working directory). This follows the same pattern as `uploads/evidence/` used by the compliance evidence upload handler.

### Database Insert

After file storage, INSERT the analysis record:

```sql
INSERT INTO analyses (id, name, description, input_type, original_filename, file_path, status, prompt_template, created_at, updated_at)
VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?, ?)
```

Where `input_type` is derived from the file extension (`"pdf"` or `"docx"`).

### Parsing via spawn_blocking

The `DocumentParser::parse()` function uses blocking `std::fs` I/O, so it must be wrapped in `spawn_blocking`:

```rust
let file_path_clone = file_path.clone();
let parsed = tokio::task::spawn_blocking(move || {
    DocumentParser::parse(std::path::Path::new(&file_path_clone))
}).await;
```

Handle the nested `Result`: the outer `Result` is from `spawn_blocking` (JoinError), the inner is from the parser (ParsingError). If parsing fails, UPDATE analysis with `status='failed'` and `error_message`, but still return 201 (the analysis record was created; failure is reflected in the status field).

After successful parsing, UPDATE the analysis with `extracted_text` from the `ParsedDocument`.

### Analysis Pipeline (Reuse from Section 03)

After parsing, the flow is identical to the text-input handler (section 03, steps 5-10):

1. Get topics from `state.topics`
2. Create `DeterministicMatcher::new(topics)`
3. Run with 30-second timeout: `tokio::time::timeout(Duration::from_secs(30), matcher.analyze(...))`
4. On success: INSERT each `NewFinding` into `analysis_findings`, UPDATE analysis with `status='completed'`, `matched_framework_ids`, `processing_time_ms`, `token_count`
5. On failure/timeout: UPDATE analysis with `status='failed'`, `error_message`
6. INSERT `audit_log` entry with `action='analysis_created'`
7. Build and return `AnalysisSummary`

Consider extracting the shared pipeline into a helper function (e.g., `run_analysis_pipeline()`) to avoid code duplication between `create_analysis` and `upload_analysis`. This helper would take the analysis ID, extracted text, prompt template, topics, and DB pool, then handle matching + findings storage + status update + audit logging.

### OpenAPI Annotation

```rust
#[utoipa::path(
    post,
    path = "/api/analyses/upload",
    tag = "analysis",
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "File uploaded and analysis created", body = AnalysisSummary),
        (status = 400, description = "Invalid file type or missing required fields")
    )
)]
```

### Audit Logging

Follow the existing pattern from compliance routes. INSERT into `audit_log`:

```sql
INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
VALUES (?, NULL, 'analysis_created', 'analysis', ?, ?, ?)
```

The `new_value` field should contain the analysis name or a JSON summary. Use `Uuid::new_v4()` for the audit log entry ID.

### Error Handling Summary

| Condition | Response |
|---|---|
| Missing `file` field | `AppError::BadRequest("No file field provided")` |
| Missing `name` field | `AppError::BadRequest("Name is required")` |
| Invalid extension (not .pdf/.docx) | `AppError::BadRequest("Only PDF and DOCX files are supported")` |
| Null bytes in filename | `AppError::BadRequest("Invalid filename")` |
| File write failure | `AppError::Internal("Failed to write file")` |
| Parser failure | 201 with `status: "failed"`, `error_message` in body |
| Matcher failure | 201 with `status: "failed"`, `error_message` in body |
| Matcher timeout (>30s) | 201 with `status: "failed"`, `error_message: "Analysis timed out after 30s"` |

### Test Fixture Setup

For integration tests, create minimal test fixture files in `backend/tests/fixtures/`:

- `test.pdf` -- a minimal valid PDF (can be a few bytes with `%PDF-1.0` header and minimal structure)
- `test.docx` -- a minimal valid DOCX (a zip file with the required `[Content_Types].xml` and `word/document.xml`)

These can be generated programmatically in a test helper or committed as binary fixtures. The parser may fail to extract meaningful text from minimal fixtures, but the handler should still return 201 with `status: "failed"` and an appropriate error message, which is the expected behavior.

To construct multipart requests in tests, use `axum::body::Body` with manually constructed multipart boundaries, or use the `reqwest::multipart` builder if available. The existing compliance tests at `backend/tests/api_tests.rs` show the `.oneshot()` pattern with `Request::builder()`.