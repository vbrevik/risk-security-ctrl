# Code Review Interview: Section 02 - Navigation Architecture

## Finding 1: Crosswalk route deviates from plan spec
**Decision:** Let go — the crosswalk route was created by a parallel session with full CrosswalkView implementation. Not our scope to modify.

## Finding 2: Extra i18n and component changes
**Decision:** Let go — these came from the parallel crosswalk session and are already committed.

## Finding 3: Tests don't test actual __root.tsx
**Decision:** Let go — testing real TanStack Router root layout requires disproportionate harness complexity. The mock tests verify the navigation contract (4 secondary links, correct hrefs, primary nav structure).

## Finding 4: validateSearch param names mismatch
**Decision:** Let go — the existing crosswalk route predates our work. Later sections will use the existing param names.

## Finding 5: No data-testid on actual __root.tsx
**Decision:** Let go — consequence of #3, acceptable tradeoff.

## Finding 6: Unsafe type assertions in validateSearch
**Decision:** Auto-fix
**Action:** Replaced `(search.x as string) ?? undefined` with `typeof search.x === "string" ? search.x : undefined` in frameworks, landscape, and concepts/search route stubs. Applied.

## Finding 7: Missing test for concepts redirect
**Decision:** Let go — redirect is trivial (single Navigate component), testing requires full router setup.
