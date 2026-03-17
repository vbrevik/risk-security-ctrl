# Code Review Interview: Section 01 - Shared Infrastructure

## Finding 1: PAGINATION BUG (Medium-High)
**Decision:** User chose "Implement pagination loop"
**Action:** Implemented `fetchAllConceptsForFramework()` that loops through all pages until `page >= total_pages`. Applied.

## Finding 2: USEMEMO DEPENDENCY INSTABILITY (Medium)
**Decision:** Auto-fix
**Action:** Replaced `[queries]` dependency with `[queryDataKeys]` where `queryDataKeys` is a stable string derived from `queries.map(q => q.dataUpdatedAt).join(",")`. Applied.

## Finding 3: ERRORS ARRAY INSTABILITY (Low-Medium)
**Decision:** Auto-fix
**Action:** Wrapped errors computation in `useMemo` with same stable `queryDataKeys` dependency. Applied.

## Finding 4: MISSING isLoading TEST (Low)
**Decision:** Let go — loading state is implicitly tested via waitFor patterns in existing tests.

## Finding 5: MISSING GRANULAR useFrameworkStats TESTS (Low)
**Decision:** Let go — all assertions are covered in the single test case.

## Finding 6: VITEST CONFIG DOES NOT EXTEND VITE CONFIG (Low)
**Decision:** Let go — standalone config works correctly, only the `@` alias matters and it's configured.

## Finding 7: NO TRIMMING IN parseCommaSeparated (Low)
**Decision:** Auto-fix
**Action:** Added `.map(s => s.trim())` before `.filter(Boolean)`. Applied.
