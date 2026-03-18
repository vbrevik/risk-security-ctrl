<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-dependencies-and-appstate
section-02-route-scaffold-and-wiring
section-03-create-analysis-text
section-04-file-upload
section-05-list-get-delete
section-06-findings-endpoint
section-07-chart-rendering
section-08-pdf-export
section-09-docx-export
section-10-export-handler
section-11-prompt-template
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-dependencies-and-appstate | - | 02, 03, 04, 07 | Yes |
| section-02-route-scaffold-and-wiring | 01 | 03, 04, 05, 06, 10, 11 | Yes |
| section-03-create-analysis-text | 01, 02 | 04, 05 | Yes |
| section-04-file-upload | 01, 02, 03 | - | No |
| section-05-list-get-delete | 02, 03 | 06, 10 | No |
| section-06-findings-endpoint | 02, 05 | 10 | No |
| section-07-chart-rendering | 01 | 08, 09 | Yes (parallel with 03-06) |
| section-08-pdf-export | 07 | 10 | No |
| section-09-docx-export | 07 | 10 | No |
| section-10-export-handler | 02, 06, 08, 09 | - | No |
| section-11-prompt-template | 02 | - | Yes (parallel with 03-10) |

## Execution Order

1. section-01-dependencies-and-appstate (no deps)
2. section-02-route-scaffold-and-wiring, section-07-chart-rendering (parallel after 01)
3. section-03-create-analysis-text, section-11-prompt-template (parallel after 02)
4. section-04-file-upload (after 03)
5. section-05-list-get-delete (after 03)
6. section-06-findings-endpoint (after 05)
7. section-08-pdf-export, section-09-docx-export (parallel after 07)
8. section-10-export-handler (after 06, 08, 09)

## Section Summaries

### section-01-dependencies-and-appstate
Add Cargo dependencies (genpdf, plotters, image, docx-rs). Add `topics: Vec<Topic>` to AppState. Load topics at startup. Download LiberationSans fonts.

### section-02-route-scaffold-and-wiring
Create `routes.rs` with router function and empty handler stubs. Nest router in lib.rs. Add `pub mod routes;` to mod.rs. Verify compilation.

### section-03-create-analysis-text
Implement `create_analysis` handler: validation, DB insert, parser (spawn_blocking), matcher (30s timeout), findings storage, audit logging.

### section-04-file-upload
Implement `upload_analysis` handler: multipart parsing, file validation, UUID filename storage, parse via spawn_blocking, reuse analysis flow from 03.

### section-05-list-get-delete
Implement `list_analyses` (pagination, exclude deleted, include_deleted param), `get_analysis` (summary stats), `delete_analysis` (soft-delete + audit).

### section-06-findings-endpoint
Implement `get_findings`: JOIN with concepts for AnalysisFindingWithConcept, filtering, sort_by allowlist validation, pagination.

### section-07-chart-rendering
Implement `charts.rs`: coverage heatmap, radar chart (manual polygon), priority bar chart. All return PNG bytes.

### section-08-pdf-export
Implement `export_pdf.rs`: genpdf document with title, summary, charts, findings tables, appendix.

### section-09-docx-export
Implement `export_docx.rs`: docx-rs document with same structure, charts embedded as PNG.

### section-10-export-handler
Implement `export_analysis` handler: format validation, load data, call generator, Content-Type/Disposition headers, audit log.

### section-11-prompt-template
Implement `get_prompt_template` and `update_prompt_template`: file read/write with validation, default fallback, audit logging.
