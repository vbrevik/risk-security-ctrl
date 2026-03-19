# Specification: Database Schema & Import Pipeline for Guidance Data

## Overview

Add database tables for concept guidance data (about text, suggested actions, transparency questions, references) and extend the import pipeline to load companion `*-guidance.json` files dynamically. Includes FTS5 index extension for full-text search across guidance content using a rebuild-based approach.

All changes are additive — no modifications to existing tables or import behavior.

## Database Schema

### New Tables (migration `004_guidance_data_schema.sql`)

1. **`concept_guidance`** — One row per concept with guidance data. Fields: id (UUID PK), concept_id (FK → concepts, UNIQUE), source_pdf, source_page, about_en, about_nb, timestamps.

2. **`concept_actions`** — Ordered suggested actions per concept. Fields: id (UUID PK), concept_id (FK → concepts), action_text_en, action_text_nb, sort_order. UNIQUE on (concept_id, sort_order).

3. **`concept_transparency_questions`** — Documentation questions per concept. Fields: id (UUID PK), concept_id (FK → concepts), question_text_en, question_text_nb, sort_order. UNIQUE on (concept_id, sort_order).

4. **`concept_references`** — Citations and resources per concept. Fields: id (UUID PK), concept_id (FK → concepts), reference_type ("academic"/"transparency_resource"), title, authors, year, venue, url, sort_order.

5. **Indexes** on concept_id for all child tables, plus reference_type index.

### FTS5 Extension

Use a **separate `concept_guidance_fts` table** with **rebuild-based sync** (not per-row triggers). After each guidance import, run `INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild')` to rebuild the index from a content view joining concepts + concept_guidance.

This approach is simpler than cross-table triggers and appropriate for batch import workflows.

## Import Pipeline

### New Function: `import_guidance_file(db, path)`
- Parse `*-guidance.json` companion file into Rust structs
- For each guidance entry: validate concept_id exists, upsert concept_guidance (ON CONFLICT on concept_id), delete-and-reinsert ordered child rows (actions, questions, references)
- Wrap each concept's guidance import in an **explicit transaction** for atomicity
- Generate UUIDs for new rows via `Uuid::new_v4().to_string()`

### Dynamic Scan in `import_all_ontologies()`
After importing framework files and relationships, **dynamically scan** `ontology-data/` for files matching `*-guidance.json` and import each. No hardcoded list.

## Key Decisions

- Rebuild-based FTS5 (no triggers, rebuild after import)
- Dynamic `*-guidance.json` file scanning (auto-extensible)
- Transaction per guidance entry (atomic per concept)
- ON CONFLICT DO UPDATE for concept_guidance upsert (preserves rowid)
- Delete-and-reinsert for ordered child rows (simpler than per-row upsert)

## Dependencies

- Needs from 01-data-extraction: `nist-ai-rmf-guidance.json` file structure
- Provides to 03-api-matcher: Populated database tables for API queries
