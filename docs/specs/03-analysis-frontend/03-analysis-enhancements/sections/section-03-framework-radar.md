# Section 3: Framework Radar Chart Component

## Overview

This section creates a new D3-based radar/spider chart component (`FrameworkRadar`) that visualizes normalized finding-type percentages per framework as polygon overlays on 4 axes. It follows the same pure-D3-in-useEffect architecture used by the existing `CoverageHeatmap` and `PriorityChart` components.

## Dependencies

- **Section 01 (colors-and-chart-data)** must be completed first. This section depends on:
  - `frameworkColors.ts` utility for deterministic color assignment via `d3.schemeTableau10`
  - `radarData` field added to the `ChartData` interface from `useChartData`

## File to Create

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/FrameworkRadar.tsx`

## File to Modify

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/index.ts` -- add export for `FrameworkRadar`

---

## Tests First

Create the test file at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/FrameworkRadar.test.tsx`.

The test file follows the same patterns as the existing `CoverageHeatmap.test.tsx` and `PriorityChart.test.tsx`:

- Mock `react-i18next` with `useTranslation` returning a passthrough `t` function
- Mock `useContainerDimensions` to return `{ width: 400, height: 400 }`
- Mock the `frameworkColors` utility (from `../../utils/frameworkColors`) to return deterministic colors

### Test stubs

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { FrameworkRadar } from "../FrameworkRadar";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}));

vi.mock("../../hooks/useContainerDimensions", () => ({
  useContainerDimensions: () => ({ width: 400, height: 400 }),
}));

vi.mock("../../utils/frameworkColors", () => ({
  getFrameworkColor: (_ids: string[], id: string) => "#3366cc",
}));

const sampleData = [
  {
    frameworkId: "iso-31000",
    values: { addressed: 60, partial: 20, gap: 15, notApplicable: 5 },
    total: 20,
  },
  {
    frameworkId: "nist-csf",
    values: { addressed: 40, partial: 30, gap: 20, notApplicable: 10 },
    total: 15,
  },
];

const frameworkIds = ["iso-31000", "nist-csf"];

describe("FrameworkRadar", () => {
  it("renders SVG element with role='img' and aria-labelledby");
  it("renders one path element per framework in data");
  it("renders no paths when data is empty");
  it("shows noData message when data array is empty");
  it("renders 4 axis labels (one per finding type)");
  it("renders concentric grid circles");
  it("legend section shows framework names matching data");
  it("limits rendering to 8 frameworks when more provided");
  it("when selectedFrameworkId is set, selected path has different styling");
});
```

### Test descriptions

1. **renders SVG element with role="img" and aria-labelledby** -- Render with `sampleData`. Query for `svg` element. Assert `role="img"` and `aria-labelledby` contains `"framework-radar-title"`.

2. **renders one path element per framework in data** -- Render with `sampleData` (2 entries). Query `svg path` elements (exclude grid/axis elements by filtering for paths with `fill` attributes that are not grid-related). Expect 2 polygon paths. The polygon paths can be identified by selecting paths within a group that has a specific class (e.g., `.radar-polygon`) or by checking that they have a `stroke` attribute with a framework color.

3. **renders no paths when data is empty** -- Render with `data={[]}`. Assert no polygon `path` elements exist in the SVG.

4. **shows noData message when data array is empty** -- Render with `data={[]}`. Assert `screen.getByText("charts.radar.noData")` is in the document. Optionally assert that the SVG is not rendered (same pattern as CoverageHeatmap empty state).

5. **renders 4 axis labels** -- Render with `sampleData`. Look for text elements corresponding to the 4 i18n keys: `charts.radar.addressed`, `charts.radar.partial`, `charts.radar.gap`, `charts.radar.notApplicable`. Each should be present in the document.

6. **renders concentric grid circles** -- Render with `sampleData`. Query `svg circle` elements. Expect at least 4 (the grid rings at 25%, 50%, 75%, 100%).

7. **legend section shows framework names matching data** -- Render with `sampleData`. Assert that `"iso-31000"` and `"nist-csf"` text appear in the rendered output (these are in the React-rendered legend below the SVG, not inside the SVG).

8. **limits rendering to 8 frameworks when more provided** -- Create data with 10 entries. Render. Count polygon `path` elements. Expect 8 (not 10). The component should sort by `total` descending and take the top 8.

9. **when selectedFrameworkId is set, selected path has different styling** -- Render with `selectedFrameworkId="iso-31000"`. Find the polygon paths. The selected path should have a `stroke-width` of `"3"` (or thicker than unselected). Unselected paths should have reduced opacity (check `fill-opacity` or `opacity` attribute).

---

## Implementation Details

### Component Architecture

`FrameworkRadar.tsx` follows the identical architecture as `CoverageHeatmap.tsx` and `PriorityChart.tsx`:

- Wrapped in shadcn `Card` with `CardHeader` / `CardTitle` / `CardDescription`
- Uses `useContainerDimensions(containerRef)` for responsive width
- Pure D3 rendering in a `useEffect` that clears and redraws on dependency changes
- React `useState` for tooltip positioning
- SVG element with `role="img"` and `aria-labelledby="framework-radar-title framework-radar-desc"`

### Props Interface

```typescript
interface FrameworkRadarProps {
  data: Array<{
    frameworkId: string;
    values: { addressed: number; partial: number; gap: number; notApplicable: number };
    total: number;
  }>;
  selectedFrameworkId?: string | null;
  frameworkIds: string[];
}
```

The `data` is the `radarData` output from `useChartData` (Section 01). The `values` are normalized percentages (0-100). `frameworkIds` is the canonical sorted list used by `getFrameworkColor` for deterministic color assignment.

