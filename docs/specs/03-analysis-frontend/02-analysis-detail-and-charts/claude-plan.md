# Implementation Plan: Analysis Detail Page with Charts

## 1. Context & Goal

This plan covers split 02 of the analysis frontend — the detail page at `/analysis/$id`. It builds on split 01 (analysis-foundation), which already provides TypeScript types, API hooks, i18n namespace, StatusBadge, and a route stub.

The detail page renders a completed document analysis with:
- Summary statistics cards
- Two interactive D3.js charts (coverage heatmap, priority breakdown)
- An expandable findings table with filtering and pagination
- Export buttons (PDF/DOCX) in the page header

The page also handles in-progress analyses via auto-polling and displays informative empty states.

## 2. Architecture Overview

### Page Structure

The page is a **single scrollable view** (no tabs) with four sections stacked vertically:

1. **Header** — back link, title, status badge, metadata, export buttons
2. **Summary Stats** — row of 6 stat cards
3. **Charts** — two-column grid (coverage heatmap + priority chart), stacked on mobile
4. **Findings Table** — filters, expandable table, pagination

### Data Flow

The page makes three data requests:

| Hook | Purpose | Parameters |
|------|---------|------------|
| `useAnalysis(id)` | Analysis metadata, status, framework IDs | `refetchInterval` when processing, `refetchOnMount: 'always'` |
| `useFindings(id, { limit: 1000 })` | All findings for chart/stat aggregation | Cached, enabled only when completed |
| `useFindings(id, { page, limit: 20, filters })` | Paginated findings for table display | Updates as user filters/pages |

A custom `useChartData(findings)` hook computes aggregated data from the full findings array:
- Per-framework coverage percentages (for heatmap)
- Per-priority counts (for bar chart)
- Addressed/gap/partial counts with percentages (for stat cards)

### Component Tree

```
AnalysisDetailPage (route)
├── PageHeader (inline in $id.tsx)
│   ├── BackLink → /analysis
│   ├── Title + StatusBadge (reuse)
│   ├── MetaInfo (input type, date)
│   └── ExportButtons (disabled when not completed)
├── ProcessingBanner (inline in $id.tsx)
├── SummaryStats
│   └── StatCard × 6
├── ChartsSection (inline in $id.tsx — grid wrapper)
│   ├── CoverageHeatmap (D3 wrapper)
│   └── PriorityChart (D3 wrapper)
├── FindingsSection (inline in $id.tsx)
│   ├── FindingsFilters (3 Select dropdowns)
│   ├── FindingsTable
│   │   ├── FindingRow × N
│   │   └── ExpandedFindingRow (conditional)
│   └── Pagination controls
└── EmptyFindings (shown when completed + zero findings)
```

### Prerequisite Fixes (from Opus Review)

Before implementing the detail page, two pre-existing bugs in split 01 code must be fixed:

1. **PaginatedResponse field name:** The `PaginatedResponse<T>` interface in `frontend/src/features/ontology/types/index.ts` uses `data: T[]`, but the backend returns `"items"`. Fix: rename the field to `items` and update all consumers (the `useAnalyses` hook at line 43 references `.data` — change to `.items`).

2. **AnalysisFinding nullability:** Five fields are `Option<String>` in the backend but typed as required `string` in TypeScript. Fix: change `evidence_text`, `recommendation`, `concept_code`, `concept_name`, `concept_definition` to `string | null` in the `AnalysisFinding` interface.

**Backend limitations (noted, not fixed in this split):**
- `sort_by` query param is accepted but ignored — backend hardcodes `ORDER BY f.priority ASC, f.sort_order ASC`
- `priority` filter is accepted but not applied in the SQL query
- These don't block the frontend — the table will display data in the backend's fixed sort order and the priority filter dropdown will pass the param (non-functional until backend is updated)

## 3. Sections

### 3.1 Prerequisite Fixes, i18n Keys & Route Shell

**What:** Fix the two pre-existing bugs, add all new i18n translation keys, and update the route stub.

**Prerequisite fix 1 — PaginatedResponse:** In `frontend/src/features/ontology/types/index.ts`, rename the `data` field of `PaginatedResponse<T>` to `items`. Then update all consumers:
- `frontend/src/features/analysis/api/index.ts` line 43: change `data?.data.some(...)` to `data?.items.some(...)`
- Any ontology feature code that references `.data` on paginated responses

**Prerequisite fix 2 — AnalysisFinding nullability:** In `frontend/src/features/analysis/types/index.ts`, change these fields to `string | null`:
- `evidence_text: string | null`
- `recommendation: string | null`
- `concept_code: string | null`
- `concept_name: string | null`
- `concept_definition: string | null`

