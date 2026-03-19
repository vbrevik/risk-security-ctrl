Now I have all the context I need. Let me produce the section content.

# Section 01: Prerequisite Fixes, i18n Keys & Route Shell

## Overview

This section addresses three concerns that must be completed before any other section in the detail page implementation:

1. **Bug fix:** Rename `PaginatedResponse.data` to `.items` to match the backend response field name, and update all consumers.
2. **Bug fix:** Add nullability to five `AnalysisFinding` fields that are `Option<String>` on the backend but typed as required `string` in TypeScript.
3. **i18n:** Add all translation keys needed by sections 02 through 06 (English and Norwegian).
4. **Route shell:** Replace the placeholder `$id.tsx` stub with a functional page shell that handles loading, error, processing, and completed states.

No other section can proceed until this section is complete.

---

## Tests

Write tests before implementing each fix. All tests use Vitest + React Testing Library.

### PaginatedResponse fix tests

**File:** `frontend/src/features/ontology/types/__tests__/paginated-response.test.ts`

These are compile-time correctness checks. The key runtime test lives in the existing hooks:

- Test that `useAnalyses` hook accesses response data via `.items` (not `.data`).
- Test that `useFindings` hook accesses response data via `.items`.
- Test that the `refetchInterval` callback in `useAnalyses` checks `data?.items.some(...)`.

Since these hooks are already tested elsewhere, the main verification is that after renaming the field, the existing test suite still passes and TypeScript compiles without errors.

### AnalysisFinding nullability tests

**File:** `frontend/src/features/analysis/types/__tests__/finding-nullability.test.ts`

```ts
/**
 * Type-level test: verify that AnalysisFinding allows null on optional fields.
 * This is a compile-time check; if it compiles, the test passes.
 */
import { describe, it, expect } from "vitest";
import type { AnalysisFinding } from "../../types";

describe("AnalysisFinding nullability", () => {
  it("accepts null for optional concept and evidence fields", () => {
    const finding: AnalysisFinding = {
      id: "f1",
      concept_id: "c1",
      framework_id: "fw1",
      finding_type: "gap",
      confidence_score: 0.85,
      evidence_text: null,
      recommendation: null,
      priority: 1,
      sort_order: 1,
      concept_code: null,
      concept_name: null,
      concept_definition: null,
    };
    expect(finding.evidence_text).toBeNull();
  });
});
```

### Route shell tests

**File:** `frontend/src/routes/analysis/__tests__/$id.test.tsx`

Test stubs (use `vi.mock` for `../../../features/analysis/api` and i18n):

- **Renders loading skeleton when `useAnalysis` is loading.** Mock `useAnalysis` to return `{ isLoading: true }`. Assert that skeleton elements are present.
- **Renders error state when `useAnalysis` returns error.** Mock `useAnalysis` to return `{ isError: true, error: new Error("fail") }`. Assert error message and back link are shown.
- **Renders "not found" message for 404 error.** Mock `useAnalysis` to return an error with status 404. Assert i18n key `detail.notFound.title` text is rendered.
- **Shows processing banner when status is "processing".** Mock `useAnalysis` to return analysis with `status: "processing"`. Assert processing banner text is visible and charts/table are not rendered.
- **Calls `useAnalysis` with `refetchInterval` when status is processing.** Verify that `useAnalysis` is called with options including `refetchInterval: 5000` when the returned status is `"processing"`.
- **Renders detail content when analysis is completed.** Mock `useAnalysis` returning a completed analysis and `useFindings` returning findings. Assert that the page renders the stats/charts/table area (actual components tested in later sections).
- **Shows `EmptyFindings` when completed with zero findings.** Mock `useFindings` returning `{ items: [], total: 0 }`. Assert empty state message is shown.

---

## Implementation Details

### Fix 1: PaginatedResponse field rename

**File to modify:** `frontend/src/features/ontology/types/index.ts`

Change the `PaginatedResponse<T>` interface from:

```ts
export interface PaginatedResponse<T> {
  data: T[];
  // ...
}
```

to:

```ts
export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}
```

**Consumers to update (all `.data` references on PaginatedResponse objects):**

| File | Line(s) | Change |
|------|---------|--------|
| `frontend/src/features/analysis/api/index.ts` | 43 | `data?.data.some(...)` to `data?.items.some(...)` |
| `frontend/src/features/ontology/api/index.ts` | 66 | `return data.data;` to `return data.items;` |
| `frontend/src/features/ontology/api/index.ts` | 135 | `return data.data;` to `return data.items;` |
| `frontend/src/features/ontology/api/index.ts` | 157 | `allConcepts.push(...data.data);` to `allConcepts.push(...data.items);` |
| `frontend/src/features/ontology/components/Graph/GraphView.tsx` | 43 | `return data.data;` to `return data.items;` |
| `frontend/src/features/ontology/components/Sidebar/Sidebar.tsx` | 32 | `return data.data;` to `return data.items;` |

After making these changes, run `pnpm typecheck` to confirm no remaining references to the old `.data` field on `PaginatedResponse`.

### Fix 2: AnalysisFinding nullability

