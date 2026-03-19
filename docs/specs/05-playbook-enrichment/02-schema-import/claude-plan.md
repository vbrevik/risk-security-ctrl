# Implementation Plan: Database Schema & Import Pipeline for Guidance Data

## 1. Background and Goals

The NIST AI RMF Playbook extraction (spec 01-data-extraction) produces a companion JSON file (`ontology-data/nist-ai-rmf-guidance.json`) containing structured guidance for 75 action-level concepts — about text, suggested actions, transparency questions, and references. This plan adds the database schema and import pipeline to load that data.

The system already has an ontology import pipeline in `backend/src/import.rs` that loads framework JSON files into SQLite via SQLx. This plan extends that pipeline with:
1. New database tables for guidance data (migration 004)
2. An FTS5 index for full-text search across guidance content
3. A new `import_guidance_file()` function for companion JSON files
4. Dynamic scanning for `*-guidance.json` files in `import_all_ontologies()`

All changes are **additive** — existing tables, imports, and behavior are untouched.

---

## 2. Database Migration

### File: `backend/migrations/004_guidance_data_schema.sql`

This is the next sequential migration after the existing 003. It creates four new tables, indexes, an FTS5 virtual table, and a content view.

### Tables

**`concept_guidance`** — One row per concept that has guidance data:

| Column | Type | Constraints |
|--------|------|-------------|
| id | TEXT | PRIMARY KEY (UUID) |
| concept_id | TEXT | FOREIGN KEY → concepts(id) ON DELETE CASCADE, UNIQUE |
| source_pdf | TEXT | NOT NULL |
| source_page | INTEGER | NOT NULL |
| about_en | TEXT | |
| about_nb | TEXT | |
| created_at | TEXT | DEFAULT (datetime('now')) |
| updated_at | TEXT | DEFAULT (datetime('now')) |

**`concept_actions`** — Ordered suggested actions per concept:

| Column | Type | Constraints |
|--------|------|-------------|
| id | TEXT | PRIMARY KEY (UUID) |
| concept_id | TEXT | FOREIGN KEY → concepts(id) ON DELETE CASCADE |
| action_text_en | TEXT | NOT NULL |
| action_text_nb | TEXT | |
| sort_order | INTEGER | NOT NULL |
| created_at | TEXT | DEFAULT (datetime('now')) |

UNIQUE constraint on (concept_id, sort_order).

**`concept_transparency_questions`** — Documentation questions per concept:

| Column | Type | Constraints |
|--------|------|-------------|
| id | TEXT | PRIMARY KEY (UUID) |
| concept_id | TEXT | FOREIGN KEY → concepts(id) ON DELETE CASCADE |
| question_text_en | TEXT | NOT NULL |
| question_text_nb | TEXT | |
| sort_order | INTEGER | NOT NULL |
| created_at | TEXT | DEFAULT (datetime('now')) |

UNIQUE constraint on (concept_id, sort_order).

**`concept_references`** — Citations and transparency resources per concept:

| Column | Type | Constraints |
|--------|------|-------------|
| id | TEXT | PRIMARY KEY (UUID) |
| concept_id | TEXT | FOREIGN KEY → concepts(id) ON DELETE CASCADE |
| reference_type | TEXT | NOT NULL, CHECK(reference_type IN ('academic', 'transparency_resource')) |
| title | TEXT | NOT NULL |
| authors | TEXT | |
| year | INTEGER | |
| venue | TEXT | |
| url | TEXT | |
| sort_order | INTEGER | NOT NULL |
| created_at | TEXT | DEFAULT (datetime('now')) |

### Indexes

```sql
-- Note: concept_guidance(concept_id) already has a UNIQUE index, no explicit index needed
CREATE INDEX idx_concept_actions_concept ON concept_actions(concept_id);
CREATE INDEX idx_concept_questions_concept ON concept_transparency_questions(concept_id);
CREATE INDEX idx_concept_references_concept ON concept_references(concept_id);
CREATE INDEX idx_concept_references_type ON concept_references(reference_type);
```

