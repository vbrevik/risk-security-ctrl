Now I have all the context I need. Let me generate the section content.

# Section 5: Detail Page Assembly and Cross-Filter Wiring

## Overview

This section wires all enhancement components (from sections 1-4) together in the analysis detail page (`$id.tsx`). It adds cross-filter state management, a filter banner, the 3-column chart grid with the new FrameworkRadar, concept drawer integration, scroll-to-table behavior, and `overrideTypeCounts` support in SummaryStats.

## Dependencies

- **Section 01** (colors and chart data): provides `frameworkColors` utility and `radarData` field on `ChartData`
- **Section 02** (heatmap cross-filter): provides `onBarClick`, `selectedFrameworkId`, `frameworkIds` props on `CoverageHeatmap`
- **Section 03** (framework radar): provides the `FrameworkRadar` component
- **Section 04** (concept drawer): provides `ConceptDrawer` component and `onConceptClick` prop on `FindingsTable`

## Files Modified

| File | Action |
|------|--------|
| `src/features/analysis/components/SummaryStats.tsx` | Add optional `overrideTypeCounts` prop |
| `src/routes/analysis/$id.tsx` | Primary integration: new state, callbacks, layout changes |
| `src/features/analysis/components/__tests__/SummaryStats.test.tsx` | Extend with override tests |

## Tests First

Extend the existing test file at `src/features/analysis/components/__tests__/SummaryStats.test.tsx`. The existing test infrastructure (mocks, helpers `makeAnalysis`, `makeChartData`) is already in place.

Add three new tests inside the existing `describe("SummaryStats", ...)` block:

### Test: overrideTypeCounts replaces finding-type card values

When `overrideTypeCounts` is provided, the 4 finding-type stats cards (total, addressed, gaps, partial) should display the overridden values instead of `chartData.typeCounts`.

```typescript
it("when overrideTypeCounts is provided, finding-type cards show overridden values", () => {
  /** Render SummaryStats with overrideTypeCounts that differ from chartData.typeCounts.
   *  Assert that the overridden total/addressed/gap values appear in the document,
   *  and the original chartData values do NOT appear. */
});
```

### Test: overrideTypeCounts does not affect analysis-level cards

When `overrideTypeCounts` is provided, the framework count, processing time, and token count cards should remain unchanged since those are analysis-level metrics.

```typescript
it("when overrideTypeCounts is provided, framework/processing/token cards remain unchanged", () => {
  /** Render with overrideTypeCounts. Assert frameworks count ("2"), processing time ("2.3s"),
   *  and token count ("15,420") still render correctly from the analysis prop. */
});
```

### Test: backward compatibility without overrideTypeCounts

```typescript
it("when overrideTypeCounts is not provided, behaves as before", () => {
  /** Render without overrideTypeCounts prop. Assert original chartData.typeCounts values
   *  appear in the document (same as existing tests, but explicit about the absence of the prop). */
});
```

### Detail page integration tests (optional, higher-level)

These tests verify the wiring in `$id.tsx`. They are more complex because they require mocking `useAnalysis`, `useFindings`, `useChartData`, and all child components. Include them only if feasible; they may be deferred to section 06.

```typescript
// Test: clicking heatmap bar updates filter state (verify FindingsTable receives new framework_id filter)
// Test: clicking already-selected bar clears the filter
// Test: filter banner shows framework name when filtered
// Test: filter banner clear button resets framework_id filter
```

## Implementation Details

### 1. SummaryStats: Add `overrideTypeCounts` Prop

**File:** `src/features/analysis/components/SummaryStats.tsx`

Add an optional prop `overrideTypeCounts` to `SummaryStatsProps`:

```typescript
interface SummaryStatsProps {
  analysis: Analysis;
  chartData: ChartData;
  isLoading?: boolean;
  overrideTypeCounts?: ChartData["typeCounts"];
}
```

Inside the component, derive the effective type counts:

```typescript
const typeCounts = overrideTypeCounts ?? chartData.typeCounts;
```

Use this `typeCounts` variable for the first 3 cards (total findings, addressed, gaps) instead of `chartData.typeCounts`. The remaining 3 cards (frameworks, processing time, token count) continue reading from `analysis` directly as they do today -- no change needed for those.

The existing `cards` array currently references `chartData.typeCounts.total`, `chartData.typeCounts.addressed`, and `chartData.typeCounts.gap` in the first three entries. Replace those references with `typeCounts.total`, `typeCounts.addressed`, and `typeCounts.gap` respectively.

### 2. Detail Page Assembly (`$id.tsx`)

**File:** `src/routes/analysis/$id.tsx`

#### New imports

Add imports for:
- `useCallback`, `useRef`, `useMemo` from React
- `X` icon from `lucide-react`
- `FrameworkRadar` from analysis components
- `ConceptDrawer` from analysis components

