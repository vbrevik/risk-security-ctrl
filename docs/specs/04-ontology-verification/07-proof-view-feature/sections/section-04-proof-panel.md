Now I have all the context needed to write the section. Let me generate the complete, self-contained section for `section-04-proof-panel`.

# Section 04: ProofPanel Component

## Overview

This section creates the `ProofPanel` component at `frontend/src/features/ontology/components/ProofPanel.tsx`. The panel is mounted inside `FrameworkProfile` (handled in section-05) when the user clicks "View Proof". It owns its own data fetching via `useFrameworkProof`, so data loading only happens when the user explicitly opens it.

## Dependencies

This section depends on:
- **section-01** (types): `FrameworkProof` interface, `toVerificationStatus` helper
- **section-02** (API hook): `useFrameworkProof(frameworkId: string | null)`
- **section-03** (VerificationBadge): `VerificationBadge` component

Do not duplicate those implementations here. Assume they exist and are importable from their respective modules.

The `react-markdown`, `remark-gfm`, and `@tailwindcss/typography` packages must already be installed (section-01). The Tailwind `@plugin "@tailwindcss/typography"` directive must be present in the main CSS file.

---

## Tests First

Create `frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx`.

The test file should import `ProofPanel` from `../ProofPanel`, mock `useFrameworkProof` from the API module, and wrap renders in a `QueryClientProvider` with a fresh `QueryClient`.

The mock pattern follows the project convention:
```typescript
vi.mock("../../api", () => ({ useFrameworkProof: vi.fn() }));
```

Tests to implement:

1. **Loading state** — when `useFrameworkProof` returns `{ isLoading: true, isError: false, data: undefined }`, three skeleton `div` elements with the class `animate-pulse` are present in the document.

2. **Error state** — when `useFrameworkProof` returns `{ isLoading: false, isError: true, data: undefined }`, a generic error message is rendered. The rendered output must NOT contain any file paths (e.g., `docs/sources/`), internal API URLs, or raw error objects.

3. **With proof_content** — when data has a non-null `proof_content` (e.g., `"# Verification\nSome text"`), text from the markdown (e.g., "Verification") appears in the document after rendering.

4. **Without proof_content (null)** — when `proof_content` is null, the i18n string for `"ontology:proof.noProof"` appears in the document ("No proof document available" in English).

5. **Source link security** — when `verification_source` is a non-null URL, the rendered `<a>` element has `rel="noopener noreferrer"` and `target="_blank"`.

6. **Source link absent** — when `verification_source` is null, no `<a>` with an external URL is rendered for the source field.

Stub template:
```typescript
// frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { ProofPanel } from "../ProofPanel";

vi.mock("../../api", () => ({
  useFrameworkProof: vi.fn(),
}));

// Helper: build a minimal FrameworkProof payload
function makeProof(overrides: Partial<...> = {}): FrameworkProof { ... }

describe("ProofPanel", () => {
  it("renders skeleton elements while loading", ...);
  it("renders generic error message without internal paths on error", ...);
  it("renders markdown content heading when proof_content is non-null", ...);
  it("renders no-proof message when proof_content is null", ...);
  it("source link has rel=noopener noreferrer when verification_source is present", ...);
  it("no external link rendered when verification_source is null", ...);
});
```

---

## Implementation

### File to create

`frontend/src/features/ontology/components/ProofPanel.tsx`

### Props interface

```typescript
interface ProofPanelProps {
  frameworkId: string;
}
```

The component takes only `frameworkId`. All other data comes from `useFrameworkProof` internally.

### Component states

The component renders four mutually exclusive states based on the query result:

**Loading state**

Render three skeleton rows. Use Tailwind classes consistent with the existing loading skeletons in `FrameworkProfile`:

```
<div className="h-4 w-full bg-muted rounded animate-pulse" />
```

Repeat three times inside a `space-y-2` wrapper.

**Error state**

Render a short, generic human-readable error message in a muted style. Do not propagate the raw error object, internal file paths, or API details into the DOM. A message like "Could not load proof document." is sufficient.

**Success with `proof_content` non-null**

