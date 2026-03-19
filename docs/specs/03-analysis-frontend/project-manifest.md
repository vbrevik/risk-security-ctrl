<!-- SPLIT_MANIFEST
01-analysis-foundation
02-analysis-detail-and-charts
END_MANIFEST -->

# Project Manifest: Analysis Frontend

## Split Structure

### 01-analysis-foundation
**Purpose:** Build the feature module skeleton, types, API hooks, i18n, navigation integration, and the simpler pages (list, create, settings).

**Scope:**
- TypeScript types mirroring backend models (Analysis, AnalysisFinding, MatcherConfig, etc.)
- TanStack Query API hooks for all endpoints (CRUD, upload, findings, export, prompt template)
- i18n namespace (`analysis.json` in en + nb)
- Navigation link in `__root.tsx`
- Route files: `/analysis`, `/analysis/create`, `/analysis/settings`
- Analysis list page with pagination, status filters, status badges
- Create analysis page with text input mode and file upload mode (drag-and-drop)
- Settings page with MatcherConfig form (thresholds, boost terms)

**Output:** Working list, create, and settings pages. The detail page route exists but shows a placeholder.

### 02-analysis-detail-and-charts
**Purpose:** Build the complex analysis detail page with summary statistics, interactive charts, expandable findings table, and export functionality.

**Scope:**
- Analysis detail page at `/analysis/{id}`
- Summary statistics cards (total findings, gaps, addressed, frameworks matched, processing time)
- Coverage heatmap chart (per-framework addressed/gap/partial percentages)
- Priority breakdown bar chart
- Expandable findings table with framework filter, type filter, priority sort
- Finding row expansion showing evidence, recommendation, concept definition
- Finding type badges (addressed=green, partial=yellow, gap=red, n/a=gray)
- Export buttons (PDF/DOCX download via API)

**Output:** Fully functional detail page with data visualization and export.

## Dependencies

```
01-analysis-foundation
  └── 02-analysis-detail-and-charts (depends on types, API hooks, i18n from 01)
```

- **02 depends on 01:** Types, API hooks, and i18n namespace must exist before the detail page can be built
- **Dependency type:** models, APIs, patterns

## Execution Order

1. `01-analysis-foundation` (no dependencies)
2. `02-analysis-detail-and-charts` (after 01)

Sequential execution required — 02 consumes the types and hooks created in 01.

## Next Steps

```bash
/deep-plan @docs/specs/03-analysis-frontend/01-analysis-foundation/spec.md
/deep-plan @docs/specs/03-analysis-frontend/02-analysis-detail-and-charts/spec.md
```
