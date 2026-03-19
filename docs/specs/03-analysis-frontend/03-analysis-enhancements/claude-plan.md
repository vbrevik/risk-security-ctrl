# Implementation Plan: Analysis Detail Page Enhancements

## Context

The analysis detail page at `/analysis/$id` displays document analysis results: summary statistics, coverage heatmap, priority chart, and a paginated findings table with filters. Three enhancements add interactivity and a new visualization.

### Current Architecture

The detail page component (`routes/analysis/$id.tsx`) owns all state:
- `filters: { framework_id?, finding_type?, priority? }` — drives the paginated findings query
- `page` — current table page
- `expandedIds: Set<string>` — which table rows are expanded

Data flow:
- `useFindings(id, { limit: 1000 })` → `useChartData(findings)` → provides `frameworkCoverage` and `priorityCounts` to charts
- `useFindings(id, { page, ...filters })` → provides paginated findings to `FindingsTable`
- Charts are read-only consumers. `FindingsTable` reports filter/page/expand changes via callbacks.

D3 charts use pure D3 in `useEffect` (D3 owns SVG DOM via refs). Both use `useContainerDimensions` for responsive sizing and React state for tooltips. Both are wrapped in shadcn `Card` components.

### Stack

React 19, TypeScript, TanStack Router/Query, D3.js, shadcn/ui, Tailwind CSS v4, i18next, Vitest + React Testing Library.

---

## Section 1: Shared Framework Color Utility and useChartData Extension

### Goal

Create a deterministic color assignment for framework IDs (used by heatmap, radar, and stats) and extend `useChartData` to produce radar-ready data.

### Framework Color Utility

Create `features/analysis/utils/frameworkColors.ts` exporting a single function that maps a framework ID to a hex color string. The function receives the sorted array of all framework IDs and an individual framework ID. It uses `d3.schemeTableau10` (10-color palette) — the framework's index in the sorted array mod 10 determines the color. This ensures stable, deterministic colors: the same framework always gets the same color regardless of render order.

The CoverageHeatmap currently uses `d3.interpolateRdYlGn` (percentage-based). This will be kept for the bar fill, but the framework labels and the new "selected" indicator should use the shared color. The FrameworkRadar polygon strokes and fills will use this utility.

### useChartData Extension

Add a `radarData` field to the `ChartData` interface returned by `useChartData`. Structure:

```typescript
radarData: Array<{
  frameworkId: string;
  values: { addressed: number; partial: number; gap: number; notApplicable: number };
  total: number;
}>
```

The `values` are **normalized percentages** (0-100). For each framework, count findings by type, divide by total findings for that framework, multiply by 100. This enables meaningful comparison across frameworks of different sizes.

Computation: inside the existing `useMemo`, after computing `frameworkCoverage` and `priorityCounts`, group findings by `framework_id`, then within each group count by `finding_type`. Map finding_type values: `"addressed"` → `addressed`, `"partially_addressed"` → `partial`, `"gap"` → `gap`, `"not_applicable"` → `notApplicable`.

Update the `EMPTY_CHART_DATA` constant to include `radarData: []`.

### Files Modified

- `features/analysis/utils/frameworkColors.ts` (new)
- `features/analysis/hooks/useChartData.ts` (extend)
- `features/analysis/index.ts` (export new utility)

---

## Section 2: CoverageHeatmap Cross-Filter Support

### Goal

Add click-to-filter behavior to the heatmap bars and visual feedback for selected state.

### New Props

Add these optional props to `CoverageHeatmapProps`:
- `onBarClick?: (frameworkId: string) => void`
- `selectedFrameworkId?: string | null`
- `frameworkIds?: string[]` — canonical list for consistent color assignment

### Click Handler

In the `useEffect` where bars are rendered via D3 `.join("rect")`, chain `.on("click", (event, d) => onBarClickRef.current?.(d.frameworkId))` and `.attr("cursor", "pointer")`.

The `onBarClick` callback reference needs to be captured in a ref (not directly in the closure) since D3 `.on()` captures the closure at attachment time. Use a `useRef` to hold the latest callback, update it on each render.

### Keyboard Accessibility

Each bar rect must be keyboard-accessible:
- Add `.attr("tabindex", "0")` and `.attr("role", "button")`
- Add `.attr("aria-label", d => d.frameworkId)` for screen readers
- Add `.on("keydown", (event, d) => { if (event.key === "Enter" || event.key === " ") onBarClickRef.current?.(d.frameworkId) })` for keyboard activation

