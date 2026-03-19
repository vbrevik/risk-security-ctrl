Now I have all the context needed. Let me generate the section content.

# Section 6: i18n and Tests

## Overview

This section adds all new internationalization keys for the radar chart, concept drawer, and cross-filter features, then provides comprehensive test coverage for all components and utilities created in Sections 1-5. This is the final section and depends on all previous sections being complete.

## Dependencies

- **Section 1**: frameworkColors utility and useChartData radarData extension (tested here)
- **Section 2**: CoverageHeatmap cross-filter props (tested here)
- **Section 3**: FrameworkRadar component (tested here)
- **Section 4**: ConceptDrawer component and FindingsTable onConceptClick (tested here)
- **Section 5**: SummaryStats overrideTypeCounts prop (tested here)

## i18n Key Additions

### File: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/i18n/locales/en/analysis.json`

Add the following keys to the existing JSON structure.

**Under `charts`, add a new `radar` object:**

```json
"radar": {
  "title": "Framework Radar",
  "description": "Normalized finding type distribution across frameworks",
  "noData": "No radar data available",
  "addressed": "Addressed",
  "partial": "Partial",
  "gap": "Gap",
  "notApplicable": "Not Applicable",
  "legend": "Legend",
  "percentage": "{{value}}%"
}
```

**Under `detail`, add a new `conceptPanel` object and two cross-filter keys:**

```json
"conceptPanel": {
  "title": "Concept Details",
  "close": "Close panel",
  "openInExplorer": "Open in Ontology Explorer",
  "definition": "Definition",
  "type": "Type",
  "framework": "Framework",
  "relatedConcepts": "Related Concepts",
  "crossMappings": "Cross-Framework Mappings",
  "loading": "Loading concept...",
  "error": "Failed to load concept details",
  "retry": "Retry"
},
"filteredBy": "Showing results for: {{framework}}",
"clearFilter": "Clear filter"
```

### File: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/i18n/locales/nb/analysis.json`

Add the Norwegian (Bokmal) equivalents at the same key paths.

**Under `charts.radar`:**

```json
"radar": {
  "title": "Rammeverkradar",
  "description": "Normalisert funntype-fordeling på tvers av rammeverk",
  "noData": "Ingen radardata tilgjengelig",
  "addressed": "Adressert",
  "partial": "Delvis",
  "gap": "Mangel",
  "notApplicable": "Ikke relevant",
  "legend": "Tegnforklaring",
  "percentage": "{{value}}%"
}
```

**Under `detail.conceptPanel` and cross-filter keys:**

```json
"conceptPanel": {
  "title": "Konseptdetaljer",
  "close": "Lukk panel",
  "openInExplorer": "Åpne i Ontologiutforsker",
  "definition": "Definisjon",
  "type": "Type",
  "framework": "Rammeverk",
  "relatedConcepts": "Relaterte konsepter",
  "crossMappings": "Kryssrammeverk-koblinger",
  "loading": "Laster konsept...",
  "error": "Kunne ikke laste konseptdetaljer",
  "retry": "Prøv igjen"
},
"filteredBy": "Viser resultater for: {{framework}}",
"clearFilter": "Fjern filter"
```

---

## Tests

All tests use Vitest and React Testing Library. D3-based component tests inspect the SVG DOM via `container.querySelectorAll()`. Components that use i18n and `useContainerDimensions` must mock those dependencies (following the existing test patterns shown below).

### Mocking Conventions (existing patterns to follow)

The existing test files establish these mocking conventions:

```typescript
// i18n mock - returns the key as-is
vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}));

// Container dimensions mock - returns fixed dimensions
vi.mock("../../hooks/useContainerDimensions", () => ({
  useContainerDimensions: () => ({ width: 400, height: 400 }),
}));
```

### Test File 1: frameworkColors utility tests (NEW)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/utils/__tests__/frameworkColors.test.ts`

This file tests the `getFrameworkColor` function from `features/analysis/utils/frameworkColors.ts`.

Tests to write:

- **returns a hex color string for a known framework ID** -- call with a sorted array and one of its members, assert return value matches `/#[0-9a-fA-F]{6}/` pattern
- **same framework ID always gets same color given same frameworkIds array** -- call twice, assert equal
- **different frameworks get different colors (up to 10)** -- create 10 distinct framework IDs, map each through the function, assert the resulting set has 10 unique values
- **wraps around after 10 frameworks (mod 10 behavior)** -- create 11 framework IDs, assert the 11th gets the same color as the 1st (index 10 mod 10 = 0)
- **order is deterministic (sorts IDs alphabetically before indexing)** -- pass the same IDs in different order in the array, assert the color for a given ID is the same regardless of input order

