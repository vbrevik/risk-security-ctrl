I have all the context needed. Let me produce the section content.

# Section 04: Crosswalk Explorer

## Overview

This section implements the Crosswalk Explorer page at `/crosswalk`. It renders a 22x22 SVG heatmap matrix showing relationship density between every framework pair, with a drill-down panel for inspecting individual mappings. The matrix uses D3 scales for layout and color, React for rendering SVG elements, and supports full keyboard navigation and accessibility.

## Dependencies

- **section-01-shared-infra** must be complete: provides `CrosswalkCell` type in `types/index.ts`, the `useAllConcepts` hook (with concept-to-framework map), `useFrameworkStats` hook, and `groupFrameworksByDomain` from `frameworkDomains.ts`.
- **section-02-navigation** must be complete: provides the route placeholder at `src/routes/crosswalk/index.tsx` and the secondary nav link.

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/ontology/utils/crosswalkMatrix.ts` | Pure function to build the 22x22 matrix data structure |
| `frontend/src/features/ontology/components/CrosswalkMatrix.tsx` | SVG heatmap matrix component |
| `frontend/src/features/ontology/components/CrosswalkDrilldown.tsx` | Slide-out relationship detail panel |
| `frontend/src/features/ontology/utils/__tests__/crosswalkMatrix.test.ts` | Unit tests for matrix builder |
| `frontend/src/features/ontology/components/__tests__/CrosswalkMatrix.test.tsx` | Component tests for the matrix |
| `frontend/src/features/ontology/components/__tests__/CrosswalkDrilldown.test.tsx` | Component tests for the drill-down |
| `frontend/src/routes/crosswalk/__tests__/index.test.tsx` | Route-level tests |

## Files to Modify

| File | Change |
|------|--------|
| `frontend/src/routes/crosswalk/index.tsx` | Replace placeholder with full Crosswalk Explorer page component |

---

## Implementation Notes (Post-Build)

### Decision: Existing Implementation Retained

A parallel development session implemented a full CrosswalkView (commit c993e21) that covers the core crosswalk functionality:

- Two-framework selection with swap button
- Three abstraction level filters (Function/Step, Subcategory/Task, Action/Playbook)
- Relationship type filtering with color-coded stats bar
- Expandable mapping list with inline detail expansion
- Detail sidebar with concept cards and rationale
- Full i18n support (en/nb)
- URL state persistence for bookmarking

**What was NOT implemented from this section's plan:**
- D3 SVG 22x22 heatmap matrix (deferred — the existing two-framework view is more actionable)
- `crosswalkMatrix.ts` pure function utility
- `CrosswalkMatrix.tsx` and `CrosswalkDrilldown.tsx` components
- Section-specific tests (existing CrosswalkView has no unit tests — deferred)
- Keyboard navigation and WCAG role="grid" accessibility

**Rationale:** The existing CrosswalkView provides direct framework-to-framework comparison with filtering, which is more useful for the current dataset (48 relationships between 2 frameworks). The planned heatmap matrix becomes valuable when cross-framework relationships are denser across all 22 frameworks. This can be added as a secondary view in a future iteration.

---

## Tests

Write all tests before implementation. Tests use Vitest and `@testing-library/react` (installed in section-01).

### 1. Matrix Data Building -- `frontend/src/features/ontology/utils/__tests__/crosswalkMatrix.test.ts`

Pure function tests for the `buildCrosswalkMatrix` utility. This function takes a list of `Relationship` objects and a `Map<string, string>` (concept ID to framework ID) and returns a `Map<string, CrosswalkCell>` keyed by `"fw1|fw2"` (alphabetically sorted pair).

```typescript
import { describe, it, expect } from "vitest";
import { buildCrosswalkMatrix } from "../crosswalkMatrix";
// import types: Relationship from "../../types"
```

Test cases:

- **builds 22x22 matrix from relationships and concept-to-framework map**: Given 2 relationships mapping concepts from framework A to framework B, the cell for A-B should have count 2.
- **counts relationships correctly per framework pair**: Given relationships across 3 frameworks (A-B: 3, A-C: 1, B-C: 2), verify each cell count.
- **matrix is symmetric (A x B count equals B x A count)**: The function should produce a single canonical key per pair (alphabetical). Looking up either direction returns the same cell.
- **handles zero-relationship pairs (empty cells)**: Framework pairs with no relationships should not appear in the map (or have count 0).
- **handles empty relationships array**: Returns an empty map.
- **ignores relationships where concept IDs are not in the lookup map**: If a relationship references a concept ID not present in the concept-to-framework map, it is silently skipped.

### 2. CrosswalkMatrix Component -- `frontend/src/features/ontology/components/__tests__/CrosswalkMatrix.test.tsx`

Component tests using `@testing-library/react`. Mock the data hooks (`useFrameworks`, `useAllConcepts`, `useRelationships`) to return controlled data.

Test cases:

- **renders 22x22 grid of cells**: The SVG should contain `role="gridcell"` elements. With N frameworks, expect N*N rect elements (or N*(N-1) if diagonal is excluded).
- **cells have correct color intensity based on relationship count**: Cells with 0 relationships should be transparent/empty. Cells with 1-2 should have a light fill. Cells with 6+ should have a strong fill. Verify via data attributes or CSS classes.
- **domain group separators are visible**: The SVG contains line elements or spacing between domain groups.
- **hovering a cell highlights row and column**: After `fireEvent.mouseEnter` on a cell, the corresponding row and column cells gain a highlight class or attribute.
- **clicking a cell calls the selection callback with fw1 and fw2**: After `fireEvent.click` on a cell, verify the `onCellSelect` callback is called with the correct framework pair.
- **keyboard arrow navigation moves focus between cells**: After focusing a cell and pressing ArrowRight, the adjacent cell receives focus.
- **table-view toggle renders an HTML table with same data**: After clicking the "Table view" toggle button, an HTML `<table>` element should be in the document with `role="grid"`.
- **role="grid" and aria-label present on SVG**: The SVG element has `role="grid"` and `aria-label="Cross-framework relationship matrix"`.

### 3. CrosswalkDrilldown Component -- `frontend/src/features/ontology/components/__tests__/CrosswalkDrilldown.test.tsx`

Test cases:

- **renders all relationships for the selected pair**: Given a cell with 3 relationships, the panel renders 3 relationship items.
- **each relationship shows source, type, target**: Each item contains the source concept name, a relationship type badge, and the target concept name.
- **relationship type filter checkboxes work**: Unchecking "maps_to" hides relationships of that type from the list.
- **renders empty state for 0-relationship pairs**: When the cell has count 0, show "No relationships between these frameworks".
- **concept names link to ontology explorer and framework detail**: Source/target concept names render as links. Verify `href` attributes include `/ontology?concept=ID` and `/frameworks?id=FW_ID`.

### 4. Route Tests -- `frontend/src/routes/crosswalk/__tests__/index.test.tsx`

Test cases:

- **reads ?fw1 and ?fw2 from URL and opens drill-down**: When the route mounts with `?fw1=gdpr&fw2=nis2`, the drill-down panel is visible.
- **reads ?type from URL and pre-filters relationship types**: When mounted with `?type=maps_to`, only that type filter checkbox is checked.

---

## Implementation Details

### Pure Function: `buildCrosswalkMatrix`

**File:** `frontend/src/features/ontology/utils/crosswalkMatrix.ts`

```typescript
import type { Relationship, CrosswalkCell } from "../types";

