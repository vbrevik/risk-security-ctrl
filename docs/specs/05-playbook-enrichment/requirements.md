# Playbook Enrichment: Expand Ontology Data Model with PDF-Sourced Guidance

## Overview

Enrich the ontology data model with structured guidance data extracted from authoritative NIST reference PDFs. Currently, each concept in the ontology JSON files contains only a one-line definition. The NIST AI RMF Playbook PDF contains significantly richer content per concept: multi-paragraph "About" sections, 4-12 suggested actions, transparency/documentation questions, and academic references with citations.

This project adds that detail to the data model while maintaining backward compatibility with all existing frameworks that lack playbook-level detail.

## Reference PDFs (already downloaded)

All source PDFs are in `docs/reference-pdfs/`:

| File | Document | Concepts Covered |
|------|----------|-----------------|
| `AI_RMF_Playbook.pdf` (2.7 MB) | NIST AI RMF Playbook | 75 action-level concepts (GOVERN, MAP, MEASURE, MANAGE) |
| `NIST.AI.100-1.pdf` (1.9 MB) | NIST AI RMF 1.0 | Framework-level descriptions for AI RMF |
| `NIST.AI.600-1.pdf` (1.1 MB) | NIST AI 600-1 GenAI Profile | Generative AI-specific risks and actions |
| `NIST.SP.800-37r2.pdf` (2.2 MB) | NIST SP 800-37 Rev 2 (parent RMF) | RMF steps for information systems |

## Current State

- `ontology-data/nist-ai-rmf.json` has 75 action-type concepts with only `definition_en` (one sentence)
- The `concept_properties` table exists in the DB schema but is completely unused
- The `ConceptData` struct in `backend/src/import.rs` maps 1:1 to the JSON fields
- FTS5 indexes `name_en`, `name_nb`, `definition_en`, `definition_nb`
- The analysis matcher (`backend/src/features/analysis/matcher.rs`) uses FTS5 + TF-IDF scoring against definitions

## Requirements

### 1. Data Extraction from PDFs

Extract structured data from the Playbook PDF for each of the 75 action-level concepts:
- **About section**: Extended multi-paragraph explanation (1-3 paragraphs)
- **Suggested Actions**: Ordered list of 4-12 specific action items
- **Transparency & Documentation Questions**: 3-6 questions organizations should document
- **AI Transparency Resources**: Named links to external resources
- **Academic References**: Citations with title, authors, year, URL where available
- **PDF page reference**: The exact page number in the source PDF for traceability (e.g., "AI_RMF_Playbook.pdf p.93")

### 2. JSON Data Model Extension

Add optional fields to the concept JSON structure:
```json
{
  "id": "nist-ai-ms-1-1",
  "code": "MEASURE 1.1",
  "name_en": "Select Measurement Approaches",
  "definition_en": "...",
  "guidance": {
    "source_pdf": "AI_RMF_Playbook.pdf",
    "source_page": 93,
    "about_en": "The development and utility of trustworthy AI...",
    "about_nb": "...",
    "suggested_actions_en": ["Establish approaches for...", "..."],
    "suggested_actions_nb": ["..."],
    "transparency_questions_en": ["How will performance metrics...", "..."],
    "transparency_questions_nb": ["..."],
    "resources": [
      { "title": "GAO-21-519SP", "url": "...", "type": "transparency" }
    ],
    "references": [
      { "title": "Designing AI Review Boards", "authors": "Sara R. Jordan", "year": 2019, "url": "..." }
    ]
  }
}
```

All `guidance` fields must be optional (wrapped in `Option<>` / `#[serde(default)]`) to maintain backward compatibility.

### 3. Database Schema Extension

New tables (additive migration, no changes to existing tables):
- `concept_guidance` - Extended about text per concept with source PDF/page tracking
- `concept_actions` - Individual suggested actions with sort_order
- `concept_transparency_questions` - Documentation questions
- `concept_references` - Academic citations and resource links

### 4. Import Pipeline Update

Extend `backend/src/import.rs`:
- Add guidance fields to `ConceptData` struct (all optional with `#[serde(default)]`)
- Import guidance data into new tables during `import_ontology_file()`
- Upsert semantics for re-import safety

### 5. API Extension

- Extend `GET /api/concepts/:id` to include guidance data in detail responses
- Keep list endpoints lean (no guidance in list responses)
- Add OpenAPI schema for new response fields

### 6. Analysis Matcher Enhancement

- Include `about_en` text in FTS5 indexing for richer search
- Suggested actions text improves keyword matching for gap analysis
- Findings can reference specific suggested actions in recommendations

### 7. Extensibility to Other Frameworks

The pattern must work for:
- NIST AI 100-1 (framework descriptions)
- NIST SP 800-37 (RMF steps)
- NIST AI 600-1 (GenAI profile actions)
- Future frameworks with similar guidance structures

### 8. Source Traceability

Every piece of extracted data must link back to:
- The source PDF filename
- The page number in the PDF
- This enables audit trails and allows users to verify claims against the original document

## Constraints

- All new fields optional — existing frameworks without guidance data must continue working unchanged
- No breaking changes to existing API responses
- Additive-only database migration
- Norwegian translations (`_nb` fields) may be deferred to a later phase but the schema must support them
- The 24 ontology JSON files must all continue to import without error
