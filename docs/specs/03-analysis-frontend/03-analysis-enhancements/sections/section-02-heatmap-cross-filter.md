Good -- the `frameworkColors.ts` file does not exist yet, which is expected (it is created in Section 1, a dependency of this section). Now I have all the context I need.

# Section 2: CoverageHeatmap Cross-Filter Support

## Overview

This section adds click-to-filter interactivity and selected-state visual feedback to the existing `CoverageHeatmap` component. When a user clicks a heatmap bar, it signals the parent to filter findings by that framework. A selected bar is visually highlighted while unselected bars are dimmed.

**Depends on:** Section 1 (the `getFrameworkColor` utility from `features/analysis/utils/frameworkColors.ts` must exist).

**Blocks:** Section 5 (page assembly wires `onBarClick` and `selectedFrameworkId` from detail page state).

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/CoverageHeatmap.tsx`

## Tests First

Extend the existing test file at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/CoverageHeatmap.test.tsx`.

The existing file already mocks `react-i18next` and `useContainerDimensions`, and defines `sampleData` with 3 framework entries. Add the following test cases inside the existing `describe("CoverageHeatmap", ...)` block:

### Test: onBarClick callback fires with correct frameworkId when a bar rect is clicked

Render with `onBarClick={mockFn}`. Query all `svg rect` elements, simulate a `click` event on the first bar. Assert `mockFn` was called once with `"iso-31000"` (the first entry in `sampleData`).

### Test: no error when onBarClick is not provided (backward compatible)

Render with only `data` prop (no `onBarClick`). Query a bar rect, simulate `click`. Assert no error is thrown. This confirms the new props are optional and existing usage is unaffected.

### Test: when selectedFrameworkId is set, SVG contains bars with reduced opacity for non-selected

Render with `selectedFrameworkId="iso-31000"`. Query all `svg rect` elements. The bar for `"iso-31000"` should have opacity `"1"`. The other two bars should have opacity `"0.3"`.

### Test: when selectedFrameworkId is null, all bars have full opacity

Render with `selectedFrameworkId={null}`. All 3 bar rects should have opacity `"1"` (or no opacity attribute, which defaults to 1).

### Test: bar rects have role="button" and tabindex="0" for keyboard accessibility

Render with `onBarClick` provided. All `svg rect` elements should have `getAttribute("role") === "button"` and `getAttribute("tabindex") === "0"`.

### Test: bar rects have aria-label containing the framework ID

Render normally. Each bar rect should have an `aria-label` attribute. The first rect's `aria-label` should contain `"iso-31000"`.

### Test stubs

```typescript
// Add these inside the existing describe block:

it("fires onBarClick with correct frameworkId when bar is clicked", () => {
  // Render with onBarClick={vi.fn()}, click first rect, assert called with "iso-31000"
});

it("does not error when onBarClick is not provided", () => {
  // Render without onBarClick, click a rect, no throw
});

it("dims non-selected bars when selectedFrameworkId is set", () => {
  // Render with selectedFrameworkId="iso-31000"
  // Assert selected bar opacity "1", others "0.3"
});

it("all bars have full opacity when selectedFrameworkId is null", () => {
  // Render with selectedFrameworkId={null}, all rects opacity "1"
});

it("bar rects have role='button' and tabindex='0'", () => {
  // Query all rects, check attributes
});

it("bar rects have aria-label with framework ID", () => {
  // Query rects, check aria-label contains frameworkId
});
```

## Implementation Details

### 1. Extend the Props Interface

Add three optional props to `CoverageHeatmapProps`:

```typescript
interface CoverageHeatmapProps {
  data: Array<{
    frameworkId: string;
    percentage: number;
    addressed: number;
    total: number;
  }>;
  onBarClick?: (frameworkId: string) => void;
  selectedFrameworkId?: string | null;
  frameworkIds?: string[];  // canonical list for consistent color assignment
}
```

All three are optional so existing usage (without cross-filtering) continues to work without changes.

### 2. Stable Callback Ref for onBarClick

D3 `.on()` captures the closure at attachment time, so passing `onBarClick` directly would use a stale reference. Use a ref to always hold the latest callback:

```typescript
const onBarClickRef = useRef(onBarClick);
onBarClickRef.current = onBarClick;
```

Update the ref on every render (before the `useEffect`). Inside the `useEffect`, call `onBarClickRef.current?.(d.frameworkId)` instead of `onBarClick?.(d.frameworkId)`.

### 3. Click Handler on Bars

In the existing `useEffect`, on the `.join("rect")` chain (where bars are rendered), add:

- `.on("click", (_event, d) => onBarClickRef.current?.(d.frameworkId))` -- fires the cross-filter callback
- `.attr("cursor", onBarClick ? "pointer" : "default")` -- visual affordance

This chains onto the existing `.on("mouseover", ...)` and `.on("mouseout", ...)` calls.

### 4. Keyboard Accessibility

On the same bar rect chain, add:

- `.attr("tabindex", "0")` -- makes bars focusable
- `.attr("role", "button")` -- announces as interactive element
- `.attr("aria-label", d => d.frameworkId)` -- screen reader label
- `.on("keydown", (event, d) => { if (event.key === "Enter" || event.key === " ") { event.preventDefault(); onBarClickRef.current?.(d.frameworkId); } })` -- keyboard activation

### 5. Selected State Styling

After the bar rendering chain (after the `.join("rect")` block), apply conditional styling based on `selectedFrameworkId`. This can be done by selecting all rects again or by chaining onto the existing selection:

- **No selection** (`selectedFrameworkId` is `null` or `undefined`): all bars at opacity 1 (default behavior, no changes needed).
- **A framework is selected**: 
  - Selected bar: full opacity (`1`), 2px stroke using the framework's color from `getFrameworkColor(frameworkIds ?? data.map(d => d.frameworkId), frameworkId)`.
  - Unselected bars: opacity `0.3`.
  - Apply via `.attr("opacity", d => d.frameworkId === selectedFrameworkId ? 1 : 0.3)` and `.attr("stroke", d => d.frameworkId === selectedFrameworkId ? getFrameworkColor(...) : "none")` and `.attr("stroke-width", d => d.frameworkId === selectedFrameworkId ? 2 : 0)`.
  - Add `.attr("transition", "opacity 0.2s ease")` or use D3 `.transition().duration(200)` for smooth changes.

Import `getFrameworkColor` from `../utils/frameworkColors` (created in Section 1).

### 6. Update useEffect Dependencies

Add `selectedFrameworkId` to the `useEffect` dependency array so that changing the selection re-renders the bars with updated styling. The final dependency array becomes:

```typescript
[data, width, t, selectedFrameworkId]
```

Note: `onBarClick` does NOT need to be in the dependency array since we access it through the ref.

### Summary of Changes to CoverageHeatmap.tsx

1. Import `getFrameworkColor` from `../utils/frameworkColors`.
2. Extend `CoverageHeatmapProps` with 3 optional props.
3. Destructure new props in function signature.
4. Add `useRef` for `onBarClick` callback.
5. In the `useEffect`, chain click handler, keyboard accessibility attributes, and keydown handler onto the bar rects.
6. After bar rendering, apply selected/unselected opacity and stroke styling.
7. Add `selectedFrameworkId` to the `useEffect` dependency array.