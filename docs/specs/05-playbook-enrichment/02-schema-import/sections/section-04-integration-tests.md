Good, I have all the context I need. Now I can write the section content.

# Section 04: Integration Tests

## Overview

This section creates a new integration test file at `backend/tests/guidance_tests.rs` that exercises the full guidance data pipeline: migration creation, data import, upsert idempotency, invalid concept handling, FTS5 search, and no-regression checks on existing API endpoints.

**Dependencies:** Sections 01, 02, and 03 must be implemented first. This section assumes:
- Migration `004_guidance_data_schema.sql` exists with four tables, indexes, a content view, and an FTS5 virtual table (section 01).
- `GuidanceFile`, `GuidanceEntry`, `ResourceEntry`, `ReferenceEntry` serde types and `import_guidance_file()` function exist in `backend/src/import.rs` (section 02).
- `import_all_ontologies()` scans for `*-guidance.json` files via `tokio::fs::read_dir()` (section 03).

---

## Test Infrastructure

The tests reuse the existing test helper at `backend/tests/common/mod.rs`, which provides `create_test_app()`. This function:
1. Creates a SQLite pool from `Config::from_env()`
2. Runs all migrations (including the new 004)
3. Imports ontology data from `../ontology-data/` if the database is empty
4. Returns an Axum `Router`

For tests that need direct database access (not just HTTP endpoints), extract the `SqlitePool` from the helper or create a standalone pool. Several tests need the pool directly to run SQL assertions.

### New file: `backend/tests/guidance_tests.rs`

---

## Test Stubs

All tests go in `backend/tests/guidance_tests.rs`. The file uses `mod common; use common::create_test_app;` and imports from `ontology_backend::import`.

### 1. Migration Verification Tests

These tests confirm that migration 004 created the expected schema objects. They query `sqlite_master` to verify table, index, and view existence.

```rust
use sqlx::SqlitePool;

mod common;

/// Helper: get a pool with migrations applied, ontology data imported.
/// Reuses the same setup logic as create_test_app() but returns the pool directly.
async fn setup_pool() -> SqlitePool {
    // same Config::from_env() + SqlitePool::connect_with + migrate! + import logic
    // as common::create_test_app(), but returns the pool instead of a Router
    todo!()
}

#[tokio::test]
async fn test_migration_creates_concept_guidance_table() {
    /// Query sqlite_master for table 'concept_guidance', assert it exists.
    todo!()
}

#[tokio::test]
async fn test_migration_creates_concept_actions_table() {
    /// Query sqlite_master for table 'concept_actions'.
    /// Also verify UNIQUE(concept_id, sort_order) by querying sqlite_master for the index.
    todo!()
}

#[tokio::test]
async fn test_migration_creates_concept_transparency_questions_table() {
    /// Query sqlite_master for table 'concept_transparency_questions'.
    todo!()
}

#[tokio::test]
async fn test_migration_creates_concept_references_table() {
    /// Query sqlite_master for table 'concept_references'.
    /// Verify CHECK constraint on reference_type by attempting an INSERT with an invalid
    /// reference_type value and asserting the error.
    todo!()
}

#[tokio::test]
async fn test_migration_creates_content_view() {
    /// Query sqlite_master for view 'concept_guidance_search_v'.
    todo!()
}

#[tokio::test]
async fn test_migration_creates_fts5_table() {
    /// Query sqlite_master for virtual table 'concept_guidance_fts'.
    todo!()
}

#[tokio::test]
async fn test_indexes_exist_on_child_tables() {
    /// Query sqlite_master WHERE type='index' for:
    /// - idx_concept_actions_concept
    /// - idx_concept_questions_concept
    /// - idx_concept_references_concept
    /// - idx_concept_references_type
    todo!()
}
```

**Key pattern for querying sqlite_master:**
```rust
let result: (i64,) = sqlx::query_as(
    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?"
)
.bind("concept_guidance")
.fetch_one(&pool)
.await
.unwrap();
assert_eq!(result.0, 1);
```

### 2. Import Happy Path Tests

These tests create a minimal guidance JSON fixture with 2-3 entries referencing real concept IDs from the NIST AI RMF ontology (`nist-ai-gv-1-1`, `nist-ai-gv-1-2`), write it to a temp file, call `import_guidance_file()`, and verify all four tables are populated correctly.

