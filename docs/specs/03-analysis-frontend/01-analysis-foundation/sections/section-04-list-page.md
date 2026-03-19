Now I have all the context needed. Let me produce the section content.

# Section 4: Analysis List Page

## Overview

This section implements the analysis list page at `/analysis`, consisting of three components (`AnalysisList`, `AnalysisCard`, `StatusBadge`) and the route file at `src/routes/analysis/index.tsx`. The page shows a filterable, paginated card grid of analyses with auto-polling for in-progress items, plus empty, loading, and error states.

## Dependencies

- **section-01-types-and-hooks** must be complete: provides `AnalysisListItem`, `AnalysisStatus`, `AnalysisListParams`, `useAnalyses` hook, `PaginatedResponse<T>`
- **section-02-i18n-and-navigation** must be complete: provides the `analysis` i18n namespace with `list.*`, `status.*`, `common.*` keys, and route file stubs
- **section-03-shadcn-components** must be complete: shadcn table/tabs/textarea installed (this section does not use them directly but they must be present for the build)

## Files to Create/Modify

| File | Action |
|------|--------|
| `src/features/analysis/components/StatusBadge.tsx` | Create |
| `src/features/analysis/components/AnalysisCard.tsx` | Create |
| `src/features/analysis/components/AnalysisList.tsx` | Create |
| `src/routes/analysis/index.tsx` | Modify (replace stub from section-02) |
| `src/features/analysis/components/__tests__/StatusBadge.test.tsx` | Create |
| `src/features/analysis/components/__tests__/AnalysisCard.test.tsx` | Create |
| `src/features/analysis/components/__tests__/AnalysisList.test.tsx` | Create |
| `src/features/analysis/index.ts` | Modify (add re-exports for new components) |

All paths are relative to `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/`.

---

## Tests First

### StatusBadge tests

**File:** `src/features/analysis/components/__tests__/StatusBadge.test.tsx`

Test cases:

- **renders green badge for completed status** -- Render `<StatusBadge status="completed" />`, assert it contains translated text `status.completed` and applies a green color class (e.g. `bg-green-...`).
- **renders yellow badge with pulse for processing status** -- Render with `status="processing"`, assert the badge text is present and the element (or a child) has the `animate-pulse` class.
- **renders red badge for failed status** -- Render with `status="failed"`, assert destructive/red styling.
- **renders blue badge for pending status** -- Render with `status="pending"`, assert blue styling.

Mock `react-i18next` so `useTranslation("analysis")` returns a `t` function that echoes keys. Use a simple render helper (no router needed since StatusBadge has no links).

### AnalysisCard tests

**File:** `src/features/analysis/components/__tests__/AnalysisCard.test.tsx`

Test cases:

- **renders analysis name and status badge** -- Provide a mock `AnalysisListItem` with `name: "Test Analysis"`, `status: "completed"`. Assert the name text and a StatusBadge are present.
- **links to /analysis/{id}** -- Provide an item with `id: "abc-123"`. Assert the rendered `<a>` element has an `href` containing `/analysis/abc-123`.

These tests need a router wrapper. Follow the existing pattern from the project: create a `renderWithRouter` helper using `createRootRoute`, `createRoute`, `createRouter`, `createMemoryHistory`, and `RouterProvider` from `@tanstack/react-router`. Also mock `react-i18next`.

### AnalysisList tests

**File:** `src/features/analysis/components/__tests__/AnalysisList.test.tsx`

Test cases:

- **renders loading skeleton while fetching** -- Pass `isLoading: true`, `analyses: undefined`, `isError: false`. Assert at least one element with `animate-pulse` class is in the document.
- **renders analysis cards when data loads** -- Pass `isLoading: false`, `isError: false`, `analyses` as an array of two mock `AnalysisListItem` objects. Assert both analysis names appear.
- **renders empty state when no analyses exist** -- Pass `isLoading: false`, `isError: false`, `analyses: []`. Assert the empty state heading text (`list.empty.title` or the i18n key) appears.
- **renders error state with retry button on error** -- Pass `isLoading: false`, `isError: true`, `analyses: undefined`, `onRetry: vi.fn()`. Assert an error message and a retry button are rendered. Click the retry button, assert `onRetry` was called.

Use `renderWithRouter` helper (cards contain links). Mock `react-i18next`.

### List page route tests

**File:** `src/routes/analysis/__tests__/index.test.tsx`

Test cases:

- **renders page title and New Analysis button** -- Render the route component with mocked `useAnalyses` returning empty data. Assert the title (from i18n `list.title`) and a "New Analysis" button are present.
- **status filter changes URL search param** -- Render with mocked hook. Interact with the status Select dropdown, choose a specific status. Assert the hook was called with the expected status filter parameter (or that the URL search param updated).

These are lighter integration tests. Mock the `useAnalyses` hook at the module level and render the route with a full router setup.

---

## Implementation Details

### StatusBadge Component

**File:** `src/features/analysis/components/StatusBadge.tsx`

A small presentational component that maps `AnalysisStatus` to badge styling.

**Props interface:**
```typescript
interface StatusBadgeProps {
  status: AnalysisStatus;
}
```

**Status-to-style mapping:**

| Status | Color scheme | Extra |
|--------|-------------|-------|
| `pending` | Blue background (`bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300`) | None |
| `processing` | Yellow background (`bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300`) | Wrapping `<span>` with `animate-pulse` |
| `completed` | Green background (`bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300`) | None |
| `failed` | Red background (`bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300`) | None |
| `deleted` | Gray/secondary | None |

