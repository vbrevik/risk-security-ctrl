Now I have all the context needed. Let me produce the section content.

# Section 02: Import Types and `import_guidance_file()` Function

## Overview

This section adds the Rust deserialization types for `*-guidance.json` companion files and the `import_guidance_file()` function that loads guidance data into the four tables created in Section 01 (migration 004). The function handles per-entry transactions, concept validation, upsert semantics, and FTS5 rebuild.

**Depends on:** Section 01 (migration 004 must exist so the guidance tables are present at runtime).

**Blocks:** Section 03 (wiring into `import_all_ontologies()`), Section 04 (integration tests).

---

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/import.rs`

All additions go into this existing file. The file already contains `FrameworkData`, `ConceptData`, `RelationshipData`, `OntologyFile`, `RelationshipsFile`, `import_ontology_file()`, `import_relationships()`, and `import_all_ontologies()`. The new types and function are purely additive.

---

## Tests First

Create a new test file at `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/guidance_tests.rs`. The tests below cover deserialization of the new types and the import function behavior. They use the existing `common::create_test_app` pattern for database setup and `tempfile` (already in dev-dependencies) for writing test JSON.

### Deserialization Tests

```rust
// File: backend/tests/guidance_tests.rs

mod common;

use ontology_backend::import::{GuidanceFile, GuidanceEntry, ResourceEntry, ReferenceEntry};

#[test]
fn test_guidance_file_deserializes_from_valid_json() {
    /// Parse a minimal valid GuidanceFile JSON string.
    /// Verify framework_id, source_pdf, and guidance vec length.
}

#[test]
fn test_guidance_entry_with_all_optional_fields_null() {
    /// Parse a GuidanceEntry where about_nb, suggested_actions_en/nb,
    /// transparency_questions_en/nb, resources, and references are all absent.
    /// Verify deserialization succeeds and optional fields are None.
}

#[test]
fn test_resource_entry_deserializes_type_field() {
    /// Parse JSON: {"title": "NIST AI 100-1", "url": "https://...", "type": "standard"}
    /// Verify resource_type == Some("standard") — the serde(rename = "type") mapping.
}

#[test]
fn test_reference_entry_with_partial_fields() {
    /// Parse a ReferenceEntry with only "title" present.
    /// Verify authors, year, venue, url are all None.
}

#[test]
fn test_unknown_json_fields_are_ignored() {
    /// Parse a GuidanceFile JSON with an extra top-level "extracted_at" field
    /// and an extra field inside a GuidanceEntry. Verify no error
    /// (serde default is to ignore unknown fields unless deny_unknown_fields is set).
}
```

### Import Function Tests

These are async tests that need a database with migrations applied and at least one concept row to reference.

```rust
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;
use tempfile::NamedTempFile;
use std::io::Write;

/// Helper: create an in-memory SQLite pool with all migrations applied
/// and insert a test concept for FK references.
async fn setup_test_db() -> SqlitePool {
    /// Connect to ":memory:" SQLite, run migrations, insert a test framework
    /// and a test concept with id "test-concept-1" into the concepts table.
}

/// Helper: return a valid guidance JSON string referencing "test-concept-1".
fn sample_guidance_json() -> String {
    /// Returns JSON with framework_id, source_pdf, and one GuidanceEntry
    /// that has about_en, 2 suggested_actions_en, 1 transparency_question_en,
    /// 1 resource, and 1 reference. concept_id = "test-concept-1".
}

#[tokio::test]
async fn test_import_guidance_populates_all_four_tables() {
    /// Write sample_guidance_json() to a temp file, call import_guidance_file(),
    /// then SELECT COUNT(*) from concept_guidance, concept_actions,
    /// concept_transparency_questions, concept_references.
    /// Verify each table has the expected row count.
}

#[tokio::test]
async fn test_concept_guidance_row_has_correct_fields() {
    /// After import, SELECT * FROM concept_guidance WHERE concept_id = 'test-concept-1'.
    /// Verify source_pdf, source_page, about_en match the JSON input.
}

