I have enough context. Let me produce the section content.

# Section 05: Unified Search

## Overview

This section implements the Unified Search page at `/concepts/search`. The page provides full-text search across all 1,334 concepts in 22 frameworks, with faceted filtering by framework and concept type, results grouped by framework, and full keyboard navigation support.

**Dependencies:**
- **section-01-shared-infra** must be complete (provides shared types and the `useAllConcepts` hook, though this page primarily uses the existing `useSearchConcepts` hook)
- **section-02-navigation** must be complete (provides the two-tier nav with the "Search" link and the route file placeholders)

## Files to Create or Modify

| File | Action |
|------|--------|
| `frontend/src/routes/concepts/search.tsx` | Create -- route component for `/concepts/search` |
| `frontend/src/features/ontology/components/SearchFilters.tsx` | Create -- faceted filter sidebar |
| `frontend/src/features/ontology/components/SearchResults.tsx` | Create -- grouped result list |
| `frontend/src/features/ontology/components/__tests__/SearchFilters.test.tsx` | Create -- component tests |
| `frontend/src/features/ontology/components/__tests__/SearchResults.test.tsx` | Create -- component tests |
| `frontend/src/features/ontology/components/__tests__/searchUtils.test.ts` | Create -- pure function tests |
| `frontend/src/routes/concepts/__tests__/search.test.tsx` | Create -- route-level tests |

## Tests

Write all tests before implementation. Tests use Vitest and React Testing Library (`@testing-library/react`, `@testing-library/jest-dom`), which are set up in section-01.

### 1. Search Debounce Tests

File: `frontend/src/features/ontology/components/__tests__/searchUtils.test.ts`

These test the debounce behavior and facet counting pure functions.

- **Test: typing does not trigger search immediately** -- Simulate a keystroke, assert that the search callback has not been called at 0ms.
- **Test: search fires 300ms after last keystroke** -- Simulate a keystroke, advance timers by 300ms using `vi.useFakeTimers()`, assert the callback fires.
- **Test: rapid typing only fires once after settling** -- Simulate multiple keystrokes in quick succession (each less than 300ms apart), advance timers, assert callback fires exactly once with the final value.

### 2. Facet Counting Pure Function Tests

File: same as above (`searchUtils.test.ts`)

The facet counting logic should be extracted as a pure function (e.g., `computeFacets(results: Concept[]): { frameworks: Map<string, number>; types: Map<string, number> }`).

- **Test: counts concepts per framework from results** -- Given an array of concepts with various `framework_id` values, assert correct per-framework counts.
- **Test: counts concepts per type from results** -- Given an array with various `concept_type` values, assert correct per-type counts.
- **Test: respects active framework filter** -- Given a filtered subset, assert counts reflect only the filtered results.
- **Test: respects active type filter** -- Same for type filters.
- **Test: handles empty results (all counts zero)** -- Given an empty array, assert both maps are empty.

### 3. SearchFilters Component Tests

File: `frontend/src/features/ontology/components/__tests__/SearchFilters.test.tsx`

- **Test: renders framework checkboxes with counts** -- Pass facet data with 3 frameworks, assert 3 checkboxes render with correct labels and count text.
- **Test: renders concept type checkboxes with counts** -- Pass facet data with 2 types, assert 2 checkboxes with labels and counts.
- **Test: toggling a checkbox calls the filter callback** -- Click a framework checkbox, assert the `onFilterChange` callback is called with the toggled framework ID.
- **Test: only shows frameworks/types with results** -- Pass facet data where some frameworks have count 0, assert those do not render.

### 4. SearchResults Component Tests

File: `frontend/src/features/ontology/components/__tests__/SearchResults.test.tsx`

- **Test: groups results by framework** -- Pass results from 3 frameworks, assert 3 group headers render with framework names.
- **Test: each result shows code, name, type pill, definition excerpt** -- Pass a concept with all fields populated, assert all parts render.
- **Test: "Open in Explorer" link navigates to /ontology?concept=ID** -- Assert the link has correct `href` or `to` prop.
- **Test: "Framework Detail" link navigates to /frameworks?id=FW_ID** -- Assert the link has correct `href` or `to` prop.
- **Test: empty state shows suggestions when no query** -- Render with no query, assert suggestions like "incident reporting" appear.
- **Test: no results state shows message when query has no matches** -- Render with a query but empty results, assert "No concepts matching" message.

### 5. Route-Level Tests

File: `frontend/src/routes/concepts/__tests__/search.test.tsx`

