# 05-frontend-dashboard: Analysis UI & Visualizations

## Summary

Frontend feature module for the Document Analysis Engine. Analysis list page, create dialog with text/file upload tabs, analysis detail page with three rich visualizations (coverage heatmap, framework radar chart, gap priority board), findings table with filtering, export buttons, and navigation integration.

## Requirements Source

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md` (UI section)
- Interview: `docs/specs/deep_project_interview.md`

## What to Build

### Feature Module Structure

```
frontend/src/features/analysis/
  api/index.ts          — TanStack Query hooks
  components/
    AnalysisList.tsx     — List page content
    CreateAnalysisDialog.tsx — Create dialog with text/upload tabs
    AnalysisDetail.tsx   — Detail page layout
    FindingsTable.tsx    — Sortable/filterable findings table
    CoverageHeatmap.tsx  — Framework × concept coverage matrix
    FrameworkRadar.tsx   — Spider/radar chart per framework
    GapPriorityBoard.tsx — Kanban-style priority board
    AnalysisStatusBadge.tsx — Status indicator
    ExportButtons.tsx    — PDF/DOCX download triggers
  types/index.ts         — TypeScript interfaces
  index.ts               — Public exports

frontend/src/routes/analysis/
  index.tsx              — Analysis list page (route: /analysis)
  $analysisId.tsx        — Analysis detail page (route: /analysis/:analysisId)

frontend/src/i18n/locales/en/analysis.json — English translations
```

### API Hooks (`api/index.ts`)

```typescript
// Query key factory
export const analysisKeys = {
  all: ['analyses'] as const,
  list: (filters) => [...analysisKeys.all, 'list', filters],
  detail: (id) => [...analysisKeys.all, 'detail', id],
  findings: (id, filters) => [...analysisKeys.all, 'findings', id, filters],
  promptTemplate: () => [...analysisKeys.all, 'prompt-template'],
};

// Hooks
useAnalyses(filters?)           — GET /api/analyses (paginated)
useAnalysis(id)                 — GET /api/analyses/:id
useAnalysisFindings(id, filters?) — GET /api/analyses/:id/findings
useCreateAnalysis()             — POST /api/analyses (mutation)
useUploadAnalysis()             — POST /api/analyses/upload (mutation, multipart)
useDeleteAnalysis()             — DELETE /api/analyses/:id (mutation)
useExportAnalysis(id, format)   — GET /api/analyses/:id/export/:format (download)
usePromptTemplate()             — GET /api/analyses/prompt-template
useUpdatePromptTemplate()       — PUT /api/analyses/prompt-template (mutation)
```

### Pages

**Analysis List Page (`/analysis`)**

- Header: "Document Analysis" title + "New Analysis" button
- Filter bar: status dropdown (all/pending/completed/failed), date range (optional)
- Table columns: Name, Status (badge), Input Type, Frameworks Matched, Findings (gap/total), Date
- Click row → navigate to detail page
- Empty state: "No analyses yet. Upload a document or describe a scenario to get started." with CTA button

**Analysis Detail Page (`/analysis/:analysisId`)**

Layout (top to bottom):
1. **Header bar**: Analysis name, status badge, created date, processing time, token count, delete button (with confirmation dialog)
2. **Export bar**: "Export PDF" and "Export DOCX" buttons
3. **Summary cards row**: Total findings, Gaps, Partially Addressed, Addressed — each as a stat card with count and color
4. **Visualization tabs**: Three tabs — "Coverage", "Framework", "Priorities"
   - Tab 1: Coverage Heatmap
   - Tab 2: Framework Radar
   - Tab 3: Gap Priority Board
5. **Findings table**: Full findings list below visualizations

### Visualizations

**Coverage Heatmap (`CoverageHeatmap.tsx`)**

A matrix/grid showing:
- X-axis: Framework names
- Y-axis: Top-level concepts (grouped by framework)
- Cells: Color-coded by finding type
  - Green: addressed
  - Yellow: partially_addressed
  - Red: gap
  - Gray: not assessed / not in scope
- Hover: Show concept name, code, confidence score
- Click cell: Filter findings table to that concept

Implementation: D3.js (already installed). SVG grid with color fills.

**Framework Radar (`FrameworkRadar.tsx`)**

Spider/radar chart showing coverage per framework:
- One radar per matched framework (or overlay multiple)
- Axes: Top-level concept categories within the framework
- Values: Percentage of child concepts that are addressed (0-100%)
- Color: Framework color from `getFrameworkColor()`

Implementation: D3.js radar chart. One chart per framework, or a selector to switch between frameworks.

**Gap Priority Board (`GapPriorityBoard.tsx`)**

Kanban-style board:
- 4 columns: Critical (P1), High (P2), Medium (P3), Low (P4)
- Cards: One per gap/partial finding, showing concept code, name, framework badge, recommendation excerpt
- Click card: Expand to show full recommendation + link to ontology explorer
- Column counts shown in header

Implementation: CSS grid or flexbox columns. Cards as shadcn/ui Card components. No drag-and-drop needed for MVP.

### Create Analysis Dialog

Two-tab dialog (shadcn/ui Dialog + Tabs):

**Tab 1: "Describe Scenario"**
- Name input (required)
- Description textarea (optional)
- Scenario/policy text textarea (required, large)
- Collapsible "Advanced Settings" accordion:
  - Prompt template JSON editor (pre-filled with default, editable)

**Tab 2: "Upload Document"**
- Name input (required)
- Description textarea (optional)
- Drag-and-drop file zone (accepts .pdf, .docx, max 20MB)
- File preview: filename, size, type
- Same "Advanced Settings" accordion

**Submit behavior:**
- Button text: "Analyze"
- On submit: show loading spinner, disable button
- On success: navigate to detail page
- On error: show error toast

### Navigation

Add "Analysis" to the main nav bar in `routes/__root.tsx`, positioned between "Compliance Tracking" and "Reports".

### i18n

Create `frontend/src/i18n/locales/en/analysis.json` with keys for:
- Page titles, button labels, status names
- Finding type labels (addressed, partially_addressed, gap)
- Priority labels (critical, high, medium, low)
- Empty states, error messages
- Visualization titles and labels

## Key Decisions

- **D3 for visualizations** — Already installed in the project, used by ontology graph. Consistent choice.
- **All 3 visualizations** — User explicitly requested coverage heatmap, radar chart, and priority board.
- **Tabbed visualization layout** — Avoids overwhelming the detail page. User switches between views.
- **No drag-and-drop on priority board** — Read-only display for MVP. Findings are computed, not manually arranged.
- **File upload via native FormData** — Use axios with `multipart/form-data`. No additional upload library needed.

## Dependencies

- **Needs from 04-backend-api-export:** All REST endpoints must be functional
- **Uses existing:** shadcn/ui (Dialog, Tabs, Table, Card, Badge, Button, Select), D3.js, TanStack Query, TanStack Router, i18next, axios, lucide-react icons

## Existing Patterns to Follow

- Feature structure: See `frontend/src/features/compliance/` for api hooks, component organization
- Route pages: See `frontend/src/routes/compliance/` for TanStack Router file-based routing
- API client: See `frontend/src/lib/api.ts` — axios instance with base URL
- Query keys: See `frontend/src/features/ontology/api/index.ts` for key factory pattern
- Components: shadcn/ui components in `frontend/src/components/ui/`
- Context: If needed, follow `ExplorerContext.tsx` pattern (useReducer + useMemo + useCallback for stable context value)
- i18n: See `frontend/src/i18n/locales/en/ontology.json` for translation key structure
