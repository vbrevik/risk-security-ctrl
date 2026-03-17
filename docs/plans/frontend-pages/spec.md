# Frontend Pages Spec: Framework Explorer Insight Pages

## Overview

Build 4 new frontend pages for the risk-security-ctrl ontology explorer that provide deeper insight into the 22 regulatory frameworks, 1,334 concepts, and 382 cross-framework relationships stored in the system.

## Stack & Design System

- **Frontend:** Vite + React + TanStack Router (file-based routing) + TanStack Query + shadcn/ui + Tailwind CSS
- **Design System:** "Technical Cartography" — IBM Plex Mono/Sans fonts, amber accents on deep slate, topo-grid backgrounds, tech-badges, feature-cards with corner-markers, connection-line SVG animations
- **Backend API:** Rust + Axum serving REST at `/api/ontology/` — frameworks, concepts (paginated), relationships, search
- **Data:** 22 frameworks across 4 domains (Risk & Security Standards, AI Governance, EU Regulations, Architecture & Models), 1,334 concepts with 32 distinct concept types, 382 cross-framework relationships (maps_to, implements, related_to, supports)

## Page 1: Framework Catalog (`/frameworks`)

### Purpose
Give each framework a dedicated profile showing its structure, scope, and connections at a glance.

### Requirements
- Grid of framework cards, grouped by domain (Risk & Security, AI Governance, EU Regulations, Architecture & Models)
- Each card shows: framework name, version, concept count by type, source URL link, number of connected frameworks
- Click a card to expand into a full profile showing:
  - Description and source link
  - Concept hierarchy (treemap or sunburst visualization)
  - Concept type breakdown (bar chart)
  - Cross-framework relationships grouped by target framework
  - "Related frameworks" section ranked by relationship density
- All data from API: GET /api/ontology/frameworks, GET /api/ontology/concepts?framework_id=X, GET /api/ontology/relationships

### Design Direction
- Industrial/utilitarian feel — dense information, monospace type for data
- Framework cards should feel like technical specification sheets
- Profile view should feel like a dossier — structured, scannable, authoritative

## Page 2: Crosswalk Explorer (`/crosswalk`)

### Purpose
Visualize the 382 cross-framework relationships as an interactive matrix, allowing practitioners to understand how frameworks relate to each other.

### Requirements
- 22x22 matrix heatmap showing relationship density between every framework pair
- Color intensity represents number of relationships (0 = empty, 1-2 = light, 3-5 = medium, 6+ = strong)
- Click a cell to drill into specific mappings — show source concept, relationship type, target concept
- Filter by relationship type (maps_to, implements, related_to, supports)
- Row/column headers show framework names with color indicators
- Summary statistics: total relationships, most connected pair, relationship type distribution
- Framework grouping option (group by domain to see cluster patterns)

### Design Direction
- Data visualization-forward — the matrix IS the page
- Monochrome base with accent color for intensity
- Drill-down panel slides in from right (similar to existing ContextPanel pattern)
- Dense, analytical feel — like a correlation matrix in a research paper

## Page 3: Regulatory Landscape (`/landscape`)

### Purpose
Help users understand which frameworks apply to their organizational context and see framework overlap.

### Requirements
- Sector selector: Financial, Healthcare, Critical Infrastructure, Government/Public Admin, Technology/AI Provider, General Enterprise
- Activity selector (multi-select): Processing personal data, Deploying AI systems, Operating critical infrastructure, Financial services, Defense/NATO context
- Based on selections, highlight applicable frameworks and dim non-applicable ones
- Show framework applicability logic (e.g., "Financial + AI = DORA + EU AI Act + GDPR + ISO 42001")
- Visualize overlap between applicable frameworks — which concepts/requirements are shared
- "Compliance stack" view — ordered list of applicable frameworks with priority/hierarchy

### Design Direction
- Interactive, filter-driven — the user builds their profile and the page responds
- Left panel for selectors, main area for the resulting framework landscape
- Applicable frameworks shown as a connected graph or layered diagram
- Non-applicable frameworks fade to gray but remain visible for context

## Page 4: Unified Concept Search (`/concepts/search`)

### Purpose
Full-text search across all 1,334 concepts with faceted filtering, showing results grouped by framework.

### Requirements
- Search input with instant results (debounced)
- Results grouped by framework with framework color indicators
- Each result shows: concept name, code, type, framework, definition excerpt, parent hierarchy breadcrumb
- Faceted filtering:
  - By framework (checkbox list with counts)
  - By concept type (control, requirement, action, technique, etc. with counts)
  - By relationship type (show concepts that have specific relationship types)
- Click a result to navigate to the ontology explorer with that concept selected
- Show result count and search time
- Empty state with suggested searches

### Design Direction
- Clean, functional search interface — Google/Algolia-inspired
- Results should be scannable — framework badges, type pills, hierarchy breadcrumbs
- Filters in a left sidebar, results in main area
- Keyboard-navigable (up/down arrows, enter to select)

## API Endpoints Available

```
GET /api/ontology/frameworks                          → Framework[]
GET /api/ontology/frameworks/:id                      → Framework
GET /api/ontology/concepts?framework_id=X&limit=500   → PaginatedResponse<Concept>
GET /api/ontology/concepts/:id                        → Concept
GET /api/ontology/concepts/:id/relationships          → ConceptWithRelationships
GET /api/ontology/relationships                       → Relationship[]
GET /api/ontology/search?q=X&framework_id=Y           → Concept[]
```

## Existing Patterns to Follow

- File-based routing: `src/routes/frameworks/index.tsx`, `src/routes/crosswalk/index.tsx`, etc.
- API hooks in `src/features/ontology/api/index.ts` using TanStack Query
- Types in `src/features/ontology/types/index.ts`
- Graph utilities in `src/features/ontology/utils/graphTransform.ts` (framework colors, etc.)
- Explorer context in `src/features/ontology/context/ExplorerContext.tsx`
- shadcn/ui components in `src/components/ui/`
- i18n with `useTranslation()` — namespaces per feature
- CSS classes: tech-badge, feature-card, corner-markers, stat-number, topo-grid, gradient-mesh

## Non-Requirements
- No authentication needed
- No data mutation (read-only pages)
- No new backend endpoints (use existing API)
- No new npm dependencies
