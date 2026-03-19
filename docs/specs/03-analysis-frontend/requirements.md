# Document Analysis Frontend

## Goal

Build a frontend feature module for the document analysis system. Users should be able to create analyses (text input or file upload), view results with findings mapped to compliance frameworks, visualize coverage via charts, and export reports as PDF or DOCX.

## Backend API (already implemented)

All endpoints are under `/api/analyses`:

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/analyses` | Create analysis from text input |
| POST | `/api/analyses/upload` | Upload PDF/DOCX file for analysis |
| GET | `/api/analyses` | List analyses (paginated, filterable by status) |
| GET | `/api/analyses/{id}` | Get analysis details |
| DELETE | `/api/analyses/{id}` | Delete analysis |
| GET | `/api/analyses/{id}/findings` | Get findings (paginated, filterable by framework, type, priority) |
| GET | `/api/analyses/{id}/export/{format}` | Export as PDF or DOCX (returns file bytes) |
| GET | `/api/analyses/prompt-template` | Get matcher configuration |
| PUT | `/api/analyses/prompt-template` | Update matcher configuration |

### Key Backend Types

```typescript
// Analysis
interface Analysis {
  id: string;
  name: string;
  description?: string;
  input_type: "text" | "pdf" | "docx";
  status: "pending" | "processing" | "completed" | "failed" | "deleted";
  error_message?: string;
  matched_framework_ids: string[];
  processing_time_ms?: number;
  token_count?: number;
  created_at: string;
  updated_at: string;
}

// Finding (from /findings endpoint, joined with concept data)
interface AnalysisFinding {
  id: string;
  concept_id: string;
  framework_id: string;
  finding_type: "addressed" | "partially_addressed" | "gap" | "not_applicable";
  confidence_score: number; // 0.0-1.0
  evidence_text?: string;
  recommendation?: string;
  priority: number; // 1-4
  concept_code?: string;
  concept_name: string;
  concept_definition?: string;
}

// Matcher Config (prompt template)
interface MatcherConfig {
  version: number;
  min_confidence_threshold: number;
  addressed_threshold: number;
  partial_threshold: number;
  max_findings_per_framework: number;
  include_addressed_findings: boolean;
  boost_terms: Record<string, number>;
}
```

## Existing Frontend Stack

- React 19 + TypeScript + Vite
- TanStack Router (file-based routing)
- TanStack Query for server state
- shadcn/ui components (button, card, dialog, input, label, select, badge)
- Tailwind CSS v4
- i18next (en + nb namespaces per feature)
- Lucide icons
- axios API client at `/src/lib/api.ts`

## Existing Patterns to Follow

- Feature modules in `src/features/{name}/` with `api/`, `types/`, `components/` subdirectories
- Routes in `src/routes/{name}/index.tsx`
- API hooks use TanStack Query (see `compliance` feature as reference)
- All user-facing text via `useTranslation()` with feature namespace
- Navigation links added to `__root.tsx` header

## Requirements

### Pages

1. **Analysis List** (`/analysis`) - Table/card list of all analyses with status badges, creation date, framework matches. Create new analysis button. Filter by status.

2. **Analysis Detail** (`/analysis/{id}`) - Shows analysis metadata, findings summary stats, per-framework findings table with sorting/filtering, and export buttons (PDF/DOCX download).

3. **Create Analysis** - Either a dedicated page or dialog. Two input modes: paste text or upload file (PDF/DOCX). Name field required. Submit triggers processing.

### Components Needed

- Analysis list with pagination
- Status badges (pending, processing, completed, failed)
- File upload with drag-and-drop
- Findings table with framework filter, type filter, priority sort
- Finding type badges (addressed=green, partial=yellow, gap=red, n/a=gray)
- Export buttons that trigger file download
- Summary statistics cards (total findings, gaps, addressed, frameworks matched)

### Constraints

- Follow existing project conventions exactly
- Use existing shadcn/ui components, add new ones only if needed
- All text in i18n (English + Norwegian)
- No global state beyond TanStack Query
- Mobile-responsive