Use the existing `Badge` component from `@/components/ui/badge` as the base, passing a `className` override for the color. The text content is `t(`status.${status}`)` from the `analysis` i18n namespace.

The Badge component supports variants `default | secondary | destructive | outline | ghost | link`, but for fine-grained color control, override with `className` rather than using variants. Alternatively, use the `outline` variant as a neutral base and add color classes.

### AnalysisCard Component

**File:** `src/features/analysis/components/AnalysisCard.tsx`

Follows the exact same pattern as the existing `AssessmentCard` in the compliance feature.

**Props interface:**
```typescript
interface AnalysisCardProps {
  analysis: AnalysisListItem;
}
```

**Structure:**

- Outer element: `<Link to="/analysis/$id" params={{ id: analysis.id }}>` wrapping a `<Card>` with hover styling
- `CardHeader`: Row with `CardTitle` (analysis name, `text-base`) and `<StatusBadge status={analysis.status} />`
- `CardContent`: Optional description (line-clamped to 2 lines). Display `input_type` as a small label/pill. If `processing_time_ms` is set, show formatted time (e.g., "2.3s").
- `CardFooter`: Created date formatted via `toLocaleDateString()`. If `error_message` is present (for failed analyses), show truncated error text in red.

Import `Card, CardHeader, CardTitle, CardContent, CardFooter` from `@/components/ui/card`. Use `useTranslation("analysis")` for any labels.

### AnalysisList Component

**File:** `src/features/analysis/components/AnalysisList.tsx`

Follows the existing `AssessmentList` pattern closely.

**Props interface:**
```typescript
interface AnalysisListProps {
  analyses: AnalysisListItem[] | undefined;
  isLoading: boolean;
  isError: boolean;
  onRetry?: () => void;
}
```

**Three states:**

1. **Loading** (`isLoading` is true): Render 3 skeleton cards in the responsive grid (`grid gap-4 sm:grid-cols-2 lg:grid-cols-3`). Each skeleton is a `div` with `h-48 rounded-lg border bg-muted/50 animate-pulse`.

2. **Error** (`isError` is true): Centered column with destructive-colored error text from `t("common.error")` and a "Try again" `<Button>` that calls `onRetry`.

3. **Empty** (analyses array is empty): Centered column with a document icon (`FileText` from lucide-react), heading `t("list.empty.title")`, description `t("list.empty.description")`, and a `<Link to="/analysis/create">` styled as a button with text like `t("list.newAnalysis")`.

4. **Data**: Responsive grid of `<AnalysisCard>` components, one per `AnalysisListItem`, keyed by `analysis.id`.

### Route File: `/analysis` List Page

**File:** `src/routes/analysis/index.tsx`

This replaces the stub created in section-02.

**Route definition:**
```typescript
export const Route = createFileRoute("/analysis/")({
  component: AnalysisListPage,
  validateSearch: (search: Record<string, unknown>) => ({
    page: Number(search.page) || 1,
    status: (search.status as string) || undefined,
  }),
});
```

**Component behavior:**

1. Read search params via `Route.useSearch()` to get `page` and `status`.
2. Call `useAnalyses({ page, limit: 12, status })` to fetch paginated data.
3. Use `useNavigate()` from TanStack Router for updating search params.

**Layout structure:**

```
<div className="space-y-6 p-6">
  {/* Header row */}
  <div className="flex items-center justify-between">
    <h1>{t("list.title")}</h1>
    <div className="flex gap-2">
      <Link to="/analysis/settings">  {/* Settings icon button */}
      <Link to="/analysis/create">    {/* "New Analysis" button */}
    </div>
  </div>

  {/* Status filter */}
  <Select>  {/* All | Pending | Processing | Completed | Failed */}

  {/* Analysis list */}
  <AnalysisList
    analyses={data?.items}
    isLoading={isLoading}
    isError={isError}
    onRetry={refetch}
  />

  {/* Pagination */}
  {data && data.total_pages > 1 && (
    <Pagination controls>
  )}
</div>
```

**Status filter:** Use the `Select` component from `@/components/ui/select` (`Select, SelectTrigger, SelectValue, SelectContent, SelectItem`). Options: "All" (value `""`), then each status value. On change, navigate with `navigate({ search: { status: value || undefined, page: 1 } })` to reset to page 1 when filter changes.

**Pagination:** Simple previous/next with page display. Two `<Button>` elements (Previous, Next) with disabled states at boundaries. Text showing "Page X of Y" between them. On click, navigate with updated `page` search param, preserving the current `status` filter.

**Auto-polling:** The `useAnalyses` hook (from section-01) already implements conditional `refetchInterval` -- it returns `5000` when any item has `status === "processing"`, `false` otherwise. No additional polling logic is needed in the route component.

### Re-exports

**File:** `src/features/analysis/index.ts`

Add the three new components to the barrel export:

```typescript
export { StatusBadge } from "./components/StatusBadge";
export { AnalysisCard } from "./components/AnalysisCard";
export { AnalysisList } from "./components/AnalysisList";
```

---

## Edge Cases

- **No analyses exist:** The empty state with a "Create your first analysis" button linking to `/analysis/create` handles this.
- **All items completed, then new processing item appears:** Auto-polling activates automatically because the hook checks the response data on each fetch.
- **Failed analysis in the list:** The card shows the `error_message` (truncated) and a red status badge. No special handling beyond display.
- **Invalid page/status in URL:** The `validateSearch` function defaults `page` to 1 and `status` to undefined, so bad URL params degrade gracefully.
- **Pagination beyond range:** The API returns empty `items` array; the empty state renders. The pagination controls are only shown when `total_pages > 1`.