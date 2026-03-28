# Code Review: section-06-i18n-and-tests

## Critical Issues
1. Missing `proof.status.unknown` key in both locale files. `VerificationBadge` uses `i18nKey: "proof.status.unknown"` for null/unrecognized statuses — without the key, Norwegian locale shows "Unknown" (English fallback) instead of "Ukjent".

## Moderate Issues
2. `common.source` duplicates `detail.source` (both translate to "Source"/"Kilde"). Not a bug — `common` namespace is appropriate — but creates two translation paths for the same concept.

## Minor Issues
3. VerificationBadge renders unconditionally for all frameworks including null-status. Tests confirm this is by design (FW_B test asserts badge present). No fix needed.