### Radar Geometry

The radar has 4 axes evenly spaced at 90-degree intervals:

- **Axes:** Top (12 o'clock), Right, Bottom, Left -- mapped to: addressed, partial, gap, notApplicable
- **Starting angle:** `Math.PI / 2` (12 o'clock). Angle slice: `(2 * Math.PI) / 4`
- **Grid rings:** 4 concentric circles at 25%, 50%, 75%, 100% of radius. Light stroke `#e5e7eb`
- **Axis lines:** Straight lines from center to each axis endpoint, same light stroke
- **Radius:** `Math.min(width, height) / 2 - 60` (60px margin for labels)
- **Radial scale:** `d3.scaleLinear().domain([0, 100]).range([0, radius])`

Vertex calculation for each data point:
```
x = cos(angle) * scale(value)
y = -sin(angle) * scale(value)
```

Axis labels are positioned just outside the outermost grid circle using i18n keys: `charts.radar.addressed`, `charts.radar.partial`, `charts.radar.gap`, `charts.radar.notApplicable`.

Percentage labels (25%, 50%, 75%, 100%) are rendered along one axis (the top axis) next to each grid ring.

### Polygon Rendering

For each framework in `data` (capped at first 8 sorted by `total` descending):

1. Extract the 4 values from `values` object in axis order: `[addressed, partial, gap, notApplicable]`
2. Compute 4 vertex coordinates using the radial scale and angles
3. Render a closed `<path>` using `d3.lineRadial()` with `d3.curveLinearClosed`
4. Style the path:
   - `fill`: framework color (from `getFrameworkColor`) at 0.15 opacity
   - `stroke`: framework color at full opacity
   - `stroke-width`: 2

Use a CSS class or D3 class (e.g., `.radar-polygon`) on the polygon group for test queryability.

### Selected State

When `selectedFrameworkId` is set:
- The selected framework's polygon: fill opacity 0.3, stroke-width 3
- All other polygons: fill opacity 0.08, stroke opacity 0.3

When `selectedFrameworkId` is null or undefined: all polygons render with default styling (fill opacity 0.15, stroke-width 2).

### Tooltip

Place invisible circles (`r=6`, `fill-opacity=0`) at each vertex of each polygon. On `mouseover`, set React tooltip state with: framework name, axis label (finding type), percentage value, raw count (derived from percentage and total). On `mouseout`, clear tooltip.

The tooltip is rendered as an absolutely positioned `<div>` outside the SVG (same pattern as existing charts):
```
className="absolute bg-popover text-popover-foreground border rounded px-2 py-1 text-xs shadow-md pointer-events-none z-10"
```

### Legend

Rendered in React JSX (not D3), below the SVG. A flex-wrap container showing each framework as a small colored circle swatch + framework ID text. This is straightforward React rendering outside the D3-managed SVG.

### Empty State

If `data` is empty or all values across all frameworks are zero, render a centered `<p>` with i18n key `charts.radar.noData` instead of the SVG. Follow the same pattern as `CoverageHeatmap` and `PriorityChart`.

### Accessibility

- SVG gets `role="img"` and `aria-labelledby="framework-radar-title framework-radar-desc"`
- Inside the SVG, D3 appends a `<title>` element with id `framework-radar-title` containing `t("charts.radar.title")`
- D3 appends a `<desc>` element with id `framework-radar-desc` containing `t("charts.radar.description")`

### Constants

```typescript
const CHART_SIZE = 400; // default height (width is responsive)
const MARGIN = 60;      // margin for axis labels
const GRID_LEVELS = 4;  // number of concentric rings
const MAX_FRAMEWORKS = 8; // cap on rendered polygons
const AXES = ["addressed", "partial", "gap", "notApplicable"] as const;
```

### D3 useEffect Structure

The `useEffect` follows this sequence (matching existing chart patterns):

1. Guard: `if (!svgRef.current || width === 0 || displayData.length === 0) return;`
2. `svg.selectAll("*").remove();` -- clear previous render
3. Set viewBox
4. Append accessibility elements (`<title>`, `<desc>`)
5. Create centered `<g>` group with translate to center
6. Draw grid rings (circles)
7. Draw axis lines (lines from center to edge)
8. Draw axis labels (text elements)
9. Draw percentage labels along one axis
10. For each framework: compute vertices, draw polygon path
11. For each framework: draw invisible vertex circles with mouse handlers
12. Return cleanup function: `() => { svg.selectAll("*").remove(); }`

Dependencies array: `[displayData, width, selectedFrameworkId, frameworkIds, t]`

Where `displayData` is derived via `useMemo` from `data` -- sorted by `total` descending, sliced to `MAX_FRAMEWORKS`.

### Export

Add to `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/index.ts`:

```typescript
export { FrameworkRadar } from "./components/FrameworkRadar";
```

---

## Imports Required

The component will import:

```typescript
import { useRef, useEffect, useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import * as d3 from "d3";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { useContainerDimensions } from "../hooks/useContainerDimensions";
import { getFrameworkColor } from "../utils/frameworkColors";
```

The `getFrameworkColor` function signature (from Section 01): `(frameworkIds: string[], frameworkId: string) => string`. It returns a hex color from `d3.schemeTableau10` based on the framework's alphabetical index in the sorted array, mod 10.

---

## Checklist

1. Create test file at `components/__tests__/FrameworkRadar.test.tsx` with the 9 test stubs
2. Create `components/FrameworkRadar.tsx` implementing the radar chart
3. Add export to `index.ts`
4. Verify all 9 tests pass with `pnpm test -- FrameworkRadar`