Now I have all the context needed. Here is the section content:

# Section 1: Database Migration

## Overview

This section creates the SQLite migration file `003_analysis_schema.sql` with two new tables (`analyses` and `analysis_findings`) and modifies `backend/src/main.rs` to enable `PRAGMA foreign_keys = ON` globally on the connection pool. These are foundational changes that later sections (section-03-models, section-05-wiring) depend on.

## Prerequisites

- No dependencies on other sections. This section can be implemented in parallel with section-02-enums.
- The existing migrations `001_initial_schema.sql` and `002_evidence_schema_update.sql` must already exist (they do).

## Tests

There are no unit tests for this section. Validation is done by:

1. The backend starting without errors (existing `sqlx::migrate!("./migrations")` call runs all migrations)
2. Tables exist and accept inserts
3. For the PRAGMA change: after pool creation, `PRAGMA foreign_keys` returns `1` (enabled)
4. Deleting an `analyses` row cascades to delete its `analysis_findings` rows

These are integration-level checks validated at startup and by later sections' tests. No new test files need to be created for this section.

## File 1: `backend/migrations/003_analysis_schema.sql`

Create this new migration file. Follow the conventions from `001_initial_schema.sql`:

- Use `CREATE TABLE IF NOT EXISTS`
- Use `CREATE INDEX IF NOT EXISTS`
- Use `TEXT DEFAULT (datetime('now'))` for timestamps
- Use `TEXT PRIMARY KEY` with UUID v4 strings for IDs
- Separate logical groups with `-- ====` section dividers
- Inline comments describing CHECK constraint values

### Table: `analyses`

Stores each analysis run. Fields in order:

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| `id` | TEXT | PRIMARY KEY | UUID v4 string |
| `name` | TEXT | NOT NULL | User-provided name |
| `description` | TEXT | (nullable) | Optional description |
| `input_type` | TEXT | NOT NULL, CHECK (`'text'`, `'pdf'`, `'docx'`) | Type of input document |
| `input_text` | TEXT | (nullable) | Set for text-type input |
| `original_filename` | TEXT | (nullable) | Original uploaded filename |
| `file_path` | TEXT | (nullable) | Server-side file storage path |
| `extracted_text` | TEXT | (nullable) | Populated after parsing a file |
| `status` | TEXT | NOT NULL DEFAULT `'pending'`, CHECK (`'pending'`, `'processing'`, `'completed'`, `'failed'`, `'deleted'`) | Soft delete via status |
| `error_message` | TEXT | (nullable) | Error details on failure |
| `prompt_template` | TEXT | (nullable) | JSON matching configuration |
| `matched_framework_ids` | TEXT | (nullable) | JSON array e.g. `["nist-csf","iso31000"]` |
| `processing_time_ms` | INTEGER | (nullable) | Wall-clock duration in ms |
| `token_count` | INTEGER | (nullable) | Document word/token estimate |
| `created_by` | TEXT | (nullable) | User ID (auth not yet implemented) |
| `created_at` | TEXT | DEFAULT `(datetime('now'))` | |
| `updated_at` | TEXT | DEFAULT `(datetime('now'))` | |

Indexes:
- `idx_analyses_status` on `(status)`
- `idx_analyses_created_by` on `(created_by)`
- `idx_analyses_created_at` on `(created_at)`

### Table: `analysis_findings`

Links an analysis to ontology concepts with scoring. Fields in order:

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| `id` | TEXT | PRIMARY KEY | UUID v4 string |
| `analysis_id` | TEXT | NOT NULL, REFERENCES `analyses(id)` ON DELETE CASCADE | Cascade ensures findings are removed with their analysis |
| `concept_id` | TEXT | NOT NULL, REFERENCES `concepts(id)` | Must reference existing ontology concept |
| `framework_id` | TEXT | NOT NULL, REFERENCES `frameworks(id)` | Denormalized for efficient filtering |
| `finding_type` | TEXT | NOT NULL, CHECK (`'addressed'`, `'partially_addressed'`, `'gap'`, `'not_applicable'`) | |
| `confidence_score` | REAL | NOT NULL, CHECK `BETWEEN 0.0 AND 1.0` | |
| `evidence_text` | TEXT | (nullable) | Document excerpt that matched |
| `recommendation` | TEXT | (nullable) | Generated action item |
| `priority` | INTEGER | NOT NULL, CHECK `BETWEEN 1 AND 4` | 1=critical, 4=low |
| `sort_order` | INTEGER | DEFAULT 0 | Display ordering |
| `created_at` | TEXT | DEFAULT `(datetime('now'))` | |

