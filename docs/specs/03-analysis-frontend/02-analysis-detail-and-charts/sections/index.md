<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-prerequisites-and-i18n
section-02-chart-data-and-stats
section-03-d3-charts
section-04-findings-table
section-05-export-and-empty-state
section-06-page-assembly
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-prerequisites-and-i18n | - | all | Yes |
| section-02-chart-data-and-stats | 01 | 03, 06 | No |
| section-03-d3-charts | 01, 02 | 06 | Yes |
| section-04-findings-table | 01 | 06 | Yes |
| section-05-export-and-empty-state | 01 | 06 | Yes |
| section-06-page-assembly | 01-05 | - | No |

## Execution Order

1. section-01-prerequisites-and-i18n (no dependencies)
2. section-02-chart-data-and-stats (after 01)
3. section-03-d3-charts, section-04-findings-table, section-05-export-and-empty-state (parallel after 02)
4. section-06-page-assembly (after all above)

## Section Summaries

### section-01-prerequisites-and-i18n
Fix PaginatedResponse field name (data → items), fix AnalysisFinding nullability, add all i18n keys for en/nb, update route shell with loading/error/processing states.

### section-02-chart-data-and-stats
Create useChartData hook for aggregating findings into framework coverage, priority counts, and type counts. Create SummaryStats component with 6 stat cards.

### section-03-d3-charts
Create useContainerDimensions hook for responsive sizing. Create CoverageHeatmap (horizontal bar chart with color gradient) and PriorityChart (vertical bar chart with P1-P4 colors) as D3 wrapper components.

### section-04-findings-table
Create FindingTypeTag badge component. Create FindingsTable with filter dropdowns, expandable rows (chevron toggle), pagination controls, and null-field fallbacks.

### section-05-export-and-empty-state
Create ExportButtons with disabled-with-tooltip pattern and loading states. Create EmptyFindings component with guidance and settings link.

### section-06-page-assembly
Wire all components in $id.tsx route. Manage filter/pagination/expansion state. Implement conditional rendering for all analysis states. Update barrel exports.
