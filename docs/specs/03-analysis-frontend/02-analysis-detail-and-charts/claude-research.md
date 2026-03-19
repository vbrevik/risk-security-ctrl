# Research: Analysis Detail Page with Charts

## 1. Codebase Research

### 1.1 D3 Integration Pattern (useD3Graph)

**Location:** `frontend/src/features/ontology/hooks/useD3Graph.ts`

The existing hook demonstrates the production pattern:
- Uses `useRef` for SVG DOM and simulation references (not state)
- Manages D3 force simulation lifecycle with `useEffect` cleanup
- Returns control functions for imperative interactions (zoomIn, zoomOut, resetView, fitToScreen, panToNode)
- Cleanup: simulation stopped in `useEffect` cleanup, `svg.selectAll("*").remove()` at effect start
- Zoom behavior stored in ref for programmatic control
- Uses Sets for fast lookup of highlighted nodes
- Dependencies array includes all props affecting D3 rendering
- **D3 version: v7.9.0**

### 1.2 Analysis Feature Architecture

**Types** (`frontend/src/features/analysis/types/index.ts`):
```typescript
type AnalysisStatus = "pending" | "processing" | "completed" | "failed" | "deleted"
type InputType = "text" | "pdf" | "docx"
type FindingType = "addressed" | "partially_addressed" | "gap" | "not_applicable"

interface Analysis {
  id: string; name: string; description: string | null
  input_type: InputType; input_text: string | null
  original_filename: string | null; file_path: string | null
  extracted_text: string | null; status: AnalysisStatus
  error_message: string | null; prompt_template: string | null
  matched_framework_ids: string[]  // Parsed from JSON string by hook
  processing_time_ms: number | null; token_count: number | null
  created_by: string | null; created_at: string; updated_at: string
}

interface AnalysisFinding {
  id: string; concept_id: string; framework_id: string
  finding_type: FindingType; confidence_score: number
  evidence_text: string; recommendation: string
  priority: number; sort_order: number
  concept_code: string; concept_name: string; concept_definition: string
}
```

**API Hooks** (`frontend/src/features/analysis/api/index.ts`):
- `useAnalysis(id)` — single analysis, parses `matched_framework_ids`, `staleTime: 5min`, `enabled: !!id`
- `useFindings(id, params?)` — paginated findings with filters (`framework_id`, `finding_type`, `priority`, `sort_by`)
- `exportAnalysis(id, format)` — direct function (not hook), creates blob download via `URL.createObjectURL`
- Query key hierarchy: `analysisKeys.all → .list() → .detail(id) → .findings(id, params)`
- Auto-polling pattern: `useAnalyses` refetches when any item has `status === "processing"`

**Existing Components:**
- `StatusBadge` — maps status → color + icon, `animate-pulse` for processing
- `AnalysisCard` — link to `/analysis/$id`, shows metadata
- `AnalysisList` — grid layout with loading/error/empty states

### 1.3 Route Structure

```
/analysis/
  index.tsx      - List page
  create.tsx     - Create form
  settings.tsx   - Matcher config
  $id.tsx        - Detail page (STUB ready for implementation)
```

The `$id.tsx` stub already exists with `createFileRoute("/analysis/$id")` and `Route.useParams()` pattern.

### 1.4 shadcn/ui Components Available

| Component | Available | Use Case |
|-----------|-----------|----------|
| Table     | ✅ | Findings table |
| Tabs      | ✅ | Overview/Findings/Export sections |
| Badge     | ✅ | Finding type indicators |
| Card      | ✅ | Summary stats, chart containers |
| Button    | ✅ | Export, actions |
| Dialog    | ✅ | Potential expanded details |
| Select    | ✅ | Filter dropdowns |
| Input     | ✅ | Search |
| Textarea  | ✅ | Evidence display |
| Label     | ✅ | Form labels |

Table provides semantic components (`Table`, `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`) with responsive scroll. No built-in expand — use state + conditional rendering.

### 1.5 Backend API Responses

**GET /api/analyses/{id}/findings** query params: `?page=1&limit=50&framework_id=&finding_type=&priority=&sort_by=`

Response: `{ data: AnalysisFinding[], total, page, limit, total_pages }`

Backend SQL joins `analysis_findings` with `concepts` (LEFT JOIN — concept fields can be NULL).

**GET /api/analyses/{id}/export/{format}** — returns binary blob, only for `status == "completed"`.

**Backend charts.rs** — has PNG-generating functions (coverage heatmap, radar, priority chart) but no API endpoints. Frontend D3 is recommended for interactive charts.

### 1.6 Testing Patterns

- **Framework:** Vitest + jsdom + `@testing-library/jest-dom/vitest`
- **Hook tests:** `vi.mock("@/lib/api")`, QueryClient wrapper with `retry: false`, `renderHook` + `waitFor`
- **Component tests:** mock `react-i18next` returning keys, `render` + `screen` queries
- **Router integration:** Components using `<Link>` need router context in tests

### 1.7 i18n

Namespace: `"analysis"`, files at `frontend/src/i18n/locales/{en,nb}/analysis.json`. Existing keys include status labels and common actions. New detail page keys need to be added.

---

## 2. Web Research

