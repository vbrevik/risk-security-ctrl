# Section 02 Code Review Interview

## C1: Row::get() panic risk
**Decision: Accept.** DB schema has NOT NULL constraints on all non-Option fields (source_pdf, source_page, action_text_en, etc.). The panicking `.get()` is safe given schema guarantees. Moved `use sqlx::Row` to module level for clarity.

## C2: Test pollution
**Auto-fixed.** Tests now use `INSERT OR IGNORE` for idempotent seeding. The empty sub-items test explicitly deletes sub-data to ensure clean state. Changed test concept from `nist-ai-gv-1-1` to `nist-ai-gv-3-1` to avoid conflict with guidance_tests.rs FTS5 integration tests that clean and rebuild data for `nist-ai-gv-1-1`.

## M4: Missing about_en/about_nb assertions
**Auto-fixed.** Added assertions checking `guidance.get("about_en").is_some()` and `guidance.get("about_nb").is_some()`.

## M5, L6, L7: Let go
