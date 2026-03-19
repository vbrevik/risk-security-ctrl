# Analysis Detail Page Enhancements

## Context

The analysis detail page (`/analysis/$id`) is fully implemented with:
- Summary stats (6 cards), coverage heatmap (D3 horizontal bars), priority chart (D3 vertical bars)
- Findings table with framework/type/priority filters, expandable rows, pagination
- Export buttons (PDF/DOCX)

Three features from the original dashboard spec (00-document-analysis/05-frontend-dashboard) were dropped during implementation. This spec covers adding them back.

## Existing Components (do not rebuild)

| Component | File | Props |
|-----------|------|-------|
| `CoverageHeatmap` | `features/analysis/components/CoverageHeatmap.tsx` | `data: Array<{frameworkId, percentage, addressed, total}>` |
| `PriorityChart` | `features/analysis/components/PriorityChart.tsx` | `data: Array<{priority, count}>` |
| `FindingsTable` | `features/analysis/components/FindingsTable.tsx` | `findings, expandedIds, onToggleExpand, frameworkIds, filters, onFilterChange, page, totalPages, onPageChange` |
| `useChartData` | `features/analysis/hooks/useChartData.ts` | Aggregates findings into chart-ready data |
| Detail page | `routes/analysis/$id.tsx` | Assembles all components |

## Enhancement 1: Heatmap-to-Table Cross-Filtering

### What
Clicking a bar in the CoverageHeatmap sets the findings table's framework filter to that framework, scrolling the table into view.

### How
- Add an `onBarClick?: (frameworkId: string) => void` callback prop to `CoverageHeatmap`
- Attach click handler to D3 bar rects
- In the detail page, wire `onBarClick` to update the `filters` state (set `framework_id`)
- After filter change, scroll the findings table section into view using a ref
- Visual feedback: highlight the clicked bar (selected state styling)
- Clicking the already-selected bar clears the filter

### Constraints
- The heatmap is a D3 SVG — click handlers go on the `<rect>` elements
- The findings table already supports `framework_id` filter via its `filters` prop
- Must work with existing filter dropdowns (clicking a bar = same as selecting from dropdown)

## Enhancement 2: Concept Links to Ontology Explorer

### What
In the findings table, concept codes and names become clickable links to the ontology explorer with that concept pre-selected.

### How
- In `FindingsTable`, wrap the concept code/name cell content in a TanStack Router `<Link>`
- Link target: `/ontology?concept={concept_id}`
- The ontology explorer already reads `concept` from URL search params and calls `selectConcept()`
- Style as a subtle link (underline on hover, accent color) — should not dominate the table
- Open in same tab (the user can navigate back)

### Constraints
- `AnalysisFinding` already has `concept_id`, `concept_code`, and `concept_name` fields
- The ontology route at `routes/ontology/index.tsx` already handles `?concept=` search param
- Only link if `concept_id` is present (it should always be, but guard defensively)

## Enhancement 3: Framework Radar/Spider Chart

### What
A radar (spider) chart showing per-framework coverage across finding categories. Each axis represents a finding type (addressed, partially_addressed, gap, not_applicable), and each framework is a polygon overlay.

### How
- New component: `FrameworkRadar` in `features/analysis/components/`
- D3-based SVG radar chart (consistent with existing D3 usage in the project)
- Props: `data: Array<{ frameworkId: string; addressed: number; partial: number; gap: number; notApplicable: number; total: number }>`
- Each framework renders as a colored polygon on the radar axes
- Color per framework should match the heatmap bar colors for consistency
- Include a legend showing framework names with their colors
- Tooltip on hover showing exact counts
- Add to `useChartData` hook: compute radar data from findings (group by framework, count by finding_type)
- Place on the detail page in the charts grid alongside existing charts

### Layout Change
The current detail page has a 2-column chart grid (heatmap + priority chart). With the radar chart, use a responsive layout:
- Desktop (>= 1024px): 3-column grid or 2-column with radar spanning full width below
- Tablet/mobile: stack vertically

### Constraints
- Use D3 for rendering (matching CoverageHeatmap and PriorityChart patterns)
- Use `useContainerDimensions` hook for responsive sizing
- Wrapped in a shadcn Card with CardHeader/CardTitle (matching existing chart cards)
- All text via i18n (`analysis` namespace)
- Limit to 8 framework overlays max for readability; if more, show top 8 by finding count

## Backend API

No backend changes required. All data comes from the existing `/api/analyses/{id}/findings` endpoint which returns `framework_id` and `finding_type` per finding.

## i18n Keys Needed

```
charts.radar.title: "Framework Coverage Radar" / "Rammeverkdekning radar"
charts.radar.description: "Coverage breakdown by finding type per framework" / "Dekning fordelt pa funn-type per rammeverk"
charts.radar.addressed: "Addressed" / "Ivaretatt"
charts.radar.partial: "Partially Addressed" / "Delvis ivaretatt"
charts.radar.gap: "Gap" / "Mangel"
charts.radar.notApplicable: "Not Applicable" / "Ikke aktuelt"
charts.radar.legend: "Frameworks" / "Rammeverk"
```

## Testing

- CoverageHeatmap: test that onBarClick callback fires with correct frameworkId
- FindingsTable: test that concept code renders as a link with correct href
- FrameworkRadar: test rendering with mock data, legend visibility, empty state
- Detail page integration: test that clicking heatmap bar updates findings table filter
