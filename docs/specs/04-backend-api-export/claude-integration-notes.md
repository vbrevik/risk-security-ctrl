# Integration Notes: Review Feedback

## Integrating

| Issue | Action |
|-------|--------|
| CRITICAL-1: docx-rs not present | Add `docx-rs` to Cargo.toml explicitly. Verify image embedding API. |
| CRITICAL-2: Types diverge from models.rs | Use existing model types. Reference AnalysisListQuery, FindingsListQuery, AnalysisSummary as-is. Add `include_deleted` field to existing AnalysisListQuery. |
| CRITICAL-3: Blocking parser I/O | Wrap parser calls in `tokio::task::spawn_blocking()`. |
| HIGH-2: Path traversal | Use `{analysis_id}.{extension}` as stored filename, keep original name in DB only. |
| HIGH-3: No max input_text | Add 500KB max text length validation. |
| HIGH-4: No timeout | Add `tokio::time::timeout(Duration::from_secs(30), ...)` around analysis. |
| MED-4: SQL ORDER BY injection | Validate sort_by against allowlist, interpolate as string literal. |
| MED-5: Topic loading | Load topics into AppState at startup. Add `topics: Vec<Topic>` to AppState. |
| MED-6: sort_order | Assign from finding index in results vector. |
| Spec gap: AnalysisFindingWithConcept | Use existing type for findings endpoint and export modules. |
| Spec gap: Failed analysis HTTP status | Return 201 with status=failed in body (creation succeeded, analysis failed). |

## Not Integrating

| Issue | Rationale |
|-------|-----------|
| HIGH-1: Move prompt template to DB | Overkill for MVP. File-based is fine for single-server deployment. Will add write lock. |
| HIGH-5: File cleanup | Out of scope for this split. Note as future work. |
| MED-1: Chart dependency bloat | User explicitly requested full charts now. Accepted. |
| MED-2: Font files | Will add download step and commit fonts. Handle missing fonts with error, not panic. |
| MED-3: genpdf/image version compat | Will verify at build time. Adjust versions if needed. |
| LOW-2: GET 404 for deleted | Intentional — deleted means inaccessible. List with include_deleted is admin-level. |
| LOW-3: Rate limiting | MVP scope. Note as known limitation. |
| LOW-4: Export caching | MVP scope. Note as optimization. |
| LOW-5: No test plan | TDD plan comes in next step. |
