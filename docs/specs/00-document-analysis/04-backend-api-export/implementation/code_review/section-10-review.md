# Section 10 Code Review: Export Analysis Route Handler

## Critical

### 1. `c.name_en` not COALESCE'd - panic on orphaned findings (92% confidence)
- SQL uses LEFT JOIN but `c.name_en` has no COALESCE. If concept is deleted, `r.get("name_en")` panics on NULL since `concept_name_en` is `String` (not Option).
- Fix: `COALESCE(c.name_en, '') as name_en`

## Important

### 2. Export succeeds for non-completed analyses (83% confidence)
- Only `status != 'deleted'` is checked. Pending/processing/failed analyses export with empty findings.
- Fix: Add status check after loading analysis.

### 3. Audit log failure discards export bytes (80% confidence)
- `await?` on audit INSERT causes 500 even after export bytes are ready. Audit is secondary.
- Fix: Use warn-and-continue instead of `?`.

### 4. `sanitize_filename` allows non-ASCII via `is_alphanumeric()` (83% confidence)
- Norwegian characters (æ, ø, å) pass through into Content-Disposition filename, which expects ISO-8859-1.
- Fix: Change to `is_ascii_alphanumeric()`.

## No Issues Found
- SQL injection: all queries use parameterized `?` placeholders
- Format validation is exhaustive
- spawn_blocking wrapping correct for CPU-bound work
- Double `?` on spawn_blocking result is correct
- Framework name fallback handles unknown IDs gracefully