### FTS5 Extension

A separate FTS5 table (not extending the existing `concepts_fts`) indexes the guidance about text alongside the concept name and definition for richer search.

**Content view:**

```sql
CREATE VIEW concept_guidance_search_v AS
SELECT
    cg.rowid AS rowid,
    c.name_en,
    c.definition_en,
    cg.about_en
FROM concept_guidance cg
JOIN concepts c ON c.id = cg.concept_id;
```

**FTS5 virtual table:**

```sql
CREATE VIRTUAL TABLE concept_guidance_fts USING fts5(
    name_en,
    definition_en,
    about_en,
    content='concept_guidance_search_v',
    content_rowid='rowid'
);
```

**No triggers.** The index is populated via the `'rebuild'` command after each import:

```sql
INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
```

This approach is simpler than cross-table triggers and appropriate for batch import workflows where data changes infrequently.

The migration ends with a rebuild to index any existing data (empty on first run, but safe for re-runs).

**Example downstream query** — to search guidance and retrieve concept details:

```sql
SELECT cg.concept_id, c.name_en, c.code, cg.about_en
FROM concept_guidance_fts
JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid
JOIN concepts c ON c.id = cg.concept_id
WHERE concept_guidance_fts MATCH 'search term'
ORDER BY rank;
```

This joins FTS results back through the rowid to get concept_id and other metadata.

**Note on Norwegian fields:** The FTS5 index currently covers English fields only (`name_en`, `definition_en`, `about_en`). When Norwegian guidance is added later, the content view will need updating to include the `_nb` columns.

**Migration numbering:** Verify no other branch has claimed migration number 004 before creating this file. If 004 is taken, use the next available number.

---

## 3. Import Types

### File: `backend/src/import.rs` (additions)

New Rust types for deserializing `*-guidance.json` companion files:

```rust
#[derive(Debug, Deserialize)]
pub struct GuidanceFile {
    pub framework_id: String,
    pub source_pdf: String,
    pub guidance: Vec<GuidanceEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GuidanceEntry {
    pub concept_id: String,
    pub source_page: i64,
    pub about_en: Option<String>,
    pub about_nb: Option<String>,
    pub suggested_actions_en: Option<Vec<String>>,
    pub suggested_actions_nb: Option<Vec<String>>,
    pub transparency_questions_en: Option<Vec<String>>,
    pub transparency_questions_nb: Option<Vec<String>>,
    pub resources: Option<Vec<ResourceEntry>>,
    pub references: Option<Vec<ReferenceEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceEntry {
    pub title: String,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReferenceEntry {
    pub title: String,
    pub authors: Option<String>,
    pub year: Option<i64>,
    pub venue: Option<String>,
    pub url: Option<String>,
}
```

These types mirror the JSON schema from spec 01-data-extraction. The `serde(rename = "type")` attribute handles the JSON `"type"` field which is a Rust keyword.

**Note:** The JSON has a top-level `extracted_at` timestamp field. This is intentionally not imported into the database — the provenance is preserved in the source JSON file itself.

### Mapping to `concept_references`

The JSON has two separate arrays that both map to the `concept_references` table:

- **`resources`** array → stored with `reference_type = "transparency_resource"`. The `ResourceEntry.resource_type` field from JSON is not used as the discriminator; all resources get the same type.
- **`references`** array → stored with `reference_type = "academic"`. The `ReferenceEntry` fields (authors, year, venue) map directly to the table columns.

---

## 4. Import Function

### `import_guidance_file(db: &SqlitePool, file_path: &Path) -> Result<(), Box<dyn Error>>`

This function reads a `*-guidance.json` file and imports its contents into the four guidance tables.

### Flow

