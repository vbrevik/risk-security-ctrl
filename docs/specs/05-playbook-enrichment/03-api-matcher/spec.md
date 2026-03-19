# 03-api-matcher: Concept Detail API & Matcher Enhancement

## Summary

Extend the `GET /api/concepts/:id` endpoint to return guidance data (about, actions, questions, references) as a nested object. Enhance the analysis matcher to leverage the richer text for improved TF-IDF scoring and more specific gap recommendations.

## Requirements Source

- Parent requirements: `docs/specs/05-playbook-enrichment/requirements.md`
- Interview: `docs/specs/05-playbook-enrichment/deep_project_interview.md`
- Manifest: `docs/specs/05-playbook-enrichment/project-manifest.md`

## What to Build

### API: Extend Concept Detail Response

#### Response Model

Extend the existing `GET /api/concepts/:id` response with an optional `guidance` field:

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
    "suggested_actions": [
      { "sort_order": 1, "text_en": "Establish approaches for detecting..." },
      { "sort_order": 2, "text_en": "Identify testing procedures..." }
    ],
    "transparency_questions": [
      { "sort_order": 1, "text_en": "How will the appropriate performance metrics..." }
    ],
    "references": [
      { "type": "academic", "title": "Designing AI Review Boards", "authors": "Sara R. Jordan", "year": 2019, "url": null },
      { "type": "transparency_resource", "title": "GAO-21-519SP", "url": null }
    ]
  },
  "related_concepts": [...]
}
```

- `guidance` is `null` / absent when concept has no guidance data (most frameworks)
- The `ConceptWithRelationships` struct needs extending or a new `ConceptDetail` struct

#### Handler Changes

- Modify `get_concept` handler in `backend/src/features/ontology/routes.rs`
- After fetching concept + relationships, also query:
  - `concept_guidance` WHERE concept_id = ?
  - `concept_actions` WHERE concept_id = ? ORDER BY sort_order
  - `concept_transparency_questions` WHERE concept_id = ? ORDER BY sort_order
  - `concept_references` WHERE concept_id = ? ORDER BY sort_order
- Assemble into nested guidance object
- Keep list endpoints (`GET /api/concepts`) unchanged — no guidance in list responses

#### OpenAPI / utoipa

- Add `ConceptGuidanceResponse`, `ActionResponse`, `QuestionResponse`, `ReferenceResponse` structs with `#[derive(ToSchema)]`
- Update the path annotation for `get_concept` to document the new response shape

### Matcher Enhancement

#### Richer Text for Scoring

The analysis matcher (`backend/src/features/analysis/matcher.rs`) currently scores concepts using `definition_en` only. With guidance data available:

1. **Extend FTS5 queries**: If `concept_guidance_fts` exists (from split 02), also search it for candidates
2. **Include `about_en` in TF-IDF scoring**: When computing keyword overlap, concatenate `definition_en` + `about_en` for richer term frequency
3. **Include suggested actions text**: The action items contain domain-specific vocabulary that improves matching

#### Better Recommendations

When generating findings/recommendations in gap classification:
- If the concept has suggested actions, reference specific actions in the recommendation text
- e.g., "Consider: Establish approaches for detecting, tracking and measuring known risks (MEASURE 1.1, Action 1)"
- This gives users actionable next steps, not just "you have a gap in MEASURE 1.1"

#### ConceptCandidate Extension

The `ConceptCandidate` struct may need an optional `about_en` field, or the scoring function needs to query guidance data separately. Consider which approach is cleaner — the struct extension is simpler but means all candidates carry more data.

## Key Decisions

- **Extend existing endpoint**: Guidance is nested in `GET /api/concepts/:id`, not a separate endpoint (interview decision)
- **List endpoints stay lean**: No guidance data in paginated concept lists
- **Matcher uses guidance text**: Both for candidate retrieval (FTS) and scoring (TF-IDF)
- **Actionable recommendations**: Findings reference specific suggested actions

## Dependencies

- **Needs from 02**: Database tables populated with guidance data, FTS5 extension
- **Provides to 04**: API response shape for frontend to consume

## Existing Code Reference

- Concept model: `backend/src/features/ontology/models.rs` (Concept, ConceptWithRelationships)
- Concept routes: `backend/src/features/ontology/routes.rs` (get_concept handler)
- Matcher: `backend/src/features/analysis/matcher.rs` (ConceptCandidate, ScoredCandidate, retrieve_fts5_candidates, compute_tfidf_score)
