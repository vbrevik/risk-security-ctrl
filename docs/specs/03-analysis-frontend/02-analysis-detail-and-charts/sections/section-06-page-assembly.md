Now I have all the context I need. Let me generate the section content.

# Section 06: Page Assembly & Integration

## Overview

This section wires all components built in sections 01-05 into the final route component at `frontend/src/routes/analysis/$id.tsx`. It manages page-level state (filters, pagination, expanded rows), implements conditional rendering for each analysis status, and updates the barrel export file.

**Dependencies:** All previous sections must be complete:
- Section 01: prerequisite fixes, i18n keys, route shell basics
- Section 02: `useChartData` hook, `SummaryStats` component
- Section 03: `CoverageHeatmap`, `PriorityChart`, `useContainerDimensions`
- Section 04: `FindingsTable`, `FindingTypeTag`, filters, pagination
- Section 05: `ExportButtons`, `EmptyFindings`

## Files to Modify

| File | Action |
|------|--------|
| `frontend/src/routes/analysis/$id.tsx` | Replace stub with full page assembly |
| `frontend/src/features/analysis/index.ts` | Add new component/hook exports |

## Tests

Write tests in `frontend/src/routes/analysis/__tests__/$id.test.tsx` (or co-located at the route level per project convention). Mock all API hooks and child components to test page-level orchestration only.

### Test Stubs

```typescript
// frontend/src/routes/analysis/__tests__/$id.test.tsx

import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";

/**
 * Mock useAnalysis, useFindings from features/analysis/api
 * Mock useChartData from features/analysis/hooks/useChartData
 * Mock Route.useParams to return { id: "test-id" }
 * Wrap renders in QueryClientProvider + RouterProvider as needed
 */

describe("AnalysisDetailPage (route integration)", () => {
  it("extracts id from route params and passes to useAnalysis", () => {
    /** Verify useAnalysis is called with the id from route params */
  });

  it("passes filter state to useFindings", () => {
    /**
     * Set up a completed analysis with findings.
     * Verify useFindings is called with current filter/pagination params.
     */
  });

  it("changing filter resets page to 1", () => {
    /**
     * Navigate to page 2, then change a filter dropdown.
     * Verify useFindings is called with page: 1 after the filter change.
     */
  });

  it("changing page updates useFindings params", () => {
    /**
     * Click the Next page button.
     * Verify useFindings is called with the incremented page number.
     */
  });

  it("toggle expand adds/removes finding id from expandedIds set", () => {
    /**
     * Click expand on a finding row.
     * Verify expanded content is visible.
     * Click again to collapse. Verify content is hidden.
     */
  });

  it("completed analysis renders SummaryStats, charts, and table", () => {
    /**
     * Mock a completed analysis with findings.
     * Verify SummaryStats, CoverageHeatmap, PriorityChart, and FindingsTable are rendered.
     */
  });

  it("processing analysis shows only header and processing banner", () => {
    /**
     * Mock an analysis with status "processing".
     * Verify processing banner is visible.
     * Verify charts and table are NOT rendered.
     */
  });

  it("failed analysis shows error message", () => {
    /**
     * Mock an analysis with status "failed".
     * Verify error message content is displayed.
     */
  });
});
```

## Implementation Details

### Route Component (`frontend/src/routes/analysis/$id.tsx`)

Replace the existing stub entirely. The component has these responsibilities:

**1. Route setup and data fetching:**

```typescript
import { createFileRoute, Link } from "@tanstack/react-router";
// Import all components and hooks from features/analysis

export const Route = createFileRoute("/analysis/$id")({
  component: AnalysisDetailPage,
});
```

Inside `AnalysisDetailPage`:

- Extract `id` via `Route.useParams()`
- Call `useAnalysis(id)` with these options added to the existing hook call:
  - `refetchOnMount: 'always'` (ensures fresh data when navigating back)
  - `refetchInterval`: conditionally set to `5000` when `analysis.status === "processing"`, otherwise `false`
- Call `useFindings(id, { limit: 1000 })` for all findings (chart data aggregation). Set `enabled: status === "completed"`.
- Call `useFindings(id, { page, limit: 20, framework_id, finding_type, priority })` for paginated table data. Set `enabled: status === "completed"`.
- Call `useChartData(allFindings?.items)` to compute aggregated chart/stat data.

**2. State management (all via `useState`):**

- `page: number` -- pagination state, default `1`
- `filters: { framework_id?: string; finding_type?: FindingType; priority?: number }` -- filter state, default `{}`
- `expandedIds: Set<string>` -- tracks which finding rows are expanded

**3. Filter change handler:** When any filter changes, always reset `page` to `1`:

```typescript
function handleFilterChange(newFilters: Partial<typeof filters>) {
  setFilters((prev) => ({ ...prev, ...newFilters }));
  setPage(1);
}
```

**4. Expand toggle handler:**

```typescript
function handleToggleExpand(id: string) {
  setExpandedIds((prev) => {
    const next = new Set(prev);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    return next;
  });
}
```

**5. Conditional rendering logic (in order of precedence):**

