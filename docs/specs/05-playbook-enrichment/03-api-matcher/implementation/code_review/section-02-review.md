# Section 02 Code Review

## Critical

**C1: Row::get() can panic on NULL.** The `.get("column")` calls inside the guidance mapping closure use the panicking variant. If a non-Option field hits NULL, it panics at runtime. Fix: verify DB NOT NULL constraints cover all non-Option fields, or switch to try_get().

**C2: Test pollution.** `test_guidance_with_empty_sub_items_returns_empty_arrays` inserts into the persistent DB without cleanup.

## Medium

**M3:** `use sqlx::Row` inside closure is unusual but functional.
**M4:** No assertion on about_en/about_nb in guidance presence test.
**M5:** Hard-coded fixture values (source_page=42) are brittle.

## Low

**L6:** Guidance queries placed after concept existence check — correct.
**L7:** sort_order exposed for actions/questions — minor API design smell.
