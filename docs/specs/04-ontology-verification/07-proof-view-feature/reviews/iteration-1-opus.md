# Opus Review

**Model:** claude-opus-4
**Generated:** 2026-03-28T13:00:00Z

---

## Plan Review: Split 07 -- Proof View Feature

### Critical Issues

**1. `verification_status` is `Option<String>` on the backend, not a required enum (Section 1.2, 1.3, entire Badge design)**

The plan defines `FrameworkProof.verification_status` as a non-nullable `VerificationStatus` (a 6-value union type). However, the backend `ProofResponse` struct at `backend/src/features/ontology/models.rs:139` declares `verification_status: Option<String>`. The backend can return `null` for this field and can also return any arbitrary string not in the 6-value union. The plan treats this as a guaranteed non-null, constrained value.

This will cause runtime crashes or type unsoundness. The plan should either:
- Make the frontend type `verification_status: string | null` and handle the null/unknown cases in VerificationBadge, or
- Document that the backend guarantees non-null for frameworks that exist (verify this is true), and add a runtime guard/fallback.

**2. The `Framework` type already receives verification fields -- the plan ignores this (Section 5.1)**

The backend `list_frameworks` endpoint at `routes.rs:55` already SELECTs `verification_status, verification_date, verification_source, verification_notes` for every framework. The frontend `Framework` interface does not include these fields, so they are silently dropped.

This means the plan's "Challenge" in Section 5.1 is solvable much more simply: just add verification fields to the existing `Framework` TypeScript interface. The data is already being fetched. There is no need for a separate `useFrameworkProof` call just to get the badge status, and no need for the `proof?: FrameworkProof | null` prop gymnastics. The badge can come from data already in the cache via `useFrameworks()`.

The separate `/proof` endpoint is still needed for `proof_content` (the markdown), but the badge in the header should use the already-fetched framework data. This avoids an extra HTTP request per framework selection.

**3. `FrameworkProfile` is used in the frameworks catalog route, not `OntologyExplorer` (Section 5.1)**

The plan says "the parent (`OntologyExplorer`) calls `useFrameworkProof`". The actual parent is `FrameworkCatalogPage` at `routes/_authenticated/frameworks/index.tsx`. An implementer following this literally would look for the wrong file.

### Moderate Issues

**4. `staleTime: Infinity` may be wrong for proof data during active verification campaigns**

Verification is an ongoing project (statuses include "needs-correction" and "unverified"). A user could verify a framework and then check the UI -- stale cache means they would never see updates without a hard refresh. Consider `staleTime: 5 * 60 * 1000` (5 minutes) to match existing concept query pattern.

**5. No handling of proof files that could be very large (Section 4.3)**

Some proof markdown files could be substantial. The plan renders all of `proof_content` inline. For large files this causes layout overflow and performance issues. Consider a max-height with overflow-y scroll, and memoizing the `ReactMarkdown` output.

**6. `@tailwindcss/typography` is NOT installed (Section 4.4)**

The `package.json` does not have `@tailwindcss/typography`. The fallback styling approach is underspecified — proof files contain tables (GFM), code blocks, and nested lists. The plan should either require installing the typography plugin (accounting for Tailwind v4's CSS-first config) or provide comprehensive custom styles for all markdown elements.

**7. "View Proof" button hidden when `proof_content` is null (Section 5.2)**

Frameworks with `verification_status` set but no proof file still have useful metadata (status, date, source, notes). Hiding the button means users cannot see this metadata. The button should be shown whenever proof metadata exists, not only when `proof_content` is non-null.

### Minor Issues

**8. Icon choice for `partially-verified` and `structure-verified` (Section 3.3)**

`AlertTriangle` for "structure-verified" is semantically misleading. "Structure verified" is a positive-leaning state. Consider `ShieldCheck` or `Info` icon.

**9. Missing export from `components/index.ts` barrel file**

The plan creates `VerificationBadge.tsx` and `ProofPanel.tsx` but does not mention updating the barrel export file.

**10. No mention of Tailwind v4 considerations**

The project uses `tailwindcss: ^4.1.18` (Tailwind v4, CSS-first config). If installing `@tailwindcss/typography`, the plugin mechanism differs from v3.

**11. Test section references `--test-threads=1` which is a Rust/cargo flag**

The flaky test warning from CLAUDE.md applies to backend tests only. Vitest handles parallelism differently.

**12. No error boundary consideration**

If `react-markdown` throws on malformed markdown, the entire `FrameworkProfile` unmounts. Consider wrapping `ProofPanel` in an error boundary.
