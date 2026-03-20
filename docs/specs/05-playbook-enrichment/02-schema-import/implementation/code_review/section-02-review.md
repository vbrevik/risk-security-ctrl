# Section 02 Code Review

## Failure Condition Check

1. **SHALL NOT use string interpolation in SQL** -- PASS
2. **SHALL NOT skip concept existence validation** -- PASS
3. **SHALL NOT leave child rows orphaned on reimport** -- PASS
4. **SHALL NOT break existing tests** -- PASS

## Issues

### HIGH: Norwegian-only data is silently dropped
If `suggested_actions_nb` is present but `suggested_actions_en` is not, the entire block is skipped. Same for transparency questions.

### MEDIUM: Concept validation runs outside the transaction
TOCTOU window between SELECT check and INSERT. Low practical risk for startup-time import.

### MEDIUM: No test for Norwegian-only or mixed-language data

### LOW: FTS5 rebuild runs even when zero entries were imported

### LOW: Error propagation aborts entire import on single-entry failure
The `?` operator returns immediately on any query failure within a transaction, skipping remaining entries and FTS5 rebuild.

### OBSERVATION: `resource_type` from JSON is captured but never stored (by design per plan)
