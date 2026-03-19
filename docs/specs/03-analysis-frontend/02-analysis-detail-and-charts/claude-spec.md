# Specification: Analysis Detail Page with Charts

## Overview

Build the analysis detail page at `/analysis/$id` that displays a completed (or in-progress) document analysis with summary statistics, interactive D3 charts, an expandable findings table with multi-column sorting and filtering, and export functionality. This is the final split of the analysis frontend feature.

## Dependencies

- **Split 01 (analysis-foundation)** provides: TypeScript types (`Analysis`, `AnalysisFinding`, `FindingType`, etc.), API hooks (`useAnalysis`, `useFindings`, `exportAnalysis`), i18n namespace (`analysis`), StatusBadge component, route stub at `$id.tsx`
- **D3.js v7.9.0** already installed, with existing `useD3Graph` hook as reference pattern
- **shadcn/ui** components installed: Table, Tabs, Badge, Card, Button, Select, Dialog

## Page Layout

The detail page is a single scrollable page with sections (no tabs):

```
┌─────────────────────────────────────────────────────┐
│ ← Back to Analyses   │  Analysis Name  │ StatusBadge│
│                       │  Input type · Created date   │
│                       │           [Export PDF] [DOCX] │
├─────────────────────────────────────────────────────┤
│ SUMMARY STATISTICS CARDS (row)                       │
│ Total Findings | Addressed (%) | Gaps (%) |          │
│ Frameworks | Processing Time | Tokens               │
├─────────────────────────────────────────────────────┤
│ CHARTS (two-column on desktop, stacked on mobile)    │
│ ┌─────────────────┐ ┌─────────────────┐             │
│ │ Coverage Heatmap│ │ Priority Chart  │             │
│ │ (horizontal bar)│ │ (vertical bar)  │             │
│ └─────────────────┘ └─────────────────┘             │
├─────────────────────────────────────────────────────┤
│ FINDINGS TABLE                                       │
│ [Framework ▼] [Type ▼] [Priority ▼] filters         │
│ ┌───────────────────────────────────────────────┐   │
│ │ Code │ Name │ Framework │ Type │ Pri │ Conf    │   │
│ │ ▶ C1 │ ...  │ NIST CSF │ Gap  │ P1  │ 0.85   │   │
│ │   Evidence: ... | Recommendation: ...          │   │
│ │ ▶ C2 │ ...  │ ISO31000 │ Addr │ P2  │ 0.92   │   │
│ └───────────────────────────────────────────────┘   │
│ Page 1 of 5  [< Prev] [Next >]                      │
├─────────────────────────────────────────────────────┤
│ (Empty state shown when zero findings)               │
└─────────────────────────────────────────────────────┘
```

## Detailed Requirements

### 1. Page Header

- Back link to `/analysis` list
- Analysis name as page title
- StatusBadge component (reuse from split 01)
- Input type indicator (text/pdf/docx)
- Created date formatted
- Export buttons in header area: "Export PDF" and "Export DOCX"
  - **Disabled with tooltip** when status is not `completed` ("Analysis must be completed to export")
  - Loading spinner on button while export is generating
  - Uses existing `exportAnalysis(id, format)` function from API hooks

### 2. Auto-Polling for Processing Analyses

- When `analysis.status === "processing"`, auto-poll with `refetchInterval` (e.g., 5 seconds)
- Show a processing banner/indicator below the header
- When status transitions to `completed`, stop polling and render full content (charts, findings)
- When status transitions to `failed`, stop polling and show error message with retry option

### 3. Summary Statistics Cards

Row of stat cards using shadcn Card component:

| Stat | Source | Display |
|------|--------|---------|
| Total Findings | `findings.total` from useFindings | Number |
| Addressed | Count findings where `finding_type === "addressed"` | Number + percentage |
| Gaps | Count findings where `finding_type === "gap"` | Number + percentage |
| Frameworks Matched | `analysis.matched_framework_ids.length` | Number + list on hover |
| Processing Time | `analysis.processing_time_ms` | Formatted (e.g., "2.3s") |
| Token Count | `analysis.token_count` | Formatted number |