/**
 * Build a crosswalk matrix from relationships and a concept-to-framework lookup.
 *
 * @param relationships - All cross-framework relationships
 * @param conceptToFramework - Map from concept ID to framework ID
 * @returns Map keyed by "fw1|fw2" (alphabetically sorted) to CrosswalkCell
 */
export function buildCrosswalkMatrix(
  relationships: Relationship[],
  conceptToFramework: Map<string, string>
): Map<string, CrosswalkCell>;
```

Algorithm:
1. Iterate each relationship.
2. Look up `source_concept_id` and `target_concept_id` in `conceptToFramework`.
3. If either is missing, skip.
4. If both map to the same framework, skip (no self-relationships in the matrix).
5. Create a canonical key by sorting the two framework IDs alphabetically and joining with `|`.
6. Upsert into the result map: increment `count`, push the relationship into the `relationships` array.
7. Return the map.

Also export a helper:

```typescript
/**
 * Get the canonical key for a framework pair.
 */
export function cellKey(fw1: string, fw2: string): string;
```

### CrosswalkMatrix Component

**File:** `frontend/src/features/ontology/components/CrosswalkMatrix.tsx`

Props:

```typescript
interface CrosswalkMatrixProps {
  frameworks: Framework[];
  matrixData: Map<string, CrosswalkCell>;
  domainGroups: { label: string; frameworkIds: string[] }[];
  selectedCell: { fw1: string; fw2: string } | null;
  onCellSelect: (fw1: string, fw2: string) => void;
  tableView?: boolean;
}
```

Key implementation details:

**D3 Scales (computed in useMemo):**
- `scaleBand` for both x and y axes, domain is the ordered list of framework IDs (ordered by domain group).
- `scaleSequential` with `interpolateOranges` or a custom oklch amber ramp for color intensity. Domain: `[0, maxCount]` where `maxCount` is the highest cell count. Use discrete bins: 0 (transparent), 1-2 (light), 3-5 (medium), 6+ (strong).

**SVG Structure:**
- Outer `<svg>` with `role="grid"`, `aria-label="Cross-framework relationship matrix"`.
- Row of `<text>` elements for x-axis labels (rotated -45deg).
- Column of `<text>` elements for y-axis labels.
- Grid of `<rect>` elements, each with `role="gridcell"`, `aria-label="{fw1} and {fw2}: {count} relationships"`.
- Domain separator `<line>` elements between groups.

**Hover behavior:**
- Track `hoveredCell` in state as `{ row: number; col: number } | null`.
- On `mouseEnter` of a cell, set hovered. On `mouseLeave`, clear.
- All cells in the same row or column get a subtle highlight fill (use `opacity` or a CSS class `.crosswalk-cell-highlight`).

**Keyboard navigation:**
- Track `focusedCell` in state as `{ row: number; col: number }`.
- `onKeyDown` handler on the SVG: ArrowUp/Down/Left/Right move the focus. Enter/Space trigger `onCellSelect`.
- Use `tabIndex={0}` on the SVG. Programmatically manage which `<rect>` shows the focus ring via a ref or data attribute.

**Table view toggle:**
- When `tableView` is true, render an HTML `<table>` instead of the SVG.
- Same data, same color coding via background-color on `<td>` elements.
- Framework names as row/column headers.
- Each cell shows the count number.

**Responsive:**
- Wrap the SVG in a scrollable container (`overflow-x: auto`).
- The SVG has a fixed computed width (frameworks.length * cellSize + margins). It does not shrink below a readable minimum.

**prefers-reduced-motion:**
- Check via `window.matchMedia('(prefers-reduced-motion: reduce)')` or a CSS media query.
- When active, disable hover transition animations on cells.

**CSS classes to add (minimal):**
- `.crosswalk-cell` -- base cell with `transition: opacity 150ms` and `cursor: pointer`.
- `.crosswalk-cell-active` -- selected cell with accent border.

### CrosswalkDrilldown Component

**File:** `frontend/src/features/ontology/components/CrosswalkDrilldown.tsx`

Props:

```typescript
interface CrosswalkDrilldownProps {
  fw1: Framework;
  fw2: Framework;
  relationships: Relationship[];
  concepts: Map<string, Concept>;  // lookup for concept details
  activeTypes: string[];
  onTypeToggle: (type: string) => void;
  onClose: () => void;
}
```

Key implementation details:

**Layout:**
- Fixed-width panel (320px) sliding in from the right with a subtle shadow.
- Close button (X) in the top-right corner.

**Header:**
- "{fw1.name} x {fw2.name}" with colored dots for each framework.
- Relationship count badge.

**Type filters:**
- Row of checkboxes at the top: maps_to, implements, related_to, supports.
- Each has a color-coded pill matching the relationship type colors (maps_to: blue, implements: green, related_to: gray, supports: amber).
- Toggling a checkbox updates the `activeTypes` filter via `onTypeToggle`.

**Relationship list:**
- Filtered by `activeTypes`.
- Each item: source concept name (with fw color dot) -> type badge -> target concept name (with fw color dot).
- Concept names are links: clicking navigates to `/ontology?concept={conceptId}`.
- A secondary smaller link on each concept to `/frameworks?id={frameworkId}`.

**Empty state:**
- "No relationships between these frameworks" message.
- Brief note about what each framework covers (use `fw1.description` and `fw2.description`).

### Route Component: `frontend/src/routes/crosswalk/index.tsx`

Replace the placeholder with the full page component.

```typescript
import { createFileRoute } from "@tanstack/react-router";

