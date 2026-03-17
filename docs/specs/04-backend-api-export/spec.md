# 04-backend-api-export: REST API & Report Generation

## Summary

HTTP layer and orchestration for the Document Analysis Engine. REST endpoints for analysis CRUD, file upload, findings queries with filtering/sorting, and PDF/DOCX export with embedded chart visualizations. Integrates audit logging and cost/metrics tracking.

## Requirements Source

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md` (Routes, Integration Points)
- Interview: `docs/specs/deep_project_interview.md`

## What to Build

### REST Endpoints (`backend/src/features/analysis/routes.rs`)

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| POST | `/api/analyses` | `create_analysis` | Create from text input (JSON body) |
| POST | `/api/analyses/upload` | `upload_analysis` | Create from file upload (multipart) |
| GET | `/api/analyses` | `list_analyses` | List analyses (paginated, filterable by status) |
| GET | `/api/analyses/:id` | `get_analysis` | Get analysis with summary stats |
| GET | `/api/analyses/:id/findings` | `get_analysis_findings` | Get findings (filter: framework_id, finding_type, priority; sort: priority, confidence, framework) |
| DELETE | `/api/analyses/:id` | `delete_analysis` | Soft-delete (status → 'deleted') |
| GET | `/api/analyses/:id/export/:format` | `export_analysis` | Export as PDF or DOCX (format = "pdf" or "docx") |
| GET | `/api/analyses/prompt-template` | `get_prompt_template` | Get current default prompt template |
| PUT | `/api/analyses/prompt-template` | `update_prompt_template` | Update default prompt template |

### Analysis Orchestration Flow

**Text input (POST `/api/analyses`):**
1. Validate request (name required, input_text non-empty)
2. Insert `analyses` row with status=pending
3. Parse text via `DocumentParser::parse_text()`
4. Store extracted_text in DB
5. Run `DeterministicMatcher::analyze()` — this returns `MatchingResult`
6. Store findings in `analysis_findings` table
7. Update analysis: status=completed, matched_framework_ids, processing_time_ms, token_count
8. Log to audit_log
9. Return analysis with summary

**File upload (POST `/api/analyses/upload`):**
1. Accept multipart: file + name + description
2. Validate file (size ≤ 20MB, extension .pdf/.docx)
3. Save file to `backend/uploads/{analysis_id}/`
4. Insert analyses row with status=pending, input_type, original_filename, file_path
5. Parse file via `DocumentParser::parse()`
6. Store extracted_text
7. Run matcher (same as steps 5-9 above)

**Error handling:** If parsing or matching fails, set status=failed with error_message. Return the analysis object (user can see it failed and why).

### Export Generation

**PDF Export** (using `genpdf`):

Report structure:
- Title page: Analysis name, date, status
- Executive summary: Framework count, total findings, gap count, addressed count
- Coverage chart: Rendered as an image embedded in the PDF
- Per-framework sections:
  - Framework name and description
  - Radar chart image (coverage by concept area)
  - Findings table: priority, concept code, name, finding type, recommendation, source reference
- Appendix: Full document text excerpt (first 2000 chars)

**DOCX Export** (using `docx-rs`):

Same structure as PDF. Tables formatted with heading rows. Charts embedded as PNG images.

**Chart rendering for export:**
- The backend needs to render charts as PNG images for embedding
- Options: (a) use a Rust charting library like `plotters` to render server-side, (b) capture chart images from frontend via an endpoint
- Recommended: `plotters` crate for server-side rendering — self-contained, no browser dependency

### Audit Logging

Log these events to the existing `audit_log` table:

| Action | Entity Type | Details |
|--------|------------|---------|
| `analysis_created` | `analysis` | analysis_id, input_type, name |
| `analysis_completed` | `analysis` | analysis_id, framework_count, finding_count, processing_time_ms |
| `analysis_failed` | `analysis` | analysis_id, error_message |
| `analysis_deleted` | `analysis` | analysis_id, name |
| `analysis_exported` | `analysis` | analysis_id, format (pdf/docx) |
| `prompt_template_updated` | `config` | old template hash, new template hash |

### Prompt Template Storage

Default prompt template stored as a JSON file at `backend/config/default-prompt-template.json`. User-specific overrides passed per analysis. The PUT endpoint updates the default file.

### Router Registration

Add `.nest("/analyses", features::analysis::routes::router())` to `api_routes()` in `backend/src/lib.rs`.

Register new handlers in OpenAPI doc (`#[openapi(paths(...))]` in main.rs).

## Key Decisions

- **Synchronous processing** — Analysis runs inline during the POST request. For MVP with deterministic matching, this is fast enough (< 2 seconds). Phase 2 with LLM may need async/background processing.
- **Server-side chart rendering** — Use `plotters` crate to render coverage heatmap, radar chart, and priority breakdown as PNG images for PDF/DOCX embedding. Avoids browser dependency.
- **Charts in exports** — User explicitly requested chart images embedded in exported reports.
- **Soft delete** — DELETE endpoint sets status='deleted', does not remove data. Audit log entry preserved.

## Dependencies

- **Needs from 01-db-models:** All model structs (Analysis, AnalysisFinding, etc.), enums, MatchingEngine trait
- **Needs from 02-document-parsing:** `DocumentParser`, `ParsedDocument`, file upload utilities
- **Needs from 03-matching-engine:** `DeterministicMatcher` (the concrete implementation)
- **Provides to 05-frontend-dashboard:** All REST endpoints the frontend consumes

## New Cargo Dependencies

- `genpdf` — PDF generation
- `plotters` — Server-side chart rendering (PNG output)
- `plotters-bitmap` — Bitmap backend for plotters

## Existing Patterns to Follow

- Route handlers: See `backend/src/features/compliance/routes.rs` for CRUD patterns, pagination, error handling
- Audit logging: See compliance routes for `audit_log` INSERT patterns
- OpenAPI: Use `#[utoipa::path(...)]` annotations on handlers
- Router nesting: Follow `api_routes()` pattern in `backend/src/lib.rs`
- Pagination: Use existing `PaginatedResponse<T>` wrapper from ontology models
- Error responses: Use existing `AppError` / `AppResult` types
