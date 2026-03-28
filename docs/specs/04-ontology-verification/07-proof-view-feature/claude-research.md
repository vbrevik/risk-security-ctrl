# Research Findings: Split 07 — Proof View Feature

---

## 1. Codebase Research

### TanStack Query Patterns (`features/ontology/api/index.ts`)

**Query key structure** — hierarchical, nested:
```typescript
ontologyKeys.framework(id) = ["ontology", "frameworks", id]
ontologyKeys.concept(id)   = ["ontology", "concept", id]
```
New proof key should follow: `ontologyKeys.proof(id) = ["ontology", "frameworks", id, "proof"]`

**Hook conventions:**
- Conditional fetching via `enabled: !!id`
- `staleTime: Infinity` for immutable/static data (frameworks, topics)
- Complex hooks return `{ data, isLoading, errors }` shapes
- No error display in hooks — delegated to components

**v5 note:** `skipToken` is the TypeScript-idiomatic way to disable a query when the argument is nullable (avoids the `enabled: !!id` + `!` assertion combo). Use it for the proof hook.

### Type Definitions (`features/ontology/types/`)

The `Framework` type has **no verification fields**:
```typescript
interface Framework {
  id: string; name: string; version: string | null;
  description: string | null; source_url: string | null;
  created_at: string; updated_at: string;
}
```
New `FrameworkProof` type needed (separate from Framework, returned by the proof endpoint):
```typescript
interface FrameworkProof {
  framework_id: string;
  verification_status: VerificationStatus;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
  proof_content: string | null;  // raw markdown from proof file
}

type VerificationStatus =
  | "verified"
  | "partially-verified"
  | "structure-verified"
  | "corrected"
  | "unverified"
  | "needs-correction";
```

### FrameworkProfile Component

**Current structure** (scrollable sections, no tabs):
1. Header (name, version, description, source link)
2. Stats Strip (4-column grid)
3. Concept Type Breakdown (bar chart)
4. Cross-Framework Connections (list)
5. Concept Hierarchy Preview (expandable tree)

**Collapsible pattern** uses `useState<Set<string>>()`. New proof section can follow same pattern — collapsible with a toggle button — or a dedicated button that shows/hides a `ProofPanel` below the header.

**Preferred integration point:** After the stats strip (high visibility), before the type breakdown. A "Verification" row in the stats area + a "View Proof" expandable section.

### Status Badge Patterns (`features/analysis/components/StatusBadge.tsx`)

```typescript
const statusStyles: Record<AnalysisStatus, string> = {
  pending:    "bg-blue-100 text-blue-800",
  processing: "bg-yellow-100 text-yellow-800",
  completed:  "bg-green-100 text-green-800",
  failed:     "bg-red-100 text-red-800",
};
```
Uses `Badge variant="outline"` from shadcn/ui. Follow this exact pattern for `VerificationBadge`.

### Testing Setup

- **Framework:** Vitest v4 + @testing-library/react v16
- **Hook tests:** `renderHook(() => useHook(), { wrapper: createWrapper() })` where wrapper provides `QueryClientProvider`
- **Component tests:** `render()` + `screen.getBy*` + `data-testid` attributes
- **Mocking:** `vi.mock("@/lib/api", () => ({ api: { get: vi.fn() } }))`
- `FrameworkProfile.test.tsx` exists and tests loading states, stats boxes, empty states

### Dependencies — Notable Absences

- No `react-markdown` installed — **needs to be added**
- No `remark-gfm` installed — **needs to be added**
- All standard shadcn/ui components present
- TanStack Query v5 present

### i18n Patterns

Namespace `"ontology"` with dot-notation nesting. New keys should go under `"proof"`:
```json
{
  "proof": {
    "title": "Verification Proof",
    "viewProof": "View Proof",
    "hideProof": "Hide Proof",
    "status": {
      "verified": "Verified",
      "partially-verified": "Partially Verified",
      "structure-verified": "Structure Verified",
      "corrected": "Corrected",
      "unverified": "Unverified",
      "needs-correction": "Needs Correction"
    },
    "date": "Verified on",
    "source": "Source",
    "notes": "Notes",
    "noProof": "No proof file available"
  }
}
```
Same keys needed in `nb/ontology.json`.

---

## 2. Markdown Rendering Research

### react-markdown Security Model

