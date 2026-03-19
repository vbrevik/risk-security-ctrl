# TDD Plan: Analysis Detail Page Enhancements

Testing framework: Vitest + React Testing Library. Mocking: `vi.mock()` for modules, `vi.fn()` for callbacks. D3 chart tests inspect SVG DOM via `container.querySelectorAll()`. All components mock `react-i18next` and `useContainerDimensions`.

---

## Section 1: Shared Framework Color Utility and useChartData Extension

### frameworkColors utility tests (`utils/__tests__/frameworkColors.test.ts`)

- Test: returns a hex color string for a known framework ID
- Test: same framework ID always gets same color given same frameworkIds array
- Test: different frameworks get different colors (up to 10)
- Test: wraps around after 10 frameworks (mod 10 behavior)
- Test: order is deterministic (sorts IDs alphabetically before indexing)

### useChartData radarData tests (extend `hooks/__tests__/useChartData.test.ts`)

- Test: radarData groups findings by framework with normalized percentages
- Test: radarData percentages sum to 100 per framework
- Test: radarData returns empty array when no findings
- Test: radarData handles single framework with all one type (100% for that type, 0% for others)
- Test: radarData includes total raw count per framework

---

## Section 2: CoverageHeatmap Cross-Filter Support

### CoverageHeatmap tests (extend `components/__tests__/CoverageHeatmap.test.tsx`)

- Test: onBarClick callback fires with correct frameworkId when a bar rect is clicked
- Test: no error when onBarClick is not provided (backward compatible)
- Test: when selectedFrameworkId is set, SVG contains bars with reduced opacity for non-selected
- Test: when selectedFrameworkId is null, all bars have full opacity
- Test: bar rects have role="button" and tabindex="0" for keyboard accessibility
- Test: bar rects have aria-label containing the framework ID

---

## Section 3: Framework Radar Chart Component

### FrameworkRadar tests (`components/__tests__/FrameworkRadar.test.tsx`)

Mock `useContainerDimensions` to return `{ width: 400, height: 400 }`. Mock `react-i18next`.

- Test: renders SVG element with role="img" and aria-labelledby
- Test: renders one path element per framework in data
- Test: renders no paths when data is empty
- Test: shows noData message when data array is empty
- Test: renders 4 axis labels (one per finding type)
- Test: renders concentric grid circles
- Test: legend section shows framework names matching data
- Test: limits rendering to 8 frameworks when more provided
- Test: when selectedFrameworkId is set, selected path has different styling

---

## Section 4: Concept Side Panel (ConceptDrawer)

### ConceptDrawer tests (`components/__tests__/ConceptDrawer.test.tsx`)

Mock `useConceptRelationships` from ontology API. Mock shadcn Sheet component.

- Test: renders nothing / closed state when conceptId is null
- Test: renders Sheet in open state when conceptId is non-null
- Test: displays concept name, code, type, framework from fetched data
- Test: displays concept definition
- Test: shows loading skeleton when data is loading
- Test: shows error state with retry button when fetch fails
- Test: "Open in Ontology Explorer" link has href `/ontology?concept={id}` and target="_blank"
- Test: close button calls onClose callback

### FindingsTable concept click tests (extend `components/__tests__/FindingsTable.test.tsx`)

- Test: concept name cell is clickable (has button role)
- Test: clicking concept name fires onConceptClick with finding's concept_id
- Test: concept code cell shows "—" when concept_code is null (not clickable)
- Test: no error when onConceptClick is not provided

---

## Section 5: Detail Page Assembly and Cross-Filter Wiring

### SummaryStats tests (extend `components/__tests__/SummaryStats.test.tsx`)

- Test: when overrideTypeCounts is provided, finding-type cards show overridden values
- Test: when overrideTypeCounts is provided, framework count / processing time / token count cards remain unchanged
- Test: when overrideTypeCounts is not provided, behaves as before (backward compatible)

### Detail page integration (these are higher-level tests, may be skipped if too complex to unit test)

- Test: clicking heatmap bar updates filter state (verify FindingsTable receives new framework_id filter)
- Test: clicking already-selected bar clears the filter
- Test: filter banner shows framework name when filtered
- Test: filter banner clear button resets framework_id filter

---

## Section 6: i18n and Tests

### i18n validation tests (extend or add to `i18n/__tests__/analysis-namespace.test.ts`)

- Test: all new radar chart keys exist in both en and nb translation files
- Test: all new concept panel keys exist in both en and nb translation files
- Test: all new cross-filter keys exist in both en and nb translation files
- Test: no new keys are empty strings
