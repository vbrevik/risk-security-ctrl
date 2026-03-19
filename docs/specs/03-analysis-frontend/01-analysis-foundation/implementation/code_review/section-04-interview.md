# Code Review Interview: Section 04 - List Page

**Date:** 2026-03-19

## Auto-fixes
1. Retry button: change `t("common.back")` to a retry-specific key; add `common.retry` to i18n files
2. Pagination: replace hardcoded "Previous"/"Next"/"Page X of Y" with i18n keys
3. Route layout: add missing `p-6` padding class
4. Settings button: add `aria-label` for accessibility

## User Decision
- **Add route tests:** User chose to add the missing route-level test file

## Let Go
- Plan says `data?.items` but `data?.data` is correct (not a bug)
- AnalysisCard doesn't use useTranslation (no labels needed yet)
- Select empty string value (works fine in practice)
- Test helper duplication (acceptable for now)
