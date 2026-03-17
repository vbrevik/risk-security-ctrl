# Code Review Interview: Section 03 - Framework Catalog

## Finding 1: statsMap may be undefined
**Decision:** Auto-fix
**Action:** Added `= new Map()` default value. Applied.

## Finding 2: Cross-framework connection resolution broken
**Decision:** Auto-fix
**Action:** Added `conceptToFramework` prop from `useAllConcepts()` hook to FrameworkProfile. Uses global concept-to-framework Map instead of local map built from selected framework only. Applied.

## Finding 3: Toast for invalid framework ?id
**Decision:** Let go — no toast infrastructure in the project. Silent redirect is acceptable UX.

## Finding 4: Clickable cross-framework connection rows
**Decision:** Let go — deferred to a future enhancement. Connection list is informational for now.

## Finding 5: Empty frameworks list empty state
**Decision:** Auto-fix
**Action:** Added "No frameworks loaded" empty state to FrameworkSidebar. Applied.

## Finding 6: Route-level test file
**Decision:** Let go — requires full router harness with mocked API, disproportionate to value.

## Finding 7: Missing FrameworkProfile tests
**Decision:** Let go — existing 4 tests cover the critical rendering paths.

## Finding 8: Corner markers on stat boxes
**Decision:** Auto-fix
**Action:** Added `corner-markers` class to stat boxes. Applied.

## Finding 9: Expanded state not reset on framework change
**Decision:** Auto-fix
**Action:** Added useEffect to clear expanded Set when framework.id changes. Applied.

## Finding 10: oklch vs getFrameworkColor
**Decision:** Let go — reusing existing getFrameworkColor is DRY and consistent.
