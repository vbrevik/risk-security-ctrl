# Condensed Spec: Backend API & Report Generation

## Scope

REST endpoints for the Document Analysis Engine: analysis CRUD, multipart file upload, findings queries, PDF/DOCX export with embedded chart visualizations, audit logging, and prompt template management.

## Endpoints

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| POST | `/api/analyses` | `create_analysis` | Create from text input (JSON body) |
| POST | `/api/analyses/upload` | `upload_analysis` | Create from file upload (multipart, ≤20MB, .pdf/.docx) |
| GET | `/api/analyses` | `list_analyses` | Paginated, filterable by status. Deleted hidden by default. |
| GET | `/api/analyses/:id` | `get_analysis` | Analysis with summary stats |
| GET | `/api/analyses/:id/findings` | `get_findings` | Filter: framework_id, finding_type, priority. Default sort: priority ASC. |
| DELETE | `/api/analyses/:id` | `delete_analysis` | Soft-delete (status → deleted) |
| GET | `/api/analyses/:id/export/:format` | `export_analysis` | PDF or DOCX with charts |
| GET | `/api/analyses/prompt-template` | `get_prompt_template` | Read global config file |
| PUT | `/api/analyses/prompt-template` | `update_prompt_template` | Write global config file |

## Processing Model

- **Synchronous** — analysis runs inline during POST request. DeterministicMatcher is fast enough for MVP (<2s).
- On failure: set status=failed with error_message, return the analysis object.

## Analysis Orchestration

**Text input flow:**
1. Validate (name required, input_text non-empty)
2. Insert `analyses` row (status=pending)
3. Parse text via `DocumentParser::parse_text()`
4. Run `DeterministicMatcher::analyze()` → `MatchingResult`
5. Store findings in `analysis_findings`
6. Update analysis: status=completed, matched_framework_ids, processing_time_ms, token_count
7. Audit log
8. Return analysis with summary

**File upload flow:**
Same but adds: multipart parsing, file validation (size ≤20MB, extension .pdf/.docx), save to `uploads/{analysis_id}/`, parse via `DocumentParser::parse()`.

## Export Generation

**PDF** (genpdf + plotters):
- Title page, executive summary
- Coverage heatmap (plotters rectangle grid, green-red interpolation)
- Per-framework sections with radar chart (manual polygon implementation)
- Findings tables (priority, code, name, type, recommendation, reference)
- Priority breakdown bar chart
- Appendix: first 2000 chars of document text

**DOCX** (docx-rs): Same structure, charts embedded as PNG images.

**Chart rendering:** All charts rendered server-side via `plotters` `BitMapBackend` → `image` crate PNG encoding → embedded in export documents.

## Prompt Template

- Single global file: `backend/config/default-prompt-template.json`
- GET returns file contents, PUT overwrites it
- Audit logged on update

## Audit Logging

Events: analysis_created, analysis_completed, analysis_failed, analysis_deleted, analysis_exported, prompt_template_updated. Use existing `audit_log` table pattern.

## Key Decisions

- Synchronous processing (interview Q1)
- Full charts in exports now (interview Q2)
- Single global prompt template file (interview Q3)
- Deleted analyses hidden from list by default (interview Q4)
- Findings sorted by priority ascending by default (interview Q5)

## Dependencies

- Split 01: DB models, enums
- Split 02: DocumentParser
- Split 03: DeterministicMatcher
- New crates: `genpdf` (with images feature), `plotters`, `image`

## Existing Patterns

- Route handlers: `AppResult<Json<T>>`, `(StatusCode, Json<T>)` for 201
- Audit: JSON-serialized new_value in audit_log INSERT
- Pagination: `PaginatedResponse<T>` with page/limit defaults
- OpenAPI: `#[utoipa::path]` with tag, responses, params
- File upload: `Multipart` extractor, `tokio::fs::write`
- Router: `.nest("/analyses", routes::router())` in `api_routes()`
