# TDD Plan: Framework Explorer Insight Pages

Testing framework: Vitest + @testing-library/react + @testing-library/jest-dom

Test files colocated in `__tests__/` directories next to the code they test.

---

## 2. Navigation Architecture

### Tests for `__root.tsx` two-tier nav
- Test: secondary nav renders 4 links (Frameworks, Crosswalk, Landscape, Search)
- Test: secondary nav links have correct `to` props
- Test: active link gets `text-foreground` class on matching route

---

## 3. Shared Infrastructure

### Tests for `frameworkDomains.ts`
- Test: groupFrameworksByDomain returns 4 groups with correct labels
- Test: each group contains the expected framework IDs
- Test: all 22 frameworks are assigned to exactly one group
- Test: handles empty framework array gracefully (returns empty groups)
- Test: unknown framework IDs go to a fallback group or are excluded

### Tests for `landscapeMapping.ts`
- Test: each sector returns its base frameworks (Financial → dora, nis2, iso27000, gdpr)
- Test: each activity adds correct additional frameworks
- Test: combined sector + activities produces no duplicates
- Test: universal frameworks (iso31000, iso31010, iso9000) always included
- Test: empty sector + no activities returns only universal frameworks

### Tests for `useAllConcepts` hook
- Test: returns combined concepts from multiple frameworks
- Test: returns errors array when some queries fail
- Test: isLoading is true while any query is pending
- Test: builds correct concept-to-framework Map

### Tests for `useFrameworkStats` hook
- Test: returns correct conceptCount per framework
- Test: returns correct conceptTypes breakdown
- Test: returns correct connectedFrameworks count
- Test: returns correct relationshipCount

---

## 4. Page 1: Framework Catalog

### Tests for `FrameworkSidebar`
- Test: renders all frameworks grouped by domain
- Test: each framework shows color dot and concept count
- Test: clicking a framework calls the selection callback
- Test: active framework is visually highlighted
- Test: renders loading skeleton when data is pending

### Tests for `FrameworkProfile`
- Test: renders framework name, version, description, source link
- Test: renders 4 stat boxes with correct values
- Test: renders concept type breakdown bar
- Test: renders connected frameworks list sorted by relationship count
- Test: renders empty state when no framework selected
- Test: "not found" toast when URL ?id references nonexistent framework

### Tests for route `frameworks/index.tsx`
- Test: reads ?id from URL and selects that framework
- Test: default selects first framework when no ?id param
- Test: clicking a framework updates the URL ?id param

---

## 5. Page 2: Crosswalk Explorer

### Tests for matrix data building (pure function)
- Test: builds 22x22 matrix from relationships and concept-to-framework map
- Test: counts relationships correctly per framework pair
- Test: matrix is symmetric (A×B count == B×A count)
- Test: handles zero-relationship pairs (empty cells)
- Test: handles empty relationships array
- Test: ignores relationships where concept IDs are not in the lookup map

### Tests for `CrosswalkMatrix` component
- Test: renders 22×22 grid of cells
- Test: cells have correct color intensity based on relationship count
- Test: domain group separators are visible
- Test: hovering a cell highlights row and column
- Test: clicking a cell calls the selection callback with fw1 and fw2
- Test: keyboard arrow navigation moves focus between cells
- Test: table-view toggle renders an HTML table with same data
- Test: role="grid" and aria-label present on SVG

### Tests for `CrosswalkDrilldown` component
- Test: renders all relationships for the selected pair
- Test: each relationship shows source → type → target
- Test: relationship type filter checkboxes work
- Test: renders empty state for 0-relationship pairs
- Test: concept names link to ontology explorer and framework detail

### Tests for route `crosswalk/index.tsx`
- Test: reads ?fw1 and ?fw2 from URL and opens drill-down
- Test: reads ?type from URL and pre-filters relationship types

---

## 6. Page 3: Regulatory Landscape

### Tests for applicability logic (pure function)
- Test: Financial sector → dora, nis2, iso27000, gdpr + universals
- Test: Financial + Deploying AI → adds eu-ai-act, nist-ai-rmf, iso42001, iso23894
- Test: Critical Infrastructure → nis2, cer-directive, iso27000, nist-csf + universals
- Test: Defense/NATO activity → adds fmn, zero-trust, cisa-ztmm
- Test: Multiple activities combine without duplicates
- Test: Empty selections return only universal frameworks

### Tests for `LandscapeSelector`
- Test: renders sector radio buttons
- Test: renders activity checkboxes
- Test: sector selection updates callback
- Test: multiple activities can be selected
- Test: clear all button resets activities

### Tests for `LandscapeResults`
- Test: applicable frameworks shown with full styling
- Test: non-applicable frameworks shown faded
- Test: overlap indicators show relationship counts between applicable frameworks
- Test: summary banner shows correct counts

### Tests for route `landscape/index.tsx`
- Test: reads ?sector and ?activities from URL
- Test: selecting sector updates URL
- Test: comma-separated activities parsed with filter(Boolean)

---

## 7. Page 4: Unified Search

### Tests for search debounce
- Test: typing does not trigger search immediately
- Test: search fires 300ms after last keystroke
- Test: rapid typing only fires once after settling

### Tests for facet counting (pure function)
- Test: counts concepts per framework from results
- Test: counts concepts per type from results
- Test: respects active framework filter
- Test: respects active type filter
- Test: handles empty results (all counts zero)

### Tests for `SearchFilters`
- Test: renders framework checkboxes with counts
- Test: renders concept type checkboxes with counts
- Test: toggling a checkbox calls the filter callback
- Test: only shows frameworks/types with results

### Tests for `SearchResults`
- Test: groups results by framework
- Test: each result shows code, name, type pill, definition excerpt
- Test: "Open in Explorer" link navigates to /ontology?concept=ID
- Test: "Framework Detail" link navigates to /frameworks?id=FW_ID
- Test: empty state shows suggestions when no query
- Test: no results state shows message when query has no matches

### Tests for route `concepts/search.tsx`
- Test: reads ?q, ?frameworks, ?types from URL
- Test: auto-focuses search input on mount
- Test: Escape key clears search input
- Test: arrow keys navigate between result cards

---

## 8. CSS and Styling

No tests needed — visual verification only.

---

## URL Param Utilities

### Tests for comma-separated param parsing
- Test: "a,b,c" → ["a", "b", "c"]
- Test: "" → [] (not [""])
- Test: "a,,b" → ["a", "b"] (no empty strings)
- Test: undefined → []
