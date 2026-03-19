# Implementation Plan: Analysis Frontend Foundation

## What We're Building

A new `analysis` feature module in a React 19 + TypeScript frontend application. This module provides the foundational layer for document analysis: TypeScript types, API hooks via TanStack Query v5, i18n translations (English + Norwegian), navigation integration, and three pages — an analysis list with auto-polling, a create page with text/file upload (including drag-and-drop with progress bar), and a matcher settings page with an inline key-value editor for boost terms.

This is the first of two frontend splits. The second split (02-analysis-detail-and-charts) builds the analysis detail page with interactive charts and an expandable findings table, consuming the types and hooks created here.

## Architecture Overview

### Feature Module Layout

```
src/features/analysis/
├── api/index.ts              # Query keys + all hooks
├── types/index.ts            # TypeScript interfaces
├── components/
│   ├── AnalysisList.tsx      # Card grid with pagination + status filter
│   ├── AnalysisCard.tsx      # Individual card with status badge
│   ├── StatusBadge.tsx       # Color-coded analysis status
│   ├── CreateAnalysisForm.tsx # Tabbed form (text + upload)
│   ├── FileDropZone.tsx      # Drag-and-drop upload area
│   ├── SettingsForm.tsx      # MatcherConfig editor
│   └── BoostTermsEditor.tsx  # Key-value pair row editor
└── index.ts                  # Re-exports

src/routes/analysis/
├── index.tsx                 # /analysis (list page)
├── create.tsx                # /analysis/create
├── settings.tsx              # /analysis/settings
└── $id.tsx                   # /analysis/{id} (placeholder for split 02)

src/i18n/locales/en/analysis.json
src/i18n/locales/nb/analysis.json
```

### Patterns to Follow

This project follows established patterns from the existing `compliance` feature module:

- **API hooks:** TanStack Query with hierarchical query keys (`["analysis", "list", params]`), mutations with `onSuccess` cache invalidation via `queryClient.invalidateQueries({ queryKey: analysisKeys.all })`
- **Components:** shadcn/ui primitives (Card, Badge, Button, Input, Select, Label), responsive grid layouts (`grid gap-4 sm:grid-cols-2 lg:grid-cols-3`)
- **Routes:** TanStack Router file-based routing with `createFileRoute`, `Route.useSearch()` for query params, `<Link>` for navigation
- **i18n:** Namespace per feature, `useTranslation("analysis")`, structured JSON with nested keys
- **Testing:** Vitest + React Testing Library, mock `@/lib/api`, `QueryClientProvider` wrapper for hook tests
- **API client:** `import { api } from "@/lib/api"` (axios with `baseURL: "/api"`)

---

## Section 1: Types and API Hooks

### Types

Define all TypeScript interfaces in `features/analysis/types/index.ts`. These must match the backend API response shapes exactly.

**Core types:**
- `AnalysisStatus` — union type: `"pending" | "processing" | "completed" | "failed" | "deleted"`
- `InputType` — union type: `"text" | "pdf" | "docx"`
- `FindingType` — union type: `"addressed" | "partially_addressed" | "gap" | "not_applicable"`
- `Analysis` — FULL entity matching backend: id, name, description (nullable), input_type, input_text (nullable), original_filename (nullable), file_path (nullable), extracted_text (nullable), status, error_message (nullable), prompt_template (nullable), matched_framework_ids (string array — NOTE: backend `get_analysis` returns this as a raw JSON string, must JSON.parse in the hook), processing_time_ms (nullable number), token_count (nullable number), created_by (nullable), timestamps
- `AnalysisListItem` — SUBSET matching the list endpoint's actual response: id, name, description (nullable), input_type, status, error_message (nullable), processing_time_ms (nullable number), created_at, updated_at. The list endpoint does NOT return matched_framework_ids or token_count.
- `AnalysisFinding` — finding with concept metadata: id, concept_id, framework_id, finding_type, confidence_score (0-1), evidence_text, recommendation, priority (1-4), sort_order, concept_code, concept_name, concept_definition
- `MatcherConfig` — version, thresholds (min_confidence, addressed, partial), max_findings_per_framework, include_addressed_findings boolean, boost_terms `Record<string, number>`
- `CreateAnalysisRequest` — name (required), description (optional), input_text (required). Note: backend also accepts prompt_template but we intentionally omit it from the create form.
- `UploadAnalysisInput` — file (File), name (string). Description is NOT supported by the upload endpoint.
- `AnalysisListParams` — page, limit, status filter (all optional)
- `FindingsListParams` — page, limit, framework_id, finding_type, priority, sort_by (all optional)
- Re-export `PaginatedResponse<T>` from ontology types (already defined there with items, total, page, limit, total_pages)

