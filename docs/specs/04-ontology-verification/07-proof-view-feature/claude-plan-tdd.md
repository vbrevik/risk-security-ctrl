# TDD Plan — Split 07: Proof View Feature

Test framework: **Vitest** + `@testing-library/react`
Test location: co-located `__tests__/` directories
Mock pattern: `vi.mock("@/lib/api", () => ({ api: { get: vi.fn() } }))`
Hook test pattern: `renderHook()` with `QueryClientProvider` wrapper

Write the tests listed here BEFORE implementing each section.

---

## Section 1: Types and Dependencies

### Tests to write first

No runtime tests for TypeScript types (they are compile-time). But:

- **Type soundness test:** Verify `toVerificationStatus(null)` returns `"unknown"`
- **Type soundness test:** Verify `toVerificationStatus("verified")` returns `"verified"`
- **Type soundness test:** Verify `toVerificationStatus("banana")` returns `"unknown"`

These can be a simple `describe("toVerificationStatus", ...)` in an existing utilities test or a new `types.test.ts`. They confirm the normalization helper works correctly before any UI depends on it.

---

## Section 2: API Hook

### Tests to write first (in `api/__tests__/hooks.test.ts`)

- **Test:** `useFrameworkProof("nist-csf")` fetches `GET /api/ontology/frameworks/nist-csf/proof` and returns data
- **Test:** `useFrameworkProof(null)` does NOT call `api.get` at all (skipToken behavior)
- **Test:** Returned `data.proof_content` is `null` when API returns null (null passthrough)
- **Test:** Returned `data.verification_status` is `null` when API returns null
- **Test:** `isLoading` is `true` while fetch in flight, `false` after resolution

---

## Section 3: VerificationBadge Component

### Tests to write first (new `__tests__/VerificationBadge.test.tsx`)

- **Test:** `status="verified"` renders text matching i18n key `proof.status.verified`
- **Test:** `status="partially-verified"` renders correct label
- **Test:** `status="structure-verified"` renders correct label
- **Test:** `status="unverified"` renders correct label
- **Test:** `status="needs-correction"` renders correct label
- **Test:** `status="corrected"` renders correct label
- **Test:** `status={null}` renders without crashing (fallback/unknown style)
- **Test:** `status="some-unknown-value"` renders without crashing
- **Test:** rendered element has `aria-label` attribute present

---

## Section 4: ProofPanel Component

### Tests to write first (new `__tests__/ProofPanel.test.tsx`)

- **Test:** loading state renders skeleton elements (pulse animation class present)
- **Test:** error state renders error message and does NOT expose internal paths
- **Test:** when `proof_content` is non-null, a heading from the markdown content appears in the document
- **Test:** when `proof_content` is null, "No proof document available" (or i18n equivalent) appears
- **Test:** when `verification_source` is non-null, rendered `<a>` tag has `rel="noopener noreferrer"`
- **Test:** when `verification_source` is null, no external link is rendered

---

## Section 5: FrameworkProfile Integration

### Tests to write first (extend `__tests__/FrameworkProfile.test.tsx`)

The existing test file creates test data with `makeConcept()` and `FW` fixtures. Extend those fixtures to include `verification_status` field.

- **Test:** When `framework.verification_status` is non-null, `VerificationBadge` is rendered in the header area
- **Test:** When `framework.verification_status` is `null`, badge renders in fallback/neutral style (not absent)
- **Test:** When `framework.verification_status` is non-null, "View Proof" button is present
- **Test:** When `framework.verification_status` is `null`, "View Proof" button is absent
- **Test:** Clicking "View Proof" button causes `ProofPanel` to appear
- **Test:** When `framework.id` changes (simulate switching frameworks), proof panel is hidden

---

## Section 6: i18n Keys

No direct tests — translation key coverage is verified at build/typecheck time (if using typed i18n). Manually verify both `en` and `nb` locale files have the `proof` namespace after adding keys.

---

## Section 7: Build Verification Tests

Before marking implementation complete:
- `pnpm build` exits 0 with no TypeScript errors
- `pnpm typecheck` exits 0
- `pnpm test` exits 0 (all Vitest tests pass)
