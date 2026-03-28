# Code Review Interview: section-05-framework-profile

## Auto-fix: Weak framework-switch test — applied
Added test "resets proof panel when switching between two verified frameworks (useEffect reset)" with `FW_C` (verification_status: "structure-verified"). Verifies that `showProof` resets to false when switching between two frameworks both having non-null verification_status. This is the real `useEffect` scenario that the prior test missed.

## Auto-fix: Hardcoded "Source" string — applied
Changed `Source` literal to `t("common.source", "Source")` to comply with CLAUDE.md internationalization rule ("never hardcode strings"). The `useTranslation("ontology")` call was already in the component; no new imports needed.

## Minor: Badge test selector — kept
`document.querySelector("[aria-label]")` could match other elements. Acceptable given the test description specifically targets the badge in a FrameworkProfile context. No change made.