### Query Keys

Hierarchical structure enabling granular cache invalidation:

```typescript
const analysisKeys = {
  all: ["analysis"] as const,
  list: (params?: AnalysisListParams) => [...analysisKeys.all, "list", params] as const,
  detail: (id: string) => [...analysisKeys.all, "detail", id] as const,
  findings: (id: string, params?: FindingsListParams) => [...analysisKeys.all, "detail", id, "findings", params] as const,
  promptTemplate: () => [...analysisKeys.all, "prompt-template"] as const,
};
```

### Hooks

**`useAnalyses(params)`** — Paginated list. Uses `refetchInterval` that activates when any item in the response has `status === "processing"` (conditional polling via the `refetchInterval` callback form: return `5000` if processing items exist, `false` otherwise).

**`useAnalysis(id)`** — Single analysis detail. `enabled: !!id`. Stale time 5 minutes.

**`useCreateAnalysis()`** — Mutation posting `CreateAnalysisRequest` to `POST /api/analyses`. On success, invalidate `analysisKeys.all`.

**`useUploadAnalysis()`** — Mutation handling multipart upload. Takes `UploadAnalysisInput { file: File; name: string }`. Internally manages a `useState<number>(0)` for progress. The `mutationFn` builds `FormData` with `file` and `name` fields, posts to `POST /api/analyses/upload` with `Content-Type: multipart/form-data` and `onUploadProgress` callback updating the progress state. Returns `{ ...mutation, progress }`. Use `onSettled` (not just `onSuccess`) to reset progress on both success and error. On success, also invalidate cache.

**`useDeleteAnalysis()`** — Mutation calling `DELETE /api/analyses/{id}`. On success, invalidate `analysisKeys.all`.

**`useFindings(id, params)`** — Paginated findings with filters. `enabled: !!id`. Builds URL params from FindingsListParams.

**`useExportAnalysis()`** — Not a TanStack Query hook. A plain async function that calls `api.get` with `responseType: "blob"`, creates a temporary `<a>` element with `URL.createObjectURL`, triggers click for download, then revokes the URL. This is a utility function, not a hook, because exports are user-triggered one-shot actions.

**`usePromptTemplate()`** — Query for GET `/api/analyses/prompt-template`. Stale time 30 seconds.

**`useUpdatePromptTemplate()`** — Mutation for PUT `/api/analyses/prompt-template`. On success, invalidate `analysisKeys.promptTemplate()`.

---

## Section 2: i18n and Navigation

### Translation Files

Create `src/i18n/locales/en/analysis.json` and `src/i18n/locales/nb/analysis.json` with keys organized by page:

**Top-level keys:** `title` ("Document Analysis" / "Dokumentanalyse")

**`list.*`:** title, newAnalysis, empty (title + description), filters (status label, all option)

**`status.*`:** pending, processing, completed, failed — used by StatusBadge

**`create.*`:** title, nameLabel, namePlaceholder, descriptionLabel, textTab, uploadTab, textPlaceholder, dropzoneText, dropzoneBrowse, uploading, submit, maxFileSize, invalidFileType, fileTooLarge, success

