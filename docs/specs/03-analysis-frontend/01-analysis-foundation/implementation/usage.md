# Analysis Frontend Foundation - Usage Guide

## Overview

The analysis feature module provides a complete frontend for creating, viewing, and configuring document analysis operations. It integrates with the backend analysis API endpoints.

## Routes

| Route | Page | Description |
|-------|------|-------------|
| `/analysis` | List Page | Paginated card grid of analyses with status filter and auto-polling |
| `/analysis/create` | Create Page | Tabbed form for text input or file upload |
| `/analysis/settings` | Settings Page | Matcher configuration editor (thresholds, boost terms) |
| `/analysis/$id` | Detail Page | Placeholder — coming in split 02 |

## Key Components

### StatusBadge
Color-coded badge for analysis status (pending=blue, processing=yellow+pulse, completed=green, failed=red).

```tsx
import { StatusBadge } from "@/features/analysis";
<StatusBadge status="completed" />
```

### AnalysisCard / AnalysisList
Card grid with loading skeletons, empty state, and error state with retry.

### CreateAnalysisForm
Tabbed form supporting:
- **Text tab**: Name, description, and text input
- **Upload tab**: Name + drag-and-drop file upload (PDF/DOCX, max 25MB) with progress tracking

### FileDropZone
Native HTML5 drag-and-drop with:
- Dual MIME + extension validation
- Drag counter pattern (prevents flicker)
- File size validation
- Keyboard-accessible hidden file input

### SettingsForm / BoostTermsEditor
Matcher configuration editor:
- Threshold number inputs (min_confidence, addressed, partial)
- Max findings per framework
- Include addressed findings toggle
- Inline boost terms key-value editor
- Reset to defaults (with confirmation)

## API Hooks

All hooks in `@/features/analysis/api`:

| Hook | Purpose |
|------|---------|
| `useAnalyses(params?)` | Paginated list with auto-polling for processing items |
| `useAnalysis(id)` | Single analysis with `matched_framework_ids` JSON parsing |
| `useCreateAnalysis()` | Create from text input |
| `useUploadAnalysis()` | Upload file with progress tracking |
| `useDeleteAnalysis()` | Delete analysis |
| `useFindings(id, params?)` | Paginated findings |
| `usePromptTemplate()` | Get matcher config |
| `useUpdatePromptTemplate()` | Update matcher config |
| `exportAnalysis(id, format)` | Download export (not a hook) |

## i18n

Translations in `en/analysis.json` and `nb/analysis.json` covering:
- List page (title, filters, empty state, pagination)
- Create page (form labels, validation messages, upload text)
- Settings page (thresholds, boost terms, save/reset)
- Status labels and common actions

## Test Coverage

110 tests across 23 test files covering:
- All API hooks with cache invalidation
- Upload progress tracking
- Component rendering states (loading, error, empty, data)
- File validation (type, size, MIME)
- Drag-and-drop interactions
- Form submission paths
- Settings save/reset behavior
- Navigation and i18n integration

## What's Next (Split 02)

The `$id.tsx` route is a placeholder. Split 02 will implement:
- Analysis detail page with findings display
- Chart visualizations (heatmap, radar, priority bars)
- Findings table with filtering and sorting
- Export functionality (PDF/DOCX download)
