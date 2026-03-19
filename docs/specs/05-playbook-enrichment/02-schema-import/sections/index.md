<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-migration-schema
section-02-import-types-and-function
section-03-wiring-and-fts5
section-04-integration-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-migration-schema | - | 02, 03, 04 | Yes |
| section-02-import-types-and-function | 01 | 03, 04 | No |
| section-03-wiring-and-fts5 | 01, 02 | 04 | No |
| section-04-integration-tests | 01, 02, 03 | - | No |

## Execution Order

1. section-01-migration-schema (no dependencies)
2. section-02-import-types-and-function (after 01)
3. section-03-wiring-and-fts5 (after 01, 02)
4. section-04-integration-tests (final)

## Section Summaries

### section-01-migration-schema
Migration `004_guidance_data_schema.sql` with four tables (concept_guidance, concept_actions, concept_transparency_questions, concept_references), indexes, CHECK constraints, content view, and FTS5 virtual table. Ends with rebuild command.

### section-02-import-types-and-function
Serde types for `*-guidance.json` files (GuidanceFile, GuidanceEntry, ResourceEntry, ReferenceEntry). The `import_guidance_file()` function with per-entry transactions, concept validation, upsert for guidance, delete-reinsert for child rows, and resources→transparency_resource/references→academic mapping. FTS5 rebuild after import.

### section-03-wiring-and-fts5
Wire `import_guidance_file()` into `import_all_ontologies()` with dynamic `*-guidance.json` scanning via `tokio::fs::read_dir()`. Update `cargo sqlx prepare` for offline query data.

### section-04-integration-tests
Integration tests in `backend/tests/guidance_tests.rs`: migration verification, import happy path, invalid concept handling, upsert idempotency, FTS5 search after rebuild, no-regression check on existing endpoints.
