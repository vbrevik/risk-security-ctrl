<!-- SPLIT_MANIFEST
01-data-extraction
02-schema-import
03-api-matcher
04-frontend-guidance
END_MANIFEST -->

# Project Manifest: Playbook Enrichment

## Overview

Enrich the ontology data model with structured guidance data from the NIST AI RMF Playbook PDF. Adds about sections, suggested actions, transparency questions, and references to each of the 75 action-level concepts — with PDF page traceability.

## Split Structure

### 01-data-extraction
**Extract structured guidance from Playbook PDF into companion JSON**

- Read all 75 action-level concept pages from `docs/reference-pdfs/AI_RMF_Playbook.pdf`
- Extract per concept: about_en, suggested_actions_en[], transparency_questions_en[], resources[], references[]
- Record source PDF filename and page number for each concept
- Output: `ontology-data/nist-ai-rmf-guidance.json` — companion file keyed by concept_id
- Semi-automated: Claude reads PDF pages, generates JSON, human reviews

**Output:** `ontology-data/nist-ai-rmf-guidance.json`

### 02-schema-import
**Database migration + import pipeline for guidance data**

- New migration adding tables: `concept_guidance`, `concept_actions`, `concept_transparency_questions`, `concept_references`
- All tables reference `concepts(id)` with ON DELETE CASCADE
- Source tracking columns: `source_pdf`, `source_page` on concept_guidance
- Extend FTS5 triggers to index `about_en` from concept_guidance
- Extend `ConceptData`/`OntologyFile` structs in `import.rs` or add new `GuidanceFile` struct
- Add `import_guidance_file()` function to read companion JSON and populate new tables
- Wire into `import_all_ontologies()` to auto-detect `*-guidance.json` companion files
- Norwegian `_nb` columns included but left NULL

**Dependencies:** 01-data-extraction (needs JSON format to define import structs)

### 03-api-matcher
**Extend concept detail API + enhance analysis matcher**

- Extend `Concept` or add `ConceptDetail` response model with nested guidance object
- Modify `GET /api/concepts/:id` handler to JOIN guidance tables and return enriched response
- Update OpenAPI/utoipa annotations for new response fields
- Enhance matcher: include `about_en` and suggested action text in TF-IDF scoring
- Gap findings can reference specific suggested actions in recommendation text
- Keep list endpoints lean (no guidance in paginated responses)

**Dependencies:** 02-schema-import (needs DB tables and imported data)

### 04-frontend-guidance
**Display guidance data in concept detail view**

- New `ConceptGuidance` component with collapsible sections:
  - About (rendered as paragraphs)
  - Suggested Actions (ordered checklist, potentially linkable to compliance items)
  - Transparency & Documentation Questions (bulleted list)
  - References (linked citations with author/year)
- Source badge showing "AI_RMF_Playbook.pdf p.93" with traceability
- TanStack Query hook for concept detail with guidance
- i18n keys added (English values populated, Norwegian deferred)
- Graceful handling when concept has no guidance data (most frameworks)

**Dependencies:** 03-api-matcher (needs API returning guidance data)

## Dependency Graph

```
01-data-extraction
        │
        ▼
02-schema-import
        │
        ▼
03-api-matcher
        │
        ▼
04-frontend-guidance
```

Linear pipeline — each split depends on the previous. No parallelism between splits, but within each split the work is internally parallelizable (e.g., extract multiple PDF sections concurrently in 01).

## Execution Order

```
/deep-plan @docs/specs/05-playbook-enrichment/01-data-extraction/spec.md
/deep-plan @docs/specs/05-playbook-enrichment/02-schema-import/spec.md
/deep-plan @docs/specs/05-playbook-enrichment/03-api-matcher/spec.md
/deep-plan @docs/specs/05-playbook-enrichment/04-frontend-guidance/spec.md
```

## Cross-Cutting Concerns

- **Backward compatibility**: All guidance fields optional. Existing 24 ontology files import unchanged.
- **Source traceability**: Every extracted item carries `source_pdf` + `source_page` through the entire pipeline to the frontend.
- **Extensibility**: Schema and import pipeline designed for future companion files from other PDFs (NIST AI 100-1, SP 800-37, AI 600-1).
- **Norwegian**: Schema supports `_nb` fields throughout. Content deferred to future phase.