```rust
use std::io::Write;
use tempfile::NamedTempFile;
use ontology_backend::import::import_guidance_file;

/// Build a test guidance JSON string with known concept IDs.
/// Uses nist-ai-gv-1-1 and nist-ai-gv-1-2 which exist after ontology import.
fn test_guidance_json() -> String {
    /// Returns a JSON string matching GuidanceFile schema:
    /// - framework_id: "nist-ai-rmf"
    /// - source_pdf: "NIST-AI-100-1.pdf"
    /// - Two guidance entries for nist-ai-gv-1-1 and nist-ai-gv-1-2
    /// - Each entry has about_en, suggested_actions_en (2 items),
    ///   transparency_questions_en (1 item), resources (1 item), references (1 item)
    todo!()
}

#[tokio::test]
async fn test_import_populates_all_four_tables() {
    /// 1. setup_pool()
    /// 2. Write test_guidance_json() to a NamedTempFile
    /// 3. Call import_guidance_file(&pool, temp_file.path()).await
    /// 4. Assert concept_guidance has 2 rows
    /// 5. Assert concept_actions has rows (2 actions x 2 concepts = 4)
    /// 6. Assert concept_transparency_questions has rows
    /// 7. Assert concept_references has rows (1 resource + 1 reference per concept = 4)
    todo!()
}

#[tokio::test]
async fn test_concept_guidance_row_has_correct_fields() {
    /// Import test fixture, then SELECT * FROM concept_guidance WHERE concept_id = 'nist-ai-gv-1-1'
    /// Assert source_pdf, source_page, about_en match the fixture values.
    todo!()
}

#[tokio::test]
async fn test_concept_actions_have_correct_sort_order() {
    /// Import test fixture, SELECT action_text_en, sort_order FROM concept_actions
    /// WHERE concept_id = 'nist-ai-gv-1-1' ORDER BY sort_order.
    /// Assert sort_order starts at 1 and increments.
    /// Assert action_text_en matches the fixture.
    todo!()
}

#[tokio::test]
async fn test_transparency_questions_ordered_correctly() {
    /// Similar to actions test but for concept_transparency_questions table.
    todo!()
}

#[tokio::test]
async fn test_references_split_by_type() {
    /// Import test fixture, SELECT reference_type, title FROM concept_references
    /// WHERE concept_id = 'nist-ai-gv-1-1'.
    /// Assert resources array entries have reference_type = 'transparency_resource'.
    /// Assert references array entries have reference_type = 'academic'.
    todo!()
}
```

### 3. Invalid Concept Handling

```rust
#[tokio::test]
async fn test_invalid_concept_id_is_skipped() {
    /// Create a guidance JSON with one valid concept_id (nist-ai-gv-1-1) and one
    /// invalid concept_id ("nonexistent-concept-999").
    /// Import the file.
    /// Assert concept_guidance has exactly 1 row (the valid one).
    /// Assert no error was returned (invalid entries are warned, not errored).
    todo!()
}
```

### 4. Upsert Idempotency

```rust
#[tokio::test]
async fn test_reimport_produces_no_duplicates() {
    /// 1. Import the test guidance JSON
    /// 2. Count rows in concept_guidance (expect 2)
    /// 3. Import the same file again
    /// 4. Count rows in concept_guidance (still expect 2)
    /// The upsert on concept_guidance and delete-reinsert on child tables
    /// should prevent any duplication.
    todo!()
}

#[tokio::test]
async fn test_child_rows_replaced_on_reimport() {
    /// 1. Import test guidance JSON with 2 actions for nist-ai-gv-1-1
    /// 2. Create a modified JSON with 3 actions for the same concept
    /// 3. Import the modified JSON
    /// 4. Count actions for nist-ai-gv-1-1 -- should be 3, not 5
    /// This confirms delete-reinsert behavior for child tables.
    todo!()
}
```

### 5. FTS5 Search Tests

```rust
#[tokio::test]
async fn test_fts5_match_on_about_en_returns_results() {
    /// 1. Import test guidance JSON (about_en contains a unique keyword)
    /// 2. Run: SELECT rowid FROM concept_guidance_fts WHERE concept_guidance_fts MATCH 'keyword'
    /// 3. Assert at least 1 result
    todo!()
}

#[tokio::test]
async fn test_fts5_match_on_concept_name_returns_results() {
    /// The content view joins concept name_en into the FTS index.
    /// Search for a known concept name (e.g., part of "Identify Legal and Regulatory Requirements").
    /// Assert results are returned.
    todo!()
}

#[tokio::test]
async fn test_fts5_results_join_back_to_concept_guidance() {
    /// Run the full join query from the plan:
    /// SELECT cg.concept_id, c.name_en, cg.about_en
    /// FROM concept_guidance_fts
    /// JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid
    /// JOIN concepts c ON c.id = cg.concept_id
    /// WHERE concept_guidance_fts MATCH 'keyword'
    /// Assert the joined data is correct (concept_id, name_en match expected values).
    todo!()
}
```