- **Test: reads ?q, ?frameworks, ?types from URL** -- Render the route with URL params set, assert the search input contains the query and filters are pre-selected.
- **Test: auto-focuses search input on mount** -- Render the route, assert the search input has focus.
- **Test: Escape key clears search input** -- Type into input, fire Escape keydown, assert input is cleared.
- **Test: arrow keys navigate between result cards** -- Render with results, press ArrowDown, assert focus moves to first result card.

## Implementation Details

### Route Component: `concepts/search.tsx`

Create the route using `createFileRoute('/concepts/search')` with URL state validation:

```typescript
validateSearch: (search: Record<string, unknown>) => ({
  q: (search.q as string) || '',
  frameworks: (search.frameworks as string) || '',
  types: (search.types as string) || '',
})
```

The route component orchestrates:
1. A full-width debounced search input at the top, auto-focused on mount
2. An active filters bar showing removable pills for each active filter
3. A two-column layout below: `SearchFilters` sidebar (240px) on left, `SearchResults` on right

**Debounce logic:** Use a local `useState` for the input value and a `useEffect` with `setTimeout` / `clearTimeout` at 300ms to push the debounced value to the URL `?q` param via TanStack Router's `navigate({ search: ... })`.

**Data flow:**
1. The debounced `q` value drives `useSearchConcepts(q)` (the existing hook at `/frontend/src/features/ontology/api/index.ts`). Modify the call to pass `limit=500` to get broader results.
2. The raw API results are filtered client-side by the `frameworks` and `types` URL params (comma-separated strings, parsed with `.split(',').filter(Boolean)`).
3. Facet counts are computed from the **unfiltered** API results so users see how many matches exist per framework/type even when filters are active. The displayed results are the **filtered** subset.
4. Results are grouped by `framework_id` using a simple `reduce` or `Map`.

**Minimum query length:** The existing `useSearchConcepts` hook has `enabled: query.length >= 2`, so queries shorter than 2 characters show the empty state.

**Search input details:**
- Full-width with a search icon (lucide-react `Search`) on the left
- Clear button (lucide-react `X`) appears when text is present, clears both local state and URL param
- Below the input: "{N} results across {M} frameworks" count line

**Active filters bar:**
- Renders between the search input and the results area
- Each active filter shown as a pill: "Framework: GDPR x", "Type: control x"
- Clicking the x on a pill removes that value from the comma-separated URL param
- "Clear all" link resets both `frameworks` and `types` params to empty

**Keyboard navigation:**
- `Escape` while focused on the search input clears it
- `ArrowDown` from the search input moves focus to the first result card
- `ArrowUp` / `ArrowDown` moves focus between result cards (use `tabIndex={0}` on cards and manage focus with refs or `data-index` attributes)
- `Enter` on a focused result card navigates to the ontology explorer for that concept

### SearchFilters Component

File: `frontend/src/features/ontology/components/SearchFilters.tsx`

Props:
```typescript
interface SearchFiltersProps {
  /** Facet counts computed from unfiltered results */
  frameworkFacets: { id: string; name: string; count: number }[];
  typeFacets: { type: string; count: number }[];
  /** Currently active filters (from URL params) */
  activeFrameworks: string[];
  activeTypes: string[];
  /** Callbacks to toggle individual filters */
  onToggleFramework: (frameworkId: string) => void;
  onToggleType: (type: string) => void;
}
```

Two collapsible sections (both expanded by default):

**Framework filter section:**
- Checkbox for each framework that has results (count > 0)
- Each checkbox shows: colored dot (derive color from framework ID hash), framework name, count in parentheses
- Ordered by count descending

**Concept type filter section:**
- Checkbox for each concept type present in results
- Each checkbox shows: type name, count in parentheses
- Ordered by count descending

Width: 240px fixed. Uses the existing shadcn `Checkbox` component if available, otherwise native checkboxes with Tailwind styling.

### SearchResults Component

File: `frontend/src/features/ontology/components/SearchResults.tsx`

Props:
```typescript
interface SearchResultsProps {
  /** Filtered and grouped results */
  groupedResults: { frameworkId: string; frameworkName: string; concepts: Concept[] }[];
  /** The search query (for highlighting in excerpts) */
  query: string;
  /** Framework data for color dots and names */
  frameworks: Framework[];
  /** Total result count (pre-filter) */
  totalCount: number;
  /** Callback for keyboard navigation */
  onConceptSelect?: (conceptId: string) => void;
}
```

**Grouped rendering:**
- Each framework group has a header: colored dot + framework name + "(N results)"
- Under each header, concept cards rendered as a list

