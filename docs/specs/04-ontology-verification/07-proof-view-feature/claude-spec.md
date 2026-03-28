# Claude Spec — Split 07: Proof View Feature

## What We're Building

A verification provenance panel that lets users see *why* each framework's data is trusted. The backend is **fully complete** (migration 005, import pipeline, API endpoint). This spec covers the **frontend-only** implementation.

## Backend Contract (Already Implemented)

Endpoint: `GET /api/ontology/frameworks/{id}/proof`

Response shape:
```json
{
  "framework_id": "nist-csf",
  "verification_status": "verified",
  "verification_date": "2026-03-28",
  "verification_source": "https://www.nist.gov/cyberframework",
  "verification_notes": "Full structure verified from official NIST CSF 2.0 publication",
  "proof_content": "# NIST CSF 2.0 Verification\n\n..."
}
```

`proof_content` is raw markdown from `docs/sources/{framework-id}-proof.md`. May be `null` if no proof file exists.

## What to Build

### 1. Type Extensions (`features/ontology/types/`)

Add to types file:
- `VerificationStatus` union type: `"verified" | "partially-verified" | "structure-verified" | "corrected" | "unverified" | "needs-correction"`
- `FrameworkProof` interface with all 6 fields from the API response

### 2. API Hook (`features/ontology/api/index.ts`)

New `useFrameworkProof(frameworkId: string | null)` hook:
- Uses `skipToken` from TanStack Query v5 for null-safe gating
- Query key: `ontologyKeys.proof(frameworkId)` extending existing key hierarchy
- `staleTime: Infinity` — proof data is immutable once written
- Only fires when `frameworkId` is non-null AND user has opened the proof panel

### 3. VerificationBadge Component (`features/ontology/components/VerificationBadge.tsx`)

Standalone badge component:
- Accepts `status: VerificationStatus`
- Uses `Record<VerificationStatus, { label: string; classes: string; icon: ReactNode }>` for style mapping
- Icons (from lucide-react): CheckCircle2 (verified/corrected), AlertTriangle (partially-verified/needs-correction), Info (structure-verified), Circle (unverified)
- Color map: verified/corrected=green, partially-verified/structure-verified=amber/blue, unverified=gray, needs-correction=red
- Renders as `<Badge variant="outline">` from shadcn/ui
- Includes `aria-label` for screen readers (WCAG 1.4.1 compliant)
- Uses i18n keys under `"ontology:proof.status.*"`

### 4. ProofPanel Component (`features/ontology/components/ProofPanel.tsx`)

Panel component showing full proof data:
- Accepts `frameworkId: string`
- Internally calls `useFrameworkProof(frameworkId)` — this is where the lazy fetch happens
- Loading state: skeleton rows
- Error state: error message with retry affordance
- Content layout:
  - Metadata row: `VerificationBadge` + date + source link (if present)
  - Notes (if present): small text block
  - Proof markdown: rendered with `react-markdown` + `remark-gfm`
  - Markdown styling: applies Tailwind `prose` classes or equivalent custom classes for readability

### 5. FrameworkProfile Integration (`features/ontology/components/FrameworkProfile.tsx`)

Changes to `FrameworkProfile`:
- Import and use `useFrameworkProof` for the proof data shape (not the content — just status for the badge)

Actually, since the badge is in the header and the proof panel is lazy, the approach is:

**Header changes:**
- Add `VerificationBadge` next to the version badge (inline in the existing header `<div className="flex items-center gap-3 mb-2">`)
- Add "View Proof" button (small, secondary) next to the source link, **only when `proof_content` is not null**
- The "View Proof" button toggles a boolean state `showProof`

