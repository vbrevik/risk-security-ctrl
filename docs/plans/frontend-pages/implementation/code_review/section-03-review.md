# Code Review: Section 03 - Framework Catalog

## Critical
1. **statsMap may be undefined** - No default value when destructuring useFrameworkStats
2. **Cross-framework connection resolution broken** - conceptFwMap only contains selected framework's concepts, so foreign concept lookups always fail

## Missing Features
3. Toast notification for invalid framework ?id
4. Clickable cross-framework connection rows (navigate to crosswalk)
5. Empty frameworks list empty state in sidebar
6. Route-level test file
7. FrameworkProfile tests for connected frameworks and not-found toast

## Minor
8. Corner markers on stat boxes
9. Expanded state not reset on framework change
10. Uses getFrameworkColor instead of plan's oklch map (reasonable deviation)
