I have all the context I need. Now I'll generate the section content.

# Section 1: Types and API Hooks

## Overview

This section creates the foundational TypeScript type definitions and TanStack Query hooks for the `analysis` feature module. These are the building blocks consumed by every other section in this implementation plan: the list page, create page, settings page, and eventually the detail page in split 02.

**Files to create:**
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/types/index.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/api/index.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/index.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/analysis/api/__tests__/hooks.test.ts`

**Dependencies:** None. This is a root section with no prerequisites.

**Blocked by this section:** All other sections (02 through 06) depend on these types and hooks.

---

## Tests First

Create the test file at `src/features/analysis/api/__tests__/hooks.test.ts`. Follow the existing test pattern from the ontology hooks tests: mock `@/lib/api` with `vi.mock`, create a `QueryClientProvider` wrapper with `retry: false`, and use `renderHook` + `waitFor`.

The test file should contain the following test stubs organized by hook:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";

vi.mock("@/lib/api", () => ({
  api: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}));

import { api } from "@/lib/api";
const mockedApi = vi.mocked(api);

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(
      QueryClientProvider,
      { client: queryClient },
      children
    );
  };
}
```

### useAnalyses

- **"returns paginated list on successful fetch"** -- Mock `api.get` to return a paginated response with sample `AnalysisListItem` objects. Verify the hook returns the data with correct structure.
- **"passes status filter as query param"** -- Call with `{ status: "completed" }` and verify `api.get` was called with a URL containing `status=completed`.
- **"refetchInterval activates when response contains processing items"** -- This is harder to test directly. Verify the hook configuration by checking that when data contains an item with `status: "processing"`, the query refetches. One approach: render the hook, verify `api.get` is called again within the polling window (use `vi.useFakeTimers`). Alternatively, test the interval callback logic in isolation.
- **"refetchInterval is false when no processing items"** -- Similar to above but with all items in `completed` status.

### useAnalysis

- **"returns analysis data for valid id"** -- Mock GET response, verify data shape.
- **"parses matched_framework_ids from JSON string to array"** -- The backend returns `matched_framework_ids` as a raw JSON string like `'["fw-1","fw-2"]'`. The hook must `JSON.parse` it into a string array. Mock the API to return the raw string format and verify the hook output is a parsed array.
- **"disabled when id is empty"** -- Render with `id = ""`, verify `api.get` was NOT called.

### useCreateAnalysis

- **"posts to /api/analyses with request body"** -- Mock `api.post`, trigger `mutate()`, verify the URL and body.
- **"invalidates analysis cache on success"** -- Use `queryClient.getQueryCache()` to verify invalidation. Or spy on `queryClient.invalidateQueries`.

### useUploadAnalysis

- **"posts FormData with file and name fields"** -- Trigger mutation with a `File` object and name. Verify `api.post` received FormData with correct fields and `Content-Type: multipart/form-data`.
- **"returns progress percentage during upload"** -- Mock `api.post` to invoke the `onUploadProgress` callback with `{ loaded: 50, total: 100 }`. Verify the returned `progress` value is `50`.
- **"resets progress on settled (both success and error)"** -- After mutation completes (success or error), verify progress is reset to `0`.
- **"invalidates analysis cache on success"** -- Verify cache invalidation on success.

### useDeleteAnalysis

- **"calls DELETE /api/analyses/{id}"** -- Trigger mutation, verify `api.delete` called with correct URL.
- **"invalidates analysis cache on success"** -- Verify cache invalidation.

### usePromptTemplate

- **"fetches from /api/analyses/prompt-template"** -- Verify correct endpoint hit.
- **"returns MatcherConfig-shaped data"** -- Mock response matching `MatcherConfig`, verify hook returns it.

### useUpdatePromptTemplate

- **"puts to /api/analyses/prompt-template"** -- Verify `api.put` called with correct URL and body.
- **"invalidates prompt-template cache on success"** -- Verify `analysisKeys.promptTemplate()` is invalidated.

---

## Implementation Details

### 1. Type Definitions (`src/features/analysis/types/index.ts`)

Define these TypeScript types matching the backend API response shapes exactly.

**Union types:**

