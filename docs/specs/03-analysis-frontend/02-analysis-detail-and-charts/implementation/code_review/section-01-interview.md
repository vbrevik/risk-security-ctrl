# Section 01 Code Review Interview

## Auto-fixes applied

1. **useAnalyses test mocks not updated for PaginatedResponse rename** — Fixed all mock data in `hooks.test.ts` to use `items:` instead of `data:` for inner paginated response field. Also fixed `result.current.data?.data[0]` → `result.current.data?.items[0]`.

2. **useAnalysis hook missing refetchInterval/refetchOnMount** — Added `refetchOnMount: "always"` and conditional `refetchInterval` to the hook (5000ms when status is processing, false otherwise).

3. **Non-generic error shows common.error twice** — Error body now shows `error?.message` with fallback to `common.error`.

4. **Completed state placeholder was user-visible text** — Changed to empty `div` with comment.

5. **Extra optional chaining on items?.some()** — Reverted to `items.some()` since items is a required field.

## Let go (not fixed)

- Route tests test a copy component (follows existing project pattern)
- No shadcn Skeleton/Alert installed (raw divs are fine)
- EmptyFindings not wired yet (section 05 creates it, section 06 wires it)
