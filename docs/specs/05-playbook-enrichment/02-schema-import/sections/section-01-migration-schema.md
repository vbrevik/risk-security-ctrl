Now I have all the context I need. Let me produce the section content.

# Section 01: Database Migration Schema

## Overview

This section creates migration file `backend/migrations/004_guidance_data_schema.sql` containing four new tables, indexes, a content view, and an FTS5 virtual table for storing and searching NIST AI RMF playbook guidance data. This migration is purely additive and does not modify any existing tables.

**Files to create:**
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/migrations/004_guidance_data_schema.sql`

**Files to modify:** None

**Dependencies:** None (this is the foundation section; sections 02, 03, and 04 all depend on it).

---

## Pre-flight Check

Before creating the migration file, confirm that no other migration has claimed the number `004`. The existing migrations are:

- `001_initial_schema.sql` -- frameworks, concepts, relationships, compliance, etc.
- `002_evidence_schema_update.sql`
- `003_analysis_schema.sql` -- analyses, analysis_findings

If another branch has introduced a `004` migration, use the next available number and adjust the filename accordingly.

---

## Tests (write these first)

All tests go in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/guidance_tests.rs`. The project uses `#[tokio::test]` with an in-memory SQLite pool created by running all migrations. Follow the existing test pattern of creating a pool, running migrations, then asserting against `sqlite_master`.

```rust
// File: backend/tests/guidance_tests.rs

// Test: migration 004 creates concept_guidance table
//   Query: SELECT name FROM sqlite_master WHERE type='table' AND name='concept_guidance'
//   Assert: returns one row

// Test: migration 004 creates concept_actions table with UNIQUE(concept_id, sort_order)
//   Insert two rows with same (concept_id, sort_order) -> expect UNIQUE constraint error

// Test: migration 004 creates concept_transparency_questions table
//   Query sqlite_master, assert table exists

// Test: migration 004 creates concept_references table with CHECK on reference_type
//   Insert row with reference_type = 'invalid_type' -> expect CHECK constraint error
//   Insert row with reference_type = 'academic' -> succeeds
//   Insert row with reference_type = 'transparency_resource' -> succeeds

// Test: migration 004 creates concept_guidance_search_v view
//   Query: SELECT name FROM sqlite_master WHERE type='view' AND name='concept_guidance_search_v'
//   Assert: returns one row

// Test: migration 004 creates concept_guidance_fts virtual table
//   Query: SELECT name FROM sqlite_master WHERE type='table' AND name='concept_guidance_fts'
//   Assert: returns one row

// Test: indexes exist on child tables
//   Query: SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_concept_%'
//   Assert: returns at least idx_concept_actions_concept, idx_concept_questions_concept,
//           idx_concept_references_concept, idx_concept_references_type

// Test: ON DELETE CASCADE removes guidance when concept is deleted
//   1. Insert a framework, a concept, a concept_guidance row, and child rows
//   2. DELETE FROM concepts WHERE id = ?
//   3. Assert concept_guidance, concept_actions, concept_transparency_questions,
//      concept_references all have zero rows for that concept_id
```

These tests validate the schema is correct before any import code is written. They require only a database pool with migrations applied and some seed data (a framework + concept row for FK satisfaction).

---

## Implementation: Migration SQL

Create file `/Users/vidarbrevik/projects/risk-security-ctrl/backend/migrations/004_guidance_data_schema.sql` with the following content. The style follows existing migrations (e.g., `003_analysis_schema.sql`): section comments with `=====` separators, `CREATE TABLE IF NOT EXISTS`, `TEXT PRIMARY KEY` for UUIDs, `datetime('now')` defaults.

### Tables

**`concept_guidance`** -- One row per concept that has guidance. The `concept_id` column has a UNIQUE constraint (which also serves as an implicit index, so no separate index is needed). Foreign key cascades to `concepts(id)`.

Columns:
- `id TEXT PRIMARY KEY` -- UUID
- `concept_id TEXT NOT NULL UNIQUE REFERENCES concepts(id) ON DELETE CASCADE`
- `source_pdf TEXT NOT NULL` -- name of the originating PDF
- `source_page INTEGER NOT NULL` -- page number in the PDF
- `about_en TEXT` -- English "about" prose (nullable)
- `about_nb TEXT` -- Norwegian "about" prose (nullable, for future use)
- `created_at TEXT DEFAULT (datetime('now'))`
- `updated_at TEXT DEFAULT (datetime('now'))`