**`settings.*`:** title, thresholds (section heading), minConfidence, addressedThreshold, partialThreshold, maxFindings, includeAddressed, boostTerms (section heading), termLabel, weightLabel, addTerm, save, saved, resetDefaults, resetConfirm

**`common.*`:** back, delete, deleteConfirm, cancel, error

### Navigation

Add `"nav.analysis": "Analysis"` / `"nav.analysis": "Analyse"` to `common.json` in both locales.

In `__root.tsx`, add a `<Link to="/analysis">` after the second separator dot (after the Search link), before the Compliance link, using `{t("nav.analysis")}`.

### Registration

In `src/i18n/index.ts`:
1. Import the new JSON files (`enAnalysis`, `nbAnalysis`)
2. Add `analysis: enAnalysis` and `analysis: nbAnalysis` to the resources object

---

## Section 3: Route Files and List Page

### Route Structure

Create four route files under `src/routes/analysis/`:

- `index.tsx` — The list page (main content of this section)
- `create.tsx` — Create page (section 4)
- `settings.tsx` — Settings page (section 5)
- `$id.tsx` — Detail placeholder (renders "Analysis detail page coming soon" with back link)

### Analysis List Page (`/analysis`)

**Component hierarchy:**
- Route component renders page header (title + buttons) and `<AnalysisList>`
- `<AnalysisList>` takes the query result (typed as `AnalysisListItem[]`) and renders a grid of `<AnalysisCard>` components
- Each `<AnalysisCard>` wraps in a `<Link to="/analysis/$id">` and shows name, `<StatusBadge>`, input type, processing time, created date. Note: frameworks count is NOT available from the list endpoint — shown only on the detail page.
- `<StatusBadge>` maps status to color: pending=blue, processing=yellow (with pulse animation), completed=green, failed=red

**Page header:** Title on the left, two buttons on the right:
- "New Analysis" → navigates to `/analysis/create`
- Settings icon → navigates to `/analysis/settings`

**Filters:** Status dropdown above the list (Select component with "All" + each status option). Filter value stored as URL search param via `Route.useSearch()` and `navigate({ search: ... })`.

**Pagination:** Below the card grid. Simple "Previous / Page X of Y / Next" controls. Page number as URL search param.

**Auto-polling:** `useAnalyses` uses conditional `refetchInterval`. When the response data contains any analysis with `status === "processing"`, return `5000` (poll every 5s). Otherwise return `false` (no polling).

**Empty state:** When no analyses exist, show centered content with a document icon, "No analyses yet" heading, description text, and "Create your first analysis" button.

**Loading state:** Skeleton cards in the grid layout while loading.

**Error state:** Error message with "Try again" button that refetches.

---

## Section 4: Create Analysis Page

### Page Structure

`/analysis/create` renders a centered form card with:
1. Back link to `/analysis`
2. Page title "New Analysis"
3. Name input (required)
4. Description textarea (optional — **only shown when Text tab is active**, the upload endpoint does not accept description)
5. Tab toggle: "Text Input" | "File Upload"
6. Tab content (text area or file drop zone)
7. Submit button

**Tab state behavior:** Both tabs preserve their state when switching. Switching tabs does not clear the text or selected file. Only the active tab's content is submitted.

### Tab: Text Input

A large `<textarea>` (install shadcn `textarea` component) with placeholder text like "Paste your document text here...". The text goes into `CreateAnalysisRequest.input_text`.

### Tab: File Upload

**`FileDropZone` component.** Native HTML5 drag-and-drop implementation (no external library):

- Drop zone `<div>` with drag event handlers (`onDragEnter`, `onDragLeave`, `onDragOver`, `onDrop`)
- **Drag counter ref** to prevent flicker when dragging over child elements
- Visual feedback: dashed border, highlight on `isDragging` state
- Hidden `<input type="file">` for keyboard accessibility (linked via `<label>`)
- Accept: `.pdf`, `.docx` (validate both MIME type and extension)
- Max size: 25MB (validate before initiating upload)
- After selection: show filename, file size, and "Remove" button
- Props: `accept: string[]`, `maxSizeMB: number`, `onFileSelected: (file: File) => void`, `onError: (msg: string) => void`

