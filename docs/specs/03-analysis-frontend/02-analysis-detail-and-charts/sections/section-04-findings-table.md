Now I have all the context needed. Let me produce the section content.

# Section 04: Findings Table with Filters

## Overview

This section creates the findings table area of the analysis detail page. It includes three sub-components:

1. **FindingTypeTag** -- a small colored badge mapping each `FindingType` to a visual variant
2. **FindingsTable** -- a shadcn Table with expandable rows, null-field fallbacks, and column headers
3. **FindingsFilters** -- three Select dropdowns (framework, finding type, priority) rendered above the table
4. **Pagination controls** -- "Page X of Y" with Previous/Next buttons below the table

All filter/pagination/expansion state is managed by the parent (the route component in section-06). This section builds the presentational components that accept state and callbacks as props.

## Dependencies

- **Section 01 (prerequisites-and-i18n):** Must be completed first. This section relies on:
  - Nullable `AnalysisFinding` fields (`evidence_text`, `recommendation`, `concept_code`, `concept_name`, `concept_definition` as `string | null`)
  - i18n keys under `findings.*` namespace in `en/analysis.json` and `nb/analysis.json`
  - `PaginatedResponse` using `.items` (not `.data`)
- **Section 06 (page-assembly):** Consumes these components; does not need to exist yet

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/analysis/components/FindingTypeTag.tsx` | Colored badge for finding type |
| `frontend/src/features/analysis/components/FindingsTable.tsx` | Table with expandable rows, filters, pagination |
| `frontend/src/features/analysis/components/__tests__/FindingTypeTag.test.tsx` | Tests for FindingTypeTag |
| `frontend/src/features/analysis/components/__tests__/FindingsTable.test.tsx` | Tests for FindingsTable |

## Types Reference

The `AnalysisFinding` interface (after section-01 fixes) has these fields relevant to this section:

```ts
interface AnalysisFinding {
  id: string;
  concept_id: string;
  framework_id: string;
  finding_type: FindingType; // "addressed" | "partially_addressed" | "gap" | "not_applicable"
  confidence_score: number;  // 0-1 float
  evidence_text: string | null;
  recommendation: string | null;
  priority: number;          // 1-4
  sort_order: number;
  concept_code: string | null;
  concept_name: string | null;
  concept_definition: string | null;
}
```

The `FindingsListParams` interface drives filter state:

```ts
interface FindingsListParams {
  page?: number;
  limit?: number;
  framework_id?: string;
  finding_type?: FindingType;
  priority?: number;
  sort_by?: string;
}
```

## Tests

Write tests before implementation. Use Vitest + React Testing Library. Mock i18n with `vi.mock` returning the translation key as the rendered text.

### FindingTypeTag tests (`__tests__/FindingTypeTag.test.tsx`)

```ts
/**
 * FindingTypeTag component tests
 *
 * - Renders green badge for "addressed" type
 * - Renders yellow badge for "partially_addressed" type
 * - Renders red badge for "gap" type
 * - Renders gray badge for "not_applicable" type
 * - Displays i18n label for each type (key: findings.type.{finding_type})
 */
```

Test strategy: render the component with each `FindingType` value, assert that the badge element is present with the correct translated label text. For color variants, assert the presence of a distinguishing CSS class or data attribute (e.g., `"destructive"` variant for gap, `"secondary"` for not_applicable). Do not assert exact hex colors.

### FindingsTable tests (`__tests__/FindingsTable.test.tsx`)

```ts
/**
 * FindingsTable component tests
 *
 * Column rendering:
 * - Renders table with correct column headers (Expand, Concept Code, Concept Name, Framework, Type, Priority, Confidence)
 * - Renders a row for each finding in the findings array
 *
 * Null fallbacks:
 * - Displays concept_code or dash fallback when null
 * - Displays concept_name or dash fallback when null
 *
 * Formatting:
 * - Displays confidence as percentage (e.g., 0.85 -> "85%")
 *
 * Row expansion:
 * - Expand button toggles row expansion via onToggleExpand callback
 * - Expanded row shows evidence text (or dash for null)
 * - Expanded row shows recommendation text (or dash for null)
 * - Expanded row shows concept definition (or dash for null)
 * - Expand button has aria-expanded attribute
 * - Multiple rows can be expanded simultaneously (controlled by expandedIds Set)
 * - Collapsing a row hides expanded content
 *
 * Filters (inline or sub-component):
 * - Renders three Select dropdowns (framework, type, priority)
 * - Framework dropdown populated from frameworkIds prop
 * - Selecting a filter calls onFilterChange with updated params
 * - "All" option clears the filter value (sets to undefined)
 *
 * Pagination:
 * - Displays "Page X of Y" text
 * - Previous button disabled on page 1
 * - Next button disabled on last page
 * - Clicking Next calls onPageChange with page + 1
 * - Clicking Previous calls onPageChange with page - 1
 */
