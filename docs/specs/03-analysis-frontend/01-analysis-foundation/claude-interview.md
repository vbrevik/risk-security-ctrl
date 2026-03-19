# Interview: Analysis Frontend Foundation

## Q1: Auto-refresh for processing analyses
**Q:** Should processing analyses auto-refresh, or manual refresh?
**A:** Auto-poll every few seconds. Use TanStack Query's `refetchInterval` while any analysis is in 'processing' state (3-5s interval).

## Q2: Upload progress feedback
**Q:** Progress bar or simple spinner during file upload?
**A:** Progress bar showing percentage, using axios `onUploadProgress`.

## Q3: Boost terms editor
**Q:** How should MatcherConfig boost terms (key-value pairs) be edited?
**A:** Inline key-value editor — list of term/weight rows with add/remove buttons.

## Q4: Failed analysis handling
**Q:** Should users retry failed analyses or just delete and recreate?
**A:** Delete only. No retry API exists, user deletes and creates a new one.

## Prior Decisions (from deep-project interview)
- Create flow: dedicated page at `/analysis/create`
- Detail page: expandable findings table + charts (covered in split 02)
- Settings page: included with MatcherConfig form
- All text via i18n (en + nb)
