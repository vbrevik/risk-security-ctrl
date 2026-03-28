# Code Review Interview: section-02-api-hook

## Critical: queryKey: [] on null path — auto-fix applied
Changed to `["__disabled__"]` sentinel. Empty array is a degenerate key shared across all null-frameworkId callers — could cause cache collisions.

## Moderate: Timer-based test — auto-fix applied
Changed `setTimeout(50ms)` to synchronous `fetchStatus === 'idle'` assertion. skipToken makes the query synchronously idle so no timer needed. More reliable and directly tests the behavior.
