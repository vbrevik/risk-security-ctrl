# Interview: Backend API & Report Generation

## Q1: Processing model for file uploads
**Q:** Should analysis run synchronously during the upload request or return immediately with background processing?
**A:** Synchronous (spec default) — block until analysis completes, return full result.

## Q2: Export scope — charts vs text-only
**Q:** Should chart rendering (plotters radar/heatmap/bar) be built now or deferred?
**A:** Full charts now — implement plotters radar/heatmap/bar charts embedded in PDF/DOCX reports.

## Q3: Prompt template storage
**Q:** Single global file or per-user database storage?
**A:** Single global file — one `default-prompt-template.json`, any user can update it.

## Q4: Soft delete visibility
**Q:** Should deleted analyses appear in list by default?
**A:** Hidden by default — excluded from list unless `?include_deleted=true`.

## Q5: Default findings sort order
**Q:** What should the default sort be for GET /analyses/:id/findings?
**A:** Priority ascending (P1 first) — most critical gaps shown first.
