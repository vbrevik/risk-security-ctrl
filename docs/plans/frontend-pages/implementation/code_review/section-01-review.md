# Code Review: Section 01 - Shared Infrastructure

Overall the implementation closely follows the plan. The types, utilities, hooks, tests, and test infrastructure are all present. Below are the issues found, ordered by severity.

1. **PAGINATION BUG (Medium-High):** In useAllConcepts, the hook fetches concepts with limit=500 but never checks whether there are additional pages. If a framework has more than 500 concepts, the remaining concepts are silently dropped. The plan says 'limit=500 per request' but does not address multi-page fetching either, so this is a latent data-loss bug inherited from the spec. At minimum, the hook should log a warning or throw when total_pages > 1, or implement pagination loop.

2. **USEMEMO DEPENDENCY INSTABILITY (Medium):** In useAllConcepts, the useMemo for 'data' has [queries] as its dependency. The 'queries' array is a new array reference on every render from useQueries, which means the useMemo will recompute on every render, defeating its purpose entirely. The conceptToFramework Map downstream then also recomputes every render since it depends on 'data'. This will cause unnecessary re-renders in all consumers. The dependency should use a stable reference.

3. **ERRORS ARRAY INSTABILITY (Low-Medium):** The 'errors' array is computed outside of useMemo, so it produces a new array reference on every render. Any consumer using it in a useEffect dependency array or passing it as a prop will trigger unnecessary re-renders or effects.

4. **MISSING isLoading TEST (Low):** The plan specifies a test 'isLoading is true while any query is pending' for useAllConcepts. This test is not present in hooks.test.ts.

5. **MISSING GRANULAR useFrameworkStats TESTS (Low):** The plan specifies four separate tests for useFrameworkStats. The implementation collapses these into a single test. While the single test does cover the assertions, the plan explicitly asked for separate test cases.

6. **VITEST CONFIG DOES NOT EXTEND VITE CONFIG (Low):** The plan says 'add one that extends the existing vite config'. The implementation creates a standalone vitest config rather than extending vite.config.ts.

7. **NO TRIMMING IN parseCommaSeparated (Low):** The urlParams utility does not trim whitespace from values. Input like 'a, b, c' would produce ['a', ' b', ' c'].
