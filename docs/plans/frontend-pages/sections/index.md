<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-shared-infra
section-02-navigation
section-03-frameworks-catalog
section-04-crosswalk-explorer
section-05-unified-search
section-06-regulatory-landscape
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-shared-infra | - | 02, 03, 04, 05, 06 | Yes |
| section-02-navigation | 01 | 03, 04, 05, 06 | No |
| section-03-frameworks-catalog | 01, 02 | - | Yes |
| section-04-crosswalk-explorer | 01, 02 | - | Yes |
| section-05-unified-search | 01, 02 | - | Yes |
| section-06-regulatory-landscape | 01, 02 | - | Yes |

## Execution Order

1. section-01-shared-infra (no dependencies — types, hooks, utilities)
2. section-02-navigation (after 01 — two-tier nav in __root.tsx)
3. section-03-frameworks-catalog, section-04-crosswalk-explorer, section-05-unified-search, section-06-regulatory-landscape (parallel after 02 — all 4 pages are independent)

## Section Summaries

### section-01-shared-infra
New TypeScript types (FrameworkStats, CrosswalkCell, LandscapeProfile). New API hooks (useAllConcepts with error handling, useFrameworkStats). Utility functions (frameworkDomains.ts for domain grouping, landscapeMapping.ts for sector/activity→framework mapping). URL param parsing utilities. Test setup (install @testing-library/react, @testing-library/jest-dom). Unit tests for all pure functions.

### section-02-navigation
Refactor __root.tsx from single nav bar to two-tier navigation. Remove existing placeholder Frameworks/Crosswalk links from primary bar. Add secondary nav bar with Frameworks, Crosswalk, Landscape, Search links. Responsive styling. Create route files for all 4 pages (empty placeholder components) and concepts/index.tsx redirect.

### section-03-frameworks-catalog
Framework Catalog page at /frameworks with master-detail layout. FrameworkSidebar component (domain-grouped framework list). FrameworkProfile component (stats, concept type breakdown bar, cross-framework connections, concept hierarchy preview). URL state with ?id param. Loading skeletons and error states. Component tests.

### section-04-crosswalk-explorer
Crosswalk Explorer page at /crosswalk. CrosswalkMatrix SVG component using D3 scales (scaleBand, scaleSequential). 22x22 heatmap with domain cluster ordering. Hover row/column highlighting. Cell click drill-down. CrosswalkDrilldown panel with relationship list and type filters. Accessibility: role="grid", keyboard nav, table-view toggle. URL state with ?fw1, ?fw2, ?type params. Loading skeleton. Component tests.

### section-05-unified-search
Unified Search page at /concepts/search. Full-width debounced search input (300ms). SearchFilters sidebar with framework and concept type checkboxes with counts. SearchResults grouped by framework with concept cards showing code, name, type pill, definition excerpt, hierarchy breadcrumb. Dual navigation links (ontology explorer + framework detail). Keyboard navigation. Empty and no-results states. URL state with ?q, ?frameworks, ?types params. Component tests.

### section-06-regulatory-landscape
Regulatory Landscape page at /landscape. LandscapeSelector with sector radio buttons and activity checkboxes. LandscapeResults showing applicable frameworks highlighted, non-applicable faded. Compliance stack view with overlap indicators. Summary banner. URL state with ?sector, ?activities params. Component tests.
