# Implementation Plan: Framework Explorer Insight Pages

## 1. Project Overview

### What We're Building

Four new frontend pages for a risk management framework explorer that contains 22 regulatory/security frameworks with 1,334 concepts and 382 cross-framework relationships. The pages provide analytical depth beyond the existing ontology graph/tree explorer:

1. **Framework Catalog** (`/frameworks`) — Master-detail view with a sidebar listing all frameworks by domain and a detail panel showing framework profiles
2. **Crosswalk Explorer** (`/crosswalk`) — 22x22 SVG heatmap matrix showing relationship density between every framework pair, with drill-down to individual mappings
3. **Regulatory Landscape** (`/landscape`) — Interactive sector/activity selector that highlights applicable frameworks for an organization's regulatory profile
4. **Unified Search** (`/concepts/search`) — Full-text search across all concepts with faceted filtering by framework and concept type

### Why These Pages

The existing ontology explorer is concept-focused (select a concept → see its details and relationships). These pages are framework-focused and relationship-focused — answering questions like "What does DORA cover?", "How do GDPR and EU AI Act overlap?", "Which frameworks apply to a financial institution deploying AI?", and "Where is 'incident reporting' addressed across all frameworks?"

### Technical Context

- **Stack:** Vite 7.2 + React 19.2 + TanStack Router 1.158 (file-based routing) + TanStack Query 5.90 + Tailwind CSS 4.1
- **UI Library:** shadcn/ui (Button, Card, Dialog, Input, Select, Badge) + lucide-react icons
- **Design System:** "Technical Cartography" — IBM Plex Mono/Sans fonts, oklch colors (deep slate primary, amber accents), topo-grid backgrounds, tech-badges, feature-cards with corner-markers
- **Visualization:** D3 7.9 already in project (used for force-directed graph in ontology explorer)
- **API:** Rust/Axum backend at `/api/ontology/` — no new endpoints needed
- **Testing:** Vitest 4.0 available, no existing test files

---

## 2. Navigation Architecture

### Two-Tier Navigation Bar

The existing single navigation bar has 5 items (Home, Ontology, Compliance, Reports, language toggle). Adding 4 more would create crowding. Solution: a two-tier navigation.

**Primary bar (existing, unchanged):** Home, Ontology, Compliance, Reports — these are the operational tools.

**Secondary bar (new):** Frameworks, Crosswalk, Landscape, Search — these are the analytical/research tools.

**Current state:** `__root.tsx` already has placeholder Frameworks and Crosswalk links in the primary bar from earlier work. These must be removed and replaced with the two-tier approach.

Implementation in `__root.tsx`:
- Remove the existing Frameworks/Crosswalk links from the primary nav
- Add a secondary `<nav>` element below the primary bar inside the same `<header>` element
- Secondary bar shares the same border and blur styling but uses smaller text (`text-xs` vs `text-sm`)
- Uses `text-foreground/40` for inactive, `text-foreground` for active state
- Secondary items: Frameworks, Crosswalk, Landscape, Search (hardcoded English — i18n deferred per spec)
- Collapses to a horizontal scroll on mobile

### Route Structure

```
src/routes/
  frameworks/
    index.tsx          # Framework Catalog (master-detail)
  crosswalk/
    index.tsx          # Crosswalk Matrix Explorer
  landscape/
    index.tsx          # Regulatory Landscape
  concepts/
    index.tsx          # Redirect to /concepts/search
    search.tsx         # Unified Search
```

Note: `concepts/index.tsx` contains a redirect to `/concepts/search` to avoid a 404 at `/concepts`.

All routes use `createFileRoute()` with `validateSearch` for URL state management. The TanStack Router Vite plugin auto-generates the route tree.

---

## 3. Shared Infrastructure

### New API Hooks

No new backend endpoints are needed. However, several pages need to combine data in ways the existing hooks don't support. Add these derived hooks to `src/features/ontology/api/index.ts`:

```typescript
function useAllConcepts(): { data: Concept[]; isLoading: boolean; errors: Error[] }
```
Fetches concepts for all frameworks in parallel using `useQueries` (limit=500 per request), returns the combined flat array. Used by Search and Crosswalk pages. Caches per-framework, so subsequent framework loads are instant.

