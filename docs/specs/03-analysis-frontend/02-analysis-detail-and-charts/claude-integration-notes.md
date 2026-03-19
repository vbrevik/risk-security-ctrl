# Integration Notes: Opus Review Feedback

## Integrated Changes

### 1. Fix PaginatedResponse field name mismatch (Blocking)
**Integrating.** Backend returns `"items"` but `PaginatedResponse<T>` uses `data`. This affects both the ontology and analysis list endpoints. Fix: update `PaginatedResponse` to use `items` instead of `data`, and update all consumers. This is a prerequisite fix in section 3.1.

### 2. Fix AnalysisFinding nullability (Blocking)
**Integrating.** Backend returns `evidence_text`, `recommendation`, `concept_code`, `concept_name`, `concept_definition` as nullable (`Option<String>`). TypeScript types must reflect this with `string | null`. Fix in section 3.1 as a prerequisite.

### 3. Drop multi-column sort, use fixed backend sort (Blocking)
**Integrating.** Backend hardcodes `ORDER BY f.priority ASC, f.sort_order ASC` and ignores `sort_by`. Multi-column client-side sort on paginated data is semantically wrong. Decision: **drop multi-column sort entirely**. The table will display sort indicators showing the fixed sort order (priority ASC) but won't support user-changeable sorting. This simplifies the implementation significantly. Backend sort/filter fixes are out of scope for this frontend split.

### 4. Priority filter not implemented in backend
**Noting but not fixing.** The priority filter dropdown will still be rendered in the UI and pass the param to the backend, but it won't work until the backend is updated. We'll add a TODO comment. This is a minor issue since the default sort already groups by priority.

### 5. Add `refetchOnMount: 'always'` for detail page
**Integrating.** When navigating back to a detail page, the 5-minute staleTime could show stale status. The detail page's `useAnalysis` call should override with `refetchOnMount: 'always'`.

### 6. Add loading state for chart data transition
**Integrating.** When status transitions from processing → completed, there's a brief window where findings haven't loaded yet. SummaryStats and charts will show a loading skeleton during this transition.

### 7. Filter reset resets pagination
**Integrating.** Changing any filter will reset page to 1. Simple and correct.

### 8. Add useContainerDimensions test file
**Integrating.** Added to test manifest.

### 9. Clarify expansion trigger — chevron button only
**Integrating.** Expansion triggered by chevron button in first cell only, not entire row. This is simpler and more accessible.

### 10. Export filename uses analysis name
**Integrating.** Change `exportAnalysis` call to pass analysis name for a user-friendly filename.

### 11. Add ProcessingBanner and ChartsSection to file manifest
**Integrating.** These will be inline in `$id.tsx` rather than separate components — the route file is their only consumer and they're small enough to not warrant extraction.

## Not Integrated

### SQL injection in backend
**Not integrating.** This is a backend security issue, not in scope for this frontend implementation plan. Will flag separately.

### Hardcoded limit of 1000 warning
**Not integrating.** Analyses have <200 findings in practice. Adding a warning for >1000 is premature. If we encounter analyses with more findings, we'll add server-side aggregation.

### D3 join-based update pattern note
**Not integrating.** The clear-and-redraw pattern is fine for these simple charts. No need to add complexity for hypothetical future scaling.

### Norwegian translation text
**Not integrating in plan.** The implementer will provide Norwegian translations during section implementation, following the existing pattern.
