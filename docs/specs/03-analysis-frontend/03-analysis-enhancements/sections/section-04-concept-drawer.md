Now I have all the context I need. Let me generate the section content.

# Section 4: Concept Side Panel (ConceptDrawer)

## Overview

This section implements two things:

1. A new **ConceptDrawer** component -- a slide-in side panel (using shadcn Sheet) that displays full ontology concept details when a user clicks a concept in the findings table.
2. Modifications to **FindingsTable** to add an `onConceptClick` callback, making concept name/code cells clickable.

This section has **no dependencies** on sections 1-3 and can be developed in parallel. Section 5 (page assembly) will wire the drawer into the detail page.

---

## Tests First

All tests use Vitest and React Testing Library. Mock `react-i18next` and `useConceptRelationships`.

### ConceptDrawer Tests

**File:** `src/features/analysis/components/__tests__/ConceptDrawer.test.tsx` (new)

These tests mock `useConceptRelationships` from `@/features/ontology/api` and verify the drawer's rendering states.

```typescript
/**
 * ConceptDrawer.test.tsx
 *
 * Mock setup:
 *   - vi.mock("react-i18next") returning { useTranslation: () => ({ t: (key) => key }) }
 *   - vi.mock("@/features/ontology/api") to control useConceptRelationships return value
 *   - The Sheet component from shadcn uses Radix Dialog under the hood; it renders
 *     in a portal. Use screen queries on document body.
 *
 * Test cases:
 */
```

1. **renders nothing / closed state when conceptId is null** -- Render `<ConceptDrawer conceptId={null} onClose={vi.fn()} />`. Assert no Sheet content is visible (e.g., the concept panel title i18n key is not in the document).

2. **renders Sheet in open state when conceptId is non-null** -- Mock `useConceptRelationships` to return `{ data: mockConceptData, isLoading: false, isError: false }`. Render with `conceptId="test-concept"`. Assert the panel title text is present.

3. **displays concept name, code, type, framework from fetched data** -- With a fully populated mock concept (`name_en`, `code`, `concept_type`, `framework_id`), assert those values appear in the rendered output.

4. **displays concept definition** -- Assert `definition_en` text from mock data is rendered.

5. **shows loading skeleton when data is loading** -- Mock `useConceptRelationships` to return `{ data: undefined, isLoading: true, isError: false }`. Assert a loading indicator is present (e.g., element with `aria-label` matching the loading i18n key, or skeleton elements).

6. **shows error state with retry button when fetch fails** -- Mock `useConceptRelationships` to return `{ data: undefined, isLoading: false, isError: true, refetch: vi.fn() }`. Assert the error i18n key text is displayed and a retry button exists. Click retry and verify `refetch` was called.

7. **"Open in Ontology Explorer" link has correct href and target** -- Assert a link element exists with `href="/ontology?concept=test-concept"` and `target="_blank"`.

8. **close button calls onClose callback** -- Render with a `vi.fn()` for `onClose`. Find and click the close trigger. Verify `onClose` was called.

### FindingsTable Concept Click Tests

**File:** `src/features/analysis/components/__tests__/FindingsTable.test.tsx` (extend existing)

Add tests to the existing `describe("FindingsTable")` block.

```typescript
/**
 * Additional FindingsTable tests for concept click behavior.
 *
 * Uses the existing makeFinding() helper and defaultProps from the test file.
 * Add onConceptClick to defaultProps as undefined (backward compat).
 *
 * Test cases:
 */
```

1. **concept name cell is clickable (has button role)** -- Render with `onConceptClick={vi.fn()}` and a finding that has a non-null `concept_name`. Query for a button containing the concept name text. Assert it exists.

2. **clicking concept name fires onConceptClick with finding's concept_id** -- Render with `onConceptClick={vi.fn()}`. Click the concept name button. Assert the callback was called with the finding's `concept_id` value (`"c1"` from the `makeFinding` helper).

3. **concept code cell shows dash when concept_code is null (not clickable)** -- The second finding in `defaultProps` has `concept_code: null`. Assert the dash character is present and there is no button wrapping it.

4. **no error when onConceptClick is not provided** -- Render without `onConceptClick` prop. Assert no crash and no concept-click buttons are rendered (cells render as plain text).

---

## Implementation Details

### ConceptDrawer Component

**File:** `src/features/analysis/components/ConceptDrawer.tsx` (new)

**Props interface:**

```typescript
interface ConceptDrawerProps {
  conceptId: string | null;
  onClose: () => void;
}
```

**Key implementation notes:**

- Import `Sheet`, `SheetContent`, `SheetHeader`, `SheetTitle`, `SheetDescription`, `SheetClose` from `@/components/ui/sheet`. If the Sheet component does not yet exist, it must be added via `npx shadcn@latest add sheet` before implementing.
- Import `useConceptRelationships` from `@/features/ontology/api`.
- Import `useTranslation` from `react-i18next` with namespace `"analysis"`.

**Behavior:**

- The Sheet's `open` prop is `conceptId !== null`.
- The `onOpenChange` handler calls `onClose` when the Sheet is closed (click outside, X button, or Escape key).
- Call `useConceptRelationships(conceptId ?? "")`. The hook has an `enabled: !!id` guard internally, so passing empty string when `conceptId` is null safely skips the query.
- **Sheet configuration:** side `"right"`, width via className `w-[400px]`.

**Content sections (top to bottom):**

