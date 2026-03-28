I have all the context I need. Here is the section content:

---

# Section 03: VerificationBadge Component

## Overview

This section creates the `VerificationBadge.tsx` component — a standalone, self-contained React component that displays a color-coded badge for a framework's verification status. It is used in `FrameworkProfile`'s header (Section 05) and inside `ProofPanel` (Section 04).

**Dependencies:** Section 01 (types and deps) must be complete before implementing this section — specifically `VerificationStatus` type and `toVerificationStatus()` helper from `frontend/src/features/ontology/types/index.ts`.

**Does not depend on:** Section 02 (API hook), Section 04, Section 05.

---

## Files to Create / Modify

| Action | Path |
|--------|------|
| CREATE | `frontend/src/features/ontology/components/VerificationBadge.tsx` |
| CREATE | `frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx` |

---

## Tests First

Write these tests in `frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx` before implementing the component.

The test file lives alongside the other component tests already in `__tests__/`. No special wrapper is needed since `VerificationBadge` has no TanStack Query or routing dependencies.

### Test stubs

```typescript
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { VerificationBadge } from "../VerificationBadge";

describe("VerificationBadge", () => {
  it('renders "Verified" label for status="verified"', () => { /* ... */ });
  it('renders correct label for status="partially-verified"', () => { /* ... */ });
  it('renders correct label for status="structure-verified"', () => { /* ... */ });
  it('renders correct label for status="unverified"', () => { /* ... */ });
  it('renders correct label for status="needs-correction"', () => { /* ... */ });
  it('renders correct label for status="corrected"', () => { /* ... */ });
  it("renders without crashing when status is null (fallback style)", () => { /* ... */ });
  it("renders without crashing when status is an unknown string", () => { /* ... */ });
  it("rendered element has aria-label attribute", () => { /* ... */ });
});
```

### Test notes

- Labels to check against are the i18n key values from `en/ontology.json` (Section 06 adds these). While implementing tests before i18n is wired, assert against the raw English strings: `"Verified"`, `"Partially Verified"`, `"Structure Verified"`, `"Corrected"`, `"Unverified"`, `"Needs Correction"`.
- For the null and unknown tests, assert that no exception is thrown and that the component renders something (e.g., a fallback element is in the document).
- For the `aria-label` test: use `screen.getByRole` or inspect the rendered container for the attribute. The badge element (the wrapper `<div>` or shadcn `<Badge>`) must carry `aria-label`.

---

## Implementation

### Props Interface

```typescript
interface VerificationBadgeProps {
  status: string | null;  // raw backend value; normalized internally via toVerificationStatus()
}
```

The component accepts the raw string (or null) as returned by the backend. It must never assume the value is one of the known statuses.

### Status-to-Style Mapping

Define a `BadgeConfig` type and a `BADGE_CONFIG` record inside the component file (not exported — implementation detail):

```typescript
type BadgeConfig = {
  colorClasses: string;
  Icon: React.ComponentType<{ className?: string; "aria-hidden"?: boolean | "true" }>;
  i18nKey: string;  // e.g. "proof.status.verified"
  label: string;    // English fallback for aria-label
};

const BADGE_CONFIG: Record<VerificationStatus | "unknown", BadgeConfig> = { ... };
```

Full mapping:

| Status key | Tailwind color classes | Icon (lucide-react) | English label |
|---|---|---|---|
| `verified` | `bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200` | `CheckCircle2` | `"Verified"` |
| `corrected` | `bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200` (slightly lighter acceptable) | `CheckCircle2` | `"Corrected"` |
| `partially-verified` | `bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200` | `AlertTriangle` | `"Partially Verified"` |
| `structure-verified` | `bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200` | `Info` | `"Structure Verified"` |
| `unverified` | `bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400` | `Circle` | `"Unverified"` |
| `needs-correction` | `bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200` | `XCircle` | `"Needs Correction"` |
| `unknown` | `bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400` | `Circle` | `"Unknown"` |

### Rendering

Use the shadcn `Badge` component (`import { Badge } from "@/components/ui/badge"`). Variant `"outline"` is recommended for a light, non-distracting appearance inside the framework header row.

Rendered structure (approximate):

```tsx
<Badge
  variant="outline"
  className={`inline-flex items-center gap-1.5 ${config.colorClasses}`}
  aria-label={config.label}
>
  <Icon className="h-3.5 w-3.5" aria-hidden="true" />
  <span>{t(config.i18nKey, config.label)}</span>
</Badge>
```

- The `aria-label` on the badge satisfies WCAG 1.4.1 — the meaning is not conveyed by color alone.
- Icons use `aria-hidden="true"` since they are decorative; the label text already conveys the status.
- The `t(key, fallback)` form of `useTranslation` passes the English string as a default, which also makes the tests work before i18n keys are wired in Section 06.

### Normalization

Call `toVerificationStatus(status)` (from Section 01, `frontend/src/features/ontology/types/index.ts`) at the top of the render function to map the raw prop to the record key:

```typescript
const normalized = toVerificationStatus(status); // "verified" | ... | "unknown"
const config = BADGE_CONFIG[normalized];
```

### Full component stub

```typescript
// frontend/src/features/ontology/components/VerificationBadge.tsx

import React from "react";
import { useTranslation } from "react-i18next";
import { CheckCircle2, AlertTriangle, Info, Circle, XCircle } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { toVerificationStatus } from "../types";
import type { VerificationStatus } from "../types";

interface VerificationBadgeProps {
  status: string | null;
}

// BadgeConfig and BADGE_CONFIG defined here (not exported)

export function VerificationBadge({ status }: VerificationBadgeProps) {
  const { t } = useTranslation("ontology");
  const normalized = toVerificationStatus(status);
  const config = BADGE_CONFIG[normalized];
  const { Icon } = config;

  return (
    <Badge
      variant="outline"
      className={`inline-flex items-center gap-1.5 ${config.colorClasses}`}
      aria-label={config.label}
    >
      <Icon className="h-3.5 w-3.5" aria-hidden />
      <span>{t(config.i18nKey, config.label)}</span>
    </Badge>
  );
}
```

---

## Accessibility Notes

- The `aria-label` on `<Badge>` provides the full status description independent of color or icon. Screen readers will announce the label.
- Icons must have `aria-hidden={true}` (or the string `"true"` — both are accepted by React).
- Do not rely on Tailwind color classes alone to convey state — the text label inside the badge is required for WCAG 1.4.1 compliance.

---

## Dependency Notes

- `toVerificationStatus` and `VerificationStatus` must be exported from `frontend/src/features/ontology/types/index.ts` before this component compiles (Section 01).
- i18n keys under `proof.status.*` are added in Section 06. Use `t(key, fallback)` so the component works correctly during development even before those keys exist.
- The shadcn `Badge` component already exists at `frontend/src/components/ui/badge.tsx` — no installation needed.
- `lucide-react` is already a project dependency.

## Actual Files Created

| File | Notes |
|------|-------|
| `frontend/src/features/ontology/components/VerificationBadge.tsx` | Created with BADGE_CONFIG record, all 6 statuses + unknown |
| `frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx` | 9 tests; i18n mocked with t returning fallback string |

## Deviations from Plan

- `aria-label` uses `t(config.i18nKey, config.label)` (translated value) instead of `config.label` (hardcoded English) — code review fix to avoid English aria-label in Norwegian locale.
- i18n mock uses `t: (key, fallback) => fallback ?? key` so tests assert against English fallback strings ("Verified" etc.) rather than keys.