**File to modify:** `frontend/src/features/analysis/types/index.ts`

In the `AnalysisFinding` interface, change these five fields from `string` to `string | null`:

```ts
export interface AnalysisFinding {
  id: string;
  concept_id: string;
  framework_id: string;
  finding_type: FindingType;
  confidence_score: number;
  evidence_text: string | null;       // was: string
  recommendation: string | null;      // was: string
  priority: number;
  sort_order: number;
  concept_code: string | null;        // was: string
  concept_name: string | null;        // was: string
  concept_definition: string | null;  // was: string
}
```

No consumers of these fields exist yet (they were not used in split 01), so this is a safe change with no cascading updates.

### i18n keys

**Files to modify:**
- `frontend/src/i18n/locales/en/analysis.json`
- `frontend/src/i18n/locales/nb/analysis.json`

Add the following top-level keys to the existing JSON. Do not remove or modify any existing keys.

**English (`en/analysis.json`)** -- add these keys at the top level alongside existing `"list"`, `"status"`, `"create"`, `"settings"`, `"common"`:

```json
{
  "detail": {
    "backToList": "Back to analyses",
    "createdAt": "Created {{date}}",
    "inputType": "Input: {{type}}",
    "processing": {
      "banner": "Analysis in progress",
      "message": "This analysis is currently being processed. Results will appear automatically when complete."
    },
    "failed": {
      "message": "This analysis failed to process. You may delete it and try again."
    },
    "notFound": {
      "title": "Analysis not found",
      "message": "The analysis you are looking for does not exist or has been removed."
    }
  },
  "stats": {
    "totalFindings": "Total Findings",
    "addressed": "Addressed",
    "gaps": "Gaps",
    "frameworks": "Frameworks",
    "processingTime": "Processing Time",
    "tokenCount": "Token Count"
  },
  "charts": {
    "coverage": {
      "title": "Framework Coverage",
      "description": "Percentage of concepts addressed per framework",
      "noData": "No coverage data available"
    },
    "priority": {
      "title": "Priority Breakdown",
      "description": "Distribution of findings by priority level",
      "noData": "No priority data available"
    }
  },
  "findings": {
    "title": "Findings",
    "filters": {
      "framework": "Framework",
      "allFrameworks": "All Frameworks",
      "findingType": "Finding Type",
      "allTypes": "All Types",
      "priority": "Priority",
      "allPriorities": "All Priorities"
    },
    "columns": {
      "expand": "",
      "conceptCode": "Code",
      "conceptName": "Concept",
      "framework": "Framework",
      "type": "Type",
      "priority": "Priority",
      "confidence": "Confidence"
    },
    "expand": "Expand details",
    "collapse": "Collapse details",
    "evidence": "Evidence",
    "recommendation": "Recommendation",
    "conceptDefinition": "Concept Definition",
    "sourceReference": "Source Reference",
    "empty": {
      "title": "No compliance findings detected",
      "description": "The analysis did not detect any compliance findings. You may want to adjust the matcher thresholds.",
      "settingsLink": "Adjust matcher settings"
    },
    "type": {
      "addressed": "Addressed",
      "partially_addressed": "Partially Addressed",
      "gap": "Gap",
      "not_applicable": "Not Applicable"
    }
  },
  "export": {
    "pdf": "Export PDF",
    "docx": "Export DOCX",
    "disabled": "Analysis must be completed to export",
    "downloading": "Downloading...",
    "error": "Export failed. Please try again."
  }
}
```

**Norwegian (`nb/analysis.json`)** -- add these keys:

```json
{
  "detail": {
    "backToList": "Tilbake til analyser",
    "createdAt": "Opprettet {{date}}",
    "inputType": "Inndata: {{type}}",
    "processing": {
      "banner": "Analyse pågår",
      "message": "Denne analysen behandles nå. Resultatene vises automatisk når den er ferdig."
    },
    "failed": {
      "message": "Denne analysen feilet under behandling. Du kan slette den og prøve igjen."
    },
    "notFound": {
      "title": "Analyse ikke funnet",
      "message": "Analysen du leter etter finnes ikke eller er fjernet."
    }
  },
  "stats": {
    "totalFindings": "Totalt antall funn",
    "addressed": "Adressert",
    "gaps": "Mangler",
    "frameworks": "Rammeverk",
    "processingTime": "Behandlingstid",
    "tokenCount": "Antall tokens"
  },
  "charts": {
    "coverage": {
      "title": "Rammeverkdekning",
      "description": "Andel konsepter adressert per rammeverk",
      "noData": "Ingen dekningsdata tilgjengelig"
    },
    "priority": {
      "title": "Prioritetsfordeling",
      "description": "Fordeling av funn etter prioritetsnivå",
      "noData": "Ingen prioritetsdata tilgjengelig"
    }
  },
  "findings": {
    "title": "Funn",
    "filters": {
      "framework": "Rammeverk",
      "allFrameworks": "Alle rammeverk",
      "findingType": "Funntype",
      "allTypes": "Alle typer",
      "priority": "Prioritet",
      "allPriorities": "Alle prioriteter"
    },
    "columns": {
      "expand": "",
      "conceptCode": "Kode",
      "conceptName": "Konsept",
      "framework": "Rammeverk",
      "type": "Type",
      "priority": "Prioritet",
      "confidence": "Konfidens"
    },
    "expand": "Utvid detaljer",
    "collapse": "Skjul detaljer",
    "evidence": "Bevis",
    "recommendation": "Anbefaling",
    "conceptDefinition": "Konseptdefinisjon",
    "sourceReference": "Kildereferanse",
    "empty": {
      "title": "Ingen samsvarsfunn oppdaget",
      "description": "Analysen fant ingen samsvarsfunn. Du kan justere matcherterskelverdiene.",
      "settingsLink": "Juster matcherinnstillinger"
    },
    "type": {
      "addressed": "Adressert",
      "partially_addressed": "Delvis adressert",
      "gap": "Mangel",
      "not_applicable": "Ikke relevant"
    }
  },
  "export": {
    "pdf": "Eksporter PDF",
    "docx": "Eksporter DOCX",
    "disabled": "Analysen må være fullført for å eksportere",
    "downloading": "Laster ned...",
    "error": "Eksport feilet. Vennligst prøv igjen."
  }
}
```