### Selected State Styling

After rendering bars, apply conditional styling based on `selectedFrameworkId`:
- If no selection: all bars render normally (existing behavior)
- If a framework is selected: the selected bar keeps full opacity; unselected bars transition to 0.3 opacity with a gray fill overlay
- Add a 2px stroke on the selected bar using the framework's color from the shared utility
- Apply `transition` attribute for smooth opacity changes

Add `selectedFrameworkId` to the `useEffect` dependency array so bars re-style when selection changes.

### Files Modified

- `features/analysis/components/CoverageHeatmap.tsx`

---

## Section 3: Framework Radar Chart Component

### Goal

A new D3-based radar/spider chart with 4 axes (finding types) showing normalized percentages per framework as polygon overlays.

### Component Structure

`features/analysis/components/FrameworkRadar.tsx` — follows the same architecture as CoverageHeatmap and PriorityChart:
- Wrapped in shadcn `Card` with `CardHeader`/`CardTitle`/`CardDescription`
- Uses `useContainerDimensions` for responsive sizing
- Pure D3 rendering in `useEffect`
- React state for tooltip
- SVG with `role="img"` and `aria-labelledby`

### Props

```typescript
interface FrameworkRadarProps {
  data: Array<{
    frameworkId: string;
    values: { addressed: number; partial: number; gap: number; notApplicable: number };
    total: number;
  }>;
  selectedFrameworkId?: string | null;
  frameworkIds: string[];  // all framework IDs for color assignment
}
```

### Radar Geometry

**Axes:** 4 axes at 90° intervals (top, right, bottom, left). Labels from i18n: addressed, partial, gap, not applicable. Positioned just outside the outermost grid circle.

**Grid rings:** 4 concentric circles at 25%, 50%, 75%, 100% with light stroke (`#e5e7eb`). Labels showing percentage on one axis (e.g., the top axis).

**Axis lines:** Straight lines from center to each axis endpoint, same light stroke.

