# Implementation Plan: Backend API & Report Generation

## 1. Overview

This plan implements the HTTP layer for the Document Analysis Engine: REST endpoints for analysis CRUD operations, multipart file upload, paginated findings queries, PDF/DOCX export with embedded chart visualizations, audit logging, and prompt template management.

The analysis pipeline runs synchronously during POST requests with a 30-second timeout — the `DeterministicMatcher` from split 03 completes in <2 seconds for typical documents. Parsing and matching are wrapped in `tokio::task::spawn_blocking()` since `DocumentParser` uses blocking I/O.

All endpoints follow the existing patterns established in `compliance/routes.rs`: `AppResult<T>` return types, `PaginatedResponse<T>` pagination, `#[utoipa::path]` OpenAPI annotations, and `audit_log` event tracking.

**Important:** This plan uses the existing model types from `backend/src/features/analysis/models.rs` (created in split 01). New types are only created where they don't already exist. The existing `AnalysisListQuery`, `FindingsListQuery`, `AnalysisSummary`, and `AnalysisFindingWithConcept` types must be used as-is or extended — not duplicated.

---

## 2. Route Handler Module

### File: `backend/src/features/analysis/routes.rs`

New file implementing all analysis REST endpoints, following the same structure as `compliance/routes.rs`.

### Router function

```rust
pub fn router() -> Router<AppState> {
    /// Registers all analysis endpoints under /api/analyses
```

Routes:
- `POST /` → `create_analysis`
- `POST /upload` → `upload_analysis` (with `DefaultBodyLimit::max(20MB)` layer)
- `GET /` → `list_analyses`
- `GET /:id` → `get_analysis`
- `GET /:id/findings` → `get_findings`
- `DELETE /:id` → `delete_analysis`
- `GET /:id/export/:format` → `export_analysis`
- `GET /prompt-template` → `get_prompt_template`
- `PUT /prompt-template` → `update_prompt_template`

### Router registration

Add `.nest("/analyses", features::analysis::routes::router())` to `api_routes()` in `backend/src/lib.rs`.

### AppState changes

Add `topics: Vec<Topic>` to `AppState` (loaded once at startup from `ontology-data/topic-tags.json`). This avoids repeated file I/O on every analysis request.

---

## 3. Request/Response Types

### Use existing types from `models.rs`

The following types already exist and should be used directly:
- `AnalysisListQuery` — add `include_deleted: Option<bool>` field (default false)
- `FindingsListQuery` — already has framework_id, finding_type, priority filters
- `AnalysisSummary` — uses `#[serde(flatten)]` on Analysis with `frameworks_matched: Vec<FrameworkFindingSummary>`
- `AnalysisFindingWithConcept` — findings JOINed with concept metadata (name, code, definition)

### New type: `CreateAnalysisRequest`

```rust
pub struct CreateAnalysisRequest {
    pub name: String,
    pub description: Option<String>,
    pub input_text: String,          // max 500KB validated
    pub prompt_template: Option<String>,
}
```

### Findings endpoint

Return `PaginatedResponse<AnalysisFindingWithConcept>` (not bare `AnalysisFinding`) so the frontend gets concept names and codes without additional requests.

### Sort validation

The `sort_by` parameter must be validated against allowlist `["priority", "confidence", "framework"]` and `sort_order` against `["asc", "desc"]` before interpolation into SQL ORDER BY clause (cannot be parameterized).

---

## 4. Analysis Creation (Text Input)

### Handler: `create_analysis`

Accepts `Json<CreateAnalysisRequest>`. Validates name non-empty, input_text non-empty and ≤500KB.

**Orchestration flow:**
1. Generate UUID for analysis ID
2. INSERT into `analyses` with status='pending', input_type='text'
3. Wrap blocking operations in `tokio::task::spawn_blocking()`:
   a. Call `DocumentParser::parse_text(&input_text)` to get `ParsedDocument`
