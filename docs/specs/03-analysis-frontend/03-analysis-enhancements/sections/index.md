<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-colors-and-chart-data
section-02-heatmap-cross-filter
section-03-framework-radar
section-04-concept-drawer
section-05-page-assembly
section-06-i18n-and-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-colors-and-chart-data | - | 02, 03, 05 | Yes |
| section-02-heatmap-cross-filter | 01 | 05 | Yes |
| section-03-framework-radar | 01 | 05 | Yes |
| section-04-concept-drawer | - | 05 | Yes |
| section-05-page-assembly | 01, 02, 03, 04 | 06 | No |
| section-06-i18n-and-tests | 01, 02, 03, 04, 05 | - | No |

## Execution Order

1. section-01-colors-and-chart-data (no dependencies)
2. section-02-heatmap-cross-filter, section-03-framework-radar, section-04-concept-drawer (parallel after 01; section-04 has no dependency on 01 either)
3. section-05-page-assembly (after 01, 02, 03, 04)
4. section-06-i18n-and-tests (final)

## Section Summaries

### section-01-colors-and-chart-data
Shared framework color utility (`frameworkColors.ts`) and `useChartData` hook extension to add `radarData` with normalized percentages.

### section-02-heatmap-cross-filter
Add `onBarClick`, `selectedFrameworkId`, `frameworkIds` props to CoverageHeatmap. Click handlers on bars, selected state styling, keyboard accessibility (`tabindex`, `role="button"`, `aria-label`, keydown handler).

### section-03-framework-radar
New `FrameworkRadar` D3 component: 4-axis radar/spider chart with polygon overlays per framework, grid rings, axis labels, tooltip, legend, selected state highlighting. Follows pure-D3-in-useEffect pattern.

### section-04-concept-drawer
New `ConceptDrawer` component using shadcn Sheet. Fetches concept data via `useConceptRelationships`. Shows name, definition, type, framework, related concepts, cross-mappings. Loading/error states. Add `onConceptClick` prop to FindingsTable.

### section-05-page-assembly
Wire everything in `$id.tsx`: cross-filter state derived from filters, `handleBarClick` with functional updater, `SummaryStats` with `overrideTypeCounts`, 3-column chart grid, filter banner, concept drawer state, scroll-to-table ref.

### section-06-i18n-and-tests
Add all new i18n keys (radar, concept panel, cross-filter) to en/nb. Comprehensive test suite: frameworkColors, useChartData radar, CoverageHeatmap click, FrameworkRadar rendering, ConceptDrawer states, FindingsTable concept click, SummaryStats override, i18n validation.
