# Section 10 Code Review Interview

## Finding 1: NULL panic on orphaned concept (Critical, Auto-fix)
- **Action:** Auto-fixed
- **Change:** Added `COALESCE(c.name_en, '') as name_en` in the findings JOIN query
- **Rationale:** LEFT JOIN can produce NULL for all concept columns; String fields would panic on NULL

## Finding 2: Export non-completed analyses (Auto-fix)
- **Action:** Auto-fixed
- **Change:** Added status check `analysis.status != AnalysisStatus::Completed` returning 400
- **Rationale:** Exporting pending/failed analyses produces empty misleading documents

## Finding 3: Audit log failure discards export (Auto-fix)
- **Action:** Auto-fixed
- **Change:** Changed `await?` to `if let Err(e) = ... { tracing::warn!(...) }`
- **Rationale:** Audit logging is secondary; should not fail the primary export operation

## Finding 4: Non-ASCII in Content-Disposition filename (Auto-fix)
- **Action:** Auto-fixed
- **Change:** Changed `is_alphanumeric()` to `is_ascii_alphanumeric()` in `sanitize_filename`
- **Rationale:** Norwegian characters (æ, ø, å) would violate RFC 6266 for the filename parameter
