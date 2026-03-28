# Code Review Interview: section-01-types-and-deps

## Moderate: Narrow KNOWN_STATUSES type
**Decision: Auto-fix attempted, then reverted**

The reviewer suggested `ReadonlySet<VerificationStatus>` for better type consistency. Attempted, but TypeScript 5.x `Set<T>.has()` requires argument of type `T`, not `string` — causes a compile error since `value` is `string` (after the null check). The `ReadonlySet<string>` approach is correct: the `as VerificationStatus` cast is guarded by `KNOWN_STATUSES.has(value)` and is runtime-safe. Reverted.

## Minor: Add undefined test
**Decision: Auto-fix applied**

Added test: `toVerificationStatus(undefined as any)` returns `"unknown"`. This covers the missing-JSON-key-at-runtime scenario that TypeScript interfaces cannot prevent.
