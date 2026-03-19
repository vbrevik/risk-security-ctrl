I have all the context I need. Let me generate the section content.

# Section 02: Chart Data Aggregation Hook and Summary Statistics

## Overview

This section creates two pieces: the `useChartData` hook that aggregates raw findings into chart-ready data structures, and the `SummaryStats` component that renders six stat cards at the top of the detail page. The hook is also consumed by the D3 chart components in section 03.

## Dependencies

- **Section 01 (prerequisites)** must be completed first. It provides:
  - Fixed `AnalysisFinding` type with nullable fields (`evidence_text`, `recommendation`, `concept_code`, `concept_name`, `concept_definition` as `string | null`)
  - Fixed `PaginatedResponse<T>` using `.items` instead of `.data`
  - All i18n keys under the `stats.*` namespace
  - The route shell in `$id.tsx` that calls `useAnalysis` and `useFindings`

## Files to Create

```
frontend/src/features/analysis/hooks/useChartData.ts
frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
frontend/src/features/analysis/components/SummaryStats.tsx
frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
```

## Tests

Write tests first in the files listed below. The testing stack is Vitest + React Testing Library + jsdom. Hook tests use `renderHook` with a QueryClient wrapper.

### `frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts`

Test the `useChartData` hook via `renderHook`. The hook is a pure computation wrapper around `useMemo`, so it does not need a QueryClient — just `renderHook` from `@testing-library/react`.

Test cases:

1. **Returns zero counts when findings array is empty** -- Pass `[]`. Expect `typeCounts.total === 0`, `frameworkCoverage` is `[]`, `priorityCounts` is `[]`.

2. **Returns zero counts when findings is undefined** -- Pass `undefined`. Same expectations as empty array.

3. **Computes correct typeCounts** -- Pass an array with known `finding_type` values (e.g., 2 addressed, 1 gap, 1 partially_addressed, 1 not_applicable). Assert `typeCounts` matches `{ addressed: 2, partiallyAddressed: 1, gap: 1, notApplicable: 1, total: 5 }`.

4. **Computes correct frameworkCoverage with percentage per framework** -- Pass findings with two different `framework_id` values where some are `addressed` and others are `gap`. Assert each entry in `frameworkCoverage` has the correct `total`, `addressed`, and `percentage` (percentage = addressed / total * 100).

5. **Computes correct priorityCounts for P1-P4** -- Pass findings with priorities 1-4 in known distribution. Assert `priorityCounts` contains entries with matching `{ priority, count }` pairs.

6. **Handles findings with mixed framework_ids correctly** -- Pass findings spanning 3+ frameworks. Assert `frameworkCoverage.length` equals number of distinct frameworks.

7. **frameworkCoverage percentage = addressed / total per framework x 100** -- Pass 3 findings for one framework (2 addressed, 1 gap). Assert percentage is approximately 66.67.

Use factory helpers to create `AnalysisFinding` objects with nullable fields set to `null` by default (to verify the hook handles nulls gracefully).

### `frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx`

Test the `SummaryStats` component with React Testing Library `render`. Mock `react-i18next` with `vi.mock` to return keys as text (standard pattern: `useTranslation` returns `t: (key) => key`).

Test cases:

1. **Renders 6 stat cards** -- Render with valid props. Query for `role="region"` or test-id on each card wrapper, or simply assert 6 Card-like containers appear.

2. **Displays total findings count** -- Assert the text content includes the total count value passed via `chartData.typeCounts.total`.

3. **Displays addressed count with percentage** -- Assert count and percentage string (e.g., "42" and "84%") appear in the document.

4. **Displays gaps count with percentage** -- Same pattern for gap count.

5. **Displays frameworks count** -- Assert the number of frameworks (from `analysis.matched_framework_ids.length`) appears.

6. **Displays formatted processing time** -- Pass `analysis.processing_time_ms = 2300`. Assert "2.3s" appears (the component should format milliseconds to seconds with one decimal).

7. **Displays formatted token count** -- Pass `analysis.token_count = 15420`. Assert a formatted number like "15,420" appears.

8. **Renders skeleton state when isLoading is true** -- Pass `isLoading: true`. Assert skeleton placeholders render (e.g., elements with `animate-pulse` class or Skeleton components).

## Implementation Details

### `useChartData` Hook

**File:** `frontend/src/features/analysis/hooks/useChartData.ts`