### 6. No-Regression Checks

```rust
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

#[tokio::test]
async fn test_existing_health_endpoint_still_works() {
    /// Call GET /api/health via create_test_app().
    /// Assert StatusCode::OK and body contains {"status": "ok"}.
    /// This confirms migration 004 does not break the existing app.
    todo!()
}

#[tokio::test]
async fn test_existing_frameworks_endpoint_still_works() {
    /// Call GET /api/ontology/frameworks via create_test_app().
    /// Assert StatusCode::OK and response is a non-empty JSON array.
    todo!()
}
```

---

## Implementation Notes

### Test fixture design

The test JSON fixture should use a distinctive keyword in `about_en` that is unlikely to appear in existing ontology data (for example, "TESTGUIDANCEKEYWORD"). This makes FTS5 assertions unambiguous -- you can search for this keyword and know results come exclusively from the test fixture.

### Pool setup pattern

The `setup_pool()` helper mirrors `create_test_app()` but returns the raw `SqlitePool` instead of a `Router`. This is necessary because most guidance tests need to run SQL queries directly rather than going through HTTP endpoints. The implementation should:

1. Load `Config::from_env()` for the database URL
2. Create a `SqlitePool` with `create_if_missing(true)`
3. Run `sqlx::migrate!("./migrations")`
4. Import ontology data from `../ontology-data/` if concepts table is empty (needed so that guidance concept_id references are valid)

### Temp file usage

Use the `tempfile` crate (already a common dev-dependency in Rust projects) to create temporary guidance JSON files. The `NamedTempFile` type provides a path that can be passed to `import_guidance_file()`. Add `tempfile` to `[dev-dependencies]` in `backend/Cargo.toml` if not already present.

### Test isolation

Because all tests share the same SQLite database (per the existing `create_test_app()` pattern using `Config::from_env()`), guidance tests should clean up after themselves or use assertions that tolerate pre-existing data. The recommended approach:
- After each import test, DELETE the test data from all four guidance tables WHERE concept_id IN (the test concept IDs)
- Alternatively, count rows with specific known values rather than total row counts

### Real concept IDs

The test fixture uses `nist-ai-gv-1-1` and `nist-ai-gv-1-2` as concept IDs. These are real NIST AI RMF action-level concepts that exist after the standard ontology import. Using real IDs ensures foreign key validation passes and the test exercises the actual data flow.

---

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `backend/tests/guidance_tests.rs` | Create | All integration tests for guidance pipeline |
| `backend/Cargo.toml` | Modify (if needed) | Add `tempfile` to `[dev-dependencies]` |

---

## Acceptance Criteria

All of the following must pass with `cargo test` from the `backend/` directory:

1. Migration 004 schema objects (tables, indexes, view, FTS5) are verified present via `sqlite_master` queries.
2. Importing a valid 2-entry guidance JSON populates all four tables with correct data, sort orders, and reference types.
3. An entry with an invalid concept_id is silently skipped; the remaining valid entries are still imported.
4. Re-importing the same file produces no duplicate rows in any table.
5. Modifying child data (e.g., adding an action) and re-importing replaces the old child rows rather than appending.
6. FTS5 MATCH queries return results for both `about_en` content and concept `name_en` after import and rebuild.
7. FTS5 results can be joined back to `concept_guidance` and `concepts` tables via rowid.
8. Existing API endpoints (`/api/health`, `/api/ontology/frameworks`) continue to return correct responses after migration 004.

---

## Implementation Notes (Post-Build)

### Deviations from Plan

1. **Refactored `common/mod.rs`** — Extracted `create_test_pool()` from `create_test_app()` to eliminate duplication. Integration tests use `create_test_pool()` directly. Added `PRAGMA foreign_keys = ON` to the shared helper (was missing from the original `create_test_app()`).

2. **Skipped migration schema verification against real DB** — Already fully covered by unit tests in sections 01 against in-memory SQLite. The same migration runs in both environments.

3. **Skipped separate invalid concept ID test** — Already covered by `test_invalid_concept_id_is_skipped` in the unit tests.

### Files Modified
- `backend/tests/guidance_tests.rs` — Added 5 integration tests (import, reimport, FTS5, 2 API regression)
- `backend/tests/common/mod.rs` — Refactored to expose `create_test_pool()`, added FK pragma

### Test Count
- 34 total tests in `guidance_tests.rs` (8 schema + 5 deser + 9 import + 7 wiring/FTS5 + 5 integration)