| Condition | Renders |
|-----------|---------|
| `useAnalysis` is loading | Full-page skeleton (card placeholders + table placeholder) |
| `useAnalysis` returns error / 404 | Error state with "not found" heading, message, and back link to `/analysis` |
| `status === "failed"` | PageHeader + error message from `analysis.error_message` |
| `status === "processing"` | PageHeader + ProcessingBanner (pulse animation, i18n message). No charts or table. |
| `status === "completed"` and chart data still loading | PageHeader + skeleton stat cards + skeleton chart placeholders |
| `status === "completed"` and zero findings | PageHeader + `<EmptyFindings />` |
| `status === "completed"` with findings | Full page: PageHeader, SummaryStats, ChartsSection, FindingsSection |

**6. Inline sub-sections (not separate component files):**

**PageHeader** -- rendered at the top in all non-loading/error states:
- `<Link to="/analysis">` with left arrow and i18n `detail.backToList` text
- Analysis name as `<h1>`
- `<StatusBadge status={analysis.status} />` (reused from section 01)
- Metadata line: input type label, created date (formatted)
- `<ExportButtons analysisId={id} analysisName={analysis.name} status={analysis.status} />`

**ProcessingBanner** -- simple `div` or shadcn Alert:
- Pulse/spinner animation
- i18n `detail.processing.banner` heading
- i18n `detail.processing.message` body text

**ChartsSection** -- grid wrapper:
- CSS classes: `grid grid-cols-1 lg:grid-cols-2 gap-6`
- Contains `<CoverageHeatmap data={chartData.frameworkCoverage} />` and `<PriorityChart data={chartData.priorityCounts} />`

**FindingsSection** -- wrapper containing:
- Heading with i18n `findings.title`
- Filter dropdowns (three shadcn Select components inline or via FindingsFilters):
  - Framework: options from `analysis.matched_framework_ids`, "All" clears filter
  - Finding Type: `addressed`, `partially_addressed`, `gap`, `not_applicable`, "All" clears filter
  - Priority: 1-4 mapped to P1-P4 labels, "All" clears filter
- `<FindingsTable findings={paginatedFindings?.items ?? []} expandedIds={expandedIds} onToggleExpand={handleToggleExpand} />`
- Pagination controls: "Page X of Y" text, Previous/Next buttons with disabled states

**7. Page layout:**

The outermost wrapper uses `max-w-7xl mx-auto p-6 space-y-6` for consistent spacing and max-width constraint.

### Barrel Export Update (`frontend/src/features/analysis/index.ts`)

Add exports for all new components and hooks created in sections 02-05:

```typescript
// Hooks
export { useChartData } from "./hooks/useChartData";
export { useContainerDimensions } from "./hooks/useContainerDimensions";

// Components
export { SummaryStats } from "./components/SummaryStats";
export { CoverageHeatmap } from "./components/CoverageHeatmap";
export { PriorityChart } from "./components/PriorityChart";
export { FindingsTable } from "./components/FindingsTable";
export { FindingTypeTag } from "./components/FindingTypeTag";
export { ExportButtons } from "./components/ExportButtons";
export { EmptyFindings } from "./components/EmptyFindings";
```

Note: The route component (`$id.tsx`) should import directly from the feature paths rather than through the barrel to avoid circular dependency issues. The barrel exports are for external consumers.

## Key Integration Points

**useAnalysis hook modification:** The existing `useAnalysis` hook in `frontend/src/features/analysis/api/index.ts` needs its options extended. The route component should pass `refetchOnMount` and `refetchInterval` via the hook. If the hook signature does not accept options, either extend it to accept an options parameter or configure the query options directly in the route. The simplest approach: add an optional second parameter to `useAnalysis` for extra query options, or configure `refetchInterval` via the query's `refetchInterval` callback pattern (checking `query.state.data?.status`).

**useFindings enabled flag:** The existing `useFindings` hook accepts `FindingsListParams` but does not have a mechanism to disable the query based on analysis status. The route component should pass `enabled: false` when the analysis is not completed. This may require extending the hook or using the `enabled` option at the call site via a wrapper.

**Export function enhancement:** The `exportAnalysis` function in `api/index.ts` currently uses `analysis-${id}.${format}` as the download filename. The `ExportButtons` component passes `analysisName` so the download uses a user-friendly name. Either modify `exportAnalysis` to accept an optional `filename` parameter, or have `ExportButtons` handle the filename override.

## Edge Cases to Handle

- **Stale page after filter change:** Always reset page to 1 when filters change to avoid requesting a page that no longer exists with the new filter set.
- **Expanded rows across page changes:** The `expandedIds` set persists across page changes. Finding IDs from a previous page will simply not match any row on the new page, so no visual issue occurs. Optionally clear `expandedIds` on page change for cleanliness.
- **Processing to completed transition:** When auto-polling detects status change from "processing" to "completed", the chart data query becomes enabled and starts loading. The skeleton state for stats/charts covers this transition window.
- **Multiple rapid filter changes:** Each filter change triggers a new `useFindings` query. TanStack Query handles deduplication and cancellation of stale requests automatically.
- **Failed analysis with error_message null:** Display a generic failure message from i18n (`detail.failed.message`) when `error_message` is null.