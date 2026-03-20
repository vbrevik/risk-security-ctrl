# Section 03 Code Review Interview

## Issue 1: Error handling contradicts plan text (HIGH)
**Decision:** Let go — the contract explicitly says "Error from one guidance file should not abort the entire import." This is consistent with the section-02 fix for resilient error handling. Plan text was aspirational; contract is authoritative.

## Issue 2: Missing "unknown framework_id" test (MEDIUM)
**Decision:** Auto-fix applied
**Fix:** Added `test_guidance_with_unknown_framework_id_imports` test verifying framework_id is metadata-only.

## Issue 3: FTS5 non-distinctive search term (MEDIUM)
**Decision:** Auto-fix applied
**Fix:** Changed `test_fts5_match_on_about_en` to use "orchestration" — a word only in about_en, not in concept name_en.

## Issue 4: Inconsistent async/sync directory reads (LOW)
**Decision:** Let go — pre-existing, out of scope

## Issue 5: No multiple guidance files test (LOW)
**Decision:** Let go — pattern coverage is sufficient
