I have all the context needed. Let me generate the section content.

# Section 03: D3 Chart Components

## Overview

This section creates two D3.js-based chart components (`CoverageHeatmap` and `PriorityChart`) and a shared `useContainerDimensions` hook for responsive sizing. Both charts wrap D3 rendering inside React components, using `useRef` + `useEffect` for imperative DOM manipulation and React-managed tooltip state.

**Dependencies:**
- Section 01 (i18n keys for `charts.*` must exist)
- Section 02 (`useChartData` hook provides the data shapes consumed by these charts)
- `d3` package (already installed: `d3@^7.9.0`)

**Blocks:** Section 06 (page assembly)

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/analysis/hooks/useContainerDimensions.ts` | ResizeObserver hook for responsive chart width |
| `frontend/src/features/analysis/hooks/__tests__/useContainerDimensions.test.ts` | Tests for the hook |
| `frontend/src/features/analysis/components/CoverageHeatmap.tsx` | Horizontal bar chart showing per-framework coverage |
| `frontend/src/features/analysis/components/__tests__/CoverageHeatmap.test.tsx` | Tests for the heatmap |
| `frontend/src/features/analysis/components/PriorityChart.tsx` | Vertical bar chart showing P1-P4 finding counts |
| `frontend/src/features/analysis/components/__tests__/PriorityChart.test.tsx` | Tests for the priority chart |

## Tests (Write First)

### `frontend/src/features/analysis/hooks/__tests__/useContainerDimensions.test.ts`

This file tests the `useContainerDimensions` hook using `renderHook`. The ResizeObserver must be mocked at the global level since jsdom does not implement it.

Test cases:

1. **Returns initial dimensions of `{ width: 0, height: 0 }`** -- Render the hook with a ref to a `div`. Before any ResizeObserver callback fires, the returned dimensions should be `{ width: 0, height: 0 }`.

2. **Updates dimensions when ResizeObserver fires** -- After rendering, simulate the ResizeObserver callback with a contentRect of `{ width: 600, height: 400 }`. After the debounce period (150ms, use `vi.advanceTimersByTime`), the hook should return `{ width: 600, height: 400 }`.

3. **Cleans up observer on unmount** -- Render, then unmount. Assert that the mocked `ResizeObserver.disconnect()` was called exactly once.

4. **Debounces rapid resize events** -- Fire multiple ResizeObserver callbacks in quick succession (e.g., widths 100, 200, 300 within 50ms). After advancing timers by 150ms, only the last value (300) should be reflected.

Mock setup: Create a `MockResizeObserver` class that stores the callback and exposes `observe`, `unobserve`, `disconnect` as `vi.fn()`. Assign it to `globalThis.ResizeObserver` in `beforeEach`. Use `vi.useFakeTimers()`.

### `frontend/src/features/analysis/components/__tests__/CoverageHeatmap.test.tsx`

Mock `react-i18next` to return translation keys as-is. Mock `useContainerDimensions` to return `{ width: 800, height: 400 }` so the chart has dimensions to render with.

Test cases:

1. **Renders SVG element inside a Card** -- Render with sample data. Query for an `svg` element. It should exist within a container that has Card styling (query by role or test ID on the Card wrapper).

2. **Renders correct number of bars matching data length** -- Provide `data` with 3 framework entries. After render, query all `rect` elements inside the SVG. Expect 3 rects.

3. **Shows chart title from i18n** -- The Card header should contain the text matching the `charts.coverage.title` translation key.

4. **Shows "No data" placeholder when data is empty** -- Render with `data={[]}`. The SVG should not be present. Instead, a text element like "No data" should appear.

5. **Renders with accessibility attributes** -- The SVG element should have `role="img"` and an `aria-labelledby` attribute pointing to a `<title>` element inside the SVG.

6. **Does not crash when data changes between renders** -- Render with 2 data items, then rerender with 5 data items. No error should be thrown. The rect count should update to 5.

Sample test data shape:
```ts
const sampleData = [
  { frameworkId: "iso-31000", percentage: 75, addressed: 15, total: 20 },
  { frameworkId: "nist-csf", percentage: 50, addressed: 10, total: 20 },
  { frameworkId: "iso-31010", percentage: 90, addressed: 18, total: 20 },
];
```

### `frontend/src/features/analysis/components/__tests__/PriorityChart.test.tsx`

Same mocking approach as CoverageHeatmap (i18n + useContainerDimensions).

Test cases:

1. **Renders SVG element inside a Card** -- Same pattern as heatmap test.

2. **Renders 4 bars for P1-P4** -- Provide data with 4 priority entries. Query `rect` elements in SVG. Expect 4.

3. **Shows chart title from i18n** -- Card header should contain the `charts.priority.title` translation key.

4. **Shows "No data" placeholder when all counts are zero** -- Render with `data` where all counts are 0. Should show placeholder text instead of SVG.

5. **Renders with accessibility attributes** -- Same pattern as heatmap.

Sample test data shape:
```ts
const sampleData = [
  { priority: 1, count: 5 },
  { priority: 2, count: 12 },
  { priority: 3, count: 8 },
  { priority: 4, count: 3 },
];
```

## Implementation Details

### `useContainerDimensions` Hook

**File:** `frontend/src/features/analysis/hooks/useContainerDimensions.ts`

**Signature:**
```ts
export function useContainerDimensions(
  ref: React.RefObject<HTMLDivElement>
): { width: number; height: number }
```

**Behavior:**
- Initialize state to `{ width: 0, height: 0 }`
- In a `useEffect`, create a `ResizeObserver` that watches `ref.current`
- On each observation entry, debounce updates by 150ms using `setTimeout` (clear previous timeout on each new event)
- Update state with `contentRect.width` and `contentRect.height` from the entry
- Cleanup: disconnect the observer and clear any pending timeout on unmount
- Return the current `{ width, height }` state

### `CoverageHeatmap` Component

**File:** `frontend/src/features/analysis/components/CoverageHeatmap.tsx`

**Props interface:**
```ts
interface CoverageHeatmapProps {
  data: Array<{
    frameworkId: string;
    percentage: number;
    addressed: number;
    total: number;
  }>;
}
```

**Structure:**
- Outer wrapper: shadcn `Card` with `CardHeader` (title + description from i18n) and `CardContent`
- Inside `CardContent`: a container `div` with a `ref` passed to `useContainerDimensions`, and an `SVGSVGElement` ref for D3
- Tooltip state: `useState<{ x: number; y: number; data: DataItem | null } | null>(null)` rendered as a positioned `div`

**D3 rendering (`useEffect` depending on `data` and container `width`):**
- Guard: skip if `width === 0` or `data.length === 0`
- Clear previous SVG content: `d3.select(svgRef.current).selectAll("*").remove()`
- Calculate dynamic height: `data.length * barHeight + margins.top + margins.bottom` (suggested `barHeight = 40`)
- Set SVG `viewBox` for responsive scaling
- Scales:
  - Y axis: `d3.scaleBand()` with domain = `data.map(d => d.frameworkId)`, range = `[0, innerHeight]`, padding = 0.2
  - X axis: `d3.scaleLinear()` with domain `[0, 100]`, range = `[0, innerWidth]`
- Color: `d3.interpolateRdYlGn(percentage / 100)` -- maps 0% to red, 50% to yellow, 100% to green
- Draw horizontal bars (`rect` elements) with width based on percentage
- Add framework name labels on the left (Y axis labels)
- Add percentage text on each bar (e.g., "75%")
- Mouse events on bars: `mouseover` sets tooltip state (position relative to container, data for display), `mouseout` clears tooltip state

**Accessibility:**
- SVG element: `role="img"`, `aria-labelledby="coverage-heatmap-title coverage-heatmap-desc"`
- Inside SVG: `<title id="coverage-heatmap-title">` and `<desc id="coverage-heatmap-desc">` elements with i18n text

**Empty state:** When `data` is empty, render a centered "No data" message inside the Card instead of the SVG.

**Cleanup:** The `useEffect` return function should select and remove all SVG children.

### `PriorityChart` Component

**File:** `frontend/src/features/analysis/components/PriorityChart.tsx`

**Props interface:**
```ts
interface PriorityChartProps {
  data: Array<{ priority: number; count: number }>;
}
```

**Structure:** Same Card + SVG + tooltip pattern as CoverageHeatmap.

**D3 rendering (`useEffect` depending on `data` and container `width`):**
- Guard: skip if `width === 0` or all counts are 0
- Clear previous SVG content
- Fixed height (suggested 300px) with margins
- Scales:
  - X axis: `d3.scaleBand()` with domain = priority labels (`["P1", "P2", "P3", "P4"]`), padding = 0.3
  - Y axis: `d3.scaleLinear()` with domain `[0, maxCount]`, range = `[innerHeight, 0]`
- Fixed colors per priority:
  - P1: `#ef4444` (red)
  - P2: `#f97316` (orange)
  - P3: `#eab308` (yellow)
  - P4: `#22c55e` (green)