Render in this order:
1. **Metadata row** — see below
2. **`<hr />`** divider (or equivalent visual separator)
3. **Markdown content** — `proof_content` rendered with `<ReactMarkdown remarkPlugins={[remarkGfm]}>` inside a scrollable, styled container

**Success with `proof_content` null**

Render:
1. Metadata row (same as above)
2. A paragraph with the text from `"ontology:proof.noProof"` (i18n key) in a muted style

### Metadata row

The metadata row is always shown in a successful state (regardless of whether `proof_content` is present). It contains:

- `<VerificationBadge status={data.verification_status} />` — imported from `./VerificationBadge`
- Formatted date: if `verification_date` is non-null, display it preceded by the i18n label `"ontology:proof.date"`. Parse the date string with the browser's `Date` constructor and format with `toLocaleDateString()`.
- Source link: if `verification_source` is non-null, render as:
  ```html
  <a href="{verification_source}" target="_blank" rel="noopener noreferrer">
    <ExternalLink className="w-3 h-3" /> {t("ontology:proof.source")}
  </a>
  ```
  `ExternalLink` is imported from `lucide-react`. This follows the exact pattern already used in `FrameworkProfile` for `source_url`.
- Notes: if `verification_notes` is non-null, render as a `<p>` with the label `"ontology:proof.notes"` followed by the notes text.

### Markdown container

The markdown content block must be scrollable and have bounded height to prevent layout overflow:

```typescript
<div className="max-h-96 overflow-y-auto prose prose-sm dark:prose-invert">
  {memoized ReactMarkdown element}
</div>
```

Memoize the `<ReactMarkdown>` element with `useMemo` keyed on `proof_content` to prevent re-renders when the parent re-renders.

```typescript
const renderedMarkdown = useMemo(
  () => (
    <ReactMarkdown remarkPlugins={[remarkGfm]}>
      {data.proof_content!}
    </ReactMarkdown>
  ),
  [data.proof_content]
);
```

### Imports required

```typescript
import React, { useMemo } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useFrameworkProof } from "../api";
import { VerificationBadge } from "./VerificationBadge";
```

### Full component skeleton

```typescript
// frontend/src/features/ontology/components/ProofPanel.tsx

interface ProofPanelProps {
  frameworkId: string;
}

/**
 * Displays verification provenance for a framework.
 * Fetches proof data lazily when mounted.
 * Renders: loading skeleton | error message | metadata + optional markdown.
 */
export function ProofPanel({ frameworkId }: ProofPanelProps) {
  const { t } = useTranslation("ontology");
  const { data, isLoading, isError } = useFrameworkProof(frameworkId);

  if (isLoading) { /* three animate-pulse skeleton rows */ }
  if (isError) { /* generic error message */ }

  const renderedMarkdown = useMemo(..., [data?.proof_content]);

  return (
    <div className="...">
      {/* Metadata row: badge + date + source + notes */}
      {/* Divider or noProof message + conditional markdown block */}
    </div>
  );
}
```

---

## Notes for Implementer

- The `useTranslation("ontology")` call requires i18n keys in the `proof` namespace (section-06 adds these, but the component will still compile without them — keys will fall back to the key string).
- The `prose dark:prose-invert` classes require the `@tailwindcss/typography` plugin to be active (configured in section-01). Without the plugin, these classes are no-ops but the component still renders correctly.
- The component is intentionally stateless beyond what TanStack Query provides. No local loading booleans or error state.
- `useFrameworkProof` is called with `frameworkId` (a `string`, never `null` here — the parent only mounts `ProofPanel` when a framework is selected). The `skipToken` handling in the hook is for callers that may pass `null`; `ProofPanel` always provides a real ID.
- After implementing, add `ProofPanel` to the barrel export at `frontend/src/features/ontology/components/index.ts` if that file exists.

## Actual Files Created

| File | Notes |
|------|-------|
| `frontend/src/features/ontology/components/ProofPanel.tsx` | Created with 4 states: loading, error, success+content, success+noProof |
| `frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx` | 6 tests; useFrameworkProof mocked, VerificationBadge mocked |

## Deviations from Plan

- `MarkdownContent` extracted to a separate function component to encapsulate the useMemo pattern cleanly.
- Error test assertion strengthened to check exact safe message text (code review fix).
- i18n fallback strings kept per spec — section-06 will add the locale keys.