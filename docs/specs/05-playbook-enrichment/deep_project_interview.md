# Deep Project Interview: Playbook Enrichment

## Date: 2026-03-18

## Context

The user wants to enrich the ontology data model with structured guidance data from NIST reference PDFs. Currently each concept has only a one-line `definition_en`. The NIST AI RMF Playbook PDF contains significantly richer content: multi-paragraph about sections, 4-12 suggested actions, transparency questions, and academic references.

Four PDFs have been downloaded to `docs/reference-pdfs/`:
- AI_RMF_Playbook.pdf (2.7 MB) — 75 action-level concepts
- NIST.AI.100-1.pdf (1.9 MB) — AI RMF 1.0
- NIST.AI.600-1.pdf (1.1 MB) — GenAI Profile
- NIST.SP.800-37r2.pdf (2.2 MB) — Parent RMF

## Interview Decisions

### Q1: Data Extraction Approach
**Decision:** Semi-automated with Claude
- Claude reads PDF page ranges and generates structured JSON
- Human reviews output for accuracy
- This becomes a separate split before code changes

### Q2: Data File Layout
**Decision:** Separate companion file
- New file `nist-ai-rmf-guidance.json` alongside existing `nist-ai-rmf.json`
- Links by concept_id
- Keeps existing file lean, diffs reviewable, extraction data independent

### Q3: PDF Scope
**Decision:** Playbook only, others later
- Focus on AI_RMF_Playbook.pdf (75 concepts, richest structured data)
- Schema designed to be extensible for other PDFs in future
- Other 3 PDFs become follow-up work

### Q4: Norwegian Translation
**Decision:** Schema supports it, defer content
- Add `_nb` columns in DB schema and `_nb` fields in JSON
- Leave them null/empty in this phase
- Translate in a future pass

### Q5: API Design
**Decision:** Extend existing endpoint
- Add guidance as optional nested object in `GET /api/concepts/:id` response
- No new endpoints needed
- Simpler integration, fewer API calls from frontend

### Q6: Frontend
**Decision:** Include frontend display
- Add guidance sections (about, actions, questions, references) to concept detail view
- Users see enriched data immediately after backend work

## Existing System Context

### Current Data Model
- `concepts` table: id, framework_id, parent_id, concept_type, code, name_en/nb, definition_en/nb, source_reference, sort_order
- `concept_properties` table: exists but completely unused (key-value pairs per concept)
- FTS5 virtual table indexes: name_en, name_nb, definition_en, definition_nb
- Import pipeline: `backend/src/import.rs` reads JSON, upserts frameworks + concepts

### Current Concept Structure (JSON)
```json
{
  "id": "nist-ai-ms-1-1",
  "framework_id": "nist-ai-rmf",
  "parent_id": "nist-ai-ms-1",
  "concept_type": "action",
  "code": "MEASURE 1.1",
  "name_en": "Select Measurement Approaches",
  "definition_en": "Approaches for measurement are selected...",
  "source_reference": "NIST AI 100-1 / AI 600-1 Playbook",
  "sort_order": 1
}
```

### Playbook Data Per Concept (example: MEASURE 1.1, page 98)
- **About**: 1-2 paragraphs of context
- **Suggested Actions**: 12 specific action items
- **Transparency & Documentation**: 6 questions
- **AI Transparency Resources**: 3 linked resources
- **References**: 5 academic citations

### Analysis Matcher
- Uses FTS5 + TF-IDF scoring against concept definitions
- Richer text (about, suggested actions) would improve matching accuracy
- Gap classification could reference specific suggested actions

### Key Constraints
- All new fields must be optional (backward compatible)
- 24 ontology JSON files must continue importing without error
- Additive-only database migration
- The `concept_properties` table exists but using dedicated tables is preferred for queryability