4. UPDATE analysis with extracted_text
5. Get topics from `state.topics` (loaded at startup)
6. Run analysis with 30-second timeout:
   ```rust
   tokio::time::timeout(Duration::from_secs(30),
       DeterministicMatcher::new(topics).analyze(text, prompt_template, &db))
   ```
7. On success: INSERT each `NewFinding` into `analysis_findings` (assign `sort_order` from index), UPDATE analysis with status='completed', matched_framework_ids (JSON array), processing_time_ms, token_count
8. On failure: UPDATE analysis with status='failed', error_message
9. INSERT audit_log entries
10. Return `(StatusCode::CREATED, Json<AnalysisSummary>)` — returns 201 even if analysis failed (creation succeeded, analysis status visible in response body)

---

## 5. File Upload

### Handler: `upload_analysis`

Accepts `Multipart` form data. Fields: file (required), name (required), description (optional), prompt_template (optional).

**Validation:**
- File size ≤ 20MB (enforced by `DefaultBodyLimit` layer on this route)
- Extension must be `.pdf` or `.docx`
- Reject filenames with null bytes

**File storage:**
- Use `{analysis_id}.{extension}` as stored filename (not original name — prevents path traversal)
- Store original filename in DB `original_filename` column
- Save to `backend/uploads/{analysis_id}.{ext}` via `tokio::fs::write`

**After saving:**
- INSERT analysis with input_type, original_filename, file_path
- Parse via `spawn_blocking(|| DocumentParser::parse(&file_path))`
- Continue with same matching flow as text input (steps 5-10 from section 4)

---

## 6. List and Get Endpoints

### Handler: `list_analyses`

Accepts `Query<AnalysisListQuery>` (existing type, extended with `include_deleted`).

- Default: `WHERE status != 'deleted'`
- If `include_deleted=true`: include all
- If `status` filter: add `AND status = ?`
- Pagination via `PaginatedResponse<Analysis>`, order by `created_at DESC`

### Handler: `get_analysis`

Accepts `Path<String>`. Returns `AnalysisSummary` (existing type with flattened Analysis + framework findings breakdown). Return 404 if not found or deleted.

### Handler: `get_findings`

Accepts `Path<String>` for analysis_id and `Query<FindingsListQuery>`.

- Return `PaginatedResponse<AnalysisFindingWithConcept>` (JOIN with concepts table for name/code)
- Validate sort_by against allowlist before SQL interpolation
- Default ORDER BY: `priority ASC, confidence_score DESC`

---

## 7. Delete Endpoint

Soft-delete: UPDATE status='deleted', updated_at. Audit log. Return 204 No Content.

---

## 8. Chart Rendering Module

### File: `backend/src/features/analysis/charts.rs`

Three chart types as PNG bytes using `plotters` + `image` crate:

### Coverage Heatmap

```rust
pub fn render_coverage_heatmap(
    frameworks: &[(String, f64)],  // (name, coverage_percentage 0.0..1.0)
) -> Result<Vec<u8>, ChartError>
```

Rectangle grid, green(1.0) to red(0.0) interpolation. Framework names as row labels.

### Radar Chart

```rust
pub fn render_radar_chart(
    labels: &[String],
    values: &[f64],  // 0.0..1.0 normalized
) -> Result<Vec<u8>, ChartError>
```

Manual implementation: `Polygon` + `PathElement` + `Text` on `DrawingArea`. Concentric guide polygons, semi-transparent data fill.

### Priority Breakdown Bar Chart

```rust
pub fn render_priority_chart(
    priorities: &[(String, i64)],
) -> Result<Vec<u8>, ChartError>
```

Color-coded bars: P1=red, P2=orange, P3=yellow, P4=green.

### Common infrastructure

