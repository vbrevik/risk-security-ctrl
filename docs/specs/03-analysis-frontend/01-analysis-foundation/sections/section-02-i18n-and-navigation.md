Now I have all the context needed. Let me generate the section content.

# Section 2: i18n and Navigation

## Overview

This section creates the analysis translation files (English and Norwegian), registers the `analysis` namespace in the i18n configuration, adds the "Analysis" navigation link to the root layout, and creates route file stubs for all four analysis routes. After completing this section, the analysis feature will be accessible via the navigation bar and all route paths will resolve (even if the page content is built in later sections).

## Dependencies

- **section-01-types-and-hooks** must be completed first (the route files import hooks from `@/features/analysis/api`)
- No dependency on section-03 (shadcn components) for this section

## Tests First

### i18n Registration Test

Create the file `src/i18n/__tests__/analysis-namespace.test.ts`. This test verifies that the analysis namespace loads correctly and returns actual translated strings rather than raw keys.

```typescript
// File: frontend/src/i18n/__tests__/analysis-namespace.test.ts

import { describe, it, expect } from "vitest";
import i18n from "../index";

describe("analysis i18n namespace", () => {
  it("analysis namespace is registered and loadable", () => {
    const bundle = i18n.getResourceBundle("en", "analysis");
    expect(bundle).toBeDefined();
    expect(bundle.title).toBeTruthy();
  });

  it("key access returns translated string, not the key itself", () => {
    const result = i18n.t("title", { ns: "analysis", lng: "en" });
    expect(result).not.toBe("title");
    expect(result).toBe("Document Analysis");
  });

  it("nb locale has analysis namespace", () => {
    const bundle = i18n.getResourceBundle("nb", "analysis");
    expect(bundle).toBeDefined();
    expect(bundle.title).toBeTruthy();
  });
});
```

### Navigation Test

Create the file `src/routes/__tests__/analysis-nav.test.tsx`. This test verifies the Analysis link appears in navigation and points to `/analysis`. Follow the exact pattern from the existing `root-nav.test.tsx` file.

```typescript
// File: frontend/src/routes/__tests__/analysis-nav.test.tsx

import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import {
  createRootRoute,
  createRoute,
  createRouter,
  createMemoryHistory,
  RouterProvider,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: Record<string, string> = {
        appName: "RSC",
        "nav.home": "Home",
        "nav.ontology": "Ontology Explorer",
        "nav.compliance": "Compliance",
        "nav.reports": "Reports",
        "nav.frameworks": "Frameworks",
        "nav.crosswalk": "Crosswalk",
        "nav.landscape": "Landscape",
        "nav.search": "Search",
        "nav.analysis": "Analysis",
      };
      return translations[key] ?? key;
    },
    i18n: { language: "en", changeLanguage: vi.fn() },
  }),
}));

/**
 * Test root layout mirroring __root.tsx after the Analysis link is added.
 * Includes the new /analysis link between Search and Compliance.
 */
function TestRootLayout() {
  /* layout mirrors __root.tsx nav ordering */
}

function renderWithRouter(initialPath = "/") {
  /* same QueryClient + createRouter pattern as root-nav.test.tsx,
     but route list includes createRoute for /analysis */
}

describe("Analysis navigation link", () => {
  it("renders Analysis link in the navigation", async () => {
    /* render at "/", find nav, assert a link with text "Analysis" exists */
  });

  it("Analysis link points to /analysis", async () => {
    /* render at "/", find the Analysis link, assert href === "/analysis" */
  });
});
```

The test stubs above follow the exact structure of the existing `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/routes/__tests__/root-nav.test.tsx`. Use `createMemoryHistory`, `createRootRoute`, `createRoute`, `createRouter`, `RouterProvider`, and a `QueryClientProvider` wrapper. The `TestRootLayout` must include the `/analysis` link in the position described below (after Search, before Compliance).

**Important:** The existing `root-nav.test.tsx` will also need updating -- its assertions about "8 navigation links" and the `hrefs` array must be updated to include the new `/analysis` link (9 links total, `/analysis` inserted between `/concepts/search` and `/compliance`).

---

## Implementation Details

### 1. Create English Translation File

**File:** `frontend/src/i18n/locales/en/analysis.json`

Create a JSON file with these top-level keys and nested structure:

- `"title"`: `"Document Analysis"`
- `"list"` object: `title`, `newAnalysis`, `empty` (nested: `title`, `description`), `filters` (nested: `status`, `all`)
- `"status"` object: `pending`, `processing`, `completed`, `failed` -- human-readable labels for the `StatusBadge` component
- `"create"` object: `title`, `nameLabel`, `namePlaceholder`, `descriptionLabel`, `textTab`, `uploadTab`, `textPlaceholder`, `dropzoneText`, `dropzoneBrowse`, `uploading`, `submit`, `maxFileSize`, `invalidFileType`, `fileTooLarge`, `success`
- `"settings"` object: `title`, `thresholds` (section heading), `minConfidence`, `addressedThreshold`, `partialThreshold`, `maxFindings`, `includeAddressed`, `boostTerms` (section heading), `termLabel`, `weightLabel`, `addTerm`, `save`, `saved`, `resetDefaults`, `resetConfirm`
- `"common"` object: `back`, `delete`, `deleteConfirm`, `cancel`, `error`

English values should be natural, user-facing English strings. For example:
- `list.empty.title` = `"No analyses yet"`
- `list.empty.description` = `"Create your first analysis to get started."`
- `create.dropzoneText` = `"Drag and drop a file here, or"`
- `create.dropzoneBrowse` = `"browse"`
- `create.fileTooLarge` = `"File exceeds the maximum size of 25MB"`
- `settings.resetConfirm` = `"Reset all settings to defaults?"`