**i18n keys to add** (namespaced under `detail`, `stats`, `charts`, `findings`, `export`):
- `detail.backToList`, `detail.createdAt`, `detail.inputType`, `detail.processing.banner`, `detail.processing.message`, `detail.failed.message`, `detail.notFound.title`, `detail.notFound.message`
- `stats.totalFindings`, `stats.addressed`, `stats.gaps`, `stats.frameworks`, `stats.processingTime`, `stats.tokenCount`
- `charts.coverage.title`, `charts.coverage.description`, `charts.priority.title`, `charts.priority.description`
- `findings.title`, `findings.filters.*`, `findings.columns.*`, `findings.expand`, `findings.collapse`, `findings.evidence`, `findings.recommendation`, `findings.conceptDefinition`, `findings.sourceReference`, `findings.empty.*`, `findings.type.*`
- `export.pdf`, `export.docx`, `export.disabled`, `export.downloading`, `export.error`

Add to both `en/analysis.json` and `nb/analysis.json`.

**Route update:** Replace the stub in `$id.tsx` with a page shell that:
- Calls `useAnalysis(id)` with `refetchOnMount: 'always'` and conditional `refetchInterval: 5000` when `status === "processing"`
- Renders loading skeleton, error state, or the detail content
- Shows a processing banner when status is processing
- Shows EmptyFindings when completed with zero findings

**Barrel export:** Update `features/analysis/index.ts` with new component exports as they're created.

### 3.2 Summary Statistics & useChartData Hook

**What:** Create the `useChartData` hook and `SummaryStats` component.

**useChartData hook** (`features/analysis/hooks/useChartData.ts`):
- Input: array of `AnalysisFinding` objects (all findings, not paginated)
- Computes and returns (memoized via `useMemo`):
  - `frameworkCoverage: Array<{ frameworkId: string, total: number, addressed: number, percentage: number }>` — for heatmap
  - `priorityCounts: Array<{ priority: number, count: number }>` — for P1-P4 bar chart
  - `typeCounts: { addressed: number, partiallyAddressed: number, gap: number, notApplicable: number, total: number }` — for stat cards