- `AnalysisStatus` = `"pending" | "processing" | "completed" | "failed" | "deleted"`
- `InputType` = `"text" | "pdf" | "docx"`
- `FindingType` = `"addressed" | "partially_addressed" | "gap" | "not_applicable"`

**Core interfaces:**

`Analysis` -- the full entity returned by `GET /api/analyses/{id}`:

| Field | Type | Notes |
|-------|------|-------|
| id | string | |
| name | string | |
| description | string \| null | |
| input_type | InputType | |
| input_text | string \| null | |
| original_filename | string \| null | |
| file_path | string \| null | |
| extracted_text | string \| null | |
| status | AnalysisStatus | |
| error_message | string \| null | |
| prompt_template | string \| null | |
| matched_framework_ids | string[] | Backend returns as raw JSON string; parsed in hook |
| processing_time_ms | number \| null | |
| token_count | number \| null | |
| created_by | string \| null | |
| created_at | string | |
| updated_at | string | |

`AnalysisListItem` -- the subset returned by the list endpoint `GET /api/analyses`:

| Field | Type |
|-------|------|
| id | string |
| name | string |
| description | string \| null |
| input_type | InputType |
| status | AnalysisStatus |
| error_message | string \| null |
| processing_time_ms | number \| null |
| created_at | string |
| updated_at | string |

The list endpoint does NOT return `matched_framework_ids` or `token_count`.

`AnalysisFinding` -- individual finding with concept metadata:

| Field | Type |
|-------|------|
| id | string |
| concept_id | string |
| framework_id | string |
| finding_type | FindingType |
| confidence_score | number | (0-1) |
| evidence_text | string |
| recommendation | string |
| priority | number | (1-4) |
| sort_order | number |
| concept_code | string |
| concept_name | string |
| concept_definition | string |

`MatcherConfig`:

| Field | Type |
|-------|------|
| version | number |
| thresholds | `{ min_confidence: number; addressed: number; partial: number }` |
| max_findings_per_framework | number |
| include_addressed_findings | boolean |
| boost_terms | `Record<string, number>` |

**Request/param types:**

- `CreateAnalysisRequest` -- `{ name: string; description?: string; input_text: string }`
- `UploadAnalysisInput` -- `{ file: File; name: string }` (frontend-only type, not an API shape)
- `AnalysisListParams` -- `{ page?: number; limit?: number; status?: AnalysisStatus }`
- `FindingsListParams` -- `{ page?: number; limit?: number; framework_id?: string; finding_type?: FindingType; priority?: number; sort_by?: string }`

**Re-export:** Import and re-export `PaginatedResponse<T>` from `@/features/ontology/types` (already defined there with fields: `data`, `total`, `page`, `limit`, `total_pages`).

### 2. Query Keys (`src/features/analysis/api/index.ts`)

Hierarchical key structure for granular cache invalidation. Invalidating `analysisKeys.all` clears everything; invalidating `analysisKeys.detail(id)` clears just that analysis and its findings.

```typescript
export const analysisKeys = {
  all: ["analysis"] as const,
  list: (params?: AnalysisListParams) => [...analysisKeys.all, "list", params] as const,
  detail: (id: string) => [...analysisKeys.all, "detail", id] as const,
  findings: (id: string, params?: FindingsListParams) =>
    [...analysisKeys.all, "detail", id, "findings", params] as const,
  promptTemplate: () => [...analysisKeys.all, "prompt-template"] as const,
};
```

### 3. Hooks (`src/features/analysis/api/index.ts`)

All hooks live in the same file alongside query keys, following the compliance feature pattern. Import `api` from `@/lib/api` (an axios instance with `baseURL: "/api"`).

**`useAnalyses(params?: AnalysisListParams)`**

- Query key: `analysisKeys.list(params)`
- Query function: Build `URLSearchParams` from `params` (page, limit, status -- skip undefined values). Call `api.get<PaginatedResponse<AnalysisListItem>>("/analyses?...")`.
- `refetchInterval`: Use the callback form. Return `5000` if `data.data.some(item => item.status === "processing")`, otherwise return `false`.
- Stale time: not set (use default).

**`useAnalysis(id: string)`**

- Query key: `analysisKeys.detail(id)`
- Query function: Call `api.get<Analysis>("/analyses/" + id)`. After receiving the response, parse `matched_framework_ids`: if it is a string, `JSON.parse` it into `string[]`; if it is already an array, leave it. Return the modified object.
- `enabled: !!id`
- `staleTime: 5 * 60 * 1000` (5 minutes)

