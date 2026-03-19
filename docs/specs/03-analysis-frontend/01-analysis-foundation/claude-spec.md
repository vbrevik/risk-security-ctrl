# Synthesized Spec: Analysis Frontend Foundation

## What We're Building

A new `analysis` feature module in the existing React frontend that provides:
1. TypeScript types mirroring backend analysis API models
2. TanStack Query hooks for all analysis endpoints
3. i18n translations (English + Norwegian Bokmål)
4. Navigation integration in the root layout
5. Analysis list page with auto-polling for processing status
6. Create analysis page with text input and drag-and-drop file upload with progress bar
7. Matcher settings page with inline key-value editor for boost terms

## Technology Stack (Existing)

- React 19 + TypeScript + Vite
- TanStack Router (file-based) + TanStack Query v5
- shadcn/ui (button, dialog, input, label, select, badge, card already installed)
- Tailwind CSS v4
- i18next with `en` and `nb` locales
- axios API client (`/src/lib/api.ts`)
- Vitest + React Testing Library

**New shadcn/ui components needed:** table, tabs, textarea (for create page text input)

## Backend API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/api/analyses` | Create from text (`{ name, description?, input_text }`) |
| POST | `/api/analyses/upload` | Create from file (multipart: `file` + `name` fields) |
| GET | `/api/analyses?page=&limit=&status=` | Paginated list |
| GET | `/api/analyses/{id}` | Single analysis detail |
| DELETE | `/api/analyses/{id}` | Delete analysis |
| GET | `/api/analyses/{id}/findings?page=&limit=&framework_id=&finding_type=&priority=` | Paginated findings |
| GET | `/api/analyses/{id}/export/{format}` | Download PDF or DOCX bytes |
| GET | `/api/analyses/prompt-template` | Get MatcherConfig |
| PUT | `/api/analyses/prompt-template` | Update MatcherConfig |

## Feature Module Structure

```
src/features/analysis/
├── api/index.ts           # Query keys + hooks
├── types/index.ts         # TypeScript interfaces
├── components/
│   ├── AnalysisList.tsx   # List with status badges, pagination
│   ├── AnalysisCard.tsx   # Individual analysis card
│   ├── StatusBadge.tsx    # Color-coded status badge
│   ├── CreateAnalysisForm.tsx  # Text/upload tabs with form
│   ├── FileDropZone.tsx   # Drag-and-drop file upload area
│   ├── SettingsForm.tsx   # MatcherConfig editor
│   └── BoostTermsEditor.tsx # Key-value pair editor
└── index.ts               # Re-exports
```

## Types

```typescript
type AnalysisStatus = "pending" | "processing" | "completed" | "failed" | "deleted";
type InputType = "text" | "pdf" | "docx";
type FindingType = "addressed" | "partially_addressed" | "gap" | "not_applicable";

interface Analysis {
  id: string;
  name: string;
  description: string | null;
  input_type: InputType;
  status: AnalysisStatus;
  error_message: string | null;
  matched_framework_ids: string[];  // JSON-parsed on backend
  processing_time_ms: number | null;
  token_count: number | null;
  created_at: string;
  updated_at: string;
}

interface AnalysisFinding {
  id: string;
  concept_id: string;
  framework_id: string;
  finding_type: FindingType;
  confidence_score: number;
  evidence_text: string | null;
  recommendation: string | null;
  priority: number;
  sort_order: number;
  created_at: string;
  concept_code: string | null;
  concept_name: string;
  concept_definition: string | null;
}

interface MatcherConfig {
  version: number;
  min_confidence_threshold: number;
  addressed_threshold: number;
  partial_threshold: number;
  max_findings_per_framework: number;
  include_addressed_findings: boolean;
  boost_terms: Record<string, number>;
}

interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}
```

## API Hooks

**Query keys pattern:**
```typescript
const analysisKeys = {
  all: ["analysis"] as const,
  list: (params?) => [...analysisKeys.all, "list", params] as const,
  detail: (id: string) => [...analysisKeys.all, "detail", id] as const,
  findings: (id: string, params?) => [...analysisKeys.all, "detail", id, "findings", params] as const,
  promptTemplate: () => [...analysisKeys.all, "prompt-template"] as const,
};
```

