# Opus Review

**Model:** claude-opus-4
**Generated:** 2026-03-19T13:40:00Z

---

## Plan Review: Analysis Detail Page with Charts

### 1. Critical Type Mismatch Between Frontend Types and Backend Response

The `AnalysisFinding` TypeScript interface declares `concept_code`, `concept_name`, and `concept_definition` as **required strings**, but the backend returns them from a LEFT JOIN as `Option<String>` values (null in JSON). The plan mentions null handling but never calls out that the TypeScript type itself needs to be fixed. Similarly, `evidence_text` and `recommendation` are `Option<String>` in the backend but required `string` in the frontend type.

### 2. Backend Response Shape Mismatch with PaginatedResponse

The backend findings endpoint returns `{ "items": [...] }` but the `PaginatedResponse<T>` type uses `data`. The `useFindings` hook casts the response to `PaginatedResponse<AnalysisFinding>`, so `.data` would be `undefined`. This is a pre-existing bug from split 01 that will surface now.

### 3. Hardcoded Limit of 1000 for "All Findings" Is Fragile

Silent data truncation risk. Should display a warning when `total > 1000` indicating stats are approximate, or paginate through all results.

### 4. SQL Injection in Backend Findings Endpoint

String formatting used for SQL queries with weak single-quote escaping. Also, the `priority` filter is not applied in the backend query at all.

### 5. Missing `sort_by` Backend Support

The backend ordering is hardcoded (`ORDER BY f.priority ASC, f.sort_order ASC`). The `sort_by` query param is never read. All sorting described in the plan will be ignored server-side.

### 6. Client-Side Secondary Sort on Paginated Data Is Semantically Wrong

Client-side secondary sorting only reorders the 20 items on the current page, not the full dataset. Combined with backend ignoring `sort_by`, multi-column sort is broken by design. Recommend: drop multi-column sort and use single-column backend sort, or fetch all data client-side.

### 7. Missing `useContainerDimensions` Hook in Test Manifest

No test file listed for the ResizeObserver hook.

### 8. D3 "Clear and Redraw" Pattern Performance

Full SVG teardown on resize is acceptable for simple charts but should be noted as not scalable.

### 9. `useAnalysis` staleTime vs Fresh Status

5-minute staleTime may show outdated status when navigating back. Should add `refetchOnMount: 'always'` for the detail page.

### 10. No Loading Guard on "All Findings" Query Transition

Brief window after status transitions to completed where findings haven't loaded yet. Stats/charts need a loading state for this.

### 11. Export Function Filename

Hardcoded as `analysis-${id}.${format}` — could use `analysis.name` for better UX.

### 12. Keyboard Navigation for Table Row Expansion

Plan says "Click a row" but also has a "button in first cell." Need to clarify: chevron button only or entire row clickable?

### 13. Filter Reset Does Not Reset Pagination

Changing filters without resetting page to 1 causes empty pages.

### 14. Missing Components in File Manifest

`ProcessingBanner` and `ChartsSection` appear in component tree but not in file manifest.

### 15. Norwegian Translations

~50 new keys but no Norwegian text specified.

### Summary of Blocking Issues

1. **PaginatedResponse field name mismatch** (`data` vs `items`)
2. **TypeScript types missing nullability** on concept/evidence/recommendation fields
3. **Backend does not implement `sort_by` or `priority` filter**
4. **Multi-column sort on paginated data is logically broken**
