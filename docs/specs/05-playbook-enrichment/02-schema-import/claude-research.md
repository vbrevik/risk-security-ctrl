# Research: Schema Import Pipeline

## Codebase Research

### Current Migrations
- `001_initial_schema.sql` — Core tables (concepts, frameworks, relationships, compliance, auth, audit) + FTS5
- `002_evidence_schema_update.sql` — Evidence table refactor
- `003_analysis_schema.sql` — Document analysis engine tables
- **Next migration:** `004_guidance_data_schema.sql`

### Existing FTS5 Setup
```sql
CREATE VIRTUAL TABLE concepts_fts USING fts5(
    name_en, name_nb, definition_en, definition_nb,
    content='concepts', content_rowid='rowid'
);
```
Three triggers maintain the index: `concepts_ai` (INSERT), `concepts_ad` (DELETE), `concepts_au` (UPDATE). Currently, the search endpoint uses LIKE patterns, not FTS5 MATCH.

### Import Pipeline (`backend/src/import.rs`)
- `import_ontology_file(db, path)` — parses JSON, upserts framework + concepts via `ON CONFLICT(id) DO UPDATE SET`
- `import_relationships(db, path)` — upserts relationships, logs warnings for missing concepts
- `import_all_ontologies(db, data_dir)` — hardcoded list of 24 framework files, imports each, then relationships.json
- No explicit transaction wrapping
- Main.rs checks `SELECT COUNT(*) FROM frameworks` vs available JSON files; re-imports if count mismatch

### UUID Pattern
`Uuid::new_v4().to_string()` — always convert to String for SQLite TEXT columns.

### SQLx Patterns
- `sqlx::query!()` (compile-time checked) for inserts/updates
- `sqlx::query_as::<_, Type>()` for fetches
- `.bind(value)` for all parameters
- Foreign keys enforced via `PRAGMA foreign_keys = ON` in after_connect

### Testing
- Real SQLite database (not in-memory), shared across tests
- `create_test_app()` runs migrations + auto-imports ontology data
- `#[tokio::test]` with oneshot requests

---

## Web Research

### FTS5 with Companion Tables
- **External content FTS5** can use a VIEW as content source (`content='view_name'`)
- Triggers on each source table maintain the index using the `'delete'` command pattern
- For bulk/batch imports: use `INSERT INTO fts_table(fts_table) VALUES('rebuild')` instead of per-row triggers
- Simpler alternative: separate `concept_guidance_fts` table (avoids cross-table trigger complexity)

### SQLx Migrations
- Additive migrations (CREATE TABLE, CREATE VIEW, CREATE TRIGGER) are safe in transactions
- FTS5 columns must NOT have type annotations
- End FTS5 creation migrations with `'rebuild'` to index existing data
- Use `ON CONFLICT DO UPDATE` (not `INSERT OR REPLACE`) to preserve rowids and avoid cascade deletes
- `ON CONFLICT DO NOTHING` for idempotent seed data

### Upsert Best Practices
| Method | Triggers Fired | FTS5 Impact |
|--------|---------------|-------------|
| `INSERT OR REPLACE` | DELETE + INSERT | Wasteful, can cascade |
| `ON CONFLICT DO UPDATE` | UPDATE | Clean, preserves rowid |
| `ON CONFLICT DO NOTHING` | None | No impact |

**Sources:** [SQLite FTS5 docs](https://www.sqlite.org/fts5.html), [SQLite UPSERT docs](https://sqlite.org/lang_upsert.html)
