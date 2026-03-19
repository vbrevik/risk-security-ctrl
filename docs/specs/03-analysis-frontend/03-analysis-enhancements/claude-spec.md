# Synthesized Specification: Analysis Detail Page Enhancements

## Overview

Three enhancements to the analysis detail page (`/analysis/$id`) that add interactivity between existing components, a new radar visualization, and a concept detail panel.

## Existing Architecture

The detail page at `routes/analysis/$id.tsx` owns:
- `filters` state → passed to `FindingsTable` and `useFindings` query
- `page` state → pagination for findings table
- `expandedIds` state → which table rows are expanded
- `useFindings(id, { limit: 1000 })` → all findings → `useChartData` hook → chart data
- `useFindings(id, { page, ...filters })` → paginated findings → table

Charts receive read-only data:
- `CoverageHeatmap` ← `chartData.frameworkCoverage`
- `PriorityChart` ← `chartData.priorityCounts`

Both D3 charts use pure D3 in `useEffect` (D3 owns SVG DOM), `useContainerDimensions` for responsive sizing, React state for tooltips, shadcn Card wrappers.

## Enhancement 1: Heatmap-to-Everything Cross-Filtering

### Behavior

Clicking a bar in CoverageHeatmap:
1. Sets `filters.framework_id` to that framework → re-fetches paginated findings
2. Highlights that framework's polygon on the radar chart (dim others to 0.15 opacity)
3. Recalculates summary stats to show only that framework's findings
4. Scrolls the findings table section into view
5. Clicking the already-selected bar clears the filter (toggle behavior)

### Implementation Details

**CoverageHeatmap changes:**
- New props: `onBarClick?: (frameworkId: string) => void`, `selectedFrameworkId?: string | null`
- In useEffect: attach `.on("click", ...)` to bar rects, calling `onBarClick`
- Apply selected styling: selected bar keeps full opacity, unselected bars dim to 0.4 opacity with gray fill
- Add `cursor: pointer` to bars

**Detail page changes:**
- New state: `selectedFrameworkId` (derived from `filters.framework_id` — same source of truth)
- `handleBarClick` callback: toggles `framework_id` in filters, resets page to 1, scrolls to table
- Pass `selectedFrameworkId` to CoverageHeatmap, FrameworkRadar, and SummaryStats
- SummaryStats receives optional `filteredFindings` prop — when selectedFrameworkId is set, stats recalculate from filtered findings only

**SummaryStats changes:**
- Accept optional `selectedFrameworkId` prop
- When set, filter `allFindings` to that framework before computing stats
- Show a clear indicator that stats are filtered (e.g., "(filtered)" label or framework name in subtitle)

## Enhancement 2: Concept Side Panel

### Behavior

Clicking a concept code or name in the findings table opens a side panel (drawer) showing full concept context from the ontology system.

### Implementation Details

**New state in detail page:**
- `selectedConceptId: string | null` — which concept's panel is open

**FindingsTable changes:**
- New prop: `onConceptClick?: (conceptId: string) => void`
- Concept code and concept name cells become clickable (styled as subtle links)
- Guard: only clickable when `concept_id` is present

**ConceptDrawer component (new):**
- Slide-in panel from the right (similar to shadcn Sheet component)
- Reuses the ontology `ContextPanel` component from `features/ontology/components/ContextPanel/ContextPanel.tsx`
- Wraps ContextPanel in an `ExplorerProvider` with the selected concept pre-set
- Uses `useConceptRelationships(conceptId)` to fetch data
- Close button and click-outside-to-close
- "Open in Ontology Explorer" link at the bottom → `/ontology?concept={id}` (new tab)

**Dependencies:**
- The ontology ContextPanel requires being inside an `ExplorerProvider` context
- Need to import and wrap: `ExplorerProvider` → set initial concept → render ContextPanel
- The ContextPanel uses `useExplorer()` context internally, so the provider must be set up correctly

**Alternative if ContextPanel coupling is too tight:**
- Build a lighter `ConceptDetail` component that uses `useConceptRelationships(id)` directly
- Shows: name, definition, type, framework, related concepts list, cross-framework mappings
- Still includes "Open in Ontology Explorer" link