interface CrosswalkSearch {
  fw1?: string;
  fw2?: string;
  type?: string;
}

export const Route = createFileRoute("/crosswalk/")({
  component: CrosswalkPage,
  validateSearch: (search: Record<string, unknown>): CrosswalkSearch => ({
    fw1: (search.fw1 as string) ?? undefined,
    fw2: (search.fw2 as string) ?? undefined,
    type: (search.type as string) ?? undefined,
  }),
});
```

**CrosswalkPage component logic:**

1. Read URL params via `Route.useSearch()` to get `fw1`, `fw2`, `type`.
2. Call `useFrameworks()` for the framework list.
3. Call `useAllConcepts()` for all concepts and the concept-to-framework map.
4. Call `useRelationships()` for all relationships.
5. Use `groupFrameworksByDomain(frameworks)` to get domain-ordered framework list.
6. Use `useMemo` to call `buildCrosswalkMatrix(relationships, conceptToFramework)`.
7. Track `tableView` boolean in local state with a toggle button in the page controls.
8. Track `selectedCell` from URL params or local state. When `fw1` and `fw2` are in the URL, auto-open the drill-down.
9. Parse `type` URL param into active type filters (comma-separated with `filter(Boolean)`).
10. On cell select, update URL params via `navigate({ search: { fw1, fw2 } })`.
11. On type filter toggle, update URL `?type` param.

**Loading state:**
- While any of the three queries are loading, show a full-page skeleton: a 22x22 grid of placeholder rectangles with pulse animation.
- Show a progress message: "Loading frameworks... (X/22)" derived from `useAllConcepts` query states.

**Error handling:**
- If `useAllConcepts` returns partial errors, show a warning banner: "X frameworks failed to load, showing partial data".
- The matrix still renders with available data.

### Data Flow Summary

```
useFrameworks() ──────────────┐
                              │