- Returns empty/zero defaults when findings array is empty or undefined
- Handles the nullable fields gracefully (null concept fields don't affect aggregation)

**SummaryStats component** (`features/analysis/components/SummaryStats.tsx`):
- Row of 6 shadcn Card components in a responsive grid (`grid-cols-2 md:grid-cols-3 lg:grid-cols-6`)
- Each card shows: icon/label, value, optional secondary text (percentage or list)
- Cards: Total Findings, Addressed (count + %), Gaps (count + %), Frameworks (count, hover shows list), Processing Time (formatted from ms), Token Count (formatted number)
- Accepts `analysis: Analysis` and `chartData` from `useChartData` as props
- Shows loading skeleton while chart data query is still loading (transition from processing → completed)

### 3.3 D3 Chart Components

**What:** Create `CoverageHeatmap` and `PriorityChart` as D3 wrapper components, plus a shared `useContainerDimensions` hook for responsive sizing.

**useContainerDimensions hook** (`features/analysis/hooks/useContainerDimensions.ts`):
- Takes a `RefObject<HTMLDivElement>`
- Uses `ResizeObserver` to track container width/height
- Returns `{ width: number, height: number }`
- Debounces updates (150ms) to avoid rapid re-renders
- Cleans up observer on unmount

**CoverageHeatmap component** (`features/analysis/components/CoverageHeatmap.tsx`):
- Props: `data: Array<{ frameworkId: string, percentage: number, addressed: number, total: number }>`
- Renders inside a Card with title from i18n
- Wraps SVG in a container div, uses `useContainerDimensions` for width
- D3 integration via `useRef<SVGSVGElement>` + `useEffect`:
  - Clear previous render with `selectAll("*").remove()`
  - Create horizontal bars with `d3.scaleBand` for Y axis (framework names), `d3.scaleLinear` for X axis (0-100%)
  - Color gradient using `d3.interpolateRdYlGn` (red → yellow → green)
  - Framework name labels on the left, percentage text on each bar
  - `viewBox` attribute for base scaling
- Tooltips: React-managed state. D3 `mouseover`/`mouseout` events update tooltip position + data in state. Tooltip renders as a positioned `<div>` with Tailwind styling.
- Accessibility: `role="img"`, `<title>` + `<desc>` via `aria-labelledby`
- Cleanup: remove all SVG children and event listeners in useEffect return
- Dynamic height: calculated from number of frameworks (bar height × count + padding)
- Shows loading skeleton while chart data is loading
- Shows "No data" placeholder when data array is empty

**PriorityChart component** (`features/analysis/components/PriorityChart.tsx`):
- Props: `data: Array<{ priority: number, count: number }>`
- Vertical bar chart with 4 bars (P1-P4)
- Fixed colors: P1=red (`#ef4444`), P2=orange (`#f97316`), P3=yellow (`#eab308`), P4=green (`#22c55e`)
- Same D3 integration pattern as CoverageHeatmap
- D3 scales: `scaleBand` for X axis (priority labels), `scaleLinear` for Y axis (counts)
- Tooltips: same React-managed pattern
- Accessibility: same pattern

**Charts layout:** Two-column grid (`grid-cols-1 lg:grid-cols-2 gap-6`) rendered inline in the route component, containing both chart cards.

### 3.4 Findings Table with Filters

**What:** Create the FindingsTable component with filter dropdowns, inline row expansion, and type badges.

**FindingTypeTag component** (`features/analysis/components/FindingTypeTag.tsx`):
- Small colored badge mapping `FindingType` to colors:
  - `addressed` → green variant
  - `partially_addressed` → yellow/warning variant
  - `gap` → red/destructive variant
  - `not_applicable` → gray/secondary variant
- Uses shadcn Badge with variant prop or custom className
- Label from i18n: `findings.type.{finding_type}`

**FindingsFilters** (inline in FindingsTable or separate):
- Three shadcn Select dropdowns in a flex row:
  - Framework: options from `analysis.matched_framework_ids`, default "All Frameworks"
  - Finding Type: addressed, partially_addressed, gap, not_applicable, default "All Types"
  - Priority: P1, P2, P3, P4, default "All Priorities" (note: backend doesn't filter on this yet — param is passed but ignored server-side)
- Filter state managed by parent (AnalysisDetailPage), passed to `useFindings` params
- **Changing any filter resets page to 1** to avoid empty-page pagination bugs

**FindingsTable component** (`features/analysis/components/FindingsTable.tsx`):
- Uses shadcn Table components for semantic markup
- Props: `findings: AnalysisFinding[]`, `expandedIds: Set<string>`, `onToggleExpand: (id: string) => void`

Column headers:
- Static headers (no user-sortable columns) — data is sorted by backend in fixed order (priority ASC, sort_order ASC)
- Columns: Expand toggle, Concept Code, Concept Name, Framework, Type, Priority, Confidence

Row rendering:
- Each finding renders a `<TableRow>` with cells for code (fallback "—" if null), name (fallback "—"), framework, type (FindingTypeTag), priority, confidence (formatted as percentage)
- **Expand toggle:** Chevron button in first cell, rotates when expanded. Triggered by button click only (not entire row).
- `aria-expanded` on toggle button, `aria-controls` pointing to expanded row `id`

Expanded row:
- Conditional `<TableRow>` with `<TableCell colSpan={7}>` containing:
  - Evidence text block (or "—" if null)
  - Recommendation text block (or "—" if null)
  - Concept definition (or "—" if null)
  - Source reference if concept_code is present
- Styled with `bg-muted` background, padding, readable text blocks

**Pagination:**
- Below the table: "Page X of Y" text + Previous/Next buttons
- Disabled Previous on page 1, disabled Next on last page
- Page state managed by parent, passed to `useFindings`

### 3.5 Export Buttons & Empty State

**ExportButtons component** (`features/analysis/components/ExportButtons.tsx`):
- Props: `analysisId: string`, `analysisName: string`, `status: AnalysisStatus`
- Two buttons: "Export PDF" and "Export DOCX"
- When `status !== "completed"`: buttons disabled, wrapped in tooltip explaining "Analysis must be completed to export"
- On click: calls `exportAnalysis(id, format)` from API hooks
  - Pass `analysisName` for a user-friendly download filename (e.g., `{name}.pdf` instead of `analysis-{id}.pdf`)
- Loading state: spinner icon + "Downloading..." text on the clicked button
- Disable both buttons while either export is in progress
- Error: toast notification on failure
- Uses shadcn Button + Tooltip components

**EmptyFindings component** (`features/analysis/components/EmptyFindings.tsx`):
- Shown when analysis is completed but has zero findings
- Centered layout with:
  - Search/document icon
  - "No compliance findings detected" heading (i18n)
  - Suggestion text to adjust matcher thresholds
  - Link button to `/analysis/settings`

### 3.6 Page Assembly & Integration

**What:** Wire everything together in the route component `$id.tsx`. This section connects all components, manages page-level state, and ensures proper data flow.

**Route component responsibilities:**
- Extract `id` from `Route.useParams()`
- Call `useAnalysis(id)` with `refetchOnMount: 'always'` and conditional `refetchInterval`
- Call `useFindings(id, { limit: 1000 })` for chart data (enabled only when status is completed)
- Call `useFindings(id, { page, limit, filters })` for table data (enabled only when status is completed)
- Pass `useChartData(allFindings)` results to SummaryStats and chart components
- Manage filter state (`framework_id`, `finding_type`, `priority`) — resetting page to 1 on filter change
- Manage expanded row state (`Set<string>`)
- Manage pagination state (`page` number)

**Conditional rendering:**
- Loading → skeleton UI (cards + table placeholder)
- Error / not found → error state with back link
- Processing → header + ProcessingBanner with pulse animation (no charts/table yet)
- Completed, chart data loading → header + stat card skeletons + chart skeletons
- Completed with findings → full page (stats, charts, table)
- Completed without findings → header + EmptyFindings

**Page layout classes:** `max-w-7xl mx-auto p-6 space-y-6`

**Inline elements** (not extracted to separate component files):
- PageHeader: back link + title + status + metadata + ExportButtons
- ProcessingBanner: simple alert/banner with processing message
- ChartsSection: grid wrapper containing CoverageHeatmap + PriorityChart
- FindingsSection: wrapper containing FindingsFilters + FindingsTable + pagination

## 4. File Manifest

### New Files
```
frontend/src/features/analysis/
  hooks/
    useChartData.ts
    useContainerDimensions.ts
    __tests__/
      useChartData.test.ts
      useContainerDimensions.test.ts
  components/
    SummaryStats.tsx
    CoverageHeatmap.tsx
    PriorityChart.tsx
    FindingsTable.tsx
    FindingTypeTag.tsx
    ExportButtons.tsx
    EmptyFindings.tsx
    __tests__/
      SummaryStats.test.tsx
      CoverageHeatmap.test.tsx
      PriorityChart.test.tsx
      FindingsTable.test.tsx
      FindingTypeTag.test.tsx
      ExportButtons.test.tsx
      EmptyFindings.test.tsx
```

### Modified Files
```
frontend/src/features/ontology/types/index.ts    — rename PaginatedResponse.data to .items
frontend/src/features/analysis/types/index.ts     — add nullability to AnalysisFinding fields
frontend/src/features/analysis/api/index.ts       — update .data references to .items
frontend/src/routes/analysis/$id.tsx              — replace stub with full page
frontend/src/features/analysis/index.ts           — add new exports
frontend/src/i18n/locales/en/analysis.json        — add detail/chart/findings keys
frontend/src/i18n/locales/nb/analysis.json        — Norwegian translations
```

## 5. Implementation Order

The sections should be implemented in order (1 → 6) as each builds on the previous:

1. **Prerequisite fixes + i18n + Route shell** — fix bugs, add i18n keys, create page shell
2. **useChartData + SummaryStats** — data aggregation hook used by charts too
3. **D3 Charts** — depends on useChartData output
4. **Findings Table** — independent of charts but needs i18n and types
5. **Export + Empty State** — small components, depend on i18n
6. **Page Assembly** — wires everything together, depends on all above

## 6. Edge Cases & Gotchas

| Case | Handling |
|------|----------|
| Analysis with `status === "processing"` | Auto-poll, show processing banner, disable export, hide charts/table |
| Analysis with `status === "failed"` | Show error message, offer delete, no charts/table |
| Analysis not found (404) | Show not-found message with back link |
| Zero findings (completed) | Show EmptyFindings component instead of charts/table |
| Null concept/evidence/recommendation fields | Display "—" fallback, TypeScript types enforce null checks |
| `matched_framework_ids` empty | Heatmap shows "No data" state, framework filter shows no options |
| Very long evidence/recommendation text | Allow text to wrap, no truncation in expanded view |
| Export while another export is loading | Disable both buttons while either is loading |
| ResizeObserver on unmounted component | Cleanup observer in useEffect return |
| D3 re-render on data change | Clear SVG children at start of each render cycle |
| Filter change with stale page | Reset page to 1 when any filter changes |
| Stale analysis status on navigation back | `refetchOnMount: 'always'` ensures fresh data |
| Chart data loading after status transition | Show skeleton cards/charts during the transition window |
| Priority filter (backend limitation) | Param passed but ignored server-side until backend is updated |