These keys must be merged into the existing JSON files (not replace them). The existing keys (`title`, `list`, `status`, `create`, `settings`, `common`) remain unchanged.

### Route shell update

**File to modify:** `frontend/src/routes/analysis/$id.tsx`

Replace the stub with a page shell. The component should:

1. Extract `id` from `Route.useParams()`.
2. Call `useAnalysis(id)` with these options:
   - `refetchOnMount: 'always'` to ensure fresh data when navigating back.
   - `refetchInterval: 5000` conditionally, only when the analysis status is `"processing"`. Use the callback form: `(query) => query.state.data?.status === "processing" ? 5000 : false`.
3. Render conditionally based on state:
   - **Loading:** A skeleton placeholder (cards + table shape using shadcn Skeleton).
   - **Error (404):** "Not found" message with i18n key `detail.notFound.title` and `detail.notFound.message`, plus a `<Link to="/analysis">` back link.
   - **Error (other):** Generic error message using `common.error` with a back link.
   - **Processing status:** Page header (back link, title, StatusBadge, metadata) plus a processing banner (`detail.processing.banner` / `detail.processing.message`). No charts or table rendered.
   - **Failed status:** Page header plus error message (`detail.failed.message`).
   - **Completed with zero findings:** Page header plus `<EmptyFindings />` component (placeholder `div` for now; the real component comes in section 05).
   - **Completed with findings:** Page header plus placeholder slots for SummaryStats, ChartsSection, and FindingsSection (these are wired in section 06).
4. Use `useTranslation("analysis")` for all display strings.

The route shell at this stage does NOT need to call `useFindings` or render actual chart/table components. It only needs to handle the `useAnalysis` call and render the correct state. Section 06 (Page Assembly) will add the remaining data calls and component wiring.

**Page layout classes:** `max-w-7xl mx-auto p-6 space-y-6`

**Key imports needed:**
- `createFileRoute`, `Link` from `@tanstack/react-router`
- `useTranslation` from `react-i18next`
- `useAnalysis` from `@/features/analysis/api`
- `StatusBadge` from `@/features/analysis/components/StatusBadge`
- shadcn `Skeleton`, `Alert`, `AlertDescription` components

### Barrel export update

**File to modify:** `frontend/src/features/analysis/index.ts`

No new component exports are needed in this section (the route file is not exported from the barrel). This file will be updated in section 06 when all components are ready.

---

## File Summary

| File | Action |
|------|--------|
| `frontend/src/features/ontology/types/index.ts` | Modify: rename `PaginatedResponse.data` to `.items` |
| `frontend/src/features/ontology/api/index.ts` | Modify: update 3 occurrences of `.data` to `.items` on paginated responses |
| `frontend/src/features/ontology/components/Graph/GraphView.tsx` | Modify: update 1 occurrence of `data.data` to `data.items` |
| `frontend/src/features/ontology/components/Sidebar/Sidebar.tsx` | Modify: update 1 occurrence of `data.data` to `data.items` |
| `frontend/src/features/analysis/types/index.ts` | Modify: add `| null` to 5 fields in `AnalysisFinding` |
| `frontend/src/features/analysis/api/index.ts` | Modify: update `.data.some(...)` to `.items.some(...)` on line 43 |
| `frontend/src/i18n/locales/en/analysis.json` | Modify: add `detail`, `stats`, `charts`, `findings`, `export` keys |
| `frontend/src/i18n/locales/nb/analysis.json` | Modify: add Norwegian translations for same keys |
| `frontend/src/routes/analysis/$id.tsx` | Modify: replace stub with page shell |
| `frontend/src/features/analysis/types/__tests__/finding-nullability.test.ts` | Create: type-level nullability test |
| `frontend/src/routes/analysis/__tests__/$id.test.tsx` | Create: route shell tests |

---

## Dependencies

This section has no dependencies on other sections. All subsequent sections (02 through 06) depend on this section being completed first.