### 2.1 D3.js + React Integration (2025)

**Two approaches:**

**A. D3 controls DOM (useRef + useEffect)** — D3 renders inside ref container. React owns lifecycle, D3 owns SVG subtree. Best for complex, interactive charts with transitions and axes.

**B. React renders SVG, D3 for math only** — React renders `<svg>`, `<rect>`, `<path>` via JSX. D3 used only for scales, layouts, path generators (`d3.scaleLinear`, `d3.line`, `d3.arc`). Cleaner React integration for simpler charts.

**Recommendation for this project:** Approach A (matches existing `useD3Graph` pattern) for the coverage heatmap, Approach B could work for simpler priority bars.

**Custom useD3 hook pattern:**
```tsx
function useD3(renderFn: (svg: d3.Selection) => void, deps: any[]) {
  const ref = useRef<SVGSVGElement>(null);
  useEffect(() => {
    if (ref.current) renderFn(d3.select(ref.current));
    return () => { if (ref.current) d3.select(ref.current).selectAll("*").remove(); };
  }, deps);
  return ref;
}
```

**Responsive charts — three techniques:**
1. `viewBox="0 0 width height"` on SVG for proportional scaling
2. `ResizeObserver` on parent container + re-render with updated dimensions
3. Debounce resize (100-200ms) to avoid performance issues

Best practice: combine `viewBox` for basic scaling + ResizeObserver for recalculating scales/ticks.

**Tooltips:** React-managed preferred — D3 mouse events call `setState`, React renders positioned `<div>`. Benefits: participates in React render cycle, can contain JSX, styled with Tailwind.

**SVG Accessibility:**
- `role="img"` on `<svg>` for static charts
- `<title>` + `<desc>` as first children, referenced via `aria-labelledby`
- `tabindex="0"` on interactive elements for keyboard nav
- Minimum 44x44px interactive hit areas

**Cleanup:** Always return cleanup from `useEffect` — remove DOM elements, cancel transitions, disconnect ResizeObserver, clear timers.

Sources: Pluralsight, DEV Community, MDN, Smashing Magazine, TPGI

### 2.2 Expandable Data Table Patterns (2025)

**Two approaches:**

**A. shadcn Collapsible + Table (simple)** — wrap each row group in `<Collapsible>`. Simple, minimal dependencies.

**B. TanStack Table + `getExpandedRowModel` (complex)** — full-featured with sorting, filtering, pagination alongside expansion:
```tsx
const table = useReactTable({
  data, columns,
  state: { expanded },
  onExpandedChange: setExpanded,
  getExpandedRowModel: getExpandedRowModel(),
});
```
Render expanded content by checking `row.getIsExpanded()` and adding a `<TableRow>` with `colSpan`.

**For this project:** Since we have server-side pagination/filtering via `useFindings`, we don't need TanStack Table's client-side models. A simpler approach with `useState<Set<string>>` for expanded IDs + shadcn Table components is cleaner:

```tsx
const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
// Toggle: add/remove from set
// Render: conditional TableRow after each main row when expanded
```

**Accessibility:**
- `aria-expanded="true/false"` on toggle button
- `aria-controls` pointing to expanded content ID
- `aria-label` on toggle buttons ("Expand/Collapse row details")
- Enter/Space to toggle expansion

**Performance:** Memoize column definitions, lazy-load expanded content if needed, virtualization with TanStack Virtual for 50,000+ rows (not needed here — analyses have <200 findings).

Sources: DEV Community, TanStack docs, shadcn/ui docs, MDN

### 2.3 Browser File Download from API Blob (2025)

**Standard pattern:**
```tsx
async function downloadFile(url: string, fallbackFilename: string) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`Download failed: ${response.status}`);
  const blob = await response.blob();
  const filename = parseFilename(response.headers.get("Content-Disposition")) ?? fallbackFilename;
  const blobUrl = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = blobUrl;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(blobUrl);
}
```

**Content-Disposition parsing:** Check `filename*` (RFC 5987, UTF-8) before `filename`. In CORS, server must expose the header.

**With axios:** `axios.get(url, { responseType: "blob" })` — auto-rejects on HTTP errors, response.data is already a Blob.

**Cleanup:** Always call `URL.revokeObjectURL()` after click — browser captures blob reference for download immediately.

**Error handling:** Check `response.ok` before `.blob()`, check `Content-Type` for JSON error responses vs expected file type, wrap in try/catch for network errors, use `AbortController` with timeout for large files.

Sources: MDN, code-boxx, javascript.info, Atlantbh

---

## 3. Key Design Decisions

| Decision | Recommendation | Rationale |
|----------|---------------|-----------|
| D3 integration approach | Approach A (useRef + useEffect) | Matches existing `useD3Graph` pattern, supports interactive tooltips |
| Chart responsiveness | viewBox + ResizeObserver | Handles both scaling and recalculation |
| Expandable table | useState Set + shadcn Table | Server-side pagination makes TanStack Table overkill |
| Export download | Existing `exportAnalysis` function | Already implemented in split 01 API hooks |
| Tooltips | React-managed state | Consistent with React patterns, Tailwind-stylable |
| Chart data | Fetch all findings (limit 1000) | Analyses have <200 findings typically |
