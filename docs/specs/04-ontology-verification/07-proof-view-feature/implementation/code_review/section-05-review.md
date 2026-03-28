# Code Review: section-05-framework-profile

## Critical Issues
None.

## Moderate Issues
1. Test "hides panel when framework changes" only switched to FW_B (null status) — didn't cover switching between two verified frameworks (the real useEffect scenario).
2. Hardcoded "Source" string without t() wrapper in the refactored source link block.

## Minor Issues
3. Badge test uses fragile `document.querySelector("[aria-label]")` — could match other elements. Acceptable for now.
