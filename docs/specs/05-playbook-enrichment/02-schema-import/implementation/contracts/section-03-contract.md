# Section 03 Contract: Wiring into import_all_ontologies() and FTS5

## GOAL
Wire `import_guidance_file()` into `import_all_ontologies()` via dynamic `*-guidance.json` scan. Verify FTS5 search works end-to-end after import.

## CONTEXT
Section 03 of 02-schema-import. Section 02 built the import function. This section integrates it into the orchestrator and validates FTS5 search.

## CONSTRAINTS
- Guidance scan must run AFTER frameworks and relationships are loaded (FK dependency)
- Use `tokio::fs::read_dir` for async consistency
- Dynamic scan pattern, no hardcoded filenames
- Error from one guidance file should not abort the entire import

## FORMAT
- Modify: `backend/src/import.rs` (add scan to `import_all_ontologies()`)
- Modify: `backend/tests/guidance_tests.rs` (add wiring + FTS5 tests)

## FAILURE CONDITIONS
- SHALL NOT scan guidance files before framework/relationship imports complete
- SHALL NOT break existing tests
- SHALL NOT hardcode guidance filenames
