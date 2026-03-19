# Usage Guide: Analysis Frontend Enhancements

## Quick Start

The analysis detail page (`/analysis/:id`) now includes enhanced visualization and interaction features. No additional setup is needed — all features are wired into the existing page.

## Features

### 1. Framework Radar Chart
A D3-based radar/spider chart showing normalized finding-type distribution across frameworks.

- **Location:** 3-column chart grid on the analysis detail page
- **Component:** `FrameworkRadar` in `frontend/src/features/analysis/components/FrameworkRadar.tsx`
- **Data:** `chartData.radarData` from the `useChartData` hook
- **Limits:** Displays up to 8 frameworks (sorted by total findings)

### 2. Cross-Filter (Heatmap → Table)
Click any bar in the CoverageHeatmap to filter the findings table by that framework.

- **Toggle:** Click the same bar again to clear the filter
- **Filter banner:** Shows which framework is active with a clear button
- **SummaryStats:** Automatically updates to show filtered type counts
- **Scroll:** Findings table scrolls into view on filter activation

### 3. Concept Drawer
Click any concept name or code in the FindingsTable to open a side panel with full ontology details.

- **Component:** `ConceptDrawer` in `frontend/src/features/analysis/components/ConceptDrawer.tsx`
- **Shows:** Name, code, type, framework, definition, related concepts, cross-framework mappings
- **Link:** "Open in Ontology Explorer" navigates to the full ontology view

### 4. Framework Colors
Deterministic color mapping for framework visualizations.

- **Utility:** `getFrameworkColor(frameworkId, frameworkIds)` in `frontend/src/features/analysis/utils/frameworkColors.ts`
- **Palette:** 10 distinct colors, assigned by sorted index (mod 10)

## API Reference

### Components

| Component | Props | Description |
|-----------|-------|-------------|
| `FrameworkRadar` | `data`, `selectedFrameworkId?`, `frameworkIds` | D3 radar chart |
| `ConceptDrawer` | `conceptId`, `onClose` | Side panel for concept details |
| `SummaryStats` | `analysis`, `chartData`, `isLoading?`, `overrideTypeCounts?` | Stats cards with optional override |
| `CoverageHeatmap` | `data`, `onBarClick?`, `selectedFrameworkId?`, `frameworkIds?` | Interactive bar chart |
| `FindingsTable` | `...existing`, `onConceptClick?` | Table with clickable concept cells |

### Utilities

| Function | Signature | Description |
|----------|-----------|-------------|
| `getFrameworkColor` | `(frameworkId: string, frameworkIds: string[]) => string` | Deterministic hex color |

### Hooks

| Hook | Return | Changes |
|------|--------|---------|
| `useChartData` | `ChartData` | Added `radarData` field |

## i18n Keys Added

- `charts.radar.*` (9 keys) — Radar chart labels
- `detail.conceptPanel.*` (11 keys) — Concept drawer UI
- `detail.filteredBy` / `detail.clearFilter` — Cross-filter banner

All keys available in English (`en`) and Norwegian Bokmal (`nb`).

## Test Coverage

235 total tests passing across the analysis feature:
- 6 frameworkColors utility tests
- 12 useChartData tests (including 5 radarData)
- 12 CoverageHeatmap tests (including 6 cross-filter)
- 9 FrameworkRadar tests
- 8 ConceptDrawer tests
- 15 FindingsTable tests (including 4 concept click)
- 11 SummaryStats tests (including 3 overrideTypeCounts)
- 25 i18n validation tests (including 22 enhancement keys)
