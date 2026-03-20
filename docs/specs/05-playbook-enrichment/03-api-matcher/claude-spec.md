# Specification: Concept Detail API & Matcher Enhancement

## Overview

Extend the existing `GET /api/ontology/concepts/:id/relationships` endpoint to include guidance data as a nested optional object. Enhance the analysis matcher to use guidance text for improved TF-IDF scoring and more specific gap recommendations.

## API: Extend Concept Detail Response

### Target Endpoint

Modify `get_concept_relationships` in `backend/src/features/ontology/routes.rs` (NOT the plain `get_concept` handler). This endpoint already returns `ConceptWithRelationships`; guidance will be added alongside `related_concepts`.

### Response Shape

```json
{
  "id": "nist-ai-ms-1-1",
  "framework_id": "nist-ai-rmf",
  "code": "MEASURE 1.1",
  "name_en": "Select Measurement Approaches",
  "definition_en": "...",
  "guidance": {
    "source_pdf": "AI_RMF_Playbook.pdf",
    "source_page": 98,
    "about_en": "The development and utility of trustworthy AI...",
    "about_nb": null,
    "suggested_actions": [
      { "sort_order": 1, "text_en": "Establish approaches for detecting...", "text_nb": null }
    ],
    "transparency_questions": [
      { "sort_order": 1, "text_en": "How will the appropriate performance metrics...", "text_nb": null }
    ],
    "references": [
      { "type": "academic", "title": "Designing AI Review Boards", "authors": "Sara R. Jordan", "year": 2019, "url": null },
      { "type": "transparency_resource", "title": "GAO-21-519SP", "url": null }
    ]
  },
  "related_concepts": [...]
}
```

### Response Rules

- `guidance` is **omitted** (not `null`) when concept has no guidance data — use `#[serde(skip_serializing_if = "Option::is_none")]`
- Most frameworks won't have guidance; only NIST AI RMF concepts have it initially
- List endpoints (`GET /api/concepts`) stay unchanged — no guidance in paginated responses
- Norwegian `_nb` fields included but may be `null`

### New Response Types

All derive `Serialize, ToSchema`:

- **`ConceptGuidanceResponse`**: source_pdf, source_page, about_en, about_nb, suggested_actions, transparency_questions, references
- **`ActionResponse`**: sort_order, text_en, text_nb
- **`QuestionResponse`**: sort_order, text_en, text_nb
- **`ReferenceResponse`**: reference_type (renamed from `type` for Rust keyword), title, authors, year, venue, url

### Handler Changes

In `get_concept_relationships`:
1. After fetching concept + relationships (existing logic), query guidance tables:
   - `concept_guidance WHERE concept_id = ?`
   - `concept_actions WHERE concept_id = ? ORDER BY sort_order`
   - `concept_transparency_questions WHERE concept_id = ? ORDER BY sort_order`
   - `concept_references WHERE concept_id = ? ORDER BY sort_order`
2. If concept_guidance row exists, assemble `ConceptGuidanceResponse`; otherwise `None`
3. Return extended response

### OpenAPI Updates

- Register new schema types in the OpenAPI builder (`main.rs`)
- Update `get_concept_relationships` path annotation to document the new response shape
- Update `ConceptWithRelationships` response body reference

## Matcher Enhancement

### FTS5 Candidate Retrieval

**Union both FTS tables** for candidate retrieval:
1. Query existing `concepts_fts` (current behavior, unchanged)
2. Query `concept_guidance_fts` with custom column weights: `bm25(concept_guidance_fts, 10.0, 3.0, 5.0)` (name=10, definition=3, about=5)
3. Merge results, deduplicate by concept_id (keep best score)

### ConceptCandidate Extension

Add optional guidance fields to `ConceptCandidate`:
- `about_en: Option<String>` — from concept_guidance
- `actions_text: Option<String>` — concatenated action texts for TF-IDF scoring

These are populated via LEFT JOIN to concept_guidance and a subquery concatenating actions. Concepts without guidance data have `None` for these fields.

### TF-IDF Scoring Enhancement

In `score_candidates()`:
- Current: extracts keywords from `name_en + definition_en`
- Enhanced: extracts keywords from `name_en + definition_en + about_en + actions_text`
- All four text fields contribute to term frequency, giving richer vocabulary for matching

### Actionable Recommendations

When generating findings/recommendations in gap classification:
- If the matched concept has suggested actions, include **all actions** in the recommendation text
- Format: "Consider: [Action 1 text] (MEASURE 1.1, Action 1)"
- This gives users specific next steps rather than just "you have a gap in MEASURE 1.1"

## Dependencies

- **Requires from 02-schema-import**: All four guidance tables populated, FTS5 table, import pipeline
- **Provides to 04-frontend-guidance**: The API response shape defined above

## Existing Code to Modify

| File | Change |
|------|--------|
| `backend/src/features/ontology/models.rs` | Add guidance response types |
| `backend/src/features/ontology/routes.rs` | Extend `get_concept_relationships` handler |
| `backend/src/features/analysis/matcher.rs` | Add guidance FTS retrieval + scoring enrichment |
| `backend/src/main.rs` | Register new OpenAPI schema types |
