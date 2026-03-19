# TDD Plan: Backend API & Report Generation

Testing follows the existing integration test pattern: `create_test_app()` → `.oneshot(Request::builder()...)` → assert response.

## Section 2: Route Handler Module

### test_router_registration
- GET `/api/analyses` returns 200 (not 404) → proves route is registered

## Section 4: Analysis Creation

### test_create_analysis_text_input
- POST `/api/analyses` with valid name + input_text → 201, response has id, status

### test_create_analysis_empty_name_rejected
- POST with empty name → 400

### test_create_analysis_empty_text_rejected
- POST with empty input_text → 400

### test_create_analysis_oversized_text_rejected
- POST with >500KB input_text → 400

### test_create_analysis_produces_findings
- POST with security-related text → response has matched_framework_ids non-empty

### test_create_analysis_failed_sets_status
- POST with text that triggers parsing error → 201 with status=failed, error_message present

## Section 5: File Upload

### test_upload_pdf_file
- POST multipart with valid PDF → 201, input_type=pdf

### test_upload_docx_file
- POST multipart with valid DOCX → 201, input_type=docx

### test_upload_invalid_extension_rejected
- POST with .txt file → 400

### test_upload_missing_file_rejected
- POST multipart without file field → 400

### test_upload_missing_name_rejected
- POST multipart without name field → 400

## Section 6: List and Get

### test_list_analyses_paginated
- Create 3 analyses, GET with limit=2 → response has 2 items, total=3, total_pages=2

### test_list_analyses_excludes_deleted
- Create analysis, DELETE it, GET list → deleted not in results

### test_list_analyses_include_deleted
- DELETE analysis, GET with include_deleted=true → deleted appears

### test_list_analyses_filter_by_status
- GET with status=completed → only completed analyses

### test_get_analysis_returns_summary
- Create analysis with findings, GET → response has total_findings, gap_count etc

### test_get_analysis_not_found
- GET nonexistent ID → 404

### test_get_findings_default_sort
- GET findings → ordered by priority ASC

### test_get_findings_filter_by_framework
- GET with framework_id=X → only findings from that framework

### test_get_findings_filter_by_type
- GET with finding_type=gap → only gap findings

### test_get_findings_returns_concept_metadata
- GET findings → response items have concept name_en and code fields

## Section 7: Delete

### test_delete_analysis_soft_delete
- DELETE → 204, GET → 404, list with include_deleted → visible with status=deleted

### test_delete_nonexistent_returns_404
- DELETE nonexistent ID → 404

## Section 8: Chart Rendering

### test_render_coverage_heatmap_returns_png
- Call with sample data → result is non-empty, starts with PNG magic bytes (0x89504E47)

### test_render_radar_chart_returns_png
- Call with labels + values → valid PNG bytes

### test_render_priority_chart_returns_png
- Call with priority counts → valid PNG bytes

### test_render_heatmap_empty_data
- Call with empty frameworks → returns error or empty image (not panic)

### test_render_radar_empty_labels
- Call with no labels → returns error (not panic)

## Section 9: PDF Export

### test_generate_pdf_returns_bytes
- Call with analysis + findings → non-empty result, starts with %PDF header

### test_generate_pdf_contains_analysis_name
- Render PDF, check bytes contain analysis name string

## Section 10: DOCX Export

### test_generate_docx_returns_bytes
- Call with analysis + findings → non-empty result, starts with PK (ZIP) header

## Section 11: Export Handler

### test_export_pdf_returns_pdf_content_type
- GET /analyses/:id/export/pdf → 200 with Content-Type: application/pdf

### test_export_docx_returns_docx_content_type
- GET /analyses/:id/export/docx → 200 with correct Content-Type

### test_export_invalid_format_returns_400
- GET /analyses/:id/export/csv → 400

### test_export_nonexistent_analysis_returns_404
- GET /analyses/fake-id/export/pdf → 404

## Section 12: Prompt Template

### test_get_prompt_template_returns_defaults
- GET → 200, response has addressed_threshold, partial_threshold fields

### test_update_prompt_template
- PUT with valid config → 200, subsequent GET returns updated values

### test_update_prompt_template_invalid_json
- PUT with malformed JSON → 400

## Section 13: Audit Logging

### test_create_analysis_creates_audit_entry
- POST analysis → audit_log has entry with action=analysis_created

### test_delete_analysis_creates_audit_entry
- DELETE analysis → audit_log has entry with action=analysis_deleted

### test_export_creates_audit_entry
- GET export → audit_log has entry with action=analysis_exported
