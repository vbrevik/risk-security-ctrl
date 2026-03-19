# Analysis Frontend Foundation

## Goal

Build the feature module skeleton for document analysis in the existing React frontend, including TypeScript types, TanStack Query API hooks, i18n translations, navigation integration, and three pages: analysis list, create analysis, and matcher settings.

## Context

- **Requirements:** See `../requirements.md` for full backend API specification and frontend stack details
- **Interview decisions:** See `../deep_project_interview.md` — create flow uses a dedicated page, settings page included
- **Depends on:** Nothing (this is the foundation split)
- **Provides to 02-analysis-detail-and-charts:** Types, API hooks, i18n namespace, route structure

## Existing Patterns to Follow

The `compliance` feature (`frontend/src/features/compliance/`) is the closest reference:
- `api/index.ts` — TanStack Query hooks with query keys, mutations with cache invalidation
- `types/index.ts` — TypeScript interfaces mirroring backend models
- `components/` — React components using shadcn/ui
- Route files in `frontend/src/routes/compliance/`

The API client is at `frontend/src/lib/api.ts` (axios instance with `baseURL: "/api"`).

i18n uses namespace-per-feature pattern with `en` and `nb` locale files.

Navigation links are in `frontend/src/routes/__root.tsx`.

## Scope

### 1. Types (`features/analysis/types/index.ts`)
TypeScript interfaces for: Analysis, AnalysisStatus, InputType, AnalysisFinding, FindingType, MatcherConfig, PaginatedResponse. Must match backend API response shapes exactly (see requirements.md for type definitions).

### 2. API Hooks (`features/analysis/api/index.ts`)
TanStack Query hooks for all backend endpoints:
- `useAnalyses(params)` — paginated list with status filter
- `useAnalysis(id)` — single analysis detail
- `useCreateAnalysis()` — mutation for text-based creation
- `useUploadAnalysis()` — mutation for file upload (multipart/form-data)
- `useDeleteAnalysis()` — mutation with cache invalidation
- `useFindings(id, params)` — paginated findings with filters
- `useExportAnalysis(id, format)` — triggers file download (PDF/DOCX)
- `usePromptTemplate()` — get current MatcherConfig
- `useUpdatePromptTemplate()` — mutation to update MatcherConfig

### 3. i18n (`i18n/locales/{en,nb}/analysis.json`)
Translation keys for all user-facing text: page titles, form labels, status values, table headers, filter labels, buttons, error messages, settings labels. Register namespace in `i18n/index.ts`.

### 4. Navigation
Add "Analysis" link to the navigation in `__root.tsx`, between "Search" and "Compliance".

### 5. Analysis List Page (`/analysis`)
- Paginated table/card list of analyses
- Status badge per analysis (pending=blue, processing=yellow, completed=green, failed=red)
- Shows: name, input type, status, matched frameworks count, created date
- Filter by status (dropdown)
- "New Analysis" button linking to `/analysis/create`
- Click row navigates to `/analysis/{id}`

### 6. Create Analysis Page (`/analysis/create`)
- Name field (required)
- Description field (optional)
- Two input modes toggled by tabs or radio:
  - **Text mode:** Large textarea for pasting text
  - **Upload mode:** File upload area with drag-and-drop for PDF/DOCX (max 25MB)
- Submit button — calls `useCreateAnalysis()` or `useUploadAnalysis()`
- On success, navigate to the new analysis detail page
- Loading/processing state feedback

### 7. Settings Page (`/analysis/settings`)
- Form showing all MatcherConfig fields with current values
- Number inputs for thresholds (min_confidence, addressed, partial)
- Number input for max_findings_per_framework
- Toggle for include_addressed_findings
- Boost terms editor (key-value pairs, add/remove)
- Save button — calls `useUpdatePromptTemplate()`
- Reset to defaults button

## Constraints

- All user-facing text via `useTranslation('analysis')`
- Use existing shadcn/ui components (may need to add `table`, `tabs`, `textarea`, `switch`, `dropdown-menu` if not yet installed)
- Mobile-responsive layouts
- No global state beyond TanStack Query