- `BitMapBackend::with_buffer` → `image::ImageBuffer::from_raw()` → PNG encoding
- Bundle LiberationSans TTF fonts in `backend/fonts/` (Regular, Bold, Italic — Apache licensed)
- Graceful error if fonts missing (return `ChartError`, don't panic)

---

## 9. PDF Export

### File: `backend/src/features/analysis/export_pdf.rs`

```rust
pub fn generate_pdf(
    analysis: &Analysis,
    findings: &[AnalysisFindingWithConcept],
    frameworks: &[(String, String)],
) -> Result<Vec<u8>, ExportError>
```

Uses `genpdf` with `images` feature. Report: title page → executive summary → coverage heatmap → per-framework sections (radar chart + findings table with concept code/name) → priority breakdown → appendix (first 2000 chars).

Font loading from `./fonts/LiberationSans*.ttf`. Render to in-memory bytes.

---

## 10. DOCX Export

### File: `backend/src/features/analysis/export_docx.rs`

```rust
pub fn generate_docx(
    analysis: &Analysis,
    findings: &[AnalysisFindingWithConcept],
    frameworks: &[(String, String)],
) -> Result<Vec<u8>, ExportError>
```

Uses `docx-rs` crate (**must be added to Cargo.toml** — split 02 used zip+quick-xml for reading, not writing). Same report structure as PDF. Charts embedded as PNG images.

---

## 11. Export Handler

Accepts `Path<(String, String)>` for (analysis_id, format). Validates format is "pdf"/"docx". Loads analysis + `AnalysisFindingWithConcept` + framework names. Calls generator. Returns bytes with Content-Type and Content-Disposition headers. Audit log.

---

## 12. Prompt Template Endpoints

### get_prompt_template

Read `backend/config/default-prompt-template.json`. If missing, return `MatcherConfig::default()` serialized.

### update_prompt_template

Accept `Json<serde_json::Value>`. Validate it deserializes as `MatcherConfig`. Write to file with file lock. Audit log.

---

## 13. Audit Logging

Follow existing compliance route pattern. Events: analysis_created, analysis_completed, analysis_failed, analysis_deleted, analysis_exported, prompt_template_updated.

---

## 14. Module Wiring

- Add to `analysis/mod.rs`: `pub mod routes;`, `pub mod charts;`, `pub mod export_pdf;`, `pub mod export_docx;`
- Nest router in `lib.rs`
- Add `topics: Vec<Topic>` to `AppState`, load at startup in `main.rs`
- Register handlers in OpenAPI doc
- **New Cargo dependencies:** `genpdf = { version = "0.2", features = ["images"] }`, `plotters = "0.3"`, `image = "0.25"`, `docx-rs = "0.4"`
- Download and commit LiberationSans font files to `backend/fonts/`

---

## 15. Error Handling

- Parsing failures: catch `ParsingError`, set status=failed, return 201 with failed analysis
- Matching failures: catch `AnalysisError`, set status=failed, return 201
- Timeout: set status=failed with "Analysis timed out after 30s"
- Export failures: return `AppError::Internal`
- File upload validation: return `AppError::BadRequest`
- Missing analysis: return `AppError::NotFound`

---

## Decision Log

| Decision | Rationale |
|----------|-----------|
| Synchronous processing with 30s timeout | DeterministicMatcher <2s typical, timeout prevents runaway |
| Full charts in exports now | User requirement, plotters is self-contained |
| Topics loaded into AppState at startup | Avoids repeated file I/O, topics are static data |
| UUID filenames for uploads | Prevents path traversal, original name stored in DB |
| Use existing model types from models.rs | Avoid type duplication, stay consistent |
| AnalysisFindingWithConcept for findings/export | Frontend and exports need concept names/codes |
| Wrap parser in spawn_blocking | DocumentParser uses blocking std::fs I/O |
| 500KB max text input | Prevents oversized text payloads |
| docx-rs added explicitly | Not present from split 02 (which used zip+quick-xml for reading) |
| File lock on prompt template writes | Prevents race condition on concurrent PUTs |