1. **Header:** SheetTitle with concept `name_en`. SheetDescription with concept `code` (if present).
2. **Metadata grid:** Two-column grid showing type (`concept_type`) and framework (`framework_id`).
3. **Definition:** Full text of `definition_en` in a paragraph.
4. **Related concepts:** If `related_concepts` array is non-empty, group by `relationship_type` and render each group as a labeled list. Each related concept shows its `concept_name_en` and `concept_framework_id`.
5. **Cross-framework mappings:** Filter `related_concepts` where the `concept_framework_id` differs from the main concept's `framework_id`. If any exist, show them in a separate section. This leverages the same data -- just a filtered view.
6. **Footer link:** An anchor styled as a button: "Open in Ontology Explorer" linking to `/ontology?concept={conceptId}` with `target="_blank"` and `rel="noopener noreferrer"`.

**States:**

- **Loading:** When `isLoading` is true, render skeleton placeholders (Tailwind `animate-pulse bg-muted rounded` divs) for the header, metadata, and definition areas.
- **Error:** When `isError` is true, show an error message using the i18n key `detail.conceptPanel.error` and a retry button that calls `refetch()` from the query result.

**i18n keys used** (will be added in section 6):
- `detail.conceptPanel.title`, `detail.conceptPanel.close`, `detail.conceptPanel.openInExplorer`
- `detail.conceptPanel.definition`, `detail.conceptPanel.type`, `detail.conceptPanel.framework`
- `detail.conceptPanel.relatedConcepts`, `detail.conceptPanel.crossMappings`
- `detail.conceptPanel.loading`, `detail.conceptPanel.error`, `detail.conceptPanel.retry`

### FindingsTable Changes

**File:** `src/features/analysis/components/FindingsTable.tsx` (modify)

**Props addition:**

Add an optional prop to `FindingsTableProps`:

```typescript
onConceptClick?: (conceptId: string) => void;
```

**Cell modifications:**

In the table body's concept name cell (currently `<TableCell>{finding.concept_name ?? "\u2014"}</TableCell>`), wrap the text in a `<button>` when both conditions are met:
- `onConceptClick` is provided
- `finding.concept_name` is non-null

The button should:
- Call `onConceptClick(finding.concept_id)` on click
- Have styling: `text-left hover:underline text-accent-foreground cursor-pointer`
- Be an inline element within the table cell

When `onConceptClick` is not provided or `concept_name` is null, render the cell as plain text (current behavior), preserving the dash fallback for null values.

Apply the same pattern to the concept code cell: wrap `finding.concept_code` in a clickable button when `onConceptClick` is provided and `concept_code` is non-null.

**Backward compatibility:** The prop is optional. When not provided, the component behaves exactly as before -- no buttons, no click handlers.

### Feature Index Export

**File:** `src/features/analysis/index.ts` (modify)

Add the export:

```typescript
export { ConceptDrawer } from "./components/ConceptDrawer";
```

### shadcn Sheet Prerequisite

The Sheet component from shadcn/ui is required but may not be installed yet. Before implementing, check if `src/components/ui/sheet.tsx` exists. If not, run:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/frontend && npx shadcn@latest add sheet
```

This generates the Sheet component using Radix Dialog primitives.

---

## Files Summary

| File | Action | Description |
|------|--------|-------------|
| `src/features/analysis/components/ConceptDrawer.tsx` | Create | New drawer component using shadcn Sheet |
| `src/features/analysis/components/__tests__/ConceptDrawer.test.tsx` | Create | Tests for drawer states and behavior |
| `src/features/analysis/components/FindingsTable.tsx` | Modify | Add `onConceptClick` prop, make concept cells clickable |
| `src/features/analysis/components/__tests__/FindingsTable.test.tsx` | Modify | Add concept click tests |
| `src/features/analysis/index.ts` | Modify | Export ConceptDrawer |
| `src/components/ui/sheet.tsx` | Create (if missing) | shadcn Sheet component (via CLI) |

---

## Dependencies

- **No dependency on sections 1-3.** This section can be implemented independently.
- **Section 5 (page assembly)** depends on this section. It will add `selectedConceptId` state to `$id.tsx` and wire `onConceptClick` from FindingsTable to `setSelectedConceptId`, and render `<ConceptDrawer>` at the page level.
- **Section 6 (i18n)** will add the translation keys referenced above. During development of this section, the `t()` calls will return their key strings (which is acceptable for testing since the mock returns keys).

---

## Key Type References

The `useConceptRelationships` hook (from `src/features/ontology/api/index.ts`) returns `ConceptWithRelationships`, which extends `Concept` with a `related_concepts: RelatedConcept[]` field.

**Concept fields used in the drawer:**
- `name_en: string` -- display name
- `code: string | null` -- concept code (e.g., "ID.AM-1")
- `concept_type: string` -- e.g., "subcategory", "function"
- `framework_id: string` -- parent framework
- `definition_en: string | null` -- full definition text

**RelatedConcept fields used:**
- `concept_name_en: string` -- related concept's name
- `concept_framework_id: string` -- which framework the related concept belongs to
- `relationship_type: string` -- e.g., "maps_to", "parent_of"
- `direction: "incoming" | "outgoing"` -- relationship direction

**AnalysisFinding fields relevant to FindingsTable changes:**
- `concept_id: string` -- always present, passed to `onConceptClick`
- `concept_code: string | null` -- displayed in code cell, clickable when non-null
- `concept_name: string | null` -- displayed in name cell, clickable when non-null