#[tokio::test]
async fn test_concept_actions_have_correct_sort_order() {
    /// After import, SELECT action_text_en, sort_order FROM concept_actions
    /// WHERE concept_id = 'test-concept-1' ORDER BY sort_order.
    /// Verify sort_order is 1-based and action texts match input order.
}

#[tokio::test]
async fn test_references_split_into_correct_types() {
    /// After import, query concept_references WHERE concept_id = 'test-concept-1'.
    /// Verify resources become reference_type = "transparency_resource"
    /// and references become reference_type = "academic".
}

#[tokio::test]
async fn test_invalid_concept_id_is_skipped() {
    /// Create JSON with two entries: one valid (test-concept-1), one with
    /// concept_id = "nonexistent-concept". Import. Verify only the valid
    /// entry exists in concept_guidance. No error returned.
}

#[tokio::test]
async fn test_reimport_produces_no_duplicates() {
    /// Import the same file twice. Verify concept_guidance has exactly 1 row
    /// (upsert), concept_actions has the original count (delete-reinsert),
    /// not double.
}

#[tokio::test]
async fn test_child_rows_replaced_on_reimport() {
    /// Import a file, then import a modified version with different action texts.
    /// Verify concept_actions contains the new texts, not the old ones.
}
```

---

## Implementation Details

### New Types

Add the following four structs to `backend/src/import.rs`, below the existing type definitions. All derive `Debug, Deserialize`. These are `pub` so tests can import them.

**`GuidanceFile`** -- top-level JSON structure:
- `framework_id: String` -- metadata, not used as a foreign key
- `source_pdf: String` -- PDF filename for provenance
- `guidance: Vec<GuidanceEntry>` -- the actual data entries

The JSON also has an `extracted_at` field at the top level. This field is intentionally not included in the struct; serde will silently ignore it since `deny_unknown_fields` is not set.

**`GuidanceEntry`** -- one entry per concept:
- `concept_id: String`
- `source_page: i64`
- `about_en: Option<String>`
- `about_nb: Option<String>`
- `suggested_actions_en: Option<Vec<String>>`
- `suggested_actions_nb: Option<Vec<String>>`
- `transparency_questions_en: Option<Vec<String>>`
- `transparency_questions_nb: Option<Vec<String>>`
- `resources: Option<Vec<ResourceEntry>>`
- `references: Option<Vec<ReferenceEntry>>`

**`ResourceEntry`** -- transparency resources:
- `title: String`
- `url: Option<String>`
- `resource_type: Option<String>` -- must use `#[serde(rename = "type")]` because `type` is a Rust keyword

**`ReferenceEntry`** -- academic references:
- `title: String`
- `authors: Option<String>`
- `year: Option<i64>`
- `venue: Option<String>`
- `url: Option<String>`

### Mapping to `concept_references` Table

Both `resources` and `references` arrays from the JSON map to the single `concept_references` table, distinguished by the `reference_type` column:

- Items from `resources` array get `reference_type = "transparency_resource"`. The `ResourceEntry.resource_type` field from JSON is NOT used as the database discriminator; the `title` and `url` fields are stored, and `authors`/`year`/`venue` are set to NULL.
- Items from `references` array get `reference_type = "academic"`. All fields (`title`, `authors`, `year`, `venue`, `url`) map directly.

Sort order for references: resources come first (1-based), then academic references continue the numbering. For example, if there are 2 resources and 3 references, sort_order runs 1..5.

### `import_guidance_file()` Function

Add this async function to `backend/src/import.rs`:

```rust
pub async fn import_guidance_file(
    db: &SqlitePool,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    /// Reads a *-guidance.json companion file and imports into the four
    /// guidance tables: concept_guidance, concept_actions,
    /// concept_transparency_questions, concept_references.
    ///
    /// Flow:
    /// 1. Read + parse JSON into GuidanceFile
    /// 2. Log framework_id and entry count
    /// 3. For each GuidanceEntry:
    ///    a. Validate concept_id exists in concepts table (SELECT id FROM concepts WHERE id = ?)
    ///       If missing: warn!() and skip this entry
    ///    b. Begin transaction (db.begin())
    ///    c. Upsert concept_guidance row (INSERT ... ON CONFLICT(concept_id) DO UPDATE SET ...)
    ///    d. DELETE existing child rows for this concept_id from all three child tables
    ///    e. INSERT new child rows with Uuid::new_v4().to_string() for id, 1-based sort_order
    ///    f. Commit transaction
    /// 4. After all entries: rebuild FTS5 index
    ///    sqlx::query("INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild')")
    /// 5. Log completion
}
```