Stats require fetching all findings (limit 1000) for accurate counts across all pages.

### 4. Coverage Heatmap Chart

- **Type:** Horizontal bar chart, one bar per framework
- **Data:** Per-framework coverage percentage = addressed findings / total findings for that framework
- **Color gradient:** Red (0%) → Yellow (50%) → Green (100%) using D3 color interpolation
- **Labels:** Framework name on the left, percentage on the right
- **Interactive:** Hover shows tooltip with exact percentage + count breakdown
- **Implementation:** D3.js wrapper component `<CoverageHeatmap data={...} />`
  - Uses `useRef` + `useEffect` pattern (matches existing `useD3Graph`)
  - SVG rendering with `viewBox` for basic responsiveness
  - ResizeObserver for recalculating when container size changes
  - Proper cleanup in useEffect return
- **Accessibility:** `role="img"`, `<title>` + `<desc>`, `aria-labelledby`
- **Responsive:** Scales with container width, labels adjust

### 5. Priority Breakdown Chart

- **Type:** Vertical bar chart, one bar per priority level (P1-P4)
- **Data:** Count of findings per priority level
- **Colors:** P1=red, P2=orange, P3=yellow, P4=green
- **Interactive:** Hover shows tooltip with count
- **Implementation:** D3.js wrapper component `<PriorityChart data={...} />`
  - Same patterns as CoverageHeatmap
- **Layout:** Side-by-side with heatmap on desktop (two-column grid), stacked on mobile

### 6. Expandable Findings Table

#### Table Structure
- Uses shadcn Table components (`Table`, `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`)
- Columns: Concept Code, Concept Name, Framework, Type, Priority, Confidence

#### Filters (above table)
- Framework dropdown: populated from `analysis.matched_framework_ids`
- Finding type dropdown: addressed, partially_addressed, gap, not_applicable
- Priority dropdown: P1, P2, P3, P4
- Filters passed to `useFindings(id, { framework_id, finding_type, priority })`

#### Multi-Column Sorting
- Click column header to sort by that column (toggles ASC/DESC)
- Shift+click to add secondary sort columns
- Sort indicator arrows on active columns
- Primary sort param passed to `useFindings` via `sort_by`
- Secondary sorts applied client-side on the paginated results

#### Pagination
- Server-side via `useFindings(id, { page, limit })`
- Default limit: 20
- Show "Page X of Y" with prev/next buttons
- Disabled prev on page 1, disabled next on last page

#### Row Expansion
- Click a row to expand/collapse
- Track expanded state via `useState<Set<string>>` with finding IDs
- Expanded row renders below the main row with `colSpan` spanning all columns
- Expanded content shows:
  - **Evidence text** (can be long — display in a readable block)
  - **Recommendation** text
  - **Concept definition** from the finding
  - **Source reference** if available (concept_code links)
- **Accessibility:** Toggle button with `aria-expanded`, `aria-controls`

#### Type Badges
- Color-coded by `finding_type`:
  - `addressed` → green badge
  - `partially_addressed` → yellow badge
  - `gap` → red badge
  - `not_applicable` → gray badge

### 7. Empty / Zero-Findings State

When a completed analysis has zero findings:
- Show an informative empty state with icon/illustration
- Message: "No compliance findings detected"
- Suggestion to adjust matcher settings with link to `/analysis/settings`
- Option to re-run the analysis

### 8. Error States

- **Loading:** Skeleton cards + skeleton table
- **Analysis not found:** 404 message with back link
- **Failed analysis:** Show error message from `analysis.error_message`, offer delete option
- **Findings fetch error:** Error message with retry button

## Data Flow

```
Route /analysis/$id
  ├── useAnalysis(id)
  │   → Analysis object (status, metadata, matched_framework_ids)
  │   → refetchInterval when status === "processing"
  │
  ├── useFindings(id, { limit: 1000 }) [for charts + stats]
  │   → All findings for aggregation
  │   → Compute: per-framework coverage, priority counts, addressed/gap counts
  │
  ├── useFindings(id, { page, limit: 20, filters... }) [for table]
  │   → Paginated findings for display
  │
  └── exportAnalysis(id, format) [on button click]
      → Blob download
```