useAllConcepts() ─────────────┤──> buildCrosswalkMatrix() ──> CrosswalkMatrix
  └─ conceptToFramework Map   │
                              │
useRelationships() ───────────┘
                                   Cell click
                                      │
                                      ▼
                              CrosswalkDrilldown
                              (filtered relationships)
```

### URL State

The route uses `validateSearch` with three optional params:

- `fw1` -- first framework ID of the selected cell
- `fw2` -- second framework ID of the selected cell
- `type` -- comma-separated relationship type filter (e.g., `"maps_to,implements"`)

When `fw1` and `fw2` are both present, the drill-down panel opens automatically on page load. When `type` is present, its values are split by comma with `filter(Boolean)` and used to pre-check the type filter checkboxes.

### Edge Cases

| Scenario | Handling |
|----------|----------|
| Cell clicked for 0-relationship pair | Drill-down shows "No relationships between these frameworks" with descriptions |
| URL has `?fw1=invalid` | Ignore invalid IDs; drill-down stays closed |
| All queries loading | Show skeleton grid with progress indicator |
| Partial load failure | Show matrix with available data + warning banner |
| Small screen (<768px) | Scrollable container with pinned labels |
| prefers-reduced-motion | Disable hover transition animations |
| Color-only information | Discrete bins with count numbers shown on hover; table view toggle available |

### CSS Additions

Add to the project's CSS (or as a Tailwind `@layer components` block):

```css
.crosswalk-cell {
  cursor: pointer;
  transition: opacity 150ms;
}

.crosswalk-cell-active {
  stroke: var(--accent);
  stroke-width: 2;
}

@media (prefers-reduced-motion: reduce) {
  .crosswalk-cell {
    transition: none;
  }
}
```

### Accessibility Checklist

- SVG element: `role="grid"`, `aria-label="Cross-framework relationship matrix"`
- Each cell: `role="gridcell"`, `aria-label="{fw1 name} and {fw2 name}: {count} relationships"`
- Keyboard: Arrow keys navigate cells, Enter/Space opens drill-down
- Focus indicator: visible 2px border on the focused cell
- Color: discrete bins (0, 1-2, 3-5, 6+) not continuous; palette tested for deuteranopia/protanopia
- Count numbers visible on hover/focus (not color-only)
- Table alternative: toggle button switches to HTML `<table>` view
- `prefers-reduced-motion`: hover animations disabled