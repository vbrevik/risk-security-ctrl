# Implementation Plan — Split 07: Proof View Feature

## Background and Goal

The risk-security-ctrl ontology explorer contains 30 security and compliance frameworks (ISO standards, NIST publications, MITRE databases, EU legislation, etc.). As part of an ontology verification effort (splits 01-06), each framework's data has been verified against authoritative sources and the results recorded in markdown proof files at `docs/sources/{framework-id}-proof.md`. The verification status and metadata have been stored in the database.

The goal of this split is to surface this verification provenance to end users in the frontend. Users should be able to see, at a glance, whether a framework's data is trustworthy and, if interested, read the full evidence trail.

The backend is completely implemented. This plan covers frontend work only.

---

## Backend Contract (Read-Only Reference)

### Frameworks List (already fetched on load)

`GET /api/ontology/frameworks` already returns verification fields per framework (they are SELECTed in the query but the frontend `Framework` interface doesn't capture them yet):
- `verification_status: string | null`
- `verification_date: string | null`
- `verification_source: string | null`
- `verification_notes: string | null`

### Proof Endpoint (lazy-loaded for markdown content)

```
GET /api/ontology/frameworks/{id}/proof
Authorization: Bearer <token>

Response:
{
  framework_id: string,
  verification_status: string | null,    // backend is Option<String>
  verification_date: string | null,
  verification_source: string | null,
  verification_notes: string | null,
  proof_content: string | null           // raw markdown from proof file
}
```

`proof_content` is null for frameworks that have no proof file yet. The backend derives the proof file path server-side from the framework ID — the client never specifies a file path.

**Important:** `verification_status` from the backend is `Option<String>`, not a guaranteed enum. Any string or null is possible. Frontend code must handle unknown values gracefully.

---

## Architecture Overview

The feature extends four existing files and creates three new ones:

```
frontend/src/
  features/ontology/
    types/
      index.ts                          — extend Framework + add FrameworkProof + VerificationStatus
    api/
      index.ts                          — add useFrameworkProof hook
    components/
      VerificationBadge.tsx             — NEW: color-coded status badge
      ProofPanel.tsx                    — NEW: lazy proof display (metadata + markdown)
      FrameworkProfile.tsx              — MODIFY: badge in header + View Proof toggle
  i18n/locales/
    en/ontology.json                    — MODIFY: add "proof" key namespace
    nb/ontology.json                    — MODIFY: add Norwegian "proof" keys
```

New package dependencies:
- `react-markdown` — renders markdown via virtual DOM (safe by default: raw HTML stripped)
- `remark-gfm` — GitHub Flavored Markdown (tables, task lists, strikethrough)
- `@tailwindcss/typography` — prose styling for rendered markdown

---

## Section 1: Types and Dependencies

### 1.1 New npm Packages

Install three packages:
```
react-markdown     — safe markdown rendering
remark-gfm         — GFM support (tables, checkboxes in proof files)
@tailwindcss/typography  — prose styling
```

**Tailwind v4 typography setup:** The project uses `tailwindcss ^4.1.18` with CSS-first config. Add the typography plugin by including `@plugin "@tailwindcss/typography"` in the main CSS file (not via `tailwind.config.js` which doesn't exist in v4).

### 1.2 VerificationStatus Type

Add a `VerificationStatus` union type for the six known values. This type is used in the badge's style mapping. A seventh `"unknown"` catch-all handles unexpected backend strings:

```typescript
type VerificationStatus =
  | "verified"
  | "partially-verified"
  | "structure-verified"
  | "corrected"
  | "unverified"
  | "needs-correction";
```

A helper function `toVerificationStatus(value: string | null): VerificationStatus | "unknown"` normalizes raw backend strings to this type, returning `"unknown"` for null or unrecognized values.

### 1.3 Extend Framework Interface

Add four nullable fields to the existing `Framework` interface. These fields are already returned by the `GET /api/ontology/frameworks` endpoint — the frontend interface just wasn't capturing them:

```typescript
interface Framework {
  // ... existing fields ...
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
}
```

### 1.4 FrameworkProof Interface

Used for the lazy proof endpoint response (primarily for `proof_content`):

```typescript
interface FrameworkProof {
  framework_id: string;
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
  proof_content: string | null;
}
```

---

## Section 2: API Hook

### 2.1 Query Key Extension

Extend `ontologyKeys` with a `proof` entry:

```typescript
proof: (id: string) => [...ontologyKeys.framework(id), "proof"] as const,
```

### 2.2 useFrameworkProof Hook

The hook fetches the proof endpoint lazily — only when the user has opened the proof panel. It takes `frameworkId: string | null` and uses `skipToken` from TanStack Query v5 when the ID is null.

Key behaviors:
- `staleTime: Infinity` — proof data is a static artifact; re-verification is intentional and rare
- `skipToken` when `frameworkId` is null — TypeScript-idiomatic v5 pattern, better type narrowing than `enabled: !!id`
- Called from `ProofPanel`, not from `FrameworkProfile`, so it fires only when the user has clicked "View Proof"

Signature:
```typescript
function useFrameworkProof(frameworkId: string | null): UseQueryResult<FrameworkProof>
```

---

## Section 3: VerificationBadge Component

### 3.1 Purpose

A standalone badge displaying verification status. Used in `FrameworkProfile`'s header. Must handle null and unknown status values gracefully.

### 3.2 Status-to-Style Mapping

Use a `Record<VerificationStatus | "unknown", BadgeConfig>` where `BadgeConfig` contains Tailwind classes, icon component, and i18n key:

| Status | Color | Icon (lucide-react) |
|--------|-------|---------------------|
| `verified` | green | `CheckCircle2` |
| `corrected` | green (lighter) | `CheckCircle2` |
| `partially-verified` | amber | `AlertTriangle` |
| `structure-verified` | blue | `Info` |
| `unverified` | gray | `Circle` |
| `needs-correction` | red | `XCircle` |
| `unknown` / null | gray | `Circle` |

Color classes (with dark mode):
- green: `bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200`
- amber: `bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200`
- blue: `bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200`
- gray: `bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400`
- red: `bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200`

### 3.3 Accessibility

Icons render with `aria-hidden="true"`. The `<Badge>` element includes an `aria-label` attribute with the full status description. This satisfies WCAG 1.4.1 (information not conveyed by color alone).

### 3.4 Props

```typescript
interface VerificationBadgeProps {
  status: string | null;  // raw backend value; component normalizes internally
}
```

---

## Section 4: ProofPanel Component

### 4.1 Purpose

Displays the full verification evidence for a selected framework. Mounted inside `FrameworkProfile` when the user clicks "View Proof". Owns its data fetching.

### 4.2 Props

```typescript
interface ProofPanelProps {
  frameworkId: string;
}
```

### 4.3 States

**Loading:** Three skeleton rows (`h-4 w-full bg-muted rounded animate-pulse`).

**Error:** Generic error message. Do not expose internal error details or paths.

**Success — with `proof_content`:**
1. Metadata row: `VerificationBadge` + formatted date (from `verification_date`) + source link (if present) + notes paragraph (if present)
2. Divider
3. Markdown section: `proof_content` rendered with `<ReactMarkdown remarkPlugins={[remarkGfm]}>`. Wrap in a `max-h-96 overflow-y-auto` container to prevent layout overflow. Apply `prose prose-sm` classes from `@tailwindcss/typography`. Memoize the ReactMarkdown element to avoid re-renders.

**Success — without `proof_content` (null):**
Render the metadata row only, followed by a "No proof document available" note (i18n: `"ontology:proof.noProof"`).

### 4.4 Source Link

If `verification_source` is present, render as `<a href="..." target="_blank" rel="noopener noreferrer">` with an `ExternalLink` icon from lucide-react. Consistent with existing source link pattern in `FrameworkProfile`.

---

## Section 5: FrameworkProfile Integration

### 5.1 Badge in Header

The current header flex row:
```
[Framework Name]  [version badge]
```

Add `VerificationBadge` inline in the same flex row, rendering `framework.verification_status` (now available directly on the `Framework` type). No extra API call needed — the badge uses data already in cache from `useFrameworks()`.

If `framework.verification_status` is null, the badge renders in the neutral "unknown" style. The badge is always shown when a framework is selected.

### 5.2 View Proof Button

Below the source link (or inline with it), add a small secondary/ghost button with a `ShieldCheck` icon.

**Visibility:** Show when `framework.verification_status !== null` — i.e., whenever the framework has any verification metadata, regardless of whether `proof_content` exists. This lets users see metadata (date, source, notes) even for frameworks without a proof file.

**State:** `const [showProof, setShowProof] = useState(false)`. Label: `"ontology:proof.viewProof"` / `"ontology:proof.hideProof"`.

### 5.3 ProofPanel Placement

When `showProof` is true, render `<ProofPanel frameworkId={framework.id} />` as a section immediately after the header block, before the Stats Strip.

### 5.4 Reset on Framework Change

`useEffect(() => setShowProof(false), [framework?.id])` — collapse the proof panel when switching frameworks.

### 5.5 Parent Component Note

`FrameworkProfile` is rendered by `FrameworkCatalogPage` at `routes/_authenticated/frameworks/index.tsx`. If the `proof` prop approach is ever revisited, changes to the parent go in that file.

---

## Section 6: i18n Keys

### English (`en/ontology.json`)

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

### Norwegian (`nb/ontology.json`)

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

---

## Section 7: Tests

### 7.1 Hook Tests (extend `api/__tests__/hooks.test.ts`)

Test `useFrameworkProof`:
- Returns proof data when `frameworkId` is a non-null string
- Does not call the API when `frameworkId` is null (skipToken behavior)
- Maps API fields to `FrameworkProof` interface correctly

Use existing `createWrapper()` / `renderHook` / `waitFor` pattern. Mock `api.get` via `vi.mock("@/lib/api", ...)`.

### 7.2 VerificationBadge Tests (new `__tests__/VerificationBadge.test.tsx`)

- Each of the 6 known status values renders the correct i18n label
- Null status renders fallback (gray/unknown) style without crashing
- Unknown string value renders fallback without crashing
- `aria-label` attribute is present on each rendered badge
- Distinct icon renders for each status group

### 7.3 ProofPanel Tests (new `__tests__/ProofPanel.test.tsx`)

- Loading state renders skeleton elements
- Error state renders error message (not internal paths or API details)
- With `proof_content` non-null: markdown heading text appears in document
- With `proof_content` null: "No proof document available" text appears
- Source link has `rel="noopener noreferrer"` when `verification_source` present

### 7.4 FrameworkProfile Tests (extend `__tests__/FrameworkProfile.test.tsx`)

- `VerificationBadge` renders when `framework.verification_status` is non-null
- `VerificationBadge` renders fallback when `framework.verification_status` is null
- "View Proof" button appears when `framework.verification_status` is non-null
- "View Proof" button absent when `framework.verification_status` is null
- Clicking "View Proof" button mounts `ProofPanel`
- Switching to a new framework (new `framework.id`) hides the proof panel

---

## Build Verification

After implementation:
1. `pnpm build` — must pass with zero TypeScript errors and no warnings about missing types
2. `pnpm test` — Vitest test suite must pass
3. `pnpm typecheck` — TypeScript strict mode check

For backend tests (if running): `cargo test --test-threads=1` for guidance tests that are race-prone.

---

## Implementation Order

1. **Dependencies** — `pnpm add react-markdown remark-gfm @tailwindcss/typography`
2. **Tailwind typography plugin** — add `@plugin "@tailwindcss/typography"` to CSS
3. **Types** — extend `Framework`, add `VerificationStatus`, `FrameworkProof`, `toVerificationStatus` helper
4. **i18n keys** — add to both locale files
5. **API hook** — `useFrameworkProof` + `ontologyKeys.proof`
6. **VerificationBadge** — standalone component, no children
7. **ProofPanel** — depends on hook + VerificationBadge
8. **FrameworkProfile** — add badge to header, "View Proof" toggle, ProofPanel mount
9. **Tests** — hook tests, badge tests, panel tests, profile integration tests
10. **Barrel export** — add new components to `components/index.ts` if barrel file exists
