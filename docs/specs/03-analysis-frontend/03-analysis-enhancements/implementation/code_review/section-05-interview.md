# Section 05 Code Review Interview

## Triage Summary

| # | Finding | Severity | Decision |
|---|---------|----------|----------|
| 1 | filteredTypeCounts from capped dataset (limit: 1000) | High | Let go - Same architectural constraint exists in useChartData already. Not a section-05 regression. |
| 2 | Dead computed values (partiallyAddressed, notApplicable never displayed) | Medium | Auto-fix - Refactored to single-pass loop |
| 3 | Five iterations instead of one reduce | Medium | Auto-fix - Combined with #2 into single for-of loop |
| 4 | Empty dependency array on handleBarClick | Low | Let go - State setters are stable per React guarantees |
| 5 | Inline function on clear button | Low | Let go - Nitpick, banner only renders when filtered |
| 6 | Test fragility (queryByText) | Low | Let go - Values chosen to not conflict |

## Auto-Fixes Applied

### Fix #2 + #3: Single-pass loop for filteredTypeCounts
Replaced 5 separate `.filter()` calls with a single `for...of` loop that counts all types in one pass. Cleaner and more efficient.

## Tests After Fixes
All 213 tests pass.
