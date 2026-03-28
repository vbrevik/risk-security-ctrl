Now I have all the context I need. Let me generate the section content for `section-05-framework-profile`.

# Section 05: FrameworkProfile Integration

## Overview

This section modifies the existing `FrameworkProfile` component to surface verification data to users. By the end of this section:

- A `VerificationBadge` appears in the framework header row alongside the name and version badge
- A "View Proof" toggle button appears below the source link (when verification metadata is present)
- Clicking the button mounts `<ProofPanel>` between the header block and the Stats Strip
- Switching frameworks collapses the proof panel automatically

**Dependencies:** This section requires sections 01, 02, 03, and 04 to be complete:
- Section 01: `Framework` interface must have `verification_status`, `verification_date`, `verification_source`, `verification_notes` fields
- Section 02: `useFrameworkProof` hook must exist (used inside `ProofPanel`)
- Section 03: `VerificationBadge` component must exist
- Section 04: `ProofPanel` component must exist

---

## Files to Modify

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/FrameworkProfile.tsx` — main changes
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx` — add new tests

---

## Tests First

Extend the existing test file at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx`.

The existing `FW` fixture must be updated to include `verification_status` and the other verification fields now present on the `Framework` interface. The test helper renders `FrameworkProfile` with a `QueryClientProvider` wrapper because `ProofPanel` (rendered when "View Proof" is clicked) calls `useFrameworkProof` internally.

### Fixture Updates

Add verification fields to the `FW` fixture object:

```typescript
const FW: Framework = {
  // ... existing fields ...
  verification_status: "verified",
  verification_date: "2025-01-15",
  verification_source: "https://example.com/proof",
  verification_notes: null,
};

const FW_NO_VERIFICATION: Framework = {
  // Same as FW_B but with explicit null verification fields
  id: "nist-csf",
  name: "NIST CSF",
  version: "2.0",
  description: null,
  source_url: null,
  created_at: "",
  updated_at: "",
  verification_status: null,
  verification_date: null,
  verification_source: null,
  verification_notes: null,
};
```

### Test Cases to Add

Add a new `describe("FrameworkProfile – verification UI", ...)` block containing these tests:

**1. Badge renders when `verification_status` is non-null**

Render `FrameworkProfile` with `framework={FW}` (which has `verification_status: "verified"`). Assert that a verification badge element is present in the document (look for the `aria-label` attribute set on the badge, or a test ID if one is added).

**2. Badge renders in fallback/neutral style when `verification_status` is null**

Render with `framework={FW_NO_VERIFICATION}` (null `verification_status`). The badge should still render — it shows the "unknown" fallback styling. Assert the badge element is present and does not throw.

**3. "View Proof" button is present when `verification_status` is non-null**

Render with `FW`. Assert `screen.getByRole("button", { name: /view proof/i })` is in the document.

**4. "View Proof" button is absent when `verification_status` is null**

Render with `FW_NO_VERIFICATION`. Assert `screen.queryByRole("button", { name: /view proof/i })` returns null.

**5. Clicking "View Proof" mounts ProofPanel**

Render with `FW`. Click the "View Proof" button via `fireEvent.click(...)` or `userEvent.click(...)`. Assert that a `ProofPanel` indicator is present in the document (e.g., the loading skeleton or a `data-testid="proof-panel"` if added, or any element that ProofPanel uniquely renders). Mock `useFrameworkProof` to return a stable loading state to prevent flakiness.

**6. Switching frameworks hides the proof panel**

Render with `FW`, click "View Proof", assert panel is visible, then rerender with `FW_NO_VERIFICATION`. Assert the proof panel is no longer visible. This tests the `useEffect` reset.

### Required Test Wrappers

Because `ProofPanel` uses TanStack Query internally, the test render must wrap with `QueryClientProvider`:

```typescript
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
}
```

Use `render(<FrameworkProfile ... />, { wrapper: createWrapper() })` for tests in the new describe block.

Mock the `useFrameworkProof` hook via `vi.mock("../../api", () => ({ useFrameworkProof: vi.fn(() => ({ isLoading: true, isError: false, data: undefined })) }))` or equivalent to prevent real HTTP calls. Adjust the mock path to match the actual import path used in `ProofPanel`.

---

## Implementation Details

### 5.1 New Imports

Add to the imports at the top of `FrameworkProfile.tsx`:

```typescript
import { useState, useEffect } from "react"; // useState already there; add if missing
import { ShieldCheck } from "lucide-react";   // add to existing lucide import
import { VerificationBadge } from "./VerificationBadge";
import { ProofPanel } from "./ProofPanel";
import { useTranslation } from "react-i18next";
```

The `useTranslation` hook is needed for the "View Proof" / "Hide Proof" button labels.

### 5.2 State

Inside the `FrameworkProfile` function body (after the existing `expanded` state), add:

```typescript
const [showProof, setShowProof] = useState(false);
const { t } = useTranslation("ontology");
```

### 5.3 Reset on Framework Change

Add a `useEffect` alongside the existing one that resets `expanded`:

```typescript
useEffect(() => {
  setShowProof(false);
}, [framework?.id]);
```

This ensures the proof panel collapses automatically when the user selects a different framework from the sidebar.

### 5.4 Header Row Changes

The current header flex row is:

```tsx
<div className="flex items-center gap-3 mb-2">
  <h2 className="text-2xl font-bold font-mono">{framework.name}</h2>
  {framework.version && (
    <span className="tech-badge">{framework.version}</span>
  )}