## Enhancement 3: Framework Radar Chart

### Behavior

A radar/spider chart with 4 axes (finding types) showing normalized percentage breakdowns per framework. Each framework renders as a colored polygon overlay.

### Data Model

The `useChartData` hook adds a new `radarData` field:
```
radarData: Array<{
  frameworkId: string;
  values: {
    addressed: number;      // percentage 0-100
    partial: number;        // percentage 0-100
    gap: number;            // percentage 0-100
    notApplicable: number;  // percentage 0-100
  };
  total: number;            // raw finding count for this framework
}>
```

Percentages are computed as: `(typeCount / frameworkTotal) * 100`

### Rendering

**Axes:** 4 axes at 90° intervals, labeled with finding type names (via i18n)
**Grid:** 3-5 concentric circles at even percentage intervals (0%, 25%, 50%, 75%, 100%)
**Polygons:** One closed `<path>` per framework, using `d3.lineRadial()` with `curveLinearClosed`
**Colors:** Assign distinct colors per framework. Use a D3 ordinal scale (e.g., `d3.schemeTableau10`). Match colors across heatmap and radar for the same framework.
**Legend:** Below the chart, showing framework name + color swatch. Clickable to toggle visibility.
**Limit:** Max 8 frameworks shown. If more, show top 8 by total finding count.

### Props

```
interface FrameworkRadarProps {
  data: Array<{ frameworkId: string; values: { addressed, partial, gap, notApplicable }; total: number }>;
  selectedFrameworkId?: string | null;  // from cross-filter
}
```

When `selectedFrameworkId` is set:
- That framework's polygon renders at full opacity with thicker stroke
- Other polygons dim to 0.15 opacity
- Tooltip on hover still works for all polygons

### Responsive Sizing

- Use `useContainerDimensions` hook (same as other charts)
- Radar radius = `Math.min(width, height) / 2 - margin`
- Wrapped in shadcn Card with CardHeader/CardTitle

## Layout Changes

### Chart Grid

Current: `grid-cols-1 lg:grid-cols-2`
New: `grid-cols-1 lg:grid-cols-2 xl:grid-cols-3`

All three charts get equal width at xl breakpoint. On lg, heatmap + priority are side-by-side, radar wraps below. On mobile, all stack.

### Framework Color Consistency

Create a shared utility: `getAnalysisFrameworkColor(frameworkId: string, index: number): string`
- Uses `d3.schemeTableau10` for up to 10 distinct colors
- Maps framework IDs to stable color indices (sorted alphabetically, then indexed)
- Used by CoverageHeatmap, FrameworkRadar, and SummaryStats (when showing filtered framework name)

## i18n Keys

New keys for the `analysis` namespace:

```
charts.radar.title
charts.radar.description
charts.radar.noData
charts.radar.addressed
charts.radar.partial
charts.radar.gap
charts.radar.notApplicable
charts.radar.legend
charts.radar.percentage

findings.conceptLink.viewDetails
findings.conceptLink.openExplorer

detail.conceptPanel.title
detail.conceptPanel.close
detail.conceptPanel.openInExplorer
detail.filteredBy
detail.clearFilter
```

## Testing Requirements

1. **CoverageHeatmap:** onBarClick fires with correct frameworkId, selected bar styling applied
2. **FrameworkRadar:** renders polygons for each framework, empty state, selected framework highlighting, legend visibility
3. **FindingsTable:** concept click callback fires with concept_id, concept cells render as clickable
4. **ConceptDrawer:** renders with concept data, close button works, "Open in Explorer" link correct
5. **useChartData:** new radarData aggregation produces correct normalized percentages
6. **Detail page integration:** clicking heatmap bar updates table filter, stats, and radar highlight

## Backend API

No backend changes needed. All data comes from:
- `GET /api/analyses/{id}/findings` — findings with framework_id, finding_type, concept_id
- `GET /api/ontology/concepts/{id}/relationships` — concept context for side panel
