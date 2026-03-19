<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-types-and-hooks
section-02-i18n-and-navigation
section-03-shadcn-components
section-04-list-page
section-05-create-page
section-06-settings-page
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-types-and-hooks | - | 02, 03, 04, 05, 06 | Yes |
| section-02-i18n-and-navigation | 01 | 04, 05, 06 | Yes |
| section-03-shadcn-components | - | 04, 05, 06 | Yes (parallel with 01, 02) |
| section-04-list-page | 01, 02, 03 | - | Yes (parallel with 05, 06) |
| section-05-create-page | 01, 02, 03 | - | Yes (parallel with 04, 06) |
| section-06-settings-page | 01, 02, 03 | - | Yes (parallel with 04, 05) |

## Execution Order

1. section-01-types-and-hooks, section-03-shadcn-components (parallel, no deps)
2. section-02-i18n-and-navigation (after 01)
3. section-04-list-page, section-05-create-page, section-06-settings-page (parallel after 02, 03)

## Section Summaries

### section-01-types-and-hooks
TypeScript interfaces (Analysis, AnalysisListItem, AnalysisFinding, MatcherConfig, request/param types) and all TanStack Query hooks (useAnalyses, useAnalysis, useCreateAnalysis, useUploadAnalysis, useDeleteAnalysis, useFindings, export utility, usePromptTemplate, useUpdatePromptTemplate). Query keys with hierarchical invalidation. Feature module skeleton (api/index.ts, types/index.ts, index.ts).

### section-02-i18n-and-navigation
Create analysis.json translation files for en and nb locales. Register analysis namespace in i18n/index.ts. Add nav.analysis to common.json. Add Analysis link to __root.tsx navigation. Create route file stubs (index.tsx, create.tsx, settings.tsx, $id.tsx placeholder).

### section-03-shadcn-components
Install shadcn/ui components not yet available: table, tabs, textarea. Run `npx shadcn@latest add table tabs textarea`. Verify build succeeds.

### section-04-list-page
Analysis list page at /analysis. AnalysisList component with card grid, pagination, status filter via URL search params. AnalysisCard component with Link, StatusBadge, metadata. StatusBadge with color coding (pending=blue, processing=yellow+pulse, completed=green, failed=red). Empty/loading/error states. Auto-polling via conditional refetchInterval.

### section-05-create-page
Create analysis page at /analysis/create. CreateAnalysisForm with name input, description (text tab only), tab toggle (Text/Upload). Text tab with textarea. Upload tab with FileDropZone (native drag-and-drop, file type/size validation, hidden input for accessibility). Progress bar during upload. Submit calls appropriate mutation, navigates to detail on success.

### section-06-settings-page
Settings page at /analysis/settings. SettingsForm with number inputs for thresholds, toggle for include_addressed_findings. BoostTermsEditor for key-value pairs (add/remove rows). Save and Reset to Defaults buttons. Form state from usePromptTemplate, save via useUpdatePromptTemplate.