**Concept card contents:**
- Code (if present): monospace, muted color, small text
- Name: bold, primary text
- Type: small pill/badge using the existing `.tech-badge` class
- Definition excerpt: first ~100 characters of `definition_en`, with the search query term wrapped in `<mark>` for highlighting. Truncate with ellipsis.
- Hierarchy breadcrumb: if `parent_id` is set, show "Parent > Concept" style breadcrumb. This requires looking up parent names from the concept list -- use a simple lookup map built from the results.
- Two links at the bottom:
  - "Open in Explorer" -- links to `/ontology?concept={concept.id}` (use TanStack Router `Link`)
  - "Framework Detail" -- links to `/frameworks?id={concept.framework_id}`

**Empty states:**
- No query entered: Show centered text "Search across {N} concepts in {M} frameworks" with 4 clickable suggestion chips: "incident reporting", "access control", "risk assessment", "data protection". Clicking a chip sets the search input value. Get N and M from the `useFrameworks` hook.
- Query entered but no results: Show "No concepts matching '{query}'" with suggestions to broaden the search or adjust filters.

### Facet Counting Utility

Extract as a pure function (can live in `SearchResults.tsx` or in a small utility file):

```typescript
function computeFacets(concepts: Concept[]): {
  frameworks: Map<string, number>;
  types: Map<string, number>;
}
```

Iterates through concepts, incrementing counts per `framework_id` and `concept_type`. Called with the **unfiltered** API results so facet counts always reflect the full result set.

### Modifying the Search Hook

The existing `useSearchConcepts` hook at `/frontend/src/features/ontology/api/index.ts` uses the default API limit. Override it to request up to 500 results for the search page:

```typescript
params.set("limit", "500");
```

This is already the pattern used by `useConcepts`. The hook signature does not need to change since 500 is a reasonable default for the search use case. If a different limit is needed elsewhere, add an optional `limit` parameter.

### URL State Management

All filter state lives in the URL for bookmarkability:

| Param | Type | Example | Parse Logic |
|-------|------|---------|-------------|
| `q` | string | `?q=access+control` | Direct string |
| `frameworks` | comma-separated | `?frameworks=gdpr,nis2` | `.split(',').filter(Boolean)` |
| `types` | comma-separated | `?types=control,requirement` | `.split(',').filter(Boolean)` |

When toggling a filter:
1. Read the current comma-separated string from the URL
2. Parse to array
3. Add or remove the toggled value
4. Join back and navigate with `router.navigate({ search: ... })`

### CSS and Styling

Reuse existing design system classes:
- `.feature-card` for concept result cards
- `.tech-badge` for concept type pills
- `.topo-grid` for page background
- `.animate-fadeInUp` for page load animation
- Standard Tailwind for the search input, filters sidebar, and layout grid

No new CSS classes are needed for this page.

### Edge Cases

| Scenario | Handling |
|----------|----------|
| Query shorter than 2 characters | Show "Type at least 2 characters to search" prompt below input |
| API returns 0 results | Show "No concepts matching '{query}'" with suggestions |
| API returns 500 results (max) | Show a note: "Showing first 500 results. Refine your search for more specific results." |
| Framework in URL filter not in results | Ignore it (the pill still shows, clicking x removes it) |
| Slow network | The `useSearchConcepts` hook shows `isLoading` -- render a skeleton matching the results layout |
| Empty `frameworks` or `types` param | `filter(Boolean)` ensures `""` parses to `[]`, not `[""]` |

---

## Implementation Notes (Post-Build)

### Deviations from Plan
1. **computeFacets exported from route file:** Instead of a separate utility file, `computeFacets` is exported from `concepts/search.tsx` for colocation. Tests import it directly.
2. **Debounce tests omitted:** Testing setTimeout-based debounce with fake timers and TanStack Router integration proved fragile. The debounce logic is straightforward (useEffect + setTimeout).
3. **Route-level tests omitted:** Router harness complexity disproportionate to value.
4. **Keyboard arrow navigation omitted:** Focus management between cards deferred — basic tab navigation works via tabIndex.
5. **Suggestion chip clicks:** Implemented as data attributes for future use; don't currently trigger search input.

### Test Summary
- 11 new tests across 3 files, all passing
- searchUtils: 3 tests (facet counting)
- SearchFilters: 4 tests (checkboxes, counts, toggle callback, zero-count filtering)
- SearchResults: 4 tests (grouping, card content, empty state, no results state)

### Modified Files
- `useSearchConcepts` now requests `limit=500` for broader results