**Hooks needed:**
- `useAnalyses(params)` — paginated list with `refetchInterval: 5000` when any item has processing status
- `useAnalysis(id)` — single detail
- `useCreateAnalysis()` — mutation for text creation
- `useUploadAnalysis()` — mutation for file upload with progress tracking via `useState`
- `useDeleteAnalysis()` — mutation with cache invalidation
- `useFindings(id, params)` — paginated findings with filters
- `usePromptTemplate()` — get current config
- `useUpdatePromptTemplate()` — mutation to save config

## Pages

### Analysis List (`/analysis`)
- Header with title and "New Analysis" button (links to `/analysis/create`)
- Status filter dropdown (all, pending, processing, completed, failed)
- Card grid of analyses showing: name, status badge, input type, frameworks count, created date
- Cards link to `/analysis/{id}` (detail page — placeholder in this split)
- Pagination controls
- Auto-poll: `refetchInterval: 5000` when any analysis is processing
- Empty state with icon and "Create your first analysis" message
- Error state with retry prompt

### Create Analysis (`/analysis/create`)
- Page title + back link to list
- Name field (required, text input)
- Description field (optional, textarea)
- Tab toggle: "Text Input" | "File Upload"
- **Text tab:** Large textarea for pasting document text
- **File tab:** `FileDropZone` component with:
  - Drag-and-drop area with visual feedback (border highlight on drag)
  - Hidden `<input type="file">` for keyboard/click accessibility
  - Accept: PDF and DOCX only
  - Max size: 25MB (validate client-side before upload)
  - Show selected filename after selection
- Progress bar during upload (percentage via axios `onUploadProgress`)
- Submit button (disabled while submitting)
- On success: navigate to `/analysis/{newId}`
- On error: show error message, keep form state

### Settings (`/analysis/settings`)
- Page title + back link
- Number inputs for: `min_confidence_threshold`, `addressed_threshold`, `partial_threshold`
- Number input for `max_findings_per_framework`
- Toggle (switch/checkbox) for `include_addressed_findings`
- `BoostTermsEditor`: list of term/weight rows with add/remove buttons
  - Each row: text input for term, number input for weight, delete button
  - "Add term" button at bottom
- Save button (calls `useUpdatePromptTemplate`)
- Success/error feedback after save
- "Reset to Defaults" button (sends `MatcherConfig::default()` values)

## i18n

### Namespace: `analysis`
Keys organized by page/component:
- `title`, `nav.analysis`
- `list.*` (title, newAnalysis, empty, filters)
- `create.*` (title, name, description, textTab, uploadTab, submit, uploading, dropzone)
- `settings.*` (title, thresholds, boostTerms, save, resetDefaults)
- `status.*` (pending, processing, completed, failed)
- `common.*` (delete, confirm, cancel, back)

Both `en/analysis.json` and `nb/analysis.json` needed. Register namespace in `i18n/index.ts`.
Add `nav.analysis` to `common.json` in both locales.

## Routes

File structure:
```
src/routes/analysis/
├── index.tsx          # /analysis — list page
├── create.tsx         # /analysis/create — create page
├── settings.tsx       # /analysis/settings — settings page
└── $id.tsx            # /analysis/{id} — detail placeholder (implemented in split 02)
```

## Navigation

Add to `__root.tsx` between Search and Compliance:
```tsx
<Link to="/analysis">{t("nav.analysis")}</Link>
```

## Edge Cases

- **Empty analysis list:** Show "No analyses yet" with icon and create button
- **Failed upload (network):** Show error, keep file selection for retry
- **File too large:** Client-side validation, show "File exceeds 25MB limit"
- **Invalid file type:** Client-side validation, show "Only PDF and DOCX files supported"
- **Processing timeout:** Analysis stuck in processing — no timeout handling needed (auto-poll continues indefinitely, user can delete)
- **Concurrent settings saves:** Last write wins (no optimistic locking needed for single-user)
- **MatcherConfig corrupt on disk:** Backend returns 500 — show error message on settings page