**Proof data availability:**
- `FrameworkProfile` receives `framework.id` (already available)
- A lightweight hook call `useFrameworkProof` is needed just to know if `proof_content` is non-null for the button visibility decision
- BUT: this would eagerly fetch just to show/hide a button — contradicts lazy fetch requirement
- **Solution:** The "View Proof" button is shown based on whether `proof_content !== null` from the proof response. Since the proof is lazy, we need a different signal.
- **Better solution:** The backend can add `has_proof: boolean` to the framework list endpoint, OR we show the button always and handle the null state inside ProofPanel, OR we only hide the button after the first fetch resolves with `proof_content: null`.
- **Simplest correct approach:** Always show the "View Proof" button for all frameworks (hide only if no verification metadata at all). Inside ProofPanel, show "No proof document available" if `proof_content` is null but metadata exists. This avoids eager fetching just for UI state.

Wait — the user specifically said "Hide the 'View Proof' button entirely" when proof_content is null. Reconciling with lazy fetch: show the button optimistically (we don't know until we fetch), and if the resolved data has `proof_content: null`, close the panel and show no button. This creates a flash. Better: the Framework type can include a `has_proof` field derived from whether a proof file exists — but that requires a backend change.

**Revised approach based on user intent:** The FrameworkProfile props already include `framework: Framework`. If the backend adds `has_proof: boolean` to the framework list/detail response, the button visibility decision is O(1) and lazy. This requires a minor backend change to the `GET /api/ontology/frameworks` endpoint. This is the cleanest approach.

**Alternative without backend change:** Show the button always and inside ProofPanel, when `proof_content` is null, render only the metadata (status badge, date, notes) with no markdown section. This is slightly different from what the user said but avoids both eager fetching AND a backend change.

**Decision recorded in plan:** Use the always-show-button approach with graceful null handling inside ProofPanel, noting that a future backend addition of `has_proof` would enable exact matching of user intent.

### 6. i18n Keys

Add to both `en/ontology.json` and `nb/ontology.json`:
```json
{
  "proof": {
    "title": "Verification Proof",
    "viewProof": "View Proof",
    "hideProof": "Hide Proof",
    "status": {
      "verified": "Verified",
      "partially-verified": "Partially Verified",
      "structure-verified": "Structure Verified",
      "corrected": "Corrected",
      "unverified": "Unverified",
      "needs-correction": "Needs Correction"
    },
    "date": "Verified on",
    "source": "Source",
    "notes": "Notes",
    "noProof": "No proof document available"
  }
}
```

### 7. New Dependencies

```bash
pnpm add react-markdown remark-gfm
```

Only two new packages needed. `react-markdown` v10.x is ESM-only and Vite-compatible. No additional Tailwind config needed.

## Files to Create/Modify

| File | Action |
|------|--------|
| `features/ontology/types/index.ts` (or types file) | Add `VerificationStatus` type + `FrameworkProof` interface |
| `features/ontology/api/index.ts` | Add `useFrameworkProof` hook + proof query key |
| `features/ontology/components/VerificationBadge.tsx` | Create |
| `features/ontology/components/ProofPanel.tsx` | Create |
| `features/ontology/components/FrameworkProfile.tsx` | Add badge to header + "View Proof" toggle |
| `i18n/locales/en/ontology.json` | Add proof keys |
| `i18n/locales/nb/ontology.json` | Add proof keys (Norwegian) |
| `package.json` | Add react-markdown + remark-gfm |
| `features/ontology/components/__tests__/VerificationBadge.test.tsx` | Create |
| `features/ontology/components/__tests__/ProofPanel.test.tsx` | Create |
| `features/ontology/api/__tests__/hooks.test.ts` | Add `useFrameworkProof` tests |

## Constraints

- Backend is done — no backend changes required (except optional `has_proof` field described above)
- Authentication: proof endpoint uses existing auth middleware; frontend auth is handled by the `_authenticated` route wrapper already
- XSS: `react-markdown` without `rehype-raw` is safe by default — no raw HTML plugin needed
- i18n: all user-facing strings must use `useTranslation("ontology")` hook
- `pnpm build` must pass with zero TypeScript errors
