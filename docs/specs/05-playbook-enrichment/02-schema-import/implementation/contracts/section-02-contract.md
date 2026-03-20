# Section 02 Contract: Import Types and `import_guidance_file()`

## GOAL
Add Rust deserialization types (`GuidanceFile`, `GuidanceEntry`, `ResourceEntry`, `ReferenceEntry`) and `import_guidance_file()` async function to `backend/src/import.rs`. The function loads `*-guidance.json` companion files into the four guidance tables (concept_guidance, concept_actions, concept_transparency_questions, concept_references) with upsert semantics and FTS5 rebuild.

## CONTEXT
Section 02 of 05-playbook-enrichment/02-schema-import. Section 01 created migration 004 with the guidance data tables. This section provides the import logic. Section 03 will wire it into `import_all_ontologies()`.

## CONSTRAINTS
- All SQL uses parameterized binds (STIG V-222607) — no string interpolation
- Concept validation before insert (STIG V-222606) — SELECT before INSERT
- Transaction per entry for atomicity of parent-child rows
- Upsert for concept_guidance; delete-reinsert for child tables
- UUID v4 for all row IDs
- FTS5 rebuild after all entries processed
- Types must be `pub` for test access

## FORMAT
- Modify: `backend/src/import.rs` (add types + function)
- Modify: `backend/tests/guidance_tests.rs` (add deserialization + import tests)

## FAILURE CONDITIONS
- SHALL NOT use string interpolation in SQL
- SHALL NOT skip concept existence validation
- SHALL NOT leave child rows orphaned on reimport (must delete before reinsert)
- SHALL NOT break existing tests
