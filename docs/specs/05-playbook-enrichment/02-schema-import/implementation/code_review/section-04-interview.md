# Section 04 Code Review Interview

## Issue A: setup_integration_pool duplicates common::create_test_app (HIGH)
**Decision:** Auto-fix applied
**Fix:** Extracted `create_test_pool()` into `common/mod.rs`. Both `create_test_app()` and integration tests now call the same pool setup. Added `PRAGMA foreign_keys = ON` to the shared helper.

## Issue C: FTS5 rowid join fragile across delete-reinsert (HIGH)
**Decision:** Let go — the content view is a trivial single-table join (`concept_guidance_search_v` joins `concept_guidance` with `concepts`). Rowids from the base table are stable. The integration test passes and validates this behavior end-to-end.

## Issue F: Missing PRAGMA foreign_keys (MEDIUM)
**Decision:** Auto-fixed as part of Issue A — added to `create_test_pool()`.

## Issue E: Missing migration schema verification against real DB (MEDIUM)
**Decision:** Let go — fully covered by unit tests with in-memory SQLite. Running the same assertions against a persistent DB adds no value.

## Issue B: FTS5 content sync (MEDIUM)
**Decision:** Let go — test asserts exactly 2 rows, which catches both empty index and stale index.

## Issue D: Missing invalid concept test with real data (LOW)
**Decision:** Let go — unit test at line 470 covers this.

## Issue G: Hardcoded relative path (LOW)
**Decision:** Let go — pre-existing pattern, out of scope.
