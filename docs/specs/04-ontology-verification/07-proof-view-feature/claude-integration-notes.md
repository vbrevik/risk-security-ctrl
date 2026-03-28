# Integration Notes — Opus Review Feedback

## What We're Integrating

### Critical #1: verification_status type safety
**Change:** `FrameworkProof.verification_status` becomes `string | null` (not the 6-value union). `VerificationStatus` union type is kept as a separate type for the known values. `VerificationBadge` receives `status: string | null` and applies a fallback style for unknown/null values.
**Why:** Backend returns `Option<String>` and cannot guarantee enum membership. A runtime crash on an unexpected status string would be worse than a neutral fallback badge.

### Critical #2: Add verification fields to Framework type
**Change:** Extend the `Framework` TypeScript interface to include `verification_status: string | null`, `verification_date: string | null`, `verification_source: string | null`, `verification_notes: string | null`. These fields are already being returned by the backend's `list_frameworks` endpoint — the frontend just wasn't capturing them.

**Consequence cascade:** The `useFrameworkProof` hook is now only needed to fetch `proof_content` (the markdown). The badge in `FrameworkProfile`'s header uses `framework.verification_status` directly (no extra HTTP request). The `proof?: FrameworkProof | null` prop on `FrameworkProfile` is eliminated. The `FrameworkProof` interface only needs the `proof_content` field (or can remain complete for the proof panel use case).

Actually, the proof panel still shows all metadata (date, source, notes, markdown) when the user opens it. The lazy fetch is still needed for `proof_content`. But the badge uses framework data already in cache. This is cleaner.

### Critical #3: Correct parent component name
**Change:** Replace all references to "OntologyExplorer" with "FrameworkCatalogPage" (`routes/_authenticated/frameworks/index.tsx`).

### Moderate #4: Show button for any framework with verification metadata
**Change:** Show "View Proof" button when `framework.verification_status !== null` (not when `proof_content` is non-null). Inside ProofPanel, when `proof_content` is null, render metadata + "No proof document available" message (not hide the panel). This matches the user's intent but correctly uses the metadata we now have from the Framework type directly.

### Moderate #5: Max-height scroll for proof markdown
**Change:** Add `max-h-96 overflow-y-auto` (or similar) container around the ReactMarkdown output in ProofPanel.

### Moderate #6: Install @tailwindcss/typography
**Change:** Install `@tailwindcss/typography` (noting Tailwind v4 plugin syntax: add to `@plugin "@tailwindcss/typography"` in CSS rather than tailwind.config.js). Use `prose prose-sm` classes on the markdown wrapper. Remove the "check if installed" ambiguity.

### Minor #8: Icon for structure-verified
**Change:** Use `Info` icon for `structure-verified` instead of `AlertTriangle`. AlertTriangle implies a warning; structure-verified is neutral-positive.

### Minor #9: Barrel export
**Change:** Add mention of updating barrel export file if one exists.

### Minor #11: Test note
**Change:** Remove `--test-threads=1` from build verification section (that flag is for `cargo test`, not vitest).

## What We're NOT Integrating

### Moderate #4 (staleTime)
**Decision:** Keep `staleTime: Infinity` for proof data. Proof files are semantic artifacts — once verified, they don't change except through deliberate re-verification. During active verification campaigns, developers can hard-reload. Matches the existing `staleTime: Infinity` convention for frameworks.

### Minor #12 (error boundary)
**Decision:** Out of scope. No other components in the ontology feature use error boundaries. Adding one only for ProofPanel would be inconsistent without a broader error boundary strategy.