**Purpose:** Takes the full array of findings (from the `useFindings(id, { limit: 1000 })` call) and computes aggregated data structures consumed by SummaryStats and the D3 chart components.

**Input:** `findings: AnalysisFinding[] | undefined`

**Return type (define and export this interface):**

```ts
export interface ChartData {
  frameworkCoverage: Array<{
    frameworkId: string;
    total: number;
    addressed: number;
    percentage: number;
  }>;
  priorityCounts: Array<{
    priority: number;
    count: number;
  }>;
  typeCounts: {
    addressed: number;
    partiallyAddressed: number;
    gap: number;
    notApplicable: number;
    total: number;
  };
}
```

**Computation logic (all wrapped in `useMemo` with `[findings]` dependency):**

1. **typeCounts** -- Iterate findings, count by `finding_type`. Map `"partially_addressed"` to the `partiallyAddressed` field, `"not_applicable"` to `notApplicable`. Set `total` to `findings.length`.

2. **frameworkCoverage** -- Group findings by `framework_id`. For each group, count total and count those with `finding_type === "addressed"`. Compute `percentage = (addressed / total) * 100`. Sort alphabetically by `frameworkId`.

3. **priorityCounts** -- Group findings by `priority` (integer 1-4). Return array of `{ priority, count }` sorted by priority ascending. Only include priorities that have at least one finding.

**Edge cases:**
- When `findings` is `undefined` or empty, return all-zero defaults: `frameworkCoverage: []`, `priorityCounts: []`, `typeCounts` with all zeros.
- Null `concept_code`, `concept_name`, `concept_definition` fields do not affect any aggregation (these fields are not used in the hook).

### `SummaryStats` Component

**File:** `frontend/src/features/analysis/components/SummaryStats.tsx`

**Props:**

```ts
interface SummaryStatsProps {
  analysis: Analysis;
  chartData: ChartData;
  isLoading?: boolean;
}
```

**Structure:** A responsive grid of 6 shadcn `Card` components:

```
grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4
```

**The 6 cards (each using `Card`, `CardHeader`, `CardContent` from `@/components/ui/card`):**

| Card | Label (i18n key) | Value | Secondary |
|------|-------------------|-------|-----------|
| Total Findings | `stats.totalFindings` | `typeCounts.total` | -- |
| Addressed | `stats.addressed` | `typeCounts.addressed` | percentage string e.g. "84%" |
| Gaps | `stats.gaps` | `typeCounts.gap` | percentage string |
| Frameworks | `stats.frameworks` | `analysis.matched_framework_ids.length` | -- |
| Processing Time | `stats.processingTime` | formatted from `analysis.processing_time_ms` | -- |
| Token Count | `stats.tokenCount` | formatted `analysis.token_count` | -- |

**Formatting helpers (inline or extracted):**
- `formatProcessingTime(ms: number | null): string` -- Returns `"—"` for null, otherwise converts to seconds with one decimal (e.g., 2300 -> "2.3s", 500 -> "0.5s", 12000 -> "12.0s").
- `formatTokenCount(count: number | null): string` -- Returns `"—"` for null, otherwise formats with locale thousands separator (e.g., `count.toLocaleString()`).
- Percentage calculation: `Math.round((count / total) * 100)` with a guard for `total === 0` returning `0`.

**Loading state:** When `isLoading` is true, render 6 Card shells with `Skeleton` components (from `@/components/ui/skeleton` if available, otherwise `div` elements with `animate-pulse bg-muted rounded h-8 w-20` styling) in place of the value and secondary text.

**i18n:** Use `useTranslation("analysis")` and access keys under `stats.*`.

### Barrel Export Update

Add the new exports to `frontend/src/features/analysis/index.ts`:

```ts
export { useChartData } from "./hooks/useChartData";
export type { ChartData } from "./hooks/useChartData";
export { SummaryStats } from "./components/SummaryStats";
```

## Integration Notes

- The `useChartData` hook is consumed by this section's `SummaryStats` and by section 03's `CoverageHeatmap` (uses `chartData.frameworkCoverage`) and `PriorityChart` (uses `chartData.priorityCounts`).
- The route component in `$id.tsx` (section 06) will call `useChartData(allFindings)` once and pass the result to multiple child components.
- The `SummaryStats` component receives the `isLoading` flag from the parent, which is `true` while the all-findings query is still fetching after an analysis transitions from processing to completed.