**Error handling for parallel requests:** Since this fires 22 HTTP requests:
- Track which queries failed via `useQueries` result `.error` fields
- Return partial results from successful queries immediately (don't block on failures)
- Expose `errors` array so pages can show "X frameworks failed to load, showing partial data"
- Failed queries auto-retry per TanStack Query defaults (3 retries with exponential backoff)
- Pages show loading skeleton until all queries settle (success or final failure)

**Concept-to-framework lookup:** This hook also provides the mapping needed by the Crosswalk matrix. Build a `Map<conceptId, frameworkId>` from the combined results in useMemo. The Crosswalk page depends on this to resolve relationship concept IDs to framework pairs.

```typescript
function useFrameworkStats(): { data: Map<string, FrameworkStats>; isLoading: boolean }
```
Derives per-framework statistics (concept count, concept type breakdown, connection count) from `useFrameworks()`, `useAllConcepts()`, and `useRelationships()`. Returns a `Map<frameworkId, stats>`. Used by Framework Catalog and home page.

### New Types

Add to `src/features/ontology/types/index.ts`:

```typescript
interface FrameworkStats {
  conceptCount: number;
  conceptTypes: Record<string, number>;
  connectedFrameworks: number;
  relationshipCount: number;
}

interface CrosswalkCell {
  sourceFrameworkId: string;
  targetFrameworkId: string;
  count: number;
  relationships: Relationship[];
}

interface LandscapeProfile {
  sector: string;
  activities: string[];
  applicableFrameworks: string[];
}
```

### Framework Domain Grouping

A utility function that groups framework IDs into domains. Used across multiple pages (home, catalog, crosswalk, landscape).

```typescript
function groupFrameworksByDomain(frameworks: Framework[]): { label: string; frameworkIds: string[] }[]
```

Hardcoded mapping (same as the home page):
- Risk & Security Standards: iso31000, iso31010, iso27000, iso9000, nist-csf, nist-800-53, nist-rmf
- AI Governance: eu-ai-act, nist-ai-rmf, iso42001, iso42005, iso23894, google-saif, mitre-atlas
- EU Regulations: gdpr, nis2, dora, cer-directive
- Architecture & Models: zero-trust, cisa-ztmm, data-centric, fmn

---

## 4. Page 1: Framework Catalog

### Layout

Master-detail pattern matching the ontology explorer: fixed-width sidebar on left (280px), detail panel fills remaining width.

### Sidebar Component

Renders all frameworks grouped by 4 domain sections. Each domain section has:
- Domain label (uppercase monospace, small) with a horizontal rule and framework count
- Framework items below: colored dot + short name + concept count badge

The framework list is always visible (no collapsing). The active framework is highlighted with accent background. Click any framework to select it and populate the detail panel.

**Default/loading states:**
- While `useFrameworks()` is loading: sidebar shows 4 skeleton domain sections with placeholder items; detail panel shows empty state "Select a framework"
- When loaded with no `?id` param: auto-select first framework in list
- When loaded with `?id=foo` but `foo` not found: select first framework, show toast "Framework not found"
- Default state: first framework in the list is selected

### Detail Panel Component

When a framework is selected, the detail panel shows:

**Header section:**
- Framework name (large, bold, monospace)
- Version badge (tech-badge style)
- Description text
- Source URL as external link with icon

**Stats strip:**
- 4 stat boxes in a row: Concepts (total), Types (distinct concept types), Connected (frameworks), Relationships (total involving this framework)
- Uses existing stat-number CSS class

**Concept Type Breakdown:**
- Horizontal stacked bar showing the proportion of each concept type (controls, requirements, actions, etc.)
- Each type segment has its own color (derive from concept type name via hash) and label
- Shows count next to each type

**Cross-Framework Connections:**
- List of connected frameworks sorted by relationship count (descending)
- Each row: framework color dot, name, relationship count badge, relationship type breakdown (small colored pills for maps_to/implements/related_to/supports)
- Click a row to navigate to `/crosswalk?fw1=THIS&fw2=CLICKED`

**Concept Hierarchy Preview:**
- Show the top-level concepts (parent_id = null) as a flat list with expand arrows
- Clicking a top-level concept expands to show its children (one level)
- This gives a structural overview without the full tree

### Data Flow

1. `useFrameworks()` → framework list for sidebar
2. `useConcepts(selectedFrameworkId)` → concepts for the selected framework
3. `useRelationships()` → filter to relationships involving selected framework
4. Derived statistics computed in `useMemo`

### URL State

`validateSearch: { id?: string }` — selected framework ID persisted in URL for bookmarking.

---

## 5. Page 2: Crosswalk Explorer

### Layout

Full-width page. Matrix visualization fills the main area. Drill-down panel slides from right (320px) when a cell is clicked.

### Matrix Component

**Rendering:** Custom SVG, not a library. D3 provides scale calculations (`scaleBand` for axes, `scaleSequential` for color intensity); React renders `<rect>` elements for cells and `<text>` for labels.

**Axes:**
- Both axes show the same 22 frameworks, ordered by domain cluster
- Domain group separators: thicker lines or slight gaps between groups
- Framework labels rotated 45° on the x-axis (top), horizontal on the y-axis (left)
- Each label has its framework color dot

**Cells:**
- Size: computed from available viewport width divided by 22 (~30-40px per cell)
- Color encoding: empty cells are transparent, 1-2 relationships are light amber, 3-5 medium, 6+ strong accent
- Diagonal cells (framework × itself) are filled differently or dimmed (no self-relationships)
- Matrix is symmetric — both upper and lower triangle are rendered

**Interaction:**
- Hover: highlight entire row + column with subtle background, show tooltip with "GDPR × DORA: 4 relationships"
- Click: select the cell, open drill-down panel
- Active cell gets a border highlight

**Responsive:** On smaller screens (<768px), show a scrollable container with pinned labels. The matrix itself doesn't resize — it scrolls.

**Accessibility (WCAG):**
- SVG element: `role="grid"`, `aria-label="Cross-framework relationship matrix"`
- Each cell: `role="gridcell"`, `aria-label="GDPR and DORA: 4 relationships"`
- Keyboard navigation: Arrow keys move between cells, Enter/Space opens drill-down
- Focus indicator: visible border on focused cell
- Color: use discrete bins (0, 1-2, 3-5, 6+) not continuous ramp; test palette for deuteranopia/protanopia
- Never rely on color alone: show count number inside cells when zoomed or on hover
- Table alternative: toggle button in controls to switch from heatmap to an HTML `<table>` view showing the same data
- Respect `prefers-reduced-motion`: disable hover animations

### Drill-Down Panel Component

Appears on the right when a cell is clicked. Shows:

**Header:** "Framework A × Framework B" with both colored dots and relationship count

**Relationship List:** Each relationship rendered as:
- Source concept name (with framework color dot) → relationship type badge → target concept name (with framework color dot)
- Relationship type color-coded: maps_to (blue), implements (green), related_to (gray), supports (amber)
- Each concept name is a link (primary: ontology explorer, secondary: framework detail)

**Filters:** Checkboxes for relationship type at the top of the panel. Toggles filter the list below.

**Empty state:** "No relationships between these frameworks" with a note about what each framework covers.

### Data Flow

1. `useFrameworks()` → framework list and ordering
2. `useAllConcepts()` → all concepts (needed to build concept-to-framework lookup map)
3. `useRelationships()` → all 382 relationships
4. Build concept-to-framework `Map<conceptId, frameworkId>` from step 2 in `useMemo`
5. Build the 22x22 matrix in `useMemo`: iterate relationships, resolve each source/target concept to its framework via the lookup map, count per pair
6. On cell click, filter relationships to the selected pair

**Loading state:** Show a full-page skeleton (22x22 grid of placeholder cells with pulse animation) while waiting for all three data sources. The matrix cannot render until all queries complete since partial data would show an incomplete picture. Show a progress indicator: "Loading frameworks... (18/22)"

### URL State

`validateSearch: { fw1?: string; fw2?: string; type?: string }` — selected cell and filter persisted in URL.

---

## 6. Page 3: Regulatory Landscape

### Layout

Two-column: selector panel on left (320px, fixed), results area on right.

### Selector Panel

**Sector selector (single-select):**
- Radio buttons or styled select: Financial, Healthcare, Critical Infrastructure, Government/Public Admin, Technology/AI Provider, General Enterprise
- Each sector has a brief description tooltip

**Activity selector (multi-select):**
- Checkboxes: Processing personal data, Deploying AI systems, Operating critical infrastructure, Financial services, Defense/NATO context
- Clear all button

**"Show My Landscape" button** — triggers the visualization update (or auto-updates on selection change)

### Applicability Logic

Hardcoded mapping function that returns applicable framework IDs for a given sector + activities combination:

| Sector | Base Frameworks |
|--------|----------------|
| Financial | dora, nis2, iso27000, gdpr |
| Healthcare | nis2, gdpr, iso27000 |
| Critical Infrastructure | nis2, cer-directive, iso27000, nist-csf |
| Government/Public Admin | nis2, gdpr, iso27000 |
| Technology/AI Provider | gdpr, iso27000 |
| General Enterprise | iso27000, gdpr |

| Activity | Additional Frameworks |
|----------|----------------------|
| Processing personal data | +gdpr (if not already) |
| Deploying AI systems | +eu-ai-act, +nist-ai-rmf, +iso42001, +iso23894 |
| Operating critical infrastructure | +cer-directive, +nist-csf (if not already) |
| Financial services | +dora (if not already) |
| Defense/NATO context | +fmn, +zero-trust, +cisa-ztmm |

Universal frameworks always included: iso31000, iso31010, iso9000

### Results Area

**Compliance Stack:**
- Ordered list of applicable frameworks, most specific first (sector-specific → activity-specific → universal)
- Each framework shown as a card: name, concept count, why it applies (e.g., "Required for financial entities"), connected applicable frameworks count
- Applicable cards are full color; non-applicable frameworks shown below as a faded list

**Overlap Indicators:**
- Between applicable framework cards, show relationship count badges connecting them
- E.g., between DORA and NIS2: "5 shared requirements"

**Summary Banner:**
- "Based on your profile: N frameworks apply, covering M concepts with K cross-framework relationships"

### URL State

`validateSearch: { sector?: string; activities?: string }` — activities as comma-separated string. All comma-separated params must use `.split(',').filter(Boolean)` to avoid empty string artifacts from empty URL params.

---

## 7. Page 4: Unified Concept Search

### Layout

Full-width search input at top, below it: filter sidebar on left (240px), results on right.

### Search Input Component

- Full-width text input with search icon, auto-focused on mount
- Debounced at 300ms — updates URL `?q=` param after debounce
- Uses existing `GET /api/ontology/search?q=X&limit=500` endpoint (override default 50-result limit)
- Minimum query length: 2 characters (matches existing API behavior)
- **Known limitation:** Facet counts are computed client-side from the loaded result set (up to 500). For very broad queries, counts may not reflect the full dataset. This is acceptable without backend changes.
- Clear button (×) when text is present
- Result count shown below: "{N} results across {M} frameworks"

### Active Filters Bar

Below search input, above results. Shows active filters as removable pills/chips:
- "Framework: GDPR ×", "Type: control ×", etc.
- "Clear all" link to reset all filters

### Filter Sidebar

**Framework filter section:**
- Collapsible, expanded by default
- Checkbox list: each framework with its color dot, name, and result count in parentheses
- Only shows frameworks that have matching results
- Ordered by result count (descending)

**Concept Type filter section:**
- Collapsible, expanded by default
- Checkbox list: each type with result count
- Only shows types present in results
- Ordered by result count (descending)

### Results List

Results grouped by framework:
- Framework group header: colored dot + framework name + result count
- Under each group, concept cards:

Each concept card shows:
- Code (if exists, in monospace, muted)
- Name (bold)
- Concept type (small pill/badge)
- Definition excerpt (first ~100 chars, with search term highlighted)
- Parent hierarchy breadcrumb (e.g., "Core Functions > Govern > GOVERN 1")
- Two action links: "Open in Explorer →" (navigates to /ontology?concept=ID) and "Framework Detail →" (navigates to /frameworks?id=FRAMEWORK_ID)

### Keyboard Navigation

- Up/Down arrows move focus between result cards
- Enter on a focused card opens the ontology explorer
- Tab moves between filter checkboxes
- Escape clears search input

### Empty States

- **No query:** "Search across {N} concepts in {M} frameworks" with suggested searches: "incident reporting", "access control", "risk assessment", "data protection"
- **No results:** "No concepts matching '{query}'" with suggestions to broaden search or adjust filters

### Data Flow

1. User types → debounce → update `?q=` URL param
2. `useSearchConcepts(q)` → API call → results
3. Client-side filter by framework and concept type from URL params
4. Group results by framework_id, compute facet counts

### URL State

`validateSearch: { q?: string; frameworks?: string; types?: string }` — frameworks and types as comma-separated strings.

---

## 8. CSS and Styling

### New CSS Classes

Minimal additions — reuse existing design system wherever possible.

**Matrix cell styles:** Add `.crosswalk-cell` with hover transition and `.crosswalk-cell-active` with border accent. CSS only, no new animations.

**Two-tier nav:** Add `.nav-secondary` with `text-xs`, `border-t`, reduced padding.

### Existing Classes to Reuse

- `.feature-card` + `.corner-markers` for framework detail cards
- `.tech-badge` for framework version badges and type pills
- `.stat-number` for stat displays
- `.topo-grid` + `.gradient-mesh` for page backgrounds
- `.animate-fadeInUp` + delay classes for page load animations

---

## 9. File Structure

```
src/
  routes/
    frameworks/
      index.tsx                 # Framework Catalog page
    crosswalk/
      index.tsx                 # Crosswalk Explorer page
    landscape/
      index.tsx                 # Regulatory Landscape page
    concepts/
      search.tsx                # Unified Search page
    __root.tsx                  # Updated with two-tier nav
  features/
    ontology/
      api/index.ts              # Updated with new hooks
      types/index.ts            # Updated with new types
      utils/
        frameworkDomains.ts     # New: domain grouping utility
        landscapeMapping.ts     # New: sector/activity → framework mapping
      components/
        CrosswalkMatrix.tsx     # New: SVG matrix component
        CrosswalkDrilldown.tsx  # New: relationship detail panel
        FrameworkSidebar.tsx    # New: catalog sidebar
        FrameworkProfile.tsx    # New: catalog detail panel
        SearchResults.tsx       # New: search result list
        SearchFilters.tsx       # New: search faceted filters
        LandscapeSelector.tsx   # New: sector/activity selectors
        LandscapeResults.tsx    # New: applicable frameworks display
```

---

## 10. Build Order

All 4 pages ship together, but implementation proceeds in dependency order:

1. **Shared infrastructure:** New types, hooks (`useAllConcepts`, `useFrameworkStats`), utilities (`frameworkDomains.ts`, `landscapeMapping.ts`)
2. **Navigation update:** Two-tier nav in `__root.tsx`
3. **Framework Catalog:** Route + FrameworkSidebar + FrameworkProfile (uses new hooks)
4. **Crosswalk Explorer:** Route + CrosswalkMatrix + CrosswalkDrilldown (uses relationships + framework grouping)
5. **Unified Search:** Route + SearchResults + SearchFilters (uses search API + faceting)
6. **Regulatory Landscape:** Route + LandscapeSelector + LandscapeResults (uses landscape mapping + framework stats)

### Testing Strategy

Vitest + React Testing Library (add @testing-library/react and @testing-library/jest-dom as dev dependencies).

**Unit tests (pure functions):**
- `frameworkDomains.ts`: groupFrameworksByDomain returns correct grouping, handles empty input
- `landscapeMapping.ts`: each sector returns expected frameworks, activities add correct frameworks, no duplicates
- Matrix building: correctly counts relationships per framework pair, handles empty relationships, symmetric output
- Facet counting: correct counts per framework and type, handles empty results
- URL param parsing: comma-separated strings → arrays, filter(Boolean) removes empties

**Component tests:**
- Framework Catalog: sidebar renders all frameworks, clicking selects framework and updates detail, URL ?id param selects correct framework
- Crosswalk Matrix: cells render with correct color intensity, click opens drill-down, drill-down shows correct relationships
- Search: debounce fires after 300ms, filters update results, keyboard navigation works
- Landscape: sector selection highlights correct frameworks, activities add frameworks, URL state persists

Test files colocated: `__tests__/` directories next to components

---

## 11. Edge Cases and Error Handling

| Scenario | Handling |
|----------|----------|
| API returns empty frameworks | Show "No frameworks loaded" empty state |
| Selected framework in URL doesn't exist | Show catalog with no selection, toast "Framework not found" |
| Crosswalk cell clicked for 0-relationship pair | Drill-down shows "No relationships between these frameworks" |
| Search query too short (<2 chars) | Show prompt "Type at least 2 characters to search" |
| Search returns 0 results | Show empty state with suggestions |
| Landscape with no selections | Show all frameworks equally (no filtering) |
| Large result sets (>500 concepts) | Paginate or virtualize results list |
| Slow network | Loading skeletons matching final layout shape |
