Now I have all the context I need. Let me produce the section content.

# Section 1: Shared Framework Color Utility and useChartData Extension

## Overview

This section introduces two foundational pieces used by later sections:

1. A **deterministic framework color utility** (`frameworkColors.ts`) that maps framework IDs to consistent hex colors using `d3.schemeTableau10`.
2. An **extension to the `useChartData` hook** that adds a `radarData` field with normalized finding-type percentages per framework.

No other sections need to be completed first. Sections 2, 3, and 5 depend on this section.

## Files To Create/Modify

| File | Action |
|------|--------|
| `src/features/analysis/utils/frameworkColors.ts` | **Create** |
| `src/features/analysis/utils/__tests__/frameworkColors.test.ts` | **Create** |
| `src/features/analysis/hooks/useChartData.ts` | **Modify** |
| `src/features/analysis/hooks/__tests__/useChartData.test.ts` | **Modify** (add radarData tests) |
| `src/features/analysis/index.ts` | **Modify** (add export) |

## Tests First

### frameworkColors utility tests

**File:** `src/features/analysis/utils/__tests__/frameworkColors.test.ts`

```typescript
import { describe, it, expect } from "vitest";
import { getFrameworkColor } from "../frameworkColors";

describe("getFrameworkColor", () => {
  /** Returns a valid hex color string for a known framework ID */
  it("returns a hex color string for a known framework ID", () => { /* ... */ });

  /** Same framework ID always maps to the same color given the same frameworkIds array */
  it("same framework ID always gets same color given same frameworkIds array", () => { /* ... */ });

  /** Different frameworks receive different colors (up to palette size of 10) */
  it("different frameworks get different colors (up to 10)", () => { /* ... */ });

  /** After 10 frameworks, colors wrap around via mod 10 */
  it("wraps around after 10 frameworks (mod 10 behavior)", () => { /* ... */ });

  /** Color assignment is deterministic: sorts IDs alphabetically before indexing */
  it("order is deterministic (sorts IDs alphabetically before indexing)", () => { /* ... */ });
});
```

The key assertions:

- Call `getFrameworkColor(["fw-a", "fw-b"], "fw-a")` and expect a string matching `/^#[0-9a-fA-F]{6}$/` (hex color).
- Call with the same arguments twice, expect the same result.
- Call with 10 distinct framework IDs, expect all 10 returned colors to be unique.
- Call with 11 framework IDs. The 11th framework (by alphabetical sort) should receive the same color as the 1st.
- Call with `["fw-b", "fw-a"]` and `["fw-a", "fw-b"]` for the same `frameworkId` argument, expect the same color (because the function sorts internally).

### useChartData radarData tests

**File:** `src/features/analysis/hooks/__tests__/useChartData.test.ts` (add a new `describe("radarData", ...)` block)

```typescript
describe("radarData", () => {
  /** Groups findings by framework and produces normalized percentages */
  it("groups findings by framework with normalized percentages", () => { /* ... */ });

  /** Percentages within each framework sum to 100 */
  it("percentages sum to 100 per framework", () => { /* ... */ });

  /** Returns empty array when no findings */
  it("returns empty array when no findings", () => { /* ... */ });

  /** Single framework with all findings of one type yields 100% for that type */
  it("handles single framework with all one type", () => { /* ... */ });

  /** Each radarData entry includes the raw total count */
  it("includes total raw count per framework", () => { /* ... */ });
});
```

Key test scenario for the percentage normalization test: given 2 frameworks:
- `fw-a`: 2 addressed, 1 gap, 1 partially_addressed (total 4) => addressed: 50, gap: 25, partial: 25, notApplicable: 0
- `fw-b`: 1 not_applicable, 1 addressed (total 2) => addressed: 50, notApplicable: 50, gap: 0, partial: 0

The existing `makeFinding` helper in the test file should be reused.

## Implementation Details

### Framework Color Utility

**File:** `src/features/analysis/utils/frameworkColors.ts`

Export a single function:

```typescript
export function getFrameworkColor(frameworkIds: string[], frameworkId: string): string
```

**Algorithm:**
1. Create a sorted copy of `frameworkIds` (alphabetical sort via `[...frameworkIds].sort()`).
2. Find the index of `frameworkId` in the sorted array.
3. Return `d3.schemeTableau10[index % 10]`.

Import only `schemeTableau10` from `d3` (or from `d3-scale-chromatic` if tree-shaking is preferred -- check what the project already imports). The existing D3 chart components import from `"d3"` directly, so use the same import style.

`d3.schemeTableau10` is an array of 10 hex color strings. No conversion needed.

If `frameworkId` is not found in `frameworkIds`, fall back to index 0 (i.e., return the first palette color). This prevents crashes on stale data.

### useChartData Extension

**File:** `src/features/analysis/hooks/useChartData.ts`

**Changes to `ChartData` interface** -- add:

```typescript
radarData: Array<{
  frameworkId: string;
  values: { addressed: number; partial: number; gap: number; notApplicable: number };
  total: number;
}>;
```

**Changes to `EMPTY_CHART_DATA`** -- add `radarData: []`.

**Changes inside `useMemo`** -- after the existing `priorityCounts` computation, add radar data computation:

1. Group findings by `framework_id` using a `Map<string, Map<FindingType, number>>`.
2. For each framework group, count findings per `finding_type`.
3. Map to the output structure:
   - `addressed` = count of `"addressed"` / total * 100
   - `partial` = count of `"partially_addressed"` / total * 100
   - `gap` = count of `"gap"` / total * 100
   - `notApplicable` = count of `"not_applicable"` / total * 100
4. Sort by `frameworkId` alphabetically (consistent with `frameworkCoverage`).

The `FindingType` type is `"addressed" | "partially_addressed" | "gap" | "not_applicable"` (from `features/analysis/types`).

Include `radarData` in the returned object alongside `frameworkCoverage`, `priorityCounts`, and `typeCounts`.

### Index Export

**File:** `src/features/analysis/index.ts`

Add this line:

```typescript
export { getFrameworkColor } from "./utils/frameworkColors";
```

## Background Context

**Existing `AnalysisFinding` type** has these relevant fields (from `src/features/analysis/types/index.ts`):
- `framework_id: string`
- `finding_type: FindingType` where `FindingType = "addressed" | "partially_addressed" | "gap" | "not_applicable"`

**Existing `useChartData` hook** (at `src/features/analysis/hooks/useChartData.ts`) takes `AnalysisFinding[] | undefined` and returns a `ChartData` object computed in a single `useMemo`. It currently computes `typeCounts`, `frameworkCoverage`, and `priorityCounts`. The radar data computation fits naturally at the end of the same `useMemo` block.

**Existing test file** at `src/features/analysis/hooks/__tests__/useChartData.test.ts` has a `makeFinding` helper that creates a default `AnalysisFinding` with overridable fields. New radar tests should use this same helper.

**D3 imports:** Existing chart components import from `"d3"` directly (e.g., `import * as d3 from "d3"`). The color utility should follow the same pattern.