**`useCreateAnalysis()`**

- Mutation function: `api.post<Analysis>("/analyses", request)`
- `onSuccess`: Invalidate `analysisKeys.all` via `queryClient.invalidateQueries`.

**`useUploadAnalysis()`**

- Uses `useState<number>(0)` for upload progress.
- Mutation function: Build `FormData` with `file` and `name` fields. Call `api.post("/analyses/upload", formData, { headers: { "Content-Type": "multipart/form-data" }, onUploadProgress: (e) => setProgress(Math.round((e.loaded * 100) / (e.total ?? 1))) })`.
- `onSuccess`: Invalidate `analysisKeys.all`.
- `onSettled`: Reset progress to `0` (runs on both success and error).
- Return: `{ ...mutation, progress }` (spread the mutation return and add progress).

**`useDeleteAnalysis()`**

- Mutation function: `api.delete("/analyses/" + id)`
- `onSuccess`: Invalidate `analysisKeys.all`.

**`useFindings(id: string, params?: FindingsListParams)`**

- Query key: `analysisKeys.findings(id, params)`
- Query function: Build `URLSearchParams` from params (page, limit, framework_id, finding_type, priority, sort_by). Call `api.get<PaginatedResponse<AnalysisFinding>>("/analyses/" + id + "/findings?...")`.
- `enabled: !!id`

**`exportAnalysis(id: string, format: string = "pdf")`**

- This is a plain async function, NOT a hook. It does not use TanStack Query.
- Calls `api.get("/analyses/" + id + "/export/" + format, { responseType: "blob" })`.
- Creates a temporary `<a>` element, sets `href` to `URL.createObjectURL(blob)`, sets `download` attribute to `analysis-${id}.${format}`, triggers `.click()`, then calls `URL.revokeObjectURL`.

**`usePromptTemplate()`**

- Query key: `analysisKeys.promptTemplate()`
- Query function: `api.get<MatcherConfig>("/analyses/prompt-template")`
- `staleTime: 30 * 1000` (30 seconds)

**`useUpdatePromptTemplate()`**

- Mutation function: `api.put("/analyses/prompt-template", config)`
- `onSuccess`: Invalidate `analysisKeys.promptTemplate()`.

### 4. Feature Module Barrel (`src/features/analysis/index.ts`)

Re-export everything from `./types` and `./api`:

```typescript
export * from "./types";
export * from "./api";
```

---

## Key Implementation Notes

1. **API base path:** The axios instance at `@/lib/api` already has `baseURL: "/api"`, so hook URLs should be relative to that (e.g., `"/analyses"` not `"/api/analyses"`).

2. **`matched_framework_ids` parsing:** The backend `get_analysis` endpoint returns this field as a serialized JSON string (e.g., `"[\"iso-31000\",\"nist-csf\"]"`). The `useAnalysis` hook must handle both cases: if the value is a `string`, run `JSON.parse`; if it is already an array (e.g., in test mocks), pass through. Wrap in try/catch and default to `[]` on parse failure.

3. **PaginatedResponse reuse:** The `PaginatedResponse<T>` generic is already defined in `src/features/ontology/types/index.ts`. Re-export it from the analysis types file rather than redefining it: `export type { PaginatedResponse } from "@/features/ontology/types"`.

4. **Upload progress with useState:** The `useUploadAnalysis` hook is the one hook that uses React state internally (for progress tracking). This means the hook must be called from a React component -- it cannot be used outside the component lifecycle. The mutation result is spread with the progress value: `return { ...mutation, progress }`.

5. **Export utility function:** `exportAnalysis` is intentionally not a hook. Exports are one-shot user-triggered actions (click a button, download a file). Using a hook would be overkill and the blob/download logic does not benefit from caching or query state.

6. **Existing test patterns to follow:** See `src/features/ontology/api/__tests__/hooks.test.ts` for the established project pattern. Key elements: `vi.mock("@/lib/api")`, `vi.mocked(api)`, `createWrapper()` function returning a `QueryClientProvider`, `renderHook` with `waitFor`, `vi.resetAllMocks()` in `beforeEach`.