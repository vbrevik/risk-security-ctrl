# Research Findings: Analysis Detail Page Enhancements

## 1. Existing Codebase Patterns

### D3 Chart Component Architecture

Both existing charts (`CoverageHeatmap`, `PriorityChart`) follow the same pattern:
- **Pure D3 in `useEffect`** — D3 owns the SVG DOM via `d3.select(svgRef.current)`
- React refs: `containerRef` (for dimensions), `svgRef` (for D3 rendering)
- `useContainerDimensions` hook provides responsive width via `ResizeObserver` (150ms debounce)
- Guard clause: `if (width === 0 || data.length === 0) return;`
- Cleanup: `svg.selectAll("*").remove()` at start of each effect
- Tooltip: React `useState<{ x: number; y: number; text: string } | null>`, positioned absolutely

**Important note:** The existing codebase uses the **pure D3 approach** (D3 manages DOM in useEffect), NOT the hybrid approach. The new radar chart should follow this same pattern for consistency.

### Color Schemes

- **Coverage Heatmap:** `d3.interpolateRdYlGn(percentage / 100)` — continuous red→yellow→green
- **Priority Chart:** Fixed map: `{ 1: "#ef4444", 2: "#f97316", 3: "#eab308", 4: "#22c55e" }`
- **Finding Type Tags:** Tailwind classes: green (addressed), yellow (partial), red (gap), gray (n/a)
- **No shared framework color utility** — colors are component-local. The ontology explorer has `getFrameworkColor()` in `features/ontology/utils/graphTransform.ts` but it's not imported by analysis features.

### Chart Card Wrapper Pattern

Both charts use shadcn Card with consistent structure:
```tsx
<Card>
  <CardHeader>
    <CardTitle>{t("charts.coverage.title")}</CardTitle>
    <CardDescription>{t("charts.coverage.description")}</CardDescription>
  </CardHeader>
  <CardContent>
    <div ref={containerRef} data-testid="coverage-heatmap">
      <svg ref={svgRef} role="img" aria-labelledby="..." />
      {tooltip && <div className="absolute ...">...</div>}
    </div>
  </CardContent>
</Card>
```

### useChartData Hook

Returns `ChartData` with three aggregations:
- `frameworkCoverage: Array<{ frameworkId, total, addressed, percentage }>`
- `priorityCounts: Array<{ priority, count }>`
- `typeCounts: { addressed, partiallyAddressed, gap, notApplicable, total }`

Uses `useMemo` keyed on findings array. Empty state returns frozen `EMPTY_CHART_DATA`.

**For the radar chart**, we need to add a new field: `radarData: Array<{ frameworkId, addressed, partial, gap, notApplicable, total }>` — grouping findings by framework AND type.

### FindingsTable Component

**Filter system:**
- Three `Select` dropdowns (framework, type, priority) with `__all__` sentinel value
- `onFilterChange` callback receives partial filter object merged with existing state
- Changing any filter should reset page to 1

**Concept columns (where links will go):**
- Column "Code": renders `finding.concept_code ?? "—"`
- Column "Concept": renders `finding.concept_name ?? "—"`
- Currently plain text in `<TableCell>`, no links

**Expanded detail section:**
- Shows: evidence_text, recommendation, concept_definition, concept_code (as "Source Reference")
- All wrapped in `<div className="space-y-2 text-sm">`

### Detail Page State Flow

```
$id.tsx owns:
  ├── filters state → passed to FindingsTable + useFindings query
  ├── page state → passed to FindingsTable + useFindings query
  ├── expandedIds state → passed to FindingsTable
  ├── useFindings(id, { limit: 1000 }) → allFindingsData → useChartData → chartData
  └── useFindings(id, { page, ...filters }) → paginatedFindings → FindingsTable

Charts receive read-only data from chartData:
  ├── CoverageHeatmap ← chartData.frameworkCoverage
  └── PriorityChart ← chartData.priorityCounts
```

For cross-filtering, `CoverageHeatmap` needs an `onBarClick` that updates the `filters` state in `$id.tsx`.

### Ontology URL Parameters

Route at `/ontology/` accepts search params:
```typescript
interface OntologySearch {
  view?: "graph" | "tree" | "compare";
  concept?: string;      // concept ID → selectConcept() in ExplorerContext
  frameworks?: string;   // comma-separated IDs
  type?: string;         // concept type filter
}
```

The `concept` param is read in `ExplorerContent` and calls `selectConcept(search.concept)`. This means linking to `/ontology?concept={concept_id}` will auto-select that concept.

### Testing Patterns

**D3 chart tests:**
- Mock `react-i18next`: `vi.mock("react-i18next", () => ({ useTranslation: () => ({ t: (key: string) => key }) }))`
- Mock `useContainerDimensions`: `vi.mock("../../hooks/useContainerDimensions", () => ({ useContainerDimensions: () => ({ width: 800, height: 400 }) }))`
- Assert SVG structure: `container.querySelectorAll("svg rect")` for bar count
- Assert empty state: `screen.getByText("charts.coverage.noData")`
- Assert accessibility: `role="img"`, `aria-labelledby`