Indexes:
- `idx_analysis_findings_analysis` on `(analysis_id)`
- `idx_analysis_findings_framework` on `(framework_id)`
- `idx_analysis_findings_type` on `(finding_type)`
- `idx_analysis_findings_priority` on `(priority)`
- `idx_analysis_findings_analysis_type_priority` on `(analysis_id, finding_type, priority)` -- composite for the most common query pattern

### Migration SQL Structure

The file should follow this structure (matching 001 conventions):

```sql
-- Document Analysis Engine - Analysis Schema
-- Split 01: Database Models Foundation

-- ============================================================================
-- ANALYSIS TABLES
-- ============================================================================

CREATE TABLE IF NOT EXISTS analyses (
    -- ... columns as specified above with CHECK constraints inline ...
);

-- ============================================================================
-- ANALYSIS FINDINGS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS analysis_findings (
    -- ... columns as specified above with FK and CHECK constraints inline ...
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Analysis indexes
CREATE INDEX IF NOT EXISTS idx_analyses_status ON analyses(status);
CREATE INDEX IF NOT EXISTS idx_analyses_created_by ON analyses(created_by);
CREATE INDEX IF NOT EXISTS idx_analyses_created_at ON analyses(created_at);

-- Finding indexes
CREATE INDEX IF NOT EXISTS idx_analysis_findings_analysis ON analysis_findings(analysis_id);
CREATE INDEX IF NOT EXISTS idx_analysis_findings_framework ON analysis_findings(framework_id);
CREATE INDEX IF NOT EXISTS idx_analysis_findings_type ON analysis_findings(finding_type);
CREATE INDEX IF NOT EXISTS idx_analysis_findings_priority ON analysis_findings(priority);
CREATE INDEX IF NOT EXISTS idx_analysis_findings_analysis_type_priority ON analysis_findings(analysis_id, finding_type, priority);
```

## File 2: `backend/src/main.rs`

Modify the existing SQLite pool creation to enable foreign key enforcement on every connection.

### Current code (lines 48-51)

```rust
let db = SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await?;
```

### Required change

Add an `.after_connect(...)` callback between `.max_connections(5)` and `.connect(...)` that executes `PRAGMA foreign_keys = ON` on each new connection. The callback signature uses `sqlx::sqlite::SqliteConnection` and returns a boxed future.

The `after_connect` callback needs:
- `use sqlx::Executor;` (if not already imported) for calling `.execute()` on the connection
- The closure receives `(conn, _meta)` where `conn` is `&mut SqliteConnection`
- Execute `PRAGMA foreign_keys = ON` as a raw SQL statement

### Foreign key integrity check

After migrations run (line 54: `sqlx::migrate!("./migrations").run(&db).await?;`), add a `PRAGMA foreign_key_check` query. This catches orphaned rows from before enforcement was enabled.

- If violations are found, log them as warnings using `tracing::warn!`
- Do NOT fail startup -- legacy data gaps should not prevent the server from starting
- If no violations, log an info message confirming FK integrity

The check should look something like:

```rust
// After migrations, check for FK violations
let violations: Vec<(String, i64, String, i64)> = sqlx::query_as(
    "PRAGMA foreign_key_check"
)
.fetch_all(&db)
.await?;

if !violations.is_empty() {
    tracing::warn!(
        "Found {} foreign key violations in existing data",
        violations.len()
    );
    for (table, rowid, parent, fkid) in &violations {
        tracing::warn!(
            "FK violation: table={}, rowid={}, parent={}, fkid={}",
            table, rowid, parent, fkid
        );
    }
} else {
    tracing::info!("Foreign key integrity check passed");
}
```

### Why this matters

Without `PRAGMA foreign_keys = ON`, SQLite silently ignores all FK constraints including `ON DELETE CASCADE`. The existing schema already defines FK constraints on `compliance_items`, `evidence`, `sessions`, etc., but they were never enforced. Enabling this globally makes all existing and new FK constraints work as intended.

## Implementation Checklist

1. Create `backend/migrations/003_analysis_schema.sql` with both tables, CHECK constraints, and all indexes
2. Modify `backend/src/main.rs` to add `.after_connect()` callback enabling `PRAGMA foreign_keys = ON`
3. Add `PRAGMA foreign_key_check` after migrations in `main.rs`
4. Run `cargo check` from `backend/` to verify compilation
5. Run `cargo run` briefly to verify migrations apply without errors (or rely on existing test infrastructure)

## Key Files

- **Create:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/migrations/003_analysis_schema.sql`
- **Modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs` (pool creation around line 48, and post-migration check around line 56)