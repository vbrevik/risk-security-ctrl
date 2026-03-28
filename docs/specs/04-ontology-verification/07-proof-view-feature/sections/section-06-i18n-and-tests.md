I now have all the context I need. Let me generate the section content.

# Section 06: i18n Keys, Barrel Export, and Build Verification

## Overview

This is the final section of the proof view feature implementation. It completes the feature by:

1. Adding the `"proof"` translation namespace to both locale files
2. Exporting the two new components from the ontology barrel file
3. Verifying the full build passes cleanly

**Dependencies:** Sections 01-05 must be complete before this section. The i18n keys must exist before tests in prior sections can resolve translation strings correctly.

---

## Files to Modify

| File | Change |
|------|--------|
| `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/i18n/locales/en/ontology.json` | Add `"proof"` key block |
| `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/i18n/locales/nb/ontology.json` | Add Norwegian `"proof"` key block |
| `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/index.ts` | Export `VerificationBadge` and `ProofPanel` |

---

## Tests

Per the TDD plan, there are no direct runtime tests for i18n — translation key coverage is validated at build and typecheck time. However, the tests written in sections 02-05 depend on these keys being present. If tests fail with missing translation strings, it is a symptom of this section not being applied yet.

The section's own verification is the build pipeline:

- `pnpm build` must exit 0 with zero TypeScript errors
- `pnpm typecheck` must exit 0
- `pnpm test` must exit 0 (all Vitest tests pass)

Run these from `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/`.

---

## Implementation

### 1. English locale — `en/ontology.json`

Add the `"proof"` block as a sibling to the existing top-level keys (e.g., after `"crosswalk"`):

```json
"proof": {
  "viewProof": "View Proof",
  "hideProof": "Hide Proof",
  "date": "Verified",
  "source": "Source",
  "notes": "Notes",
  "noProof": "No proof document available",
  "status": {
    "verified": "Verified",
    "partially-verified": "Partially Verified",
    "structure-verified": "Structure Verified",
    "corrected": "Corrected",
    "unverified": "Unverified",
    "needs-correction": "Needs Correction"
  }
}
```

### 2. Norwegian locale — `nb/ontology.json`

Add the Norwegian `"proof"` block as a sibling after `"crosswalk"`:

```json
"proof": {
  "viewProof": "Vis bevis",
  "hideProof": "Skjul bevis",
  "date": "Verifisert",
  "source": "Kilde",
  "notes": "Notater",
  "noProof": "Ingen bevisdokument tilgjengelig",
  "status": {
    "verified": "Verifisert",
    "partially-verified": "Delvis verifisert",
    "structure-verified": "Struktur verifisert",
    "corrected": "Korrigert",
    "unverified": "Ikke verifisert",
    "needs-correction": "Trenger korrigering"
  }
}
```

### 3. Barrel export — `components/index.ts`

The existing barrel at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/index.ts` currently exports 8 components. Append exports for the two new components created in sections 03 and 04:

```typescript
export { VerificationBadge } from "./VerificationBadge";
export { ProofPanel } from "./ProofPanel";
```

These exports assume the component files are named `VerificationBadge.tsx` and `ProofPanel.tsx` respectively and live directly under the `components/` directory, consistent with the existing export pattern.

---

## Build Verification Checklist

Run all three commands from `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/`:

1. `pnpm typecheck` — runs TypeScript strict check. Common failures at this stage:
   - Missing export from barrel (fix: check the export lines above)
   - `Framework` interface missing verification fields (fix: section 01 not applied)
   - `useFrameworkProof` return type mismatch (fix: section 02 hook signature)

2. `pnpm build` — Vite production build. Common failures:
   - Missing npm packages (`react-markdown`, `remark-gfm`, `@tailwindcss/typography`) — fix: section 01 install step
   - Tailwind typography plugin not registered — fix: `@plugin "@tailwindcss/typography"` missing from main CSS file (section 01)
   - JSON syntax error in locale files — fix: validate JSON, check for missing comma before the new `"proof"` block

3. `pnpm test` — Vitest. Common failures at this stage:
   - Translation key not found (`t("ontology:proof.viewProof")` returns key string) — locale file not updated
   - `VerificationBadge` or `ProofPanel` import fails — barrel not updated or component file missing
   - Race conditions in guidance tests (pre-existing) — rerun with `--test-threads=1` if needed

All three must exit 0 before the feature is considered complete.

---

## Notes

- The locale JSON files currently end with the `"crosswalk"` block as the last key. Add the `"proof"` block after it, before the closing `}`. Ensure a comma separates `"crosswalk"` from `"proof"` in each file.
- Do not add `"proof"` translations to any other namespace files (`common.json`, `compliance.json`, etc.). The `"proof"` namespace is exclusive to `ontology.json` because proof data is ontology-specific.
- The `VerificationBadge` and `ProofPanel` components are used inside `FrameworkProfile` (section 05), not as standalone route-level components. The barrel export is for any future consumers and for clean import paths within the feature.

---

## Actual Files Modified

| File | Notes |
|------|-------|
| `frontend/src/i18n/locales/en/ontology.json` | Added `common.source` and `proof.*` keys including `proof.status.unknown` (added via code review) |
| `frontend/src/i18n/locales/nb/ontology.json` | Added Norwegian equivalents including `proof.status.unknown: "Ukjent"` |
| `frontend/src/features/ontology/components/index.ts` | Added `VerificationBadge` and `ProofPanel` exports |
| `frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts` | Added 4 verification fields to Framework fixture |
| `frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx` | Added 4 verification fields to Framework fixture |
| `frontend/src/features/ontology/components/__tests__/LandscapeResults.test.tsx` | Added 4 verification fields to 3 Framework fixtures |
| `frontend/src/features/ontology/components/__tests__/SearchResults.test.tsx` | Added 4 verification fields to 2 Framework fixtures |

## Deviations from Plan

- **`proof.status.unknown` added**: Not in the original spec key list. `VerificationBadge` uses this key for null/unrecognized status; identified via code review and added to both locale files.
- **`common.source` added**: `FrameworkProfile` uses `t("common.source", "Source")` (section-05 fix). Added `common` block to both locale files to provide proper translations.
- **Framework fixture updates in 4 pre-existing test files**: `frameworkDomains.test.ts`, `FrameworkSidebar.test.tsx`, `LandscapeResults.test.tsx`, `SearchResults.test.tsx` all used Framework fixtures missing the new verification fields from section-01. These were not mentioned in the spec but required for TypeScript build compatibility.
- **`pnpm build` exits non-zero**: Pre-existing TypeScript errors in `analysis/`, `auth/`, `compliance/` features and TanStack Router types cause build failure that existed before this feature. `pnpm typecheck` and `pnpm test` both exit 0.

## Final Test Count

288 tests passing, 0 failing.