**Vertex calculation:**
- `angleSlice = (2 * Math.PI) / 4`
- Starting angle: `Math.PI / 2` (12 o'clock position)
- Radial scale: `d3.scaleLinear().domain([0, 100]).range([0, radius])`
- For each data point: `x = cos(angle) * scale(value)`, `y = -sin(angle) * scale(value)`

**Radius:** `Math.min(width, height) / 2 - margin`. Margin ~60px to accommodate labels.

### Polygon Rendering

For each framework in `data` (limit to first 8 by total finding count):
1. Compute 4 vertex coordinates from normalized percentages
2. Generate closed path using `d3.lineRadial()` with `d3.curveLinearClosed`
3. Render `<path>` with:
   - `fill`: framework color at 0.15 opacity
   - `stroke`: framework color at full opacity
   - `stroke-width`: 2 (or 3 if selected)

**Selected state:** When `selectedFrameworkId` is set, the selected framework's polygon renders at full fill opacity (0.3) with thicker stroke (3px). All other polygons dim to 0.08 fill opacity and 0.3 stroke opacity.

### Tooltip

Place invisible circles (r=6) at each vertex of each polygon. On mouseover, show tooltip with: framework name, axis label (finding type), percentage value, raw count.

Use the same tooltip pattern as existing charts: React state `useState<TooltipData | null>`, absolutely positioned div.

### Legend

Below the SVG, render a flex-wrap list of framework names with color swatches. Each item shows a small colored circle + framework ID text. This is rendered in React JSX (outside the D3-managed SVG), not by D3.

### Empty State

If `data` is empty or all values are zero, show a centered "No data" message using the i18n key `charts.radar.noData`.

### Files Created

- `features/analysis/components/FrameworkRadar.tsx`

---

## Section 4: Concept Side Panel (ConceptDrawer)

### Goal

A slide-in panel that shows full ontology concept context when a concept is clicked in the findings table.

### Architecture Decision

The ontology `ContextPanel` component (`features/ontology/components/ContextPanel/ContextPanel.tsx`) requires being inside an `ExplorerProvider` context. Rather than creating a tight coupling, build a **standalone ConceptDrawer** that:
1. Uses shadcn `Sheet` component (right-side drawer)
2. Fetches concept data via `useConceptRelationships(conceptId)` from the ontology API
3. Renders concept details in a layout inspired by ContextPanel but independent of it
4. Includes an "Open in Ontology Explorer" link

This avoids importing the full `ExplorerProvider`/`ExplorerContext` into the analysis feature, keeping features loosely coupled.

### ConceptDrawer Component

`features/analysis/components/ConceptDrawer.tsx`

**Props:**
```typescript
interface ConceptDrawerProps {
  conceptId: string | null;
  onClose: () => void;
}
```

**Behavior:**
- When `conceptId` is non-null, the Sheet opens
- Fetches `useConceptRelationships(conceptId ?? "")` from `features/ontology/api` — passing empty string when null, relying on the hook's `enabled: !!id` guard to skip the query
- Displays: concept name, code, type, framework, definition
- Shows related concepts grouped by relationship type (if any)
- Shows cross-framework mappings (if any)
- Loading state: skeleton placeholders
- Error state: error message with retry button when the concept fetch fails
- Footer: "Open in Ontology Explorer" link → `/ontology?concept={id}` (opens new tab via `target="_blank"`)

**Sheet configuration:**
- Side: "right"
- Width: ~400px (Tailwind `w-[400px]`)
- Overlay: semi-transparent backdrop
- Close: X button in header + click outside

### FindingsTable Changes

Add prop: `onConceptClick?: (conceptId: string) => void`

In the table body, wrap concept code and concept name cells in a `<button>` (not a `<Link>` since we're opening a drawer, not navigating). Style with `text-left hover:underline text-accent-foreground cursor-pointer`. The `concept_id` field is always present (non-nullable `string` in the type), so all concept cells are clickable. Guard visibility on `concept_code` or `concept_name` being non-null (since those are the visible text elements).

### Detail Page Wiring

New state in `$id.tsx`: `selectedConceptId: string | null`

Pass `onConceptClick` to FindingsTable. Render `<ConceptDrawer conceptId={selectedConceptId} onClose={() => setSelectedConceptId(null)} />` at the page level.

### Files Created/Modified

- `features/analysis/components/ConceptDrawer.tsx` (new)
- `features/analysis/components/FindingsTable.tsx` (add onConceptClick)
- `routes/analysis/$id.tsx` (add state and drawer)
- `features/analysis/index.ts` (export ConceptDrawer)

---

## Section 5: Detail Page Assembly and Cross-Filter Wiring

### Goal

Wire all enhancements together in the detail page: cross-filtering state, chart grid layout, concept drawer, and SummaryStats filtering.

### New State

```typescript
// Existing
const [filters, setFilters] = useState<{ framework_id?: string; finding_type?: FindingType; priority?: number }>({});
const [page, setPage] = useState(1);
const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());

// New
const [selectedConceptId, setSelectedConceptId] = useState<string | null>(null);
```

The `selectedFrameworkId` is derived from `filters.framework_id` — no separate state needed. This keeps the heatmap selection and the table's framework filter in sync automatically.

### handleBarClick Callback

```typescript
function handleBarClick(frameworkId: string):
  - If filters.framework_id === frameworkId: clear it (toggle off)
  - Else: set filters.framework_id = frameworkId
  - Reset page to 1
  - Scroll findings table section into view (via ref)
```

Use `useCallback` with NO `filters` dependency — use functional updater `setFilters(prev => ({ ...prev, framework_id: prev.framework_id === frameworkId ? undefined : frameworkId }))` to avoid recreating the callback on every filter change.

### SummaryStats Filtering

SummaryStats currently receives the full `ChartData` object including `typeCounts`. When a framework is selected, only the finding-type cards (total, addressed, gaps, partial) should update. Framework count, processing time, and token count are analysis-level metrics and do not change.

Add an optional `overrideTypeCounts` prop to `SummaryStats`. When provided, it replaces `chartData.typeCounts` for the 4 finding-type cards only.

In the detail page, derive `filteredTypeCounts` via `useMemo` when `filters.framework_id` is set: filter `allFindingsData` by framework, count by finding_type. Pass to SummaryStats as `overrideTypeCounts`.

Add a visual indicator (banner or subtitle) showing "Filtered by: {frameworkId}" with a clear button.

### Chart Grid Layout

Change the chart grid classes:
- From: `grid grid-cols-1 lg:grid-cols-2 gap-6`
- To: `grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6`

Pass to charts:
- `CoverageHeatmap`: add `onBarClick={handleBarClick}` and `selectedFrameworkId={filters.framework_id}`
- `FrameworkRadar`: `data={chartData.radarData}`, `selectedFrameworkId={filters.framework_id}`, `frameworkIds={analysis.matched_framework_ids}`
- `PriorityChart`: unchanged

### Filter Banner

When `filters.framework_id` is set, show a small banner above the charts (or stats) section:
- Text: "Showing results for: {frameworkId}"
- Clear button (X icon) that clears `filters.framework_id`
- Styled subtly: `bg-muted rounded px-3 py-1 text-sm`

### Concept Drawer

Render `<ConceptDrawer>` at the bottom of the page component. Pass `selectedConceptId` and close handler.

### Table Scroll Target

Add a `ref` to the findings table wrapper `<div>`. In `handleBarClick`, call `ref.current?.scrollIntoView({ behavior: "smooth", block: "start" })`.

### Files Modified

- `routes/analysis/$id.tsx` (primary integration)
- `features/analysis/components/SummaryStats.tsx` (accept filtered type counts)

---

## Section 6: i18n and Tests

### i18n Updates

Add keys to both `en/analysis.json` and `nb/analysis.json`:

**Radar chart:**
- `charts.radar.title` / `charts.radar.description` / `charts.radar.noData`
- `charts.radar.addressed` / `charts.radar.partial` / `charts.radar.gap` / `charts.radar.notApplicable`
- `charts.radar.legend` / `charts.radar.percentage`

**Concept drawer:**
- `detail.conceptPanel.title` / `detail.conceptPanel.close` / `detail.conceptPanel.openInExplorer`
- `detail.conceptPanel.definition` / `detail.conceptPanel.type` / `detail.conceptPanel.framework`
- `detail.conceptPanel.relatedConcepts` / `detail.conceptPanel.crossMappings`
- `detail.conceptPanel.loading` / `detail.conceptPanel.error` / `detail.conceptPanel.retry`

**Cross-filter:**
- `detail.filteredBy` / `detail.clearFilter`

### Test Coverage

**useChartData tests** (`hooks/__tests__/useChartData.test.ts`):
- Test radarData aggregation: given findings across 2 frameworks, verify normalized percentages
- Test radarData with single framework
- Test radarData empty when no findings

**CoverageHeatmap tests** (extend existing):
- Test onBarClick fires with correct frameworkId when bar clicked
- Test selected bar styling applied (opacity changes)
- Test no crash when onBarClick not provided (backward compatible)

**FrameworkRadar tests** (`components/__tests__/FrameworkRadar.test.tsx`):
- Test renders SVG with correct number of path elements (one per framework)
- Test empty state message when data is empty
- Test accessibility: role="img", aria-labelledby
- Test legend renders framework names
- Test selectedFrameworkId changes styling

**FindingsTable tests** (extend existing):
- Test concept cells are clickable (concept_id is always present)
- Test onConceptClick fires with correct concept_id
- Test concept code cell shows "—" and is not clickable when concept_code is null

**ConceptDrawer tests** (`components/__tests__/ConceptDrawer.test.tsx`):
- Test renders when conceptId is non-null
- Test hidden when conceptId is null
- Test "Open in Explorer" link has correct href
- Test close button calls onClose
- Mock `useConceptRelationships` to return test data

### Files Created/Modified

- `i18n/locales/en/analysis.json`
- `i18n/locales/nb/analysis.json`
- `features/analysis/hooks/__tests__/useChartData.test.ts` (extend)
- `features/analysis/components/__tests__/CoverageHeatmap.test.tsx` (extend)
- `features/analysis/components/__tests__/FrameworkRadar.test.tsx` (new)
- `features/analysis/components/__tests__/FindingsTable.test.tsx` (extend)
- `features/analysis/components/__tests__/ConceptDrawer.test.tsx` (new)

---

## Dependency Order

```
Section 1 (colors + useChartData) → no dependencies
Section 2 (heatmap cross-filter) → depends on Section 1 (uses framework colors)
Section 3 (radar chart) → depends on Section 1 (uses radarData + colors)
Section 4 (concept drawer) → no dependencies on Sections 1-3
Section 5 (page assembly) → depends on Sections 1-4
Section 6 (i18n + tests) → depends on Sections 1-5
```

Sections 2, 3, and 4 can be developed in parallel after Section 1 is complete.
