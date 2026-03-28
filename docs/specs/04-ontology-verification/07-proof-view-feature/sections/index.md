<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-types-and-deps
section-02-api-hook
section-03-verification-badge
section-04-proof-panel
section-05-framework-profile
section-06-i18n-and-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-types-and-deps | — | all | Yes |
| section-02-api-hook | 01 | 04, 05 | No |
| section-03-verification-badge | 01 | 05 | Yes (with 02) |
| section-04-proof-panel | 01, 02, 03 | 05 | No |
| section-05-framework-profile | 01, 02, 03, 04 | 06 | No |
| section-06-i18n-and-tests | 01–05 | — | No |

## Execution Order

1. **section-01** — types + install deps (no dependencies; must go first)
2. **section-02, section-03** — API hook and VerificationBadge (parallel after section-01)
3. **section-04** — ProofPanel (depends on 01, 02, 03)
4. **section-05** — FrameworkProfile integration (depends on all prior)
5. **section-06** — i18n keys, tests, build verification (final)

## Section Summaries

### section-01-types-and-deps
Install `react-markdown`, `remark-gfm`, `@tailwindcss/typography`. Configure Tailwind v4 typography plugin. Extend `Framework` TypeScript interface with verification fields. Add `VerificationStatus` union type, `FrameworkProof` interface, and `toVerificationStatus()` helper function. No tests (type-level).

### section-02-api-hook
Add `ontologyKeys.proof()` query key to the existing key hierarchy. Implement `useFrameworkProof(frameworkId: string | null)` hook using TanStack Query v5 `skipToken` for null-safe lazy fetching. `staleTime: Infinity`. Write hook unit tests: fetches when id provided, skips when null.

### section-03-verification-badge
Create `VerificationBadge.tsx` component. Status-to-style Record mapping (6 known statuses + "unknown" fallback). Color: green/amber/blue/gray/red. Icons: `CheckCircle2`, `AlertTriangle`, `Info`, `Circle`, `XCircle` from lucide-react. `<Badge variant="outline">` from shadcn/ui. WCAG-compliant `aria-label`. Write component tests: all 6 statuses, null, unknown string.

### section-04-proof-panel
Create `ProofPanel.tsx` component. Calls `useFrameworkProof(frameworkId)` internally (lazy fetch happens here). Loading skeleton, error state, metadata row (badge + date + source link + notes), markdown section using `<ReactMarkdown remarkPlugins={[remarkGfm]}>` inside `max-h-96 overflow-y-auto prose prose-sm` container. Handles null `proof_content` with "No proof document" message. Write component tests: all states.

### section-05-framework-profile
Modify `FrameworkProfile.tsx`. Add `VerificationBadge` to header row (uses `framework.verification_status` from extended Framework type — no extra API call). Add "View Proof" toggle button (shown when `framework.verification_status !== null`). Manage `showProof` state with `useEffect` reset on framework change. Mount `<ProofPanel>` when `showProof` is true. Extend `FrameworkProfile.test.tsx` with badge and proof panel integration tests.

### section-06-i18n-and-tests
Add `"proof"` key namespace to both `en/ontology.json` and `nb/ontology.json`. Update barrel export `components/index.ts` if it exists. Run `pnpm build`, `pnpm typecheck`, `pnpm test` to verify all pass.
