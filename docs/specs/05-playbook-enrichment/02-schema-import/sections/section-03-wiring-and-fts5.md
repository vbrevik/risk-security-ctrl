Now I have all the context needed. Let me produce the section content.

# Section 03: Wiring into import_all_ontologies() and FTS5

## Overview

This section wires the `import_guidance_file()` function (built in section 02) into the existing `import_all_ontologies()` orchestrator. It adds a dynamic scan for `*-guidance.json` files in the ontology data directory and triggers their import after all frameworks and relationships are loaded. It also covers updating the SQLx offline query data.

**Dependencies:**
- **section-01-migration-schema** must be complete (the four guidance tables, indexes, content view, and FTS5 virtual table must exist)
- **section-02-import-types-and-function** must be complete (`import_guidance_file()` and all serde types must exist in `backend/src/import.rs`)

## Tests (Write First)

All tests go in `backend/tests/guidance_tests.rs`. These tests specifically cover the wiring and FTS5 behavior; import function tests belong to section 02.

### Wiring Tests

```rust
// File: backend/tests/guidance_tests.rs (append to file from section-02)

// Test: import_all_ontologies imports *-guidance.json files after frameworks
// Setup: create a temp dir with a minimal framework JSON and a matching *-guidance.json.
// Call import_all_ontologies(). Verify concept_guidance table has rows.
// This proves the dynamic scan found and processed the guidance file.

// Test: no *-guidance.json files in directory does not cause error
// Setup: create a temp dir with only framework JSON files (no guidance files).
// Call import_all_ontologies(). Verify it completes without error.

// Test: guidance file with unknown framework_id still imports
// The framework_id field in *-guidance.json is metadata only, not a foreign key.
// Setup: create a guidance JSON where framework_id is "nonexistent-framework".
// The concepts referenced must exist (they are FKs), but framework_id is just stored text.
// Verify import completes without error.
```

### FTS5 Tests

```rust
// Test: after import + rebuild, MATCH query on about_en text returns results
// Setup: import guidance data that includes an about_en field with a distinctive word.
// Query: SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH 'distinctive_word'
// Verify at least one result is returned.

// Test: MATCH query on concept name_en returns results via the content view
// Setup: import a concept with name_en = "Risk Assessment" and guidance for that concept.
// Rebuild FTS. Query MATCH 'Risk Assessment'. Verify result found.
// This works because the content view joins concept_guidance with concepts.

// Test: FTS results join back to concept_guidance and concepts via rowid
// Setup: import guidance, rebuild FTS, run the full join query from the plan:
//   SELECT cg.concept_id, c.name_en, c.code, cg.about_en
//   FROM concept_guidance_fts
//   JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid
//   JOIN concepts c ON c.id = cg.concept_id
//   WHERE concept_guidance_fts MATCH 'search_term'
//   ORDER BY rank;
// Verify the joined result contains correct concept_id, name_en, and about_en.
```

## Implementation

### File to Modify: `backend/src/import.rs`

The only code change in this section is adding the guidance file scan to `import_all_ontologies()`.

### Where to Add the Code

In the existing `import_all_ontologies()` function, insert the guidance scan **after** the relationships import block and **before** the final `info!("Full ontology import completed")` log line. The placement is critical: guidance rows reference `concepts(id)` via foreign keys, so all framework and concept imports must complete first.

### Code to Add

After the relationships import block (lines 232-240 in the current file) and before the final info log (line 242), add:

```rust
// Scan for *-guidance.json companion files
let mut entries = tokio::fs::read_dir(data_dir).await?;
while let Some(entry) = entries.next_entry().await? {
    let name = entry.file_name().to_string_lossy().to_string();
    if name.ends_with("-guidance.json") {
        import_guidance_file(db, &entry.path()).await?;
    }
}
```

This uses `tokio::fs::read_dir` for async consistency with the rest of the codebase (the existing code already uses `tokio::fs::read_to_string` in other import functions).

### Behavior Details

- **Auto-extensible pattern:** Any file matching `*-guidance.json` in the data directory is automatically picked up. When future guidance files like `nist-csf-guidance.json` are added, no code change is needed.
- **Error propagation:** If `import_guidance_file()` returns an error, it propagates up via `?`. This matches the existing behavior for framework imports. The `import_guidance_file()` function itself (from section 02) handles individual concept_id validation internally by skipping invalid entries with warnings.
- **Ordering:** The scan order from `read_dir` is filesystem-dependent and not guaranteed. This is acceptable because each guidance file is independent and targets different concepts. There are no cross-file dependencies.
- **No hardcoded filenames:** Unlike the framework imports which use a hardcoded list (`framework_files` array), guidance files use dynamic scanning. This is intentional per the plan to make the system auto-extensible.

### SQLx Offline Query Data

After all code changes are complete (including sections 01 and 02), run from `backend/`:

```bash
cargo sqlx prepare
```

This regenerates the `.sqlx/` directory with offline query metadata. The `sqlx::query!()` macros used in `import_guidance_file()` (section 02) perform compile-time schema validation, and the offline data allows CI builds without a live database connection.

**Important:** This command requires a running database with all migrations applied. The typical workflow is:

1. Ensure the SQLite database exists with migrations applied (`cargo sqlx migrate run`)
2. Run `cargo sqlx prepare`
3. Commit the updated `.sqlx/` directory

This step should be done once after all sections (01, 02, 03) are implemented, not after each section individually.

### No New Imports Needed

The `tokio::fs` module is already available since `tokio` is a dependency of the project and `tokio::fs::read_to_string` is used elsewhere in the same file. The `import_guidance_file` function is defined in the same file (`import.rs`) so no new `use` statements are required.

## Verification Checklist

1. The `import_all_ontologies()` function scans for `*-guidance.json` after relationships are imported
2. The scan uses `tokio::fs::read_dir` (async, not `std::fs::read_dir`)
3. Files not matching the `*-guidance.json` pattern are ignored
4. An empty data directory (no guidance files) does not produce an error
5. FTS5 search returns results after guidance import (the rebuild happens inside `import_guidance_file` from section 02)
6. FTS5 results correctly join back to `concept_guidance` and `concepts` tables
7. `cargo sqlx prepare` has been run and `.sqlx/` is committed
8. All existing tests still pass (no regressions from the additive changes)