# Section 01 Code Review

## Critical: PRAGMA foreign_key_check crash risk
- Tuple uses `String` for parent column but SQLite can return NULL
- `?` operator propagates errors, killing startup — violates plan intent
- **Auto-fix:** Change tuple to `Option<String>`, wrap in match to catch decode errors

## Medium: NOT NULL on timestamps
- Added NOT NULL to created_at/updated_at but existing tables don't have it
- **Auto-fix:** Remove NOT NULL from timestamps to match existing convention

## Medium: sort_order NOT NULL
- Plan says DEFAULT 0 (nullable), impl says NOT NULL DEFAULT 0
- **Auto-fix:** Remove NOT NULL, keep DEFAULT 0

## Low: FK on concept_id/framework_id has no cascade
- Correct per plan. Noted: deleting concepts will fail if findings reference them.
- **Let go:** Intentional design — findings reference stable ontology data.