### Test File 2: useChartData radarData tests (EXTEND)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts`

The existing file has a `makeFinding` helper and tests for `typeCounts`, `frameworkCoverage`, and `priorityCounts`. Add a new `describe("radarData", ...)` block.

Existing `makeFinding` helper produces findings with fields `id`, `concept_id`, `framework_id`, `finding_type`, `confidence_score`, `evidence_text`, `recommendation`, `priority`, `sort_order`, `concept_code`, `concept_name`, `concept_definition`.

Tests to add:

- **radarData groups findings by framework with normalized percentages** -- provide findings across 2 frameworks (e.g., fw-a with 2 addressed + 1 gap, fw-b with 1 partial + 1 not_applicable), assert `radarData` has 2 entries with correct normalized values (e.g., fw-a: addressed=66.67, gap=33.33, partial=0, notApplicable=0)
- **radarData percentages sum to 100 per framework** -- for each framework entry, assert `values.addressed + values.partial + values.gap + values.notApplicable` is approximately 100
- **radarData returns empty array when no findings** -- `renderHook(() => useChartData([]))`, assert `result.current.radarData` equals `[]`
- **radarData handles single framework with all one type** -- all findings are "gap" for one framework, assert gap=100, others=0
- **radarData includes total raw count per framework** -- assert each entry's `total` matches the number of findings for that framework

### Test File 3: CoverageHeatmap cross-filter tests (EXTEND)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/CoverageHeatmap.test.tsx`

Existing file renders `CoverageHeatmap` with `sampleData` array of `{ frameworkId, percentage, addressed, total }`. Uses existing mocks for `react-i18next` and `useContainerDimensions`. Add tests in a new `describe("cross-filter", ...)` block.

Tests to add:

- **onBarClick callback fires with correct frameworkId when a bar rect is clicked** -- render with `onBarClick={vi.fn()}`, find first `rect` in SVG, `fireEvent.click(rect)`, assert mock called with the first framework ID
- **no error when onBarClick is not provided (backward compatible)** -- render without `onBarClick`, click a rect, assert no thrown error
- **when selectedFrameworkId is set, SVG contains bars with reduced opacity for non-selected** -- render with `selectedFrameworkId="iso-31000"`, query all rects, assert non-selected rects have opacity attribute less than 1
- **when selectedFrameworkId is null, all bars have full opacity** -- render with `selectedFrameworkId={null}`, query all rects, assert none have reduced opacity
- **bar rects have role="button" and tabindex="0" for keyboard accessibility** -- render with `onBarClick`, query rects, assert each has `getAttribute("role") === "button"` and `getAttribute("tabindex") === "0"`
- **bar rects have aria-label containing the framework ID** -- render with sampleData, query rects, assert each has an `aria-label` attribute containing its framework ID string

### Test File 4: FrameworkRadar tests (NEW)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/FrameworkRadar.test.tsx`

Mock `react-i18next` and `useContainerDimensions` (return `{ width: 400, height: 400 }`). Import `FrameworkRadar` from `../FrameworkRadar`.

Define test data:

```typescript
const sampleRadarData = [
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
```

Tests to write:

- **renders SVG element with role="img" and aria-labelledby** -- render with sampleRadarData, assert SVG has `role="img"` and `aria-labelledby` containing a title ID
- **renders one path element per framework in data** -- `container.querySelectorAll("svg path")`, filter to polygon paths (those with fill opacity), assert count matches data length
- **renders no paths when data is empty** -- render with `data={[]}`, assert no polygon path elements
- **shows noData message when data array is empty** -- render with `data={[]}`, assert `screen.getByText("charts.radar.noData")` is in the document
- **renders 4 axis labels (one per finding type)** -- assert text elements exist for `charts.radar.addressed`, `charts.radar.partial`, `charts.radar.gap`, `charts.radar.notApplicable`
- **renders concentric grid circles** -- query `svg circle` elements, assert at least 4 (the grid rings)
- **legend section shows framework names matching data** -- assert text content includes "iso-31000" and "nist-csf" outside the SVG (in React-rendered legend)
- **limits rendering to 8 frameworks when more provided** -- create data array with 10 entries, assert path count is capped at 8
- **when selectedFrameworkId is set, selected path has different styling** -- render with `selectedFrameworkId="iso-31000"`, inspect path stroke-width or opacity for differentiation

