# Interview Transcript: 02-schema-import

## Q1: FTS5 Strategy
**Q:** Extend existing concepts_fts triggers, create separate concept_guidance_fts table, or use rebuild-based approach?

**A:** Rebuild-based approach — Use `INSERT INTO fts_table(fts_table) VALUES('rebuild')` after each import instead of per-row triggers. Simpler, works well for batch imports.

## Q2: Import Wiring
**Q:** Hardcoded guidance files or dynamic scan for `*-guidance.json`?

**A:** Dynamic scan — Scan `ontology-data/` for files matching `*-guidance.json` pattern after framework import.

## Q3: Transaction Handling
**Q:** Wrap each concept's guidance import in a transaction, the entire file, or no explicit transactions?

**A:** Transaction per guidance entry — Wrap each concept's guidance import in a transaction for atomicity.
