# 02-schema-import: Database Schema & Import Pipeline for Guidance Data

## Summary

Add database tables for concept guidance data and extend the import pipeline to load companion guidance JSON files. Includes FTS5 index extension for richer full-text search. All changes are additive — no modifications to existing tables or import behavior.

## Requirements Source

- Parent requirements: `docs/specs/05-playbook-enrichment/requirements.md`
- Interview: `docs/specs/05-playbook-enrichment/deep_project_interview.md`
- Manifest: `docs/specs/05-playbook-enrichment/project-manifest.md`

## What to Build

### Database Migration

New migration file (next sequential number after existing migrations). Tables:

#### `concept_guidance`
One row per concept that has guidance data.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | UUID |
| concept_id | TEXT FK → concepts(id) ON DELETE CASCADE | UNIQUE constraint |
| source_pdf | TEXT NOT NULL | e.g., "AI_RMF_Playbook.pdf" |
| source_page | INTEGER NOT NULL | Page number in PDF |
| about_en | TEXT | Extended description (1-3 paragraphs) |
| about_nb | TEXT | Norwegian translation (NULL for now) |
| created_at | TEXT DEFAULT datetime('now') | |
| updated_at | TEXT DEFAULT datetime('now') | |

#### `concept_actions`
Ordered list of suggested actions per concept.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | UUID |
| concept_id | TEXT FK → concepts(id) ON DELETE CASCADE | |
| action_text_en | TEXT NOT NULL | The suggested action |
| action_text_nb | TEXT | Norwegian (NULL for now) |
| sort_order | INTEGER NOT NULL | 1-based ordering |
| created_at | TEXT DEFAULT datetime('now') | |

UNIQUE constraint on (concept_id, sort_order).

#### `concept_transparency_questions`
Documentation questions per concept.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | UUID |
| concept_id | TEXT FK → concepts(id) ON DELETE CASCADE | |
| question_text_en | TEXT NOT NULL | |
| question_text_nb | TEXT | NULL for now |
| sort_order | INTEGER NOT NULL | |
| created_at | TEXT DEFAULT datetime('now') | |

UNIQUE constraint on (concept_id, sort_order).

#### `concept_references`
Academic citations and transparency resources per concept.

| Column | Type | Notes |
|--------|------|-------|
| id | TEXT PK | UUID |
| concept_id | TEXT FK → concepts(id) ON DELETE CASCADE | |
| reference_type | TEXT NOT NULL | "academic" or "transparency_resource" |
| title | TEXT NOT NULL | |
| authors | TEXT | NULL for resources |
| year | INTEGER | NULL if unknown |
| venue | TEXT | Conference/journal name |
| url | TEXT | External link if available |
| sort_order | INTEGER NOT NULL | |
| created_at | TEXT DEFAULT datetime('now') | |

#### Indexes
- `idx_concept_guidance_concept` on concept_guidance(concept_id)
- `idx_concept_actions_concept` on concept_actions(concept_id)
- `idx_concept_questions_concept` on concept_transparency_questions(concept_id)
- `idx_concept_references_concept` on concept_references(concept_id)
- `idx_concept_references_type` on concept_references(reference_type)

#### FTS5 Extension

Update the FTS5 virtual table or create a supplementary one to index `about_en` from `concept_guidance`. This enables full-text search across the richer guidance text. Consider the approach carefully — FTS5 content tables are tricky with JOINs. Options:
- Extend the existing `concepts_fts` triggers to also pull `about_en` from `concept_guidance` (requires careful trigger design)
- Create a separate `concept_guidance_fts` FTS5 table (simpler, but requires searching two tables)

### Import Pipeline (`backend/src/import.rs`)

#### New Types

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
    pub r#type: Option<String>,
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

#### New Function: `import_guidance_file()`

- Reads a `*-guidance.json` companion file
- For each entry: validates concept_id exists in DB
- Upserts into `concept_guidance` (ON CONFLICT on concept_id)
- Deletes and re-inserts actions/questions/references (simpler than per-row upsert for ordered lists)
- Generates UUIDs for new rows

#### Wire into `import_all_ontologies()`

After importing framework files and relationships, scan `ontology-data/` for `*-guidance.json` files and import each. This makes the pattern automatically extensible to future companion files (e.g., `nist-csf-guidance.json`).

## Key Decisions

- **Dedicated tables over concept_properties**: Better queryability, proper typing, and indexing vs. the unused key-value `concept_properties` table (interview decision)
- **Companion file pattern**: Import pipeline detects `*-guidance.json` files automatically
- **Upsert semantics**: Safe to re-run import without duplicating data
- **Norwegian columns**: Present in schema, NULL until translation phase

## Dependencies

- **Needs from 01**: The `nist-ai-rmf-guidance.json` file structure (to define Rust structs)
- **Provides to 03**: Populated database tables for API to query

## Existing Code Reference

- Current migration: `backend/migrations/001_initial_schema.sql` (concepts table, FTS5 setup)
- Analysis migration: `backend/migrations/003_analysis_schema.sql`
- Import code: `backend/src/import.rs` (ConceptData struct, import_ontology_file function)
- Check next migration number before creating