**FindingsTable tests:**
- Helper factory: `makeFinding(overrides)` returns fully populated `AnalysisFinding`
- Assert columns: check i18n keys in header
- Assert interactions: `fireEvent.click()` on expand buttons, pagination
- Assert callbacks: `vi.fn()` spies verify `onToggleExpand`, `onFilterChange`, `onPageChange`

**Component mocking:**
- TanStack Router mocked via `vi.mock("@tanstack/react-router")`
- `Link` component mocked as `(props) => <a href={props.to}>{props.children}</a>`

### i18n Conventions

- Namespace: `"analysis"`
- Key hierarchy: `section.subsection.key` (e.g., `charts.coverage.title`)
- Dynamic keys used: `t(\`findings.type.${finding.finding_type}\`)` — enum suffix interpolation is an established pattern
- Both `en/analysis.json` and `nb/analysis.json` must be updated together

### Chart Grid Layout

Current layout in `$id.tsx`:
```tsx
<div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <CoverageHeatmap data={chartData.frameworkCoverage} />
  <PriorityChart data={chartData.priorityCounts} />
</div>
```

Adding a third chart means changing to: `grid-cols-1 lg:grid-cols-2 xl:grid-cols-3` or keeping 2-column and letting the radar chart span full width below.

---

## 2. Web Research: D3 Radar/Spider Charts

### Vertex Calculation Math

Core formula for converting polar to cartesian coordinates on a radar:
```
angleSlice = (2 * Math.PI) / numAxes
angle = (Math.PI / 2) + (angleSlice * i)   // Start at 12 o'clock
x = cos(angle) * radialScale(value)
y = -sin(angle) * radialScale(value)        // Negate Y for SVG coordinates
```

Use `d3.scaleLinear()` for `radialScale`, mapping data domain (e.g., 0→maxCount) to pixel radius.

### Multiple Polygon Overlays

Each dataset (framework) renders as one `<path>` with:
- `d3.lineRadial()` generator with `d3.curveLinearClosed` to close the polygon
- `fill-opacity: 0.15` for see-through overlapping
- Distinct `stroke` color per framework
- **Limit: 8 overlays max** for readability (spec already specifies this)

### Tooltip Strategy

Place invisible `<circle>` elements at each vertex with D3 `.on("mouseover")` events. Tooltip content: framework name, axis label, exact count.

### Approach Decision

The existing codebase uses **pure D3 in useEffect**, so the radar chart should too for consistency. The web research recommends the hybrid approach for new projects, but matching existing patterns is more important here.

### Grid Layout Options

Concentric grid rings: draw 3-5 circles at even intervals using `d3.range()`. Label each ring with the count value. Draw straight lines from center to each axis vertex.

**Sources:**
- [D3 Spider Chart Tutorial - Danny Yang](https://yangdanny97.github.io/blog/2019/03/01/D3-Spider-Chart)
- [React Graph Gallery - Radar Chart](https://www.react-graph-gallery.com/radar-chart)
- [Building Spider Chart with D3 and React - DEV Community](https://dev.to/simbamkenya/building-spider-chart-with-d3-js-and-react-js-50pj)

---

## 3. Web Research: D3 Click Handlers for Cross-Filtering

### Pattern for Existing Pure-D3 Components

Since the codebase uses D3 `.on()` event handlers (not React JSX event props), the click handler goes in the `useEffect`:

```javascript
bars.on("click", function(event, d) {
  onBarClick?.(d.frameworkId);
});
```

The `onBarClick` callback is a prop from the parent component. D3 captures it via closure.

### Selected State Styling

In the same `useEffect`, after rendering bars, apply conditional styling:
```javascript
bars
  .attr("fill", d => d.frameworkId === selectedFrameworkId
    ? d3.interpolateRdYlGn(d.percentage / 100)  // Keep original color
    : "#e5e7eb")                                   // Gray out unselected
  .attr("opacity", d => selectedFrameworkId && d.frameworkId !== selectedFrameworkId ? 0.4 : 1)
  .attr("stroke", d => d.frameworkId === selectedFrameworkId ? "#1e40af" : "none")
  .attr("stroke-width", 2)
  .attr("cursor", "pointer");
```

### Auto-Scroll to Table

After clicking a bar and updating the filter state, scroll the findings table into view:
```typescript
const tableRef = useRef<HTMLDivElement>(null);

const handleBarClick = (frameworkId: string) => {
  setFilters(prev => ({
    ...prev,
    framework_id: prev.framework_id === frameworkId ? undefined : frameworkId,
  }));
  setPage(1);
  tableRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
};
```

**Sources:**
- [Reusable Chart Components with React and D3](https://busypeoples.github.io/post/d3-with-react-js/)
- [D3 Graph Gallery - Interactivity](https://d3-graph-gallery.com/graph/interactivity_tooltip.html)