**`concept_actions`** -- Ordered suggested actions. Has a composite UNIQUE constraint on `(concept_id, sort_order)` to prevent duplicate ordering.

Columns:
- `id TEXT PRIMARY KEY`
- `concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE`
- `action_text_en TEXT NOT NULL`
- `action_text_nb TEXT`
- `sort_order INTEGER NOT NULL`
- `created_at TEXT DEFAULT (datetime('now'))`
- Constraint: `UNIQUE(concept_id, sort_order)`

**`concept_transparency_questions`** -- Documentation/transparency questions, also ordered.

Columns:
- `id TEXT PRIMARY KEY`
- `concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE`
- `question_text_en TEXT NOT NULL`
- `question_text_nb TEXT`
- `sort_order INTEGER NOT NULL`
- `created_at TEXT DEFAULT (datetime('now'))`
- Constraint: `UNIQUE(concept_id, sort_order)`

**`concept_references`** -- Citations and transparency resources. Uses a CHECK constraint to restrict `reference_type` to known values.

Columns:
- `id TEXT PRIMARY KEY`
- `concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE`
- `reference_type TEXT NOT NULL CHECK(reference_type IN ('academic', 'transparency_resource'))`
- `title TEXT NOT NULL`
- `authors TEXT`
- `year INTEGER`
- `venue TEXT`
- `url TEXT`
- `sort_order INTEGER NOT NULL`
- `created_at TEXT DEFAULT (datetime('now'))`

### Indexes

```sql
CREATE INDEX idx_concept_actions_concept ON concept_actions(concept_id);
CREATE INDEX idx_concept_questions_concept ON concept_transparency_questions(concept_id);
CREATE INDEX idx_concept_references_concept ON concept_references(concept_id);
CREATE INDEX idx_concept_references_type ON concept_references(reference_type);
```

No index is needed on `concept_guidance(concept_id)` because the UNIQUE constraint creates one implicitly.

### Content View

The view joins `concept_guidance` with `concepts` to combine the guidance "about" text with the concept name and definition. This view serves as the content source for the FTS5 virtual table.

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

The `rowid` column is critical -- FTS5 uses it to join search results back to the source rows.

### FTS5 Virtual Table

```sql
CREATE VIRTUAL TABLE concept_guidance_fts USING fts5(
    name_en,
    definition_en,
    about_en,
    content='concept_guidance_search_v',
    content_rowid='rowid'
);
```

This is a **content-synced** FTS5 table (external content). It does not store its own copy of the text; it reads from the view. The tradeoff is that the index must be explicitly rebuilt after data changes. No triggers are used -- the rebuild command is called programmatically after each import batch (implemented in section 02).

### Initial FTS5 Rebuild

The migration ends with a rebuild command. On first run this indexes zero rows (the tables are empty), but it is safe and ensures the FTS index is in a valid state:

```sql
INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
```

### Example Downstream Query

For reference (not part of the migration), this is how FTS results join back to get concept metadata:

```sql
SELECT cg.concept_id, c.name_en, c.code, cg.about_en
FROM concept_guidance_fts
JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid
JOIN concepts c ON c.id = cg.concept_id
WHERE concept_guidance_fts MATCH 'search term'
ORDER BY rank;
```

### Norwegian Fields Note

The FTS5 index currently covers English fields only (`name_en`, `definition_en`, `about_en`). When Norwegian guidance translations are added in the future, the content view and FTS5 table will need to be updated to include the `_nb` columns. This is a known limitation, not a bug.

---

## Verification Checklist

After implementing this section:

1. The file `backend/migrations/004_guidance_data_schema.sql` exists and contains all four tables, indexes, the view, the FTS5 virtual table, and the rebuild command.
2. Running `cargo test` from `backend/` does not break any existing tests (the migration is purely additive).
3. The schema tests in `backend/tests/guidance_tests.rs` pass, confirming tables, indexes, view, FTS5 table, constraints, and cascade behavior all work correctly.
4. Run `cargo sqlx prepare` from `backend/` to update the `.sqlx/` offline query data with the new schema. (This step is also referenced in section 03 but should be done here to keep the build green.)