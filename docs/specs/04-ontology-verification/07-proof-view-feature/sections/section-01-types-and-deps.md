I have all the information needed. Here is the section content:

# Section 01: Types and Dependencies

## Overview

This is the foundation section for the Proof View Feature (Split 07). It must be completed before any other section. No runtime tests exist here — TypeScript types are verified at compile time. However, a small helper function `toVerificationStatus` does have unit tests.

This section covers:
1. Installing three new npm packages
2. Configuring the Tailwind v4 typography plugin
3. Extending the `Framework` interface with verification fields
4. Adding the `VerificationStatus` union type, `FrameworkProof` interface, and `toVerificationStatus()` helper

**Blocks:** All other sections (02 through 06) depend on this section completing first.

---

## Tests First

The only testable logic in this section is the `toVerificationStatus()` normalization helper. Write these tests before implementing the helper.

**File to create:** `frontend/src/features/ontology/types/__tests__/types.test.ts`

The test suite is a simple `describe("toVerificationStatus", ...)` block:

```typescript
// Stub — fill in assertions once helper is implemented
describe("toVerificationStatus", () => {
  it("returns 'unknown' for null input");
  it("returns 'verified' for 'verified' input");
  it("returns 'unknown' for an unrecognized string like 'banana'");
  it("returns 'partially-verified' for 'partially-verified' input");
  it("returns 'needs-correction' for 'needs-correction' input");
});
```

Import the helper from `../index` (the types barrel). These tests are pure unit tests — no mocks, no React rendering, no QueryClient.

---

## 1. Install npm Packages

From the `frontend/` directory, run:

```
pnpm add react-markdown remark-gfm @tailwindcss/typography
```

- `react-markdown` — renders markdown as React elements (raw HTML is stripped by default, making it safe)
- `remark-gfm` — GitHub Flavored Markdown support: tables, task lists, strikethrough (proof files use GFM syntax)
- `@tailwindcss/typography` — provides the `prose` utility classes for styled markdown rendering

---

## 2. Configure Tailwind v4 Typography Plugin

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/index.css`

The project uses Tailwind v4 (`^4.1.18`) with CSS-first configuration. There is no `tailwind.config.js`. Plugins are registered with `@plugin` directives in the CSS file.

Add this line after the existing `@import "tailwindcss";` line (line 2):

```css
@plugin "@tailwindcss/typography";
```

Do not add a `tailwind.config.js` — the v4 CSS-first approach is already established in this project.

---

## 3. Extend the Framework Interface

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts`

The existing `Framework` interface (lines 2–10) does not capture the four verification fields that the `GET /api/ontology/frameworks` endpoint already returns. Add them as nullable fields:

```typescript
export interface Framework {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  source_url: string | null;
  created_at: string;
  updated_at: string;
  // Verification provenance (returned by the API, added in split 07)
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
}
```

The fields are typed as `string | null` (not as `VerificationStatus | null`) because the backend sends `Option<String>` — any string or null is possible. Type narrowing happens in the `toVerificationStatus()` helper at usage sites.

---

## 4. Add VerificationStatus Type and FrameworkProof Interface

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts`

Append the following to the types file (after the existing exports):

### VerificationStatus union type

```typescript
export type VerificationStatus =
  | "verified"
  | "partially-verified"
  | "structure-verified"
  | "corrected"
  | "unverified"
  | "needs-correction";
```

These are the six known status strings produced by the backend verification pipeline. A seventh `"unknown"` catch-all is intentionally excluded from this union — it is only used in the helper return type below.

### toVerificationStatus helper

```typescript
/**
 * Normalizes a raw backend verification_status string to a typed
 * VerificationStatus or "unknown" for null/unrecognized values.
 * Used by VerificationBadge and ProofPanel for safe style mapping.
 */
export function toVerificationStatus(value: string | null): VerificationStatus | "unknown";
```

The implementation checks `value` against the six known strings and returns `"unknown"` for anything else (including null). Use a Set or explicit string comparison — no regex needed.

### FrameworkProof interface

```typescript
/**
 * Response shape of GET /api/ontology/frameworks/{id}/proof
 * Used by useFrameworkProof hook and ProofPanel component.
 */
export interface FrameworkProof {
  framework_id: string;
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
  proof_content: string | null; // raw markdown; null if no proof file exists
}
```

`proof_content` is null for frameworks that have no proof file yet. Components must handle null gracefully with a "No proof document available" fallback (see section 04).

---

## File Summary

| File | Action | Notes |
|------|--------|-------|
| `frontend/src/index.css` | Modified | Added `@plugin "@tailwindcss/typography"` after `@import "tailwindcss"` |
| `frontend/src/features/ontology/types/index.ts` | Modified | Extended `Framework`, added `VerificationStatus`, `FrameworkProof`, `toVerificationStatus` |
| `frontend/src/features/ontology/types/__tests__/types.test.ts` | Created | 10 unit tests for `toVerificationStatus` (all 6 statuses, null, empty string, unknown, undefined) |
| `frontend/package.json` + `pnpm-lock.yaml` | Modified | Added react-markdown@10.1.0, remark-gfm@4.0.1, @tailwindcss/typography@0.5.19 |

## Deviations from Plan

- `KNOWN_STATUSES` kept as `ReadonlySet<string>` (not `ReadonlySet<VerificationStatus>`): TypeScript 5.x `Set<T>.has()` requires argument of type `T`, making `ReadonlySet<VerificationStatus>` incompatible with a `string` argument after the null check. The `as VerificationStatus` cast is runtime-safe due to the `has()` guard.
- Added 1 extra test: `undefined` input (covers missing JSON key at runtime).

---

## Downstream Dependencies

The following sections consume the types added here:

- **Section 02** (`section-02-api-hook`) — imports `FrameworkProof` as the hook's return data type
- **Section 03** (`section-03-verification-badge`) — imports `VerificationStatus` and `toVerificationStatus` for the status-to-style mapping
- **Section 04** (`section-04-proof-panel`) — imports `FrameworkProof` for its props and data shape
- **Section 05** (`section-05-framework-profile`) — uses extended `Framework` fields directly (no extra API call)

Do not proceed to any of those sections until `pnpm typecheck` passes after completing the changes here.