# Research: Analysis Frontend Foundation

## Codebase Research

### API Hook Patterns (from compliance feature)

**Query keys:** Hierarchical structure for nested invalidation:
```typescript
export const complianceKeys = {
  all: ["compliance"] as const,
  assessments: (filters?) => [...complianceKeys.all, "assessments", filters] as const,
  assessment: (id: string) => [...complianceKeys.all, "assessment", id] as const,
  score: (id: string) => [...complianceKeys.all, "assessment", id, "score"] as const,
};
```

**Mutations:** `useMutation` with `onSuccess` invalidating `queryClient.invalidateQueries({ queryKey: keys.all })`.

**Queries:** `useQuery` with `staleTime` (1min for lists, 5min for details), `enabled` for conditional queries, URLSearchParams for query params.

### Component Patterns

- **Lists:** Loading skeletons, error states, empty states with icons. Grid layout: `grid gap-4 sm:grid-cols-2 lg:grid-cols-3`
- **Cards:** `Link` wrapping for navigation, `Badge` for status, color coding by ranges
- **Dialogs:** Controlled via `open`/`onOpenChange` props, form reset on success, disabled during mutation
- **Filters:** Controlled state as props, `Select` with "all" option

### Type Definition Pattern

Union types for enums (`type Status = "draft" | "completed"`), interfaces for entities, separate request types, filter types with optional fields.

### Route Patterns

File-based with TanStack Router:
- `createFileRoute("/path/")({ component: Page })`
- Search params via `validateSearch` + `Route.useSearch()`
- Navigation: `<Link to="/path/$id" params={{ id }}>`

### i18n Setup

Namespace-per-feature. Import JSON files, register in `resources` object in `i18n/index.ts`. Usage: `const { t } = useTranslation("analysis")`.

### Testing Setup

**Vitest + React Testing Library.** jsdom environment. Setup file imports `@testing-library/jest-dom/vitest`.

Hook tests: `renderHook` with `QueryClientProvider` wrapper, `vi.mock("@/lib/api")`.
Component tests: `render` + `screen.findByTestId`, mock `react-i18next`.

### shadcn/ui Components Available

Installed: button, dialog, input, label, select, badge, card.
**NOT installed:** table, tabs, textarea, switch, dropdown-menu — will need to add.

### API Client

`axios.create({ baseURL: "/api" })` with 401 redirect interceptor. Import as `import { api } from "@/lib/api"`.

---

## Web Research

### TanStack Query File Upload Patterns

**Multipart upload with progress:**
```typescript
const useUploadFile = () => {
  const queryClient = useQueryClient();
  const [progress, setProgress] = useState(0);

  return {
    ...useMutation({
      mutationFn: async (file: File) => {
        const fd = new FormData();
        fd.append("file", file);
        return axios.post("/api/analyses/upload", fd, {
          headers: { "Content-Type": "multipart/form-data" },
          onUploadProgress: (ev) => setProgress(Math.round((ev.loaded * 100) / (ev.total ?? 1))),
        });
      },
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ["analysis"] });
        setProgress(0);
      },
    }),
    progress,
  };
};
```

**Key points:**
- `fetch()` doesn't support upload progress — use `axios`
- Validate file size client-side before uploading
- Return Promise from `onSuccess` to keep `isPending` until refetch completes

### Native Drag-and-Drop File Upload

**No library needed.** Key implementation details:

1. **Drag counter ref** to prevent flicker from child elements
2. **`e.preventDefault()` on dragOver** is mandatory for drop to fire
3. **Hidden `<input type="file">`** as keyboard-accessible fallback (accessibility requirement)
4. **File type validation:** PDF = `application/pdf`, DOCX = `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
5. Visual feedback: border color change on `isDragging` state

**Pattern:** `FileDropZone` component accepting `accept`, `maxSizeMB`, `onFilesAccepted`, `onError` props.

### shadcn/ui Data Table with Expandable Rows

**Two approaches:**

1. **Simple (Collapsible + Table):** Use `shadcn add table collapsible`. Wrap row pairs in `<Collapsible asChild>` with fragment. Good for read-only tables.

2. **Full-featured (TanStack Table):** Use `getExpandedRowModel()`, `row.toggleExpanded()`, `row.getIsExpanded()`. Composes with sorting (`getSortedRowModel`) and filtering (`getFilteredRowModel`).

**Recommendation for this project:** TanStack Table (Approach B) — we need sorting, filtering, pagination, AND expansion. The full TanStack Table API handles all of these composably.

**Key shadcn components to install:** `table`, `collapsible` (or just `table` if using TanStack Table directly).

---

## Testing Approach

Follow existing vitest + RTL patterns:
- Hook tests with `QueryClientProvider` wrapper and mocked `api`
- Component tests with mocked i18n and query providers
- Run with `pnpm test`
