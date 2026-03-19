# Integration Notes: Opus Review Feedback

## Integrating

1. **FTS5 query pattern** — Add example query showing how to join FTS results back to concepts. Integrating.
2. **CHECK constraint on reference_type** — Matches existing pattern in migration 003. Integrating.
3. **Drop redundant index** — UNIQUE on concept_id already creates an index. Integrating.
4. **Document mapping logic** — resources→"transparency_resource", references→"academic". Integrating.
5. **tokio::fs::read_dir()** — Use async I/O for consistency. Integrating.
6. **Fix pattern claim** — Pre-validation is an improvement, not a match. Integrating.
7. **Verify migration number** — Add a note to check for collisions. Integrating.

## Not Integrating

- **Single transaction for entire file (#8)** — User explicitly chose "transaction per guidance entry" in interview. Per-entry atomicity means a corrupt entry doesn't block other entries. 75 entries is fast enough.
- **Remove UUID id column (#12)** — Keeping for consistency with all other tables in the schema (every table uses UUID PK). Changing the pattern here would be inconsistent.
- **FK to concept_guidance instead of concepts (#2)** — The import pattern always deletes+reinserts child rows, so orphans can't occur through the pipeline. Direct FK to concepts is simpler and consistent with the flat relationship model.
- **extracted_at field (#4)** — Not needed in the database. The provenance is in the JSON file itself. Adding a note that it's intentionally skipped.