`react-markdown` builds a virtual DOM from a syntax tree — it does NOT use string-based HTML injection. Raw HTML in markdown is silently dropped (not rendered) unless you explicitly opt in with `rehype-raw`. The `defaultUrlTransform` blocks `javascript:` and `data:` protocol attacks in links.

**For proof files (server-authored markdown, not user input):** The safest pattern is:
```tsx
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

// Raw HTML in markdown is dropped by default — safe without any extra plugins
<ReactMarkdown remarkPlugins={[remarkGfm]}>{proofContent}</ReactMarkdown>
```

No raw HTML plugins needed — raw HTML in proof files will be escaped/stripped, which is acceptable. Proof files contain headings, lists, links, and code blocks — all handled natively.

**Packages to add:**
- `react-markdown` (v10.x, ESM-compatible with Vite)
- `remark-gfm` (GitHub Flavored Markdown — tables, checkboxes)

If embedded HTML support is ever needed in the future, add `rehype-raw` + `rehype-sanitize` with `defaultSchema` from `hast-util-sanitize`. Plugin ordering is critical: `rehype-raw` must come before `rehype-sanitize`.

### Lighter Alternatives Evaluated

| Library | Size | Verdict |
|---------|------|---------|
| `react-markdown` | ~150KB | Recommended — safe virtual DOM model, Vite-compatible |
| `marked` + DOMPurify | ~50KB | Requires unsafe string injection + manual sanitization |
| `markdown-it` | ~60KB | Not React-native |

`react-markdown` is the right choice.

---

## 3. TanStack Query v5 On-Demand Fetch Patterns

### Recommended: `skipToken` + panel state

`skipToken` is the v5 TypeScript-idiomatic way to disable a query when input is nullable:
```typescript
import { skipToken } from '@tanstack/react-query';

export function useFrameworkProof(frameworkId: string | null) {
  return useQuery({
    queryKey: ontologyKeys.proof(frameworkId ?? ""),
    queryFn: frameworkId
      ? () => api.get<FrameworkProof>(`/ontology/frameworks/${frameworkId}/proof`).then(r => r.data)
      : skipToken,
    staleTime: Infinity,  // proof data is immutable once verified
  });
}
```

Key v5 behaviors:
- When `skipToken`: `status === 'pending'`, `fetchStatus === 'idle'` — query is idle, not loading
- `isFetching` is true during active network activity (useful for showing a spinner on re-open)
- Do NOT use `isLoading` to show spinner for a disabled query — `isLoading` is only true when `fetchStatus === 'fetching'`
- `skipToken` disables `refetch()` — if imperative refetch is needed, use `enabled: false` instead

### Integration with FrameworkProfile

Proof data should be fetched when the framework is selected (not deferred further). The `frameworkId` prop is already available in `FrameworkProfile`. The hook fires when `frameworkId` is non-null and caches indefinitely.

---

## 4. Verification Status Badge Color Conventions

From industry standards (Carbon/IBM, Spectrum/Adobe, GitHub):

| Status | Color | Tailwind classes |
|--------|-------|-----------------|
| `verified` | Green | `bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200` |
| `corrected` | Green (lighter) | `bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300` |
| `partially-verified` | Amber | `bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200` |
| `structure-verified` | Blue | `bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200` |
| `unverified` | Gray | `bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400` |
| `needs-correction` | Red | `bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200` |

**Accessibility (WCAG 1.4.1):** Never convey status by color alone. Always pair with:
1. A distinct icon shape (check, triangle, dash, x) via `aria-hidden="true"`
2. A text label
3. `aria-label` or `className="sr-only"` text for screen readers

---

## 5. Testing Approach

**Hook test** (`useFrameworkProof` in `api/__tests__/hooks.test.ts`):
```typescript
const { result } = renderHook(
  () => useFrameworkProof("nist-csf"),
  { wrapper: createWrapper() }
);
await waitFor(() => expect(result.current.isLoading).toBe(false));
expect(result.current.data?.verification_status).toBe("verified");
```

**Component tests** for `VerificationBadge`:
- Each status variant renders correct color class and label
- `aria-label` is present

**Component tests** for `ProofPanel`:
- Loading state renders skeleton
- Error state renders error message
- Proof content renders (check for heading text)
- "No proof file" state renders fallback

Extend `FrameworkProfile.test.tsx`:
- Verification badge renders in profile header area
- "View Proof" button toggles proof panel visibility
