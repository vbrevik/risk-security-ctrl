# Combined Specification: Framework Explorer Insight Pages

## Project Context

The risk-security-ctrl application is a risk management framework explorer for governmental IT security. It contains 22 regulatory/security frameworks (1,334 concepts, 382 cross-framework relationships) covering domains from ISO risk management to EU AI Act to NIST cybersecurity. The existing frontend has a home page, ontology graph/tree explorer, compliance tracker, and reports page.

**Goal:** Build 4 new frontend pages that unlock the data's analytical potential — giving users framework profiles, cross-framework relationship visualization, regulatory applicability guidance, and unified search.

## Technical Stack

- **Frontend:** Vite 7.2 + React 19.2 + TanStack Router 1.158 (file-based) + TanStack Query 5.90 + Tailwind CSS 4.1
- **UI:** shadcn/ui (Button, Card, Dialog, Input, Label, Select, Badge) + lucide-react icons
- **Design System:** "Technical Cartography" — IBM Plex Mono/Sans, oklch colors, amber on slate, topo-grid backgrounds
- **Visualization:** D3 7.9 (already in project for graph), custom SVG for heatmap
- **API:** REST at `/api/ontology/` (frameworks, concepts, relationships, search)
- **Testing:** Vitest 4.0 available, no existing test files

## Data Available

- 22 frameworks across 4 domains: Risk & Security (7), AI Governance (7), EU Regulations (4), Architecture & Models (4)
- 1,334 concepts with 32 distinct types (control, requirement, action, technique, etc.)
- 382 cross-framework relationships (140 maps_to, 118 related_to, 101 implements, 23 supports)
- Framework connectivity ranges from ISO 31000 (connected to 20 others) to ISO 31010 (connected to 2)
- Concepts per framework: 13 (data-centric) to 344 (NIST SP 800-53)

## Design Decisions (from Interview)

1. **Framework Catalog:** Master-detail pattern — sidebar catalog with detail panel (like ontology explorer)
2. **Crosswalk Matrix:** Ordered by domain cluster to show within-domain density
3. **Regulatory Landscape:** URL state for shareable filter selections
4. **Unified Search:** Results link to both ontology explorer (primary) and framework detail (secondary)
5. **Navigation:** Two-tier — Primary: Home, Ontology, Compliance, Reports. Secondary: Frameworks, Crosswalk, Landscape, Search
6. **Delivery:** All 4 pages shipped together as a single release

## Page Specifications

### Page 1: Framework Catalog (`/frameworks` + `/frameworks/$id`)

**Layout:** Master-detail. Left sidebar shows all frameworks grouped by domain as compact cards. Main area shows the selected framework's full profile.

**Sidebar (always visible):**
- 4 domain sections (Risk & Security, AI Governance, EU Regulations, Architecture & Models)
- Each framework card: color dot, short name, concept count
- Click to select, highlight active

**Detail Panel:**
- Framework header: full name, version, description, source URL link
- Stats row: concept count, concept types breakdown, connected frameworks count, relationship count
- Concept type breakdown: horizontal bar chart (controls, requirements, actions, etc.)
- Cross-framework connections: list of connected frameworks with relationship counts, clickable to navigate to crosswalk
- Top concepts: first-level hierarchy showing the framework's structure

**Route:** `routes/frameworks/index.tsx` with `validateSearch: { id?: string }` for selected framework

### Page 2: Crosswalk Explorer (`/crosswalk`)

**Layout:** Full-page matrix with drill-down panel.

**Matrix:**
- 22x22 SVG heatmap, frameworks grouped by domain cluster
- Domain group headers visible on axes
- Cell color: empty (no fill), 1-2 (light), 3-5 (medium), 6+ (strong accent)
- Hover: highlight row + column, show tooltip with framework pair name and count
- Click: open drill-down panel

**Drill-down Panel (slides from right):**
- Shows all relationships between selected framework pair
- Each relationship: source concept → [type] → target concept
- Filter by relationship type (maps_to, implements, related_to, supports)
- Links to navigate to either concept in ontology explorer

**Controls:**
- Relationship type filter (checkbox: maps_to, implements, related_to, supports)
- Summary stats: total relationships, most connected pair, type distribution

**Implementation:** Custom SVG with D3 scales (scaleBand for axes, scaleSequential for color). React renders `<rect>` elements. useMemo for scale calculations.

**Route:** `routes/crosswalk/index.tsx` with `validateSearch: { fw1?: string; fw2?: string; type?: string }`

### Page 3: Regulatory Landscape (`/landscape`)

**Layout:** Left panel with selectors, main area with resulting framework map.

**Selectors:**
- Sector (single-select): Financial, Healthcare, Critical Infrastructure, Government/Public Admin, Technology/AI Provider, General Enterprise
- Activities (multi-select): Processing personal data, Deploying AI systems, Operating critical infrastructure, Financial services, Defense/NATO context

**Main Area:**
- Applicable frameworks highlighted with full color and detail
- Non-applicable frameworks shown faded/grayed
- "Compliance stack" view: ordered list of applicable frameworks with overlap indicators
- Summary: "Based on your profile, N frameworks apply with M shared requirements"

**Applicability Logic (hardcoded mapping):**
- Financial + AI → DORA, EU AI Act, GDPR, ISO 42001, NIS2, ISO 27000
- Healthcare + AI → EU AI Act, GDPR, ISO 42001, NIS2
- Critical Infrastructure → NIS2, CER, ISO 27000, NIST CSF
- Government + Processing personal data → GDPR, NIS2, ISO 27000
- Defense/NATO → FMN, ISO 27000, Zero Trust, CISA ZTMM

**Route:** `routes/landscape/index.tsx` with `validateSearch: { sector?: string; activities?: string }`

### Page 4: Unified Concept Search (`/concepts/search`)

**Layout:** Search input top, filter sidebar left, results main area.

**Search Input:**
- Full-width, auto-focused, debounced (300ms)
- Uses existing `GET /api/ontology/search?q=X` endpoint
- Show result count and active filter pills

**Filter Sidebar:**
- Framework filter: checkbox list with per-framework result counts, colored dots
- Concept type filter: checkbox list with counts (control, requirement, action, etc.)
- Collapsible sections

**Results:**
- Grouped by framework (framework header → concept cards)
- Each result: code (if exists), name, concept type pill, definition excerpt, parent hierarchy breadcrumb
- Primary action: navigate to `/ontology?concept=ID` (opens ontology explorer with concept selected)
- Secondary link: navigate to `/frameworks?id=FRAMEWORK_ID` (opens framework detail)
- Keyboard: arrow keys navigate results, Enter opens primary action

**Route:** `routes/concepts/search.tsx` with `validateSearch: { q?: string; frameworks?: string; types?: string }`

## Navigation Changes

**Two-tier navigation in `__root.tsx`:**
- Primary bar (existing): Home, Ontology, Compliance, Reports
- Secondary bar (new): Frameworks, Crosswalk, Landscape, Search

Both bars use the same monospace font styling. Secondary bar slightly smaller/lighter.

## Non-Requirements
- No authentication
- No data mutation (all pages are read-only)
- No new backend endpoints
- No new npm dependencies (use existing D3, shadcn/ui, TanStack Query)
- No i18n for new pages initially (can be added later)
