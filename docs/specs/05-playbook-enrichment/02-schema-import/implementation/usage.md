# Usage Guide: Guidance Data Schema Import

## Quick Start

The guidance data import system loads `*-guidance.json` companion files into four database tables, making NIST AI RMF playbook guidance searchable via FTS5 full-text search.

### Automatic Import (default)

Guidance files are automatically imported when the server starts, as part of `import_all_ontologies()`. Place any `*-guidance.json` file in the `ontology-data/` directory and restart.

```bash
cd backend
cargo run
```

### Programmatic Import

```rust
use ontology_backend::import::import_guidance_file;
use sqlx::SqlitePool;
use std::path::Path;

let pool: SqlitePool = /* your pool */;
import_guidance_file(&pool, Path::new("path/to/nist-ai-rmf-guidance.json")).await?;
```

## Guidance JSON Format

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "NIST-AI-100-1.pdf",
  "extracted_at": "2026-03-20T00:00:00Z",
  "guidance": [
    {
      "concept_id": "nist-ai-gv-1-1",
      "source_page": 35,
      "about_en": "English description",
      "about_nb": "Norwegian description",
      "suggested_actions_en": ["Action 1", "Action 2"],
      "suggested_actions_nb": ["Handling 1", "Handling 2"],
      "transparency_questions_en": ["Question 1"],
      "transparency_questions_nb": ["Spørsmål 1"],
      "resources": [
        {"title": "NIST AI 100-1", "url": "https://...", "type": "standard"}
      ],
      "references": [
        {"title": "Paper Title", "authors": "Smith et al.", "year": 2023, "venue": "NeurIPS", "url": "https://..."}
      ]
    }
  ]
}
```

All fields except `concept_id` and `source_page` are optional. Bilingual arrays (`_en`/`_nb`) can have different lengths — the longer array determines the row count.

## FTS5 Full-Text Search

After import, guidance content is searchable via the `concept_guidance_fts` virtual table:

```sql
-- Simple keyword search
SELECT * FROM concept_guidance_fts
WHERE concept_guidance_fts MATCH 'governance';

-- Full join query (recommended for UI)
SELECT cg.concept_id, c.name_en, c.code, cg.about_en
FROM concept_guidance_fts
JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid
JOIN concepts c ON c.id = cg.concept_id
WHERE concept_guidance_fts MATCH 'risk assessment'
ORDER BY rank;
```

The FTS5 index covers: `name_en` (from concepts), `definition_en` (from concepts), and `about_en` (from guidance).

## Database Schema

| Table | Purpose |
|-------|---------|
| `concept_guidance` | One row per concept — source PDF, page, about text |
| `concept_actions` | Ordered suggested actions per concept |
| `concept_transparency_questions` | Ordered documentation questions per concept |
| `concept_references` | Academic refs + transparency resources per concept |
| `concept_guidance_fts` | FTS5 virtual table for full-text search |
| `concept_guidance_search_v` | Content view feeding FTS5 |

## Import Behavior

- **Upsert:** Re-importing updates existing guidance rows (no duplicates)
- **Delete-reinsert:** Child rows (actions, questions, references) are replaced on reimport
- **Error resilience:** Invalid concept IDs are skipped with warnings; one bad entry doesn't abort the file
- **FTS5 rebuild:** Happens automatically after each file import

## API Reference

### Public Types (from `ontology_backend::import`)

- `GuidanceFile` — Top-level JSON structure
- `GuidanceEntry` — Per-concept guidance data
- `ResourceEntry` — Transparency resource
- `ReferenceEntry` — Academic reference

### Public Functions

- `import_guidance_file(db: &SqlitePool, file_path: &Path) -> Result<()>` — Import a single guidance JSON file
- `import_all_ontologies(db: &SqlitePool, data_dir: &Path) -> Result<()>` — Import all ontology + guidance files
