# Analysis Detail Page with Charts

## Goal

Build the analysis detail page at `/analysis/{id}` with summary statistics, interactive data visualization charts, an expandable findings table with filtering, and export functionality.

## Context

- **Requirements:** See `../requirements.md` for backend API spec and type definitions
- **Interview decisions:** See `../deep_project_interview.md` — charts + tables, expandable findings rows
- **Depends on:** `01-analysis-foundation` provides TypeScript types, API hooks (`useAnalysis`, `useFindings`, `useExportAnalysis`), i18n namespace, and route structure
- **Provides to:** Nothing (this is the final split)

## Existing Patterns

- D3.js is already a project dependency (used in ontology graph visualization)
- The ontology feature has a `useD3Graph` hook that can serve as a reference for D3 integration
- Charts in the backend export (charts.rs) render: coverage heatmap, radar chart, priority bar chart
- The frontend charts should visualize the same data but as interactive SVG (not static PNG)

## Scope

### 1. Analysis Detail Page Shell (`/analysis/{id}`)
- Load analysis via `useAnalysis(id)` — show loading/error states
- Page header: analysis name, status badge, input type, created date
- Navigation back to list
- Tabs or sections for: Overview, Findings, Export

### 2. Summary Statistics Cards
Row of stat cards at the top:
- Total findings count
- Addressed count (with percentage)
- Gaps count (with percentage)
- Frameworks matched (count + list)
- Processing time
- Token count

### 3. Coverage Heatmap Chart
- Per-framework horizontal bar chart showing coverage percentage
- Coverage = addressed findings / total findings per framework
- Color gradient: red (0%) → yellow (50%) → green (100%)
- Framework name labels on the left
- Interactive: hover shows exact percentage tooltip
- Uses D3.js for rendering into an SVG element

### 4. Priority Breakdown Chart
- Vertical bar chart showing finding counts by priority level (P1-P4)
- Color-coded: P1=red, P2=orange, P3=yellow, P4=green
- Interactive: hover shows count tooltip
- Uses D3.js

### 5. Expandable Findings Table
- Columns: Concept Code, Concept Name, Framework, Type, Priority, Confidence
- **Filters** (above table):
  - Framework dropdown (populated from analysis.matched_framework_ids)
  - Finding type dropdown (addressed, partially_addressed, gap, not_applicable)
  - Priority dropdown (P1-P4)
- **Sorting:** Click column headers to sort (priority ASC default)
- **Pagination:** Via `useFindings(id, params)` with page/limit
- **Row expansion:** Click a row to expand and show:
  - Evidence text
  - Recommendation
  - Concept definition
  - Source reference (if available)
- **Type badges:** Color-coded by finding_type (addressed=green, partial=yellow, gap=red, n/a=gray)

### 6. Export Buttons
- Two buttons: "Export PDF" and "Export DOCX"
- On click: call `GET /api/analyses/{id}/export/{format}`
- Trigger browser file download with correct filename
- Show loading state while generating
- Error handling if export fails (analysis not completed, server error)

## Technical Considerations

### Chart Library Approach
D3.js is already available. Two approaches:
- **Direct D3 in useEffect:** More control, matches existing useD3Graph pattern
- **Wrapper components:** Create `<CoverageHeatmap data={...} />` and `<PriorityChart data={...} />` components that encapsulate D3 logic

Recommend wrapper components for cleaner React integration and reusability.

### Data Flow for Charts
Charts need aggregated data computed from the findings list. The backend paginated findings endpoint returns findings per-page, but charts need all findings for the analysis. Options:
- Fetch all findings (no pagination) for chart data — add a dedicated hook or use large limit
- Compute chart data server-side — would require new backend endpoint (out of scope)
- Use the analysis summary stats already in the analysis response for basic counts; only the per-framework breakdown requires iterating findings

Recommend: Fetch findings with a large limit (1000) for chart computation, since analyses typically have <200 findings.

### File Download
Use `axios` with `responseType: 'blob'` for export, then create a temporary `<a>` element with `URL.createObjectURL` for the download trigger.

## Constraints

- All text via `useTranslation('analysis')` (i18n keys added in split 01)
- Use existing shadcn/ui components where applicable
- D3 charts render into SVG — no canvas
- Mobile: charts should be responsive (scale with container width)
- Graceful handling of analyses with zero findings (show "no findings" state)
