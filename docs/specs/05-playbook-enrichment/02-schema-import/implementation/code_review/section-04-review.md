# Section 04 Code Review

## Failure Condition Check
1. **SHALL NOT duplicate tests** -- PARTIALLY VIOLATED (import/upsert tests overlap with unit tests; value is in real concept IDs)
2. **SHALL NOT leave test data** -- PARTIALLY: cleanup_guidance uses cleanup-before-test pattern, but not panic-safe
3. **SHALL NOT break existing tests** -- PASS

## Issues

### HIGH: setup_integration_pool duplicates common::create_test_app (A)
### HIGH: FTS5 rowid join assumption may be fragile across delete-reinsert (C)
### MEDIUM: No PRAGMA foreign_keys = ON in integration pool (F)
### MEDIUM: Missing migration schema verification against real DB (E)
### MEDIUM: FTS5 content sync not explicitly tested (B)
### LOW: Missing invalid concept ID test with real data (D)
### LOW: Hardcoded relative path ../ontology-data (G)