Key implementation notes:

- **Concept validation (STIG V-222606):** Pre-check with `SELECT id FROM concepts WHERE id = ?` before inserting. If the concept does not exist, log `warn!("Concept {} not found, skipping guidance entry", concept_id)` and continue to the next entry. This is cleaner than catching FK violations after the fact.

- **All queries use parameterized binds (STIG V-222607):** Use `sqlx::query!()` or `sqlx::query()` with `.bind()` for every parameter. No string interpolation in SQL.

- **Transaction per entry:** Each `GuidanceEntry` is wrapped in its own transaction via `db.begin()`. If inserting child rows fails, only that entry rolls back. The existing pattern in `import.rs` does not use transactions, but guidance data has parent-child relationships that require atomicity.

- **UUID generation:** Use `uuid::Uuid::new_v4().to_string()` for the `id` column of every inserted row (concept_guidance, concept_actions, concept_transparency_questions, concept_references). The `uuid` crate with `v4` feature is already in `Cargo.toml`.

- **Upsert for concept_guidance:** `INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en, about_nb) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT(concept_id) DO UPDATE SET source_pdf = excluded.source_pdf, source_page = excluded.source_page, about_en = excluded.about_en, about_nb = excluded.about_nb, updated_at = datetime('now')`. On initial insert, `id` is a new UUID. On conflict, the id stays the same (only the SET columns update).

- **Delete-reinsert for child tables:** Before inserting new child rows, delete all existing rows: `DELETE FROM concept_actions WHERE concept_id = ?`, and same for `concept_transparency_questions` and `concept_references`. This ensures re-import replaces data rather than appending.

- **Sort order:** 1-based, derived from the `enumerate()` index. For `concept_references`, resources are numbered first, then academic references continue the sequence.

- **FTS5 rebuild:** After all entries are processed (even if some were skipped), execute the rebuild command. This uses `sqlx::query()` (not the macro) because FTS5 control commands are not standard SQL that the macro can type-check.

- **Logging:** Follow the existing pattern using `tracing::{info, warn}`. Log at info level for start/completion, warn level for skipped entries.

---

## Checklist

1. Add `GuidanceFile`, `GuidanceEntry`, `ResourceEntry`, `ReferenceEntry` structs to `backend/src/import.rs`
2. Add `import_guidance_file()` async function to `backend/src/import.rs`
3. Ensure the new types and function are `pub` so they are accessible from tests
4. Create `backend/tests/guidance_tests.rs` with deserialization and import tests
5. Run `cargo test` to verify deserialization tests pass
6. Run import tests against in-memory SQLite with migrations applied
7. Verify no regressions in existing tests (`cargo test`)

---

## Implementation Notes (Post-Build)

### Deviations from Plan

1. **Extracted `import_guidance_entry()` helper** — Per-entry logic was moved into a private async helper function to enable error isolation. If one entry fails, the error is logged as a warning and remaining entries continue processing. This matches the resilience pattern used in `import_relationships()`.

2. **Bilingual array iteration uses max(en, nb) length** — The plan only iterated over `_en` arrays. The implementation iterates over `max(en.len(), nb.len())` so Norwegian-only data is preserved. When `_en` is absent, an empty string is used (the `action_text_en` / `question_text_en` columns are NOT NULL).

### Files Modified
- `backend/src/import.rs` — Added 4 types + `import_guidance_file()` + `import_guidance_entry()` helper
- `backend/tests/guidance_tests.rs` — Added 12 new tests (5 deserialization + 7 import function tests including bilingual coverage)

### Test Count
- 22 total tests in `guidance_tests.rs` (8 existing schema + 5 deserialization + 9 import)