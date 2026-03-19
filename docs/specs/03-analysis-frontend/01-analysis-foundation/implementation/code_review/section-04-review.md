# Code Review: Section 04 - List Page

## High
1. Retry button uses `t("common.back")` instead of retry text — wrong semantics
2. Missing route-level test file (`routes/analysis/__tests__/index.test.tsx`)

## Medium
3. Hardcoded "Previous"/"Next"/"Page X of Y" — needs i18n
4. Missing `p-6` padding on route layout div
5. Plan says `data?.items` but `data?.data` is correct (plan discrepancy, not a bug)

## Low
6. AnalysisCard missing useTranslation
7. Select empty string value may cause issues with Radix
8. Accessibility: icon button needs aria-label, skeletons need aria-busy
9. renderWithRouter helper duplicated across test files