### Upload Progress

When the file tab is active and form is submitted:
1. `useUploadAnalysis()` mutation fires
2. Progress state (0-100) drives a progress bar component below the drop zone
3. The progress bar shows percentage text
4. Submit button shows "Uploading..." with spinner, disabled state
5. On success: navigate to `/analysis/{newId}`
6. On error: show error message, keep form state for retry

### Text Submit Flow

When text tab is active:
1. Validate: name is non-empty, input_text is non-empty
2. `useCreateAnalysis()` mutation fires
3. Submit button shows spinner, disabled state
4. On success: navigate to `/analysis/{newId}`
5. On error: show error message, keep form state

### Validation

- Name: required, show inline error if empty on submit
- Text: required when text tab active
- File: required when upload tab active, validated for type and size before upload
- Disable submit while mutation is pending

---

## Section 5: Settings Page

### Page Structure

`/analysis/settings` renders a form card:
1. Back link to `/analysis`
2. Page title "Matcher Configuration"
3. Threshold section
4. Options section
5. Boost terms section
6. Action buttons

### Threshold Fields

Number inputs with labels:
- `min_confidence_threshold` — min 0, max 1, step 0.05
- `addressed_threshold` — min 0, max 1, step 0.05
- `partial_threshold` — min 0, max 1, step 0.05
- `max_findings_per_framework` — min 1, max 500, step 1

Each field shows its label from i18n and the current value from `usePromptTemplate()`.

### Options

- `include_addressed_findings` — checkbox or toggle switch

### Boost Terms Editor (`BoostTermsEditor` component)

An inline key-value editor for `Record<string, number>`:

- Renders a list of rows, each with: text input for the term key, number input for the weight (step 0.1), and a delete button (trash icon)
- "Add Term" button at the bottom adds a new empty row
- State: managed as `Array<{ term: string; weight: number }>` internally, converted to/from `Record<string, number>` at the boundary
- Validation: no empty term keys, no duplicate keys, weight > 0

### Form State

Use `useState` initialized from `usePromptTemplate().data` when it loads. The form is controlled — changes update local state. On save, convert to `MatcherConfig` shape and call `useUpdatePromptTemplate()`.

**Save button:** Calls mutation, shows success toast/message on success, error message on failure.

**Reset to Defaults button:** Confirmation prompt ("Reset all settings to defaults?"), then sets local state to `MatcherConfig` default values (version: 1, min_confidence: 0.1, addressed: 0.6, partial: 0.3, max_findings: 50, include_addressed: true, default boost terms). Save is NOT automatic — user must click Save after resetting.

---

## Section 6: New shadcn/ui Components

This split requires three shadcn/ui components not yet installed:

- **table** — Used by the findings table in split 02, but also useful for settings layout
- **tabs** — For the text/upload toggle on the create page
- **textarea** — For the text input mode on the create page

Install via `npx shadcn@latest add table tabs textarea` before component development begins.

---

## Edge Cases and Error Handling

| Scenario | Behavior |
|----------|----------|
| Empty analysis list | Show empty state with create button |
| Failed upload (network error) | Show error message, keep file selection |
| File too large (>25MB) | Client-side validation, show error, don't upload |
| Invalid file type | Client-side validation, show "Only PDF/DOCX supported" |
| Analysis stuck in processing | Auto-poll continues; user can delete |
| Failed analysis | Show error_message in card, user can delete (no retry) |
| Settings API returns 500 (corrupt config) | Show error message on settings page |
| Concurrent settings saves | Last write wins (acceptable for single-user) |
| Create succeeds but nav fails | Unlikely; navigate in onSuccess callback |
| Upload progress stalls | No timeout; user sees stalled progress, can navigate away |