```

Test setup: create a helper function that builds mock `AnalysisFinding` objects with configurable null fields. Wrap renders in the i18n mock. For filter tests, provide `frameworkIds` as `["iso-31000", "nist-csf"]` and assert that the select options include these values plus an "All" option.

## Implementation Details

### FindingTypeTag Component

**File:** `frontend/src/features/analysis/components/FindingTypeTag.tsx`

Props interface:

```ts
interface FindingTypeTagProps {
  type: FindingType;
}
```

Implementation notes:
- Use the shadcn `Badge` component (already installed at `@/components/ui/badge`)
- Map each `FindingType` to a Badge variant or className:
  - `"addressed"` -- green: use `className="bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"` or a custom variant
  - `"partially_addressed"` -- yellow/amber warning style
  - `"gap"` -- use the `"destructive"` variant (red)
  - `"not_applicable"` -- use the `"secondary"` variant (gray)
- Label text comes from i18n: `t("findings.type.addressed")`, `t("findings.type.partially_addressed")`, etc.
- Use `useTranslation("analysis")` hook

### FindingsTable Component

**File:** `frontend/src/features/analysis/components/FindingsTable.tsx`

Props interface:

```ts
interface FindingsTableProps {
  findings: AnalysisFinding[];
  expandedIds: Set<string>;
  onToggleExpand: (id: string) => void;
  // Filter props
  frameworkIds: string[];
  filters: {
    framework_id?: string;
    finding_type?: FindingType;
    priority?: number;
  };
  onFilterChange: (filters: {
    framework_id?: string;
    finding_type?: FindingType;
    priority?: number;
  }) => void;
  // Pagination props
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}
```

Implementation notes:

**Filters section** (rendered above the table):
- Three shadcn `Select` components in a `flex flex-wrap gap-4` row
- Framework select: options from `frameworkIds` prop plus an "All Frameworks" default. Value is `filters.framework_id` or empty string for "All". On change, call `onFilterChange({ ...filters, framework_id: value || undefined })`.
- Finding type select: four options (`addressed`, `partially_addressed`, `gap`, `not_applicable`) plus "All Types". Labels from i18n.
- Priority select: options for P1 through P4 (values 1-4) plus "All Priorities". Note: the backend currently ignores this filter param, but the frontend passes it through. This is a known limitation documented in the plan.
- All filter labels from i18n keys under `findings.filters.*`

**Table structure:**
- Use shadcn `Table`, `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell` from `@/components/ui/table`
- Column headers (static, no sorting): expand toggle (narrow, no label), Concept Code, Concept Name, Framework, Type, Priority, Confidence
- Header labels from i18n keys: `findings.columns.conceptCode`, `findings.columns.conceptName`, etc.

**Row rendering:**
- Iterate `findings` array, rendering one `TableRow` per finding
- Cells:
  - **Expand toggle:** A `Button` (variant `"ghost"`, size `"sm"`) containing a chevron icon (e.g., `ChevronRight` from lucide-react). When the finding ID is in `expandedIds`, rotate the chevron 90 degrees via `className="rotate-90 transition-transform"`. The button has `aria-expanded={expandedIds.has(finding.id)}` and `aria-controls={`finding-detail-${finding.id}`}`. On click, calls `onToggleExpand(finding.id)`.
  - **Concept code:** `finding.concept_code ?? "\u2014"` (em-dash fallback for null)
  - **Concept name:** `finding.concept_name ?? "\u2014"`
  - **Framework:** `finding.framework_id`
  - **Type:** render `<FindingTypeTag type={finding.finding_type} />`
  - **Priority:** display as `P${finding.priority}` (e.g., "P1", "P2")
  - **Confidence:** format as percentage: `${Math.round(finding.confidence_score * 100)}%`

**Expanded row:**
- Conditionally rendered when `expandedIds.has(finding.id)` is true
- A `TableRow` immediately after the main row, with `id={`finding-detail-${finding.id}`}`
- Contains a single `TableCell` with `colSpan={7}`
- Styled with `bg-muted` background and padding
- Content organized as labeled blocks:
  - **Evidence:** heading label from `t("findings.evidence")`, then `finding.evidence_text ?? "\u2014"`
  - **Recommendation:** heading label from `t("findings.recommendation")`, then `finding.recommendation ?? "\u2014"`
  - **Concept Definition:** heading label from `t("findings.conceptDefinition")`, then `finding.concept_definition ?? "\u2014"`
  - **Source Reference:** only shown if `finding.concept_code` is not null. Label from `t("findings.sourceReference")`, displays the concept code as a reference identifier.

**Pagination section** (rendered below the table):
- A flex row with justify-between alignment
- Left: "Page X of Y" text (from i18n or inline template)
- Right: two `Button` components (variant `"outline"`)
  - Previous: disabled when `page === 1`, onClick calls `onPageChange(page - 1)`
  - Next: disabled when `page >= totalPages`, onClick calls `onPageChange(page + 1)`

### i18n Keys Used

This section consumes these keys (defined in section-01). Listed here for implementer reference:

```
findings.title
findings.filters.framework
findings.filters.allFrameworks
findings.filters.type
findings.filters.allTypes
findings.filters.priority
findings.filters.allPriorities
findings.columns.conceptCode
findings.columns.conceptName
findings.columns.framework
findings.columns.type
findings.columns.priority
findings.columns.confidence
findings.expand
findings.collapse
findings.evidence
findings.recommendation
findings.conceptDefinition
findings.sourceReference
findings.type.addressed
findings.type.partially_addressed
findings.type.gap
findings.type.not_applicable
```

## Edge Cases

| Case | Handling |
|------|---------|
| Null `concept_code`, `concept_name`, `concept_definition` | Display em-dash ("\u2014") fallback |
| Null `evidence_text` or `recommendation` | Display em-dash in expanded row |
| `concept_code` is null in expanded row | Hide the "Source Reference" block entirely |
| Empty findings array | Table renders with headers but no rows (the parent page handles showing `EmptyFindings` instead of this component when there are zero total findings) |
| `frameworkIds` is empty | Framework filter select shows only the "All Frameworks" option |
| Priority filter (backend limitation) | The dropdown works in the UI and the value is passed to the API query params, but the backend currently ignores the `priority` filter. This is a known limitation. |
| Page 1 of 1 | Both Previous and Next buttons are disabled |
| Filter change from parent | Parent resets page to 1 when any filter changes (handled in section-06, not in this component) |
| Very long evidence/recommendation text | Allow text to wrap naturally in expanded row. No truncation. |
| Confidence score of 0 or 1 | Displays as "0%" or "100%" respectively |

## Implementation Checklist

1. Write `FindingTypeTag.test.tsx` with all test stubs
2. Implement `FindingTypeTag.tsx` -- map FindingType to Badge variants with i18n labels
3. Write `FindingsTable.test.tsx` with all test stubs (columns, null fallbacks, expansion, filters, pagination)
4. Implement `FindingsTable.tsx` -- filters section, table with expandable rows, pagination controls
5. Run `pnpm test` from `frontend/` to verify all tests pass
6. Run `pnpm typecheck` from `frontend/` to verify no type errors