1. Read and parse the JSON file into `GuidanceFile`
2. Log the framework_id and number of guidance entries
3. For each `GuidanceEntry`:
   a. **Validate concept_id exists** — query `SELECT id FROM concepts WHERE id = ?`. If not found, log a warning and skip. Note: this is an improvement over the existing `import_relationships()` which catches FK errors after the fact. Pre-validation provides clearer logging.
   b. **Begin transaction** — use `db.begin()` to start an explicit transaction for this entry
   c. **Upsert `concept_guidance`** — `INSERT INTO concept_guidance ... ON CONFLICT(concept_id) DO UPDATE SET` for source_pdf, source_page, about_en, about_nb, updated_at
   d. **Delete existing child rows** — `DELETE FROM concept_actions WHERE concept_id = ?`, same for transparency_questions and references
   e. **Insert new child rows** — loop with `Uuid::new_v4().to_string()` for each id, 1-based sort_order
   f. **Commit transaction**
4. After all entries: **rebuild FTS5 index** — `INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild')`
5. Log completion

### Concept Validation (STIG V-222606)

Before inserting guidance data, verify the concept_id exists in the `concepts` table. If it doesn't, log a warning with the invalid concept_id and skip that entry. This prevents orphaned guidance data and matches the existing pattern in `import_relationships()` where missing concepts are warned about but don't halt the import.

### Query Safety (STIG V-222607)

All database queries use SQLx parameterized query macros (`sqlx::query!()` with `?` bind parameters). No string concatenation or formatting for SQL construction.

---

## 5. Wiring into import_all_ontologies()

After the existing framework imports and relationships import, add a **dynamic scan** for guidance files:

```rust
// After: import_relationships(...)

// Scan for *-guidance.json files
// Use tokio::fs for async consistency with the rest of the codebase
let mut entries = tokio::fs::read_dir(data_dir).await?;
while let Some(entry) = entries.next_entry().await? {
    let name = entry.file_name().to_string_lossy().to_string();
    if name.ends_with("-guidance.json") {
        import_guidance_file(db, &entry.path()).await?;
    }
}
```

This pattern is auto-extensible — when `nist-csf-guidance.json` or other companion files are added later, they'll be imported automatically.

The guidance import runs **after** framework/concept imports so that foreign key references to `concepts(id)` are valid.

---

## 6. SQLx Offline Query Data

After adding the migration and new queries, run `cargo sqlx prepare` to regenerate the offline query data in `.sqlx/`. This is required because the project uses `sqlx::query!()` macros which are compile-time checked against the schema.

---

## 7. Testing Strategy

### Migration Tests

In `backend/tests/`:
- Run the existing `create_test_app()` which auto-runs all migrations including the new 004
- Verify the new tables exist by querying `sqlite_master`
- Verify indexes exist

### Import Function Tests

1. **Happy path:** Create a test guidance JSON string with 2-3 entries, write to a temp file, import, verify rows exist in all four tables with correct data
2. **Invalid concept_id:** Include an entry with a non-existent concept_id, verify it's skipped with a warning (not an error)
3. **Upsert behavior:** Import the same file twice, verify no duplicate rows (concept_guidance upsert, child rows replaced)
4. **Transaction atomicity:** If one child row insert fails, the entire concept entry should be rolled back

### FTS5 Tests

1. **Rebuild after import:** After importing guidance, verify `SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH 'keyword'` returns results
2. **Search across fields:** Verify searching for concept name, definition, or about text all return results

### Existing Tests

Run the existing test suite to verify no regressions — the new migration and import code are additive and should not affect existing functionality.

---

## 8. File Changes Summary

### New Files
- `backend/migrations/004_guidance_data_schema.sql` — Migration with tables, indexes, view, FTS5

### Modified Files
- `backend/src/import.rs` — Add `GuidanceFile` types, `import_guidance_file()`, guidance scan in `import_all_ontologies()`

### Test Files
- `backend/tests/guidance_tests.rs` — New test file for import and FTS5 tests

### Runtime Output
- `.sqlx/` — Updated offline query data (after `cargo sqlx prepare`)