#### New state

```typescript
const [selectedConceptId, setSelectedConceptId] = useState<string | null>(null);
```

The `selectedFrameworkId` is **not** separate state. It is derived from `filters.framework_id`. This keeps heatmap selection and table filtering automatically in sync.

#### Scroll ref

```typescript
const findingsRef = useRef<HTMLDivElement>(null);
```

Attach this ref to the `<div>` wrapping the findings table section.

#### handleBarClick callback

```typescript
const handleBarClick = useCallback((frameworkId: string) => {
  setFilters(prev => ({
    ...prev,
    framework_id: prev.framework_id === frameworkId ? undefined : frameworkId,
  }));
  setPage(1);
  findingsRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
}, []);
```

Key design choices:
- Uses functional updater on `setFilters` so the callback has **no dependency on `filters`** and never needs recreation.
- Toggling: if the same framework is clicked again, it clears the filter.
- Resets page to 1 on every filter change.
- Scrolls the findings table into view.

#### Filtered type counts (useMemo)

```typescript
const filteredTypeCounts = useMemo(() => {
  if (!filters.framework_id || !allFindingsData?.data) return undefined;
  const filtered = allFindingsData.data.filter(
    f => f.framework_id === filters.framework_id
  );
  /** Count by finding_type to produce a ChartData["typeCounts"] object.
   *  Same counting logic as useChartData but for a filtered subset. */
}, [filters.framework_id, allFindingsData?.data]);
```

This is passed to `SummaryStats` as `overrideTypeCounts={filteredTypeCounts}`.

#### Chart grid layout change

Change the chart grid from 2-column to 3-column at xl breakpoint:

```
Before: <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
After:  <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
```

The three children inside the grid:
1. `<CoverageHeatmap>` with new props: `onBarClick={handleBarClick}`, `selectedFrameworkId={filters.framework_id}`, `frameworkIds={analysis.matched_framework_ids}`
2. `<FrameworkRadar>` with props: `data={chartData.radarData}`, `selectedFrameworkId={filters.framework_id}`, `frameworkIds={analysis.matched_framework_ids}`
3. `<PriorityChart>` unchanged: `data={chartData.priorityCounts}`

#### Filter banner

When `filters.framework_id` is set, render a banner between SummaryStats and the chart grid:

```tsx
{filters.framework_id && (
  <div className="flex items-center gap-2 bg-muted rounded px-3 py-1 text-sm">
    <span>{t("detail.filteredBy", { framework: filters.framework_id })}</span>
    <button
      onClick={() => setFilters(prev => ({ ...prev, framework_id: undefined }))}
      className="ml-auto hover:bg-accent rounded p-0.5"
      aria-label={t("detail.clearFilter")}
    >
      <X className="h-3 w-3" />
    </button>
  </div>
)}
```

The i18n keys `detail.filteredBy` and `detail.clearFilter` are defined in section 06.

#### Concept drawer

Add `onConceptClick` prop to the `FindingsTable` instance:

```tsx
<FindingsTable
  {...existingProps}
  onConceptClick={(conceptId) => setSelectedConceptId(conceptId)}
/>
```

Render the `ConceptDrawer` at the bottom of the page component (inside the root `<div>` but outside any conditional blocks so it can animate open/closed regardless of other state):

```tsx
<ConceptDrawer
  conceptId={selectedConceptId}
  onClose={() => setSelectedConceptId(null)}
/>
```

#### Findings section ref

Wrap the findings table section with the scroll ref:

```tsx
<div ref={findingsRef}>
  <h2 className="text-lg font-semibold mb-4">{t("findings.title")}</h2>
  <FindingsTable ... />
</div>
```

### Complete Structure of the Modified Return JSX

The `hasFindings` block in the return JSX should follow this order:

1. `<SummaryStats>` with `overrideTypeCounts={filteredTypeCounts}`
2. Filter banner (conditional on `filters.framework_id`)
3. Chart grid (3 charts in responsive grid)
4. Findings table section (with ref)
5. `<ConceptDrawer>` (always rendered, opens/closes based on `selectedConceptId`)

---

## Implementation Notes

### Actual files modified
- `frontend/src/features/analysis/components/SummaryStats.tsx` — Added `overrideTypeCounts` optional prop
- `frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx` — Added 3 override tests + `radarData` to helper
- `frontend/src/routes/analysis/$id.tsx` — Full page assembly with cross-filter wiring

### Deviations from plan
- `filteredTypeCounts` uses a single `for...of` loop instead of multiple `.filter()` calls (code review auto-fix for efficiency)
- Detail page integration tests deferred to section 06 as the plan suggested

### Test count
- 3 new SummaryStats tests (override values, analysis cards unchanged, backward compatibility)
- All 213 tests passing