# Interview Transcript: Analysis Detail Page with Charts

## Q1: Processing state behavior on detail page

**Q:** When a user navigates to a detail page for an analysis that's still 'processing', what should happen?

**A:** Auto-poll and live update. Refetch every few seconds, charts/table appear as findings arrive.

## Q2: Chart placement in page layout

**Q:** Should the coverage heatmap and priority chart be visible on the main Overview tab alongside summary stats, or have their own dedicated tab?

**A:** Charts on Overview tab. Summary stats + charts together as the landing view.

## Q3: Finding detail expansion pattern

**Q:** Should finding details load inline in the expanded row, or use a side panel/dialog?

**A:** Inline expansion. Expanded row shows evidence, recommendation, concept definition directly in table.

## Q4: Export button placement

**Q:** Where should export buttons be positioned? Should we show a progress indicator for large exports?

**A:** Header actions area. Export buttons in the page header alongside back navigation and status.

## Q5: Chart data source

**Q:** Should we fetch all findings client-side for chart computation, or add a backend endpoint for aggregated chart data?

**A:** Client-side aggregation. Fetch all findings, compute framework stats in JS. Simpler, analyses have <200 findings.

## Q6: Table sorting capability

**Q:** Should the findings table support multi-column sorting or single-column sort?

**A:** Multi-column sort. Hold shift+click to add secondary sort columns.

## Q7: Export guard for non-completed analyses

**Q:** When a user tries to export a processing/failed analysis, should we disable buttons with tooltip, hide them, or show error on click?

**A:** Disable with tooltip. Buttons visible but grayed out, hover shows "Analysis must be completed to export".

## Q8: Zero-findings empty state

**Q:** For completed analyses with no findings, what should the empty state look like?

**A:** Informative empty state. Illustration/icon + message + suggestion to adjust settings or re-run.