- Draw vertical bars (`rect` elements) with height based on count
- Add X axis labels (P1-P4) and Y axis tick marks
- Add count text above each bar
- Mouse events for tooltips: same pattern as heatmap

**Accessibility:** Same `role="img"` + `aria-labelledby` pattern with unique IDs (`priority-chart-title`, `priority-chart-desc`).

**Empty state:** When all counts are 0, show "No data" placeholder instead of SVG.

### Charts Layout (for reference -- implemented in Section 06)

The two charts sit in a two-column responsive grid in the route component:
```tsx
<div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <CoverageHeatmap data={chartData.frameworkCoverage} />
  <PriorityChart data={chartData.priorityCounts} />
</div>
```

This grid wrapper is rendered inline in `$id.tsx` during page assembly (Section 06).

## Key Implementation Notes

- **D3 version:** The project uses D3 v7. Import specific modules (e.g., `import * as d3 from "d3"` or individual imports like `import { select, scaleBand, scaleLinear, interpolateRdYlGn } from "d3"`).
- **D3 + React pattern:** D3 manages the SVG DOM imperatively inside `useEffect`. React manages the Card wrapper, tooltip overlay, and empty/loading states. Never mix React rendering inside the SVG with D3 manipulation.
- **Re-renders:** Each `useEffect` call clears the SVG with `selectAll("*").remove()` before drawing. This prevents stale elements when data changes.
- **Tooltip positioning:** Use the container div's `getBoundingClientRect()` to compute tooltip position relative to the page. The tooltip `div` should use `position: absolute` within a `position: relative` container.
- **ResizeObserver cleanup:** The `useContainerDimensions` hook must disconnect the observer and clear timeouts in the cleanup function to avoid memory leaks and state updates on unmounted components.
- **Existing UI components:** Use `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent` from `@/components/ui/card` (already installed).
- **i18n keys used:** `charts.coverage.title`, `charts.coverage.description`, `charts.priority.title`, `charts.priority.description` (added in Section 01).
- **Data shapes come from `useChartData`** (Section 02): `frameworkCoverage` feeds CoverageHeatmap, `priorityCounts` feeds PriorityChart. The prop types above match the hook's return types.