# Code Review: section-02-api-hook

## Critical Issues
1. `queryKey: []` when frameworkId is null — degenerate shared cache key across all null-frameworkId callers. Should use a sentinel like `["__disabled__"]` or similar.

## Moderate Issues
2. Null-path test uses `setTimeout(50ms)` instead of asserting query state (`fetchStatus === 'idle'`) — unreliable timer-based test.

## Minor Issues
None.