### Test File 5: ConceptDrawer tests (NEW)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/ConceptDrawer.test.tsx`

Mock `useConceptRelationships` from the ontology API. The shadcn Sheet component renders a dialog-like structure with `[data-state="open"]` / `[data-state="closed"]` attributes.

```typescript
vi.mock("../../../ontology/api", () => ({
  useConceptRelationships: vi.fn(),
}));
```

Define mock return values for different states (loading, success, error).

Tests to write:

- **renders nothing / closed state when conceptId is null** -- render with `conceptId={null}`, assert the Sheet is not visible (no open state)
- **renders Sheet in open state when conceptId is non-null** -- mock hook to return success data, render with `conceptId="c1"`, assert Sheet content is visible
- **displays concept name, code, type, framework from fetched data** -- mock hook to return concept data with known values, assert those values appear in the rendered output
- **displays concept definition** -- assert the definition text from mock data appears
- **shows loading skeleton when data is loading** -- mock hook to return `{ isLoading: true }`, assert skeleton/pulse elements are present
- **shows error state with retry button when fetch fails** -- mock hook to return `{ isError: true }`, assert error message and retry button appear
- **"Open in Ontology Explorer" link has correct href and target** -- assert a link element with `href="/ontology?concept=c1"` and `target="_blank"` exists
- **close button calls onClose callback** -- render with `onClose={vi.fn()}`, click the close button, assert the mock was called

### Test File 6: FindingsTable concept click tests (EXTEND)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/FindingsTable.test.tsx`

Existing file has `makeFinding` helper and `defaultProps`. Add a new `describe("concept click", ...)` block.

Tests to add:

- **concept name cell is clickable (has button role)** -- render with `onConceptClick={vi.fn()}`, find the button within the concept name cell for a finding with non-null `concept_name`, assert it exists
- **clicking concept name fires onConceptClick with finding's concept_id** -- render with `onConceptClick={vi.fn()}`, click the concept name button, assert mock called with `"c1"`
- **concept code cell shows em-dash and is not clickable when concept_code is null** -- the second finding in `defaultProps` has `concept_code: null`, assert its code cell shows "\u2014" and does not contain a button
- **no error when onConceptClick is not provided** -- render without `onConceptClick`, assert no crash (backward compatible)

### Test File 7: SummaryStats overrideTypeCounts tests (EXTEND)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx`

Existing file has `makeAnalysis` and `makeChartData` helpers. Add a new `describe("overrideTypeCounts", ...)` block.

The `ChartData` type will need `radarData` added after Section 1. The `makeChartData` helper should include `radarData: []` to match the updated type.

Tests to add:

- **when overrideTypeCounts is provided, finding-type cards show overridden values** -- render with `overrideTypeCounts={{ addressed: 10, partiallyAddressed: 2, gap: 3, notApplicable: 1, total: 16 }}`, assert "16" appears for total, "10" for addressed, "3" for gaps
- **when overrideTypeCounts is provided, framework count / processing time / token count cards remain unchanged** -- same render as above, assert "2" (frameworks), "2.3s" (processing time), "15,420" (tokens) still appear
- **when overrideTypeCounts is not provided, behaves as before (backward compatible)** -- render without `overrideTypeCounts`, assert original values from `chartData.typeCounts` appear

### Test File 8: i18n validation tests (EXTEND)

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/i18n/__tests__/analysis-namespace.test.ts`

Existing file imports `i18n` from `../index` and has 3 tests. Add a new `describe("enhancement keys", ...)` block.

Tests to add:

- **all new radar chart keys exist in both en and nb translation files** -- for each key in `["charts.radar.title", "charts.radar.description", "charts.radar.noData", "charts.radar.addressed", "charts.radar.partial", "charts.radar.gap", "charts.radar.notApplicable", "charts.radar.legend", "charts.radar.percentage"]`, assert `i18n.t(key, { ns: "analysis", lng: "en" })` does not return the key itself, and similarly for `lng: "nb"`
- **all new concept panel keys exist in both en and nb translation files** -- for each key in `["detail.conceptPanel.title", "detail.conceptPanel.close", "detail.conceptPanel.openInExplorer", "detail.conceptPanel.definition", "detail.conceptPanel.type", "detail.conceptPanel.framework", "detail.conceptPanel.relatedConcepts", "detail.conceptPanel.crossMappings", "detail.conceptPanel.loading", "detail.conceptPanel.error", "detail.conceptPanel.retry"]`, same assertion
- **all new cross-filter keys exist in both en and nb translation files** -- for keys `["detail.filteredBy", "detail.clearFilter"]`, same assertion
- **no new keys are empty strings** -- for all new keys listed above, assert `i18n.t(key, { ns: "analysis", lng: "en" }).length > 0` and same for nb

Use `it.each` or a helper loop to keep the test code concise. Example pattern:

```typescript
const radarKeys = [
  "charts.radar.title",
  "charts.radar.description",
  // ... etc
];

