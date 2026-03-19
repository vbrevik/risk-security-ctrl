# TDD Plan: Analysis Detail Page with Charts

Testing framework: Vitest + React Testing Library + jsdom. Mocking: `vi.mock` for i18n and API modules. Hook testing via `renderHook` with QueryClient wrapper.

---

## 3.1 Prerequisite Fixes, i18n Keys & Route Shell

### PaginatedResponse fix
- Test: useAnalyses hook returns items under `.items` property (not `.data`)
- Test: useFindings hook returns findings under `.items` property
- Test: refetchInterval in useAnalyses checks `data?.items.some(...)` for processing status

### AnalysisFinding nullability
- Test: Finding with null concept_code/concept_name/concept_definition renders without crash
- Test: Finding with null evidence_text/recommendation renders without crash

### Route shell ($id.tsx)
- Test: Route renders loading skeleton when useAnalysis is loading
- Test: Route renders error state when useAnalysis returns error
- Test: Route renders "not found" message for 404 error
- Test: Route shows processing banner when status is "processing"
- Test: Route calls useAnalysis with refetchInterval when status is processing
- Test: Route renders detail content when analysis is completed
- Test: Route shows EmptyFindings when completed with zero findings

---

## 3.2 Summary Statistics & useChartData Hook

### useChartData hook
- Test: Returns zero counts when findings array is empty
- Test: Returns zero counts when findings is undefined
- Test: Computes correct typeCounts (addressed, partiallyAddressed, gap, notApplicable, total)
- Test: Computes correct frameworkCoverage with percentage per framework
- Test: Computes correct priorityCounts for P1-P4
- Test: Handles findings with mixed framework_ids correctly
- Test: frameworkCoverage percentage = addressed / total per framework × 100

### SummaryStats component
- Test: Renders 6 stat cards
- Test: Displays total findings count
- Test: Displays addressed count with percentage
- Test: Displays gaps count with percentage
- Test: Displays frameworks count
- Test: Displays formatted processing time (e.g., "2.3s")
- Test: Displays formatted token count
- Test: Renders skeleton state when isLoading is true

---

## 3.3 D3 Chart Components

### useContainerDimensions hook
- Test: Returns initial dimensions of { width: 0, height: 0 }
- Test: Updates dimensions when ResizeObserver fires
- Test: Cleans up observer on unmount
- Test: Debounces rapid resize events

### CoverageHeatmap component
- Test: Renders SVG element inside a Card
- Test: Renders correct number of bars matching data length
- Test: Shows chart title from i18n
- Test: Shows "No data" placeholder when data is empty
- Test: Renders with accessibility attributes (role="img", aria-labelledby)
- Test: Does not crash when data changes between renders

### PriorityChart component
- Test: Renders SVG element inside a Card
- Test: Renders 4 bars for P1-P4
- Test: Shows chart title from i18n
- Test: Shows "No data" placeholder when all counts are zero
- Test: Renders with accessibility attributes

---

## 3.4 Findings Table with Filters

### FindingTypeTag component
- Test: Renders green badge for "addressed" type
- Test: Renders yellow badge for "partially_addressed" type
- Test: Renders red badge for "gap" type
- Test: Renders gray badge for "not_applicable" type
- Test: Displays i18n label for each type

### FindingsTable component
- Test: Renders table with correct column headers
- Test: Renders a row for each finding
- Test: Displays concept_code or "—" fallback for null
- Test: Displays concept_name or "—" fallback for null
- Test: Displays confidence as percentage (e.g., "85%")
- Test: Expand button toggles row expansion
- Test: Expanded row shows evidence text
- Test: Expanded row shows recommendation text
- Test: Expanded row shows concept definition or "—" for null
- Test: Expand button has aria-expanded attribute
- Test: Multiple rows can be expanded simultaneously
- Test: Collapsing a row hides expanded content

### FindingsFilters (if separate component)
- Test: Renders three Select dropdowns
- Test: Framework dropdown populated from matched_framework_ids
- Test: Selecting a filter calls onChange with updated params
- Test: "All" option clears the filter value

### Pagination
- Test: Displays "Page X of Y" text
- Test: Previous button disabled on page 1
- Test: Next button disabled on last page
- Test: Clicking Next increments page
- Test: Clicking Previous decrements page

---

## 3.5 Export Buttons & Empty State

### ExportButtons component
- Test: Renders PDF and DOCX export buttons
- Test: Buttons disabled when status is not "completed"
- Test: Disabled buttons show tooltip text
- Test: Clicking PDF button calls exportAnalysis with "pdf" format
- Test: Clicking DOCX button calls exportAnalysis with "docx" format
- Test: Shows loading spinner on clicked button during export
- Test: Both buttons disabled while any export is in progress
- Test: Shows error state when export fails

### EmptyFindings component
- Test: Renders "No compliance findings detected" heading
- Test: Renders suggestion text
- Test: Renders link to /analysis/settings
- Test: Link navigates to settings page

---

## 3.6 Page Assembly & Integration

### AnalysisDetailPage (route integration)
- Test: Extracts id from route params and passes to useAnalysis
- Test: Passes filter state to useFindings
- Test: Changing filter resets page to 1
- Test: Changing page updates useFindings params
- Test: Toggle expand adds/removes finding id from expandedIds set
- Test: Completed analysis renders SummaryStats, charts, and table
- Test: Processing analysis shows only header and processing banner
- Test: Failed analysis shows error message
