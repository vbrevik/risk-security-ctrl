# Section 02 Code Review Interview

## Issue 1: Norwegian-only data silently dropped (HIGH)
**Decision:** Fix now (user approved)
**Fix:** Refactored action/question insertion to iterate over `max(en.len(), nb.len())`, using empty string for missing `_en` entries (NOT NULL column constraint). Added `import_guidance_entry()` helper.
**Tests added:** `test_norwegian_only_actions_are_preserved`, `test_mismatched_bilingual_array_lengths`

## Issue 2: Concept validation outside transaction (MEDIUM)
**Decision:** Let go — startup-time import, no concurrency risk

## Issue 3: No test for Norwegian/mixed-language data (MEDIUM)
**Decision:** Auto-fixed as part of Issue 1

## Issue 4: FTS5 rebuild when zero entries imported (LOW)
**Decision:** Let go — negligible cost

## Issue 5: Error propagation aborts entire import (LOW)
**Decision:** Auto-fix applied
**Fix:** Extracted per-entry logic into `import_guidance_entry()` helper. Errors are caught with `if let Err(e)` and logged as warnings, allowing remaining entries to proceed.