it.each(radarKeys)("radar key '%s' exists in en and nb", (key) => {
  const en = i18n.t(key, { ns: "analysis", lng: "en" });
  const nb = i18n.t(key, { ns: "analysis", lng: "nb" });
  expect(en).not.toBe(key);
  expect(nb).not.toBe(key);
  expect(en.length).toBeGreaterThan(0);
  expect(nb.length).toBeGreaterThan(0);
});
```

---

## Files Summary

### New files

| File | Purpose |
|------|---------|
| `src/features/analysis/utils/__tests__/frameworkColors.test.ts` | Tests for framework color utility |
| `src/features/analysis/components/__tests__/FrameworkRadar.test.tsx` | Tests for radar chart component |
| `src/features/analysis/components/__tests__/ConceptDrawer.test.tsx` | Tests for concept side panel |

### Modified files

| File | Changes |
|------|---------|
| `src/i18n/locales/en/analysis.json` | Add `charts.radar.*`, `detail.conceptPanel.*`, `detail.filteredBy`, `detail.clearFilter` keys |
| `src/i18n/locales/nb/analysis.json` | Add same keys with Norwegian translations |
| `src/features/analysis/hooks/__tests__/useChartData.test.ts` | Add `describe("radarData", ...)` block with 5 tests |
| `src/features/analysis/components/__tests__/CoverageHeatmap.test.tsx` | Add `describe("cross-filter", ...)` block with 6 tests |
| `src/features/analysis/components/__tests__/FindingsTable.test.tsx` | Add `describe("concept click", ...)` block with 4 tests |
| `src/features/analysis/components/__tests__/SummaryStats.test.tsx` | Add `describe("overrideTypeCounts", ...)` block with 3 tests, update `makeChartData` to include `radarData: []` |
| `src/i18n/__tests__/analysis-namespace.test.ts` | Add `describe("enhancement keys", ...)` block validating all new keys in en and nb |

## Implementation Checklist

1. Add all new i18n keys to `en/analysis.json` (radar, conceptPanel, cross-filter)
2. Add all new i18n keys to `nb/analysis.json` with Norwegian translations
3. Create `frameworkColors.test.ts` with 5 tests
4. Extend `useChartData.test.ts` with radarData tests (5 tests)
5. Extend `CoverageHeatmap.test.tsx` with cross-filter tests (6 tests)
6. Create `FrameworkRadar.test.tsx` with 9 tests
7. Create `ConceptDrawer.test.tsx` with 8 tests
8. Extend `FindingsTable.test.tsx` with concept click tests (4 tests)
9. Extend `SummaryStats.test.tsx` with overrideTypeCounts tests (3 tests, update makeChartData helper)
10. Extend `analysis-namespace.test.ts` with enhancement key validation tests
11. Run `pnpm test` to verify all tests pass

---

## Implementation Notes

### Actual files modified
- `frontend/src/i18n/locales/en/analysis.json` — Added radar, conceptPanel, and cross-filter keys
- `frontend/src/i18n/locales/nb/analysis.json` — Added Norwegian Bokmal equivalents
- `frontend/src/i18n/__tests__/analysis-namespace.test.ts` — Added 22 enhancement key validation tests

### Deviations from plan
- Test files 1-7 (frameworkColors, useChartData radarData, CoverageHeatmap cross-filter, FrameworkRadar, ConceptDrawer, FindingsTable concept click, SummaryStats override) were already implemented during their respective sections (01-05). This section only needed the i18n keys and validation tests.
- i18n validation uses a single `it.each(allNewKeys)` block instead of separate describe blocks per category.

### Test count
- 22 new i18n validation tests in this section
- All 235 tests passing across the full suite