### 2. Create Norwegian Translation File

**File:** `frontend/src/i18n/locales/nb/analysis.json`

Identical key structure to the English file, but with Norwegian Bokmal values. For example:
- `"title"`: `"Dokumentanalyse"`
- `list.newAnalysis` = `"Ny analyse"`
- `list.empty.title` = `"Ingen analyser ennå"`
- `status.pending` = `"Venter"`
- `status.processing` = `"Behandler"`
- `status.completed` = `"Fullført"`
- `status.failed` = `"Feilet"`
- `create.title` = `"Ny analyse"`
- `settings.title` = `"Matcherkonfigurasjon"`
- `common.back` = `"Tilbake"`
- `common.delete` = `"Slett"`
- `common.cancel` = `"Avbryt"`

### 3. Register Namespace in i18n Config

**File to modify:** `frontend/src/i18n/index.ts`

Add two import lines following the existing pattern:

```typescript
import enAnalysis from "./locales/en/analysis.json";
import nbAnalysis from "./locales/nb/analysis.json";
```

Add entries to the `resources` object:

```typescript
const resources = {
  en: {
    common: enCommon,
    ontology: enOntology,
    compliance: enCompliance,
    reports: enReports,
    analysis: enAnalysis,   // <-- add
  },
  nb: {
    common: nbCommon,
    ontology: nbOntology,
    compliance: nbCompliance,
    reports: nbReports,
    analysis: nbAnalysis,   // <-- add
  },
};
```

### 4. Add Navigation Key to common.json

**File to modify:** `frontend/src/i18n/locales/en/common.json`

Add `"analysis": "Analysis"` inside the `"nav"` object.

**File to modify:** `frontend/src/i18n/locales/nb/common.json`

Add `"analysis": "Analyse"` inside the `"nav"` object.

### 5. Add Analysis Link to Root Layout

**File to modify:** `frontend/src/routes/__root.tsx`

Insert a new `<Link>` element for `/analysis` in the `<nav>` section. The plan specifies it should go **after the second separator dot** (which is after the Search link) and **before the Compliance link**.

Looking at the current `__root.tsx` (lines 67-68), the second separator `<span>` is at line 68, followed by Compliance at line 69. The new link goes between them:

```tsx
<span className="text-border mx-1.5">·</span>
<Link
  to="/analysis"
  className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
>
  {t("nav.analysis")}
</Link>
<Link
  to="/compliance"
  ...
```

The `className` follows the exact same pattern as every other nav link in the file.

### 6. Create Route File Stubs

Create four route files under `frontend/src/routes/analysis/`. These are minimal stubs that establish the routes. Sections 04, 05, and 06 will flesh out the actual page content.

**File:** `frontend/src/routes/analysis/index.tsx`

```typescript
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/")({
  component: AnalysisListPage,
});

function AnalysisListPage() {
  return <div>Analysis list page — implemented in section-04</div>;
}
```

**File:** `frontend/src/routes/analysis/create.tsx`

```typescript
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/create")({
  component: CreateAnalysisPage,
});

function CreateAnalysisPage() {
  return <div>Create analysis page — implemented in section-05</div>;
}
```

**File:** `frontend/src/routes/analysis/settings.tsx`

```typescript
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/settings")({
  component: AnalysisSettingsPage,
});

function AnalysisSettingsPage() {
  return <div>Settings page — implemented in section-06</div>;
}
```

**File:** `frontend/src/routes/analysis/$id.tsx`

This is a placeholder for split 02 (analysis detail and charts). It should render a minimal message with a back link:

```typescript
import { createFileRoute, Link } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/$id")({
  component: AnalysisDetailPage,
});

function AnalysisDetailPage() {
  const { id } = Route.useParams();
  return (
    <div>
      <Link to="/analysis">&larr; Back</Link>
      <p>Analysis detail page for {id} — coming in split 02</p>
    </div>
  );
}
```

### 7. Update Existing Navigation Test

**File to modify:** `frontend/src/routes/__tests__/root-nav.test.tsx`

The existing test asserts exactly 8 navigation links and a specific `hrefs` array. After adding the Analysis link, update:

- The `TestRootLayout` function to include `<Link to="/analysis">Analysis</Link>` between Search and Compliance
- The routes array to include `createRoute` for `/analysis`
- The mock translations to include `"nav.analysis": "Analysis"`
- The assertion from `toHaveLength(8)` to `toHaveLength(9)`
- The `hrefs` array assertion to include `"/analysis"` between `"/concepts/search"` and `"/compliance"`

---

## Files Summary

| Action | File Path |
|--------|-----------|
| Create | `frontend/src/i18n/locales/en/analysis.json` |
| Create | `frontend/src/i18n/locales/nb/analysis.json` |
| Modify | `frontend/src/i18n/index.ts` |
| Modify | `frontend/src/i18n/locales/en/common.json` |
| Modify | `frontend/src/i18n/locales/nb/common.json` |
| Modify | `frontend/src/routes/__root.tsx` |
| Create | `frontend/src/routes/analysis/index.tsx` |
| Create | `frontend/src/routes/analysis/create.tsx` |
| Create | `frontend/src/routes/analysis/settings.tsx` |
| Create | `frontend/src/routes/analysis/$id.tsx` |
| Create | `frontend/src/i18n/__tests__/analysis-namespace.test.ts` |
| Create | `frontend/src/routes/__tests__/analysis-nav.test.tsx` |
| Modify | `frontend/src/routes/__tests__/root-nav.test.tsx` |