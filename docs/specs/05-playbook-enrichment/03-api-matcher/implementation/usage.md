# API Matcher Guidance Enrichment — Usage Guide

## What Was Built

This implementation extends the concept detail API and analysis matcher with NIST AI RMF Playbook guidance data:

### API Changes

**GET /api/ontology/concepts/{id}/relationships** now returns an optional `guidance` field for concepts with playbook data:

```json
{
  "id": "nist-ai-gv-1-1",
  "name_en": "Legal and Regulatory Requirements",
  "related_concepts": [...],
  "guidance": {
    "source_pdf": "nist-ai-rmf-playbook.pdf",
    "source_page": 42,
    "about_en": "About this concept...",
    "about_nb": null,
    "suggested_actions": [
      { "sort_order": 1, "text_en": "Map applicable regulations", "text_nb": null }
    ],
    "transparency_questions": [
      { "sort_order": 1, "text_en": "How is compliance tracked?", "text_nb": null }
    ],
    "references": [
      { "type": "academic", "title": "AI Governance Paper", "authors": "Smith", "year": 2024, "venue": "ICML", "url": null }
    ]
  }
}
```

- `guidance` is omitted entirely (not null) when no guidance data exists
- Non-NIST AI RMF concepts see no change in API response
- OpenAPI schemas registered: `ConceptGuidanceResponse`, `ActionResponse`, `QuestionResponse`, `ReferenceResponse`

### Matcher Changes

**Candidate Enrichment:** FTS queries now LEFT JOIN guidance tables, populating `about_en` and `actions_text` on matched candidates. A second FTS query against `concept_guidance_fts` broadens recall using BM25 weights (name=10, definition=3, about=5).

**TF-IDF Scoring:** Keyword extraction now includes `about_en` and `actions_text`, giving guidance-enriched concepts richer vocabulary profiles for better matching.

**Recommendations:** Findings for concepts with guidance data include a "Suggested Actions:" section with formatted action items:

```
Document does not address Governance: Establish governance...
Recommended action: review and implement appropriate controls.

Suggested Actions:
- Establish governance board (GV-1.1, Action 1)
- Define risk appetite (GV-1.1, Action 2)
```

## How to Populate Guidance Data

Create a `*-guidance.json` file in `ontology-data/` following the schema from `backend/src/import.rs`:

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "nist-ai-rmf-playbook.pdf",
  "guidance": [
    {
      "concept_id": "nist-ai-gv-1-1",
      "source_page": 42,
      "about_en": "About text...",
      "suggested_actions": [
        { "text_en": "Action text", "text_nb": null }
      ],
      "transparency_questions": [
        { "text_en": "Question text", "text_nb": null }
      ],
      "references": [
        { "type": "academic", "title": "Paper", "authors": "Smith", "year": 2024 }
      ]
    }
  ]
}
```

Run `cargo run` — `import_all_ontologies` auto-discovers `*-guidance.json` files.

## Files Modified

| File | Changes |
|------|---------|
| `backend/src/features/ontology/models.rs` | 4 new response structs, guidance field on ConceptWithRelationships |
| `backend/src/features/ontology/routes.rs` | 4 concurrent guidance queries via tokio::try_join! |
| `backend/src/main.rs` | 4 OpenAPI schema registrations |
| `backend/src/features/analysis/matcher.rs` | ConceptCandidate enrichment, GapConceptRow, guidance FTS, TF-IDF expansion, actions in recommendations |
| `backend/tests/api_tests.rs` | 6 guidance API tests |
| `backend/tests/integration_guidance_tests.rs` | 8 integration tests (new file) |