</div>
```

Add `<VerificationBadge>` inline in this flex row, after the version badge:

```tsx
<div className="flex items-center gap-3 mb-2">
  <h2 className="text-2xl font-bold font-mono">{framework.name}</h2>
  {framework.version && (
    <span className="tech-badge">{framework.version}</span>
  )}
  <VerificationBadge status={framework.verification_status} />
</div>
```

The badge is always rendered when a framework is selected. When `verification_status` is null, `VerificationBadge` renders the neutral "unknown" style — it never returns null.

### 5.5 "View Proof" Button

Below the source link anchor (or in its place if no source URL), add the toggle button. The button is conditionally shown only when `framework.verification_status !== null`:

```tsx
{framework.verification_status !== null && (
  <button
    onClick={() => setShowProof((prev) => !prev)}
    className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors mt-1"
  >
    <ShieldCheck className="w-3 h-3" />
    {showProof ? t("proof.hideProof") : t("proof.viewProof")}
  </button>
)}
```

Place this after the `framework.source_url` block so both the source link and the proof button can be visible simultaneously.

### 5.6 ProofPanel Placement

After the entire header `<div>` block (the one containing name, description, source link, and proof button), and before the Stats Strip section, add the conditional ProofPanel mount:

```tsx
{showProof && <ProofPanel frameworkId={framework.id} />}
```

The structural order in the returned JSX should be:

1. Header `<div>` (name, version badge, VerificationBadge, description, source link, View Proof button)
2. `{showProof && <ProofPanel frameworkId={framework.id} />}`
3. Stats Strip (`{stats && ...}`)
4. Concept Type Breakdown
5. Cross-Framework Connections
6. Concept Hierarchy

### 5.7 i18n Note

The `t("proof.viewProof")` and `t("proof.hideProof")` keys are added in Section 06 (i18n). If implementing this section before Section 06 is complete, the button will render the raw key string as fallback — this is acceptable during development.

---

## Behavior Summary

| Condition | Badge | Button | ProofPanel |
|-----------|-------|--------|------------|
| `framework` is null | — (empty state) | — | — |
| `verification_status` is null | Renders (neutral/unknown style) | Hidden | Never shown |
| `verification_status` is non-null, `showProof` false | Renders (colored) | "View Proof" | Hidden |
| `verification_status` is non-null, `showProof` true | Renders (colored) | "Hide Proof" | Mounted |
| Framework switches | Stays visible | Resets to "View Proof" | Unmounted |

---

## No Extra API Calls

The badge reads `framework.verification_status` directly from the `Framework` object, which is already in the TanStack Query cache from `useFrameworks()`. No additional network request is made for the badge. The proof endpoint is only called by `ProofPanel` after the user clicks "View Proof".

---

## Build Check After This Section

After completing this section run from `frontend/`:

```
pnpm typecheck
pnpm test
```

TypeScript will catch any mismatch between the `Framework` interface (updated in Section 01) and the props passed to `VerificationBadge` or `ProofPanel`. If Section 01 is not yet complete, compilation will fail on `framework.verification_status` — complete Section 01 first.

---

## Actual Files Modified

| File | Notes |
|------|-------|
| `frontend/src/features/ontology/components/FrameworkProfile.tsx` | Added VerificationBadge, showProof state, useEffect reset, View/Hide Proof button, ProofPanel mount; renamed `t` iteration var to `relType`; fixed "Source" hardcode to `t("common.source", "Source")` |
| `frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx` | Updated FW/FW_B fixtures with verification fields; added 7 tests in "FrameworkProfile – verification UI" describe block |

## Deviations from Plan

- **`t` variable shadowing fix**: `conn.types.map((t) => ...)` renamed to `conn.types.map((relType) => ...)` to avoid shadowing `useTranslation`'s `t` function. Not in original plan but required for correctness.
- **"Source" i18n fix**: Applied `t("common.source", "Source")` per CLAUDE.md internationalization rule. Spec showed raw `"Source"` literal; fixed via code review.
- **useEffect reset test added for two-verified-framework scenario**: Original plan's "hides panel when framework changes" test only tested switching to a null-status framework. Added `FW_C` test to cover `useEffect` firing when switching between two non-null-status frameworks (the actual reset path).
- **t() fallbacks added to View/Hide Proof button**: Button uses `t("proof.hideProof", "Hide Proof")` and `t("proof.viewProof", "View Proof")` with English fallbacks so tests pass without section-06 i18n keys loaded.

## Final Test Count

288 tests passing, 0 failing. `pnpm typecheck` clean.