Two separate `useFindings` calls:
1. One with large limit for chart/stat aggregation (cached, refetched when analysis completes)
2. One with pagination/filters for the table (changes as user interacts)

## i18n Keys to Add

New keys needed in `analysis.json` (en + nb):

```
detail.title
detail.backToList
detail.createdAt
detail.inputType
detail.processing.banner
detail.processing.message
detail.failed.message
detail.notFound.title
detail.notFound.message
stats.totalFindings
stats.addressed
stats.gaps
stats.frameworks
stats.processingTime
stats.tokenCount
charts.coverage.title
charts.coverage.description
charts.priority.title
charts.priority.description
charts.tooltip.coverage
charts.tooltip.priority
findings.title
findings.filters.framework
findings.filters.findingType
findings.filters.priority
findings.filters.allFrameworks
findings.filters.allTypes
findings.filters.allPriorities
findings.columns.code
findings.columns.name
findings.columns.framework
findings.columns.type
findings.columns.priority
findings.columns.confidence
findings.expand
findings.collapse
findings.evidence
findings.recommendation
findings.conceptDefinition
findings.sourceReference
findings.empty.title
findings.empty.message
findings.empty.adjustSettings
findings.empty.reRun
findings.type.addressed
findings.type.partially_addressed
findings.type.gap
findings.type.not_applicable
export.pdf
export.docx
export.disabled
export.downloading
export.error
```

## Component Hierarchy

```
AnalysisDetailPage (route component)
├── PageHeader
│   ├── BackLink
│   ├── AnalysisTitle + StatusBadge
│   ├── MetaInfo (input type, date)
│   └── ExportButtons
├── ProcessingBanner (conditional)
├── SummaryStats
│   └── StatCard × 6
├── ChartsSection
│   ├── CoverageHeatmap (D3 wrapper)
│   └── PriorityChart (D3 wrapper)
├── FindingsSection
│   ├── FindingsFilters
│   │   ├── FrameworkSelect
│   │   ├── TypeSelect
│   │   └── PrioritySelect
│   ├── FindingsTable
│   │   ├── SortableHeader
│   │   ├── FindingRow (× N)
│   │   └── FindingExpandedRow (conditional)
│   └── FindingsPagination
└── EmptyState (conditional)
```

## Technical Constraints

- All text via `useTranslation('analysis')` — no hardcoded strings
- D3 charts render into SVG, no canvas
- Charts must be responsive (scale with container width via viewBox + ResizeObserver)
- Graceful handling of null concept fields (concept_code, concept_name, concept_definition from LEFT JOIN)
- `matched_framework_ids` already parsed from JSON string by `useAnalysis` hook
- Export only works for `status === "completed"` analyses
- Follow existing patterns: cleanup in useEffect, query key hierarchy, i18n mocking in tests

## Files to Create/Modify

### New Files
- `frontend/src/features/analysis/components/SummaryStats.tsx`
- `frontend/src/features/analysis/components/CoverageHeatmap.tsx`
- `frontend/src/features/analysis/components/PriorityChart.tsx`
- `frontend/src/features/analysis/components/FindingsTable.tsx`
- `frontend/src/features/analysis/components/FindingTypeTag.tsx`
- `frontend/src/features/analysis/components/ExportButtons.tsx`
- `frontend/src/features/analysis/components/EmptyFindings.tsx`
- `frontend/src/features/analysis/hooks/useChartData.ts`
- Test files for each component
- Test file for useChartData hook

### Modified Files
- `frontend/src/routes/analysis/$id.tsx` — replace stub with full implementation
- `frontend/src/features/analysis/index.ts` — add new component exports
- `frontend/src/i18n/locales/en/analysis.json` — add detail/chart/findings keys
- `frontend/src/i18n/locales/nb/analysis.json` — Norwegian translations
