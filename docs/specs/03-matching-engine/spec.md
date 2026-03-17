# 03-matching-engine: Framework Detection & Concept Scoring

## Summary

The analytical core of the Document Analysis Engine. Implements the `MatchingEngine` trait with a `DeterministicMatcher` that uses a two-stage pipeline: FTS5 candidate retrieval followed by keyword overlap scoring (TF-IDF-like). Automatically detects relevant frameworks, classifies findings, validates references, and generates prioritized recommendations.

## Requirements Source

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md` (Analysis Pipeline steps 3-7)
- Interview: `docs/specs/deep_project_interview.md`

## What to Build

### DeterministicMatcher (`backend/src/features/analysis/matcher.rs`)

Implements `MatchingEngine` trait from 01-db-models. The analysis pipeline:

#### Stage 1: Framework Detection

Determine which of the 10+ loaded frameworks are relevant to the document:

1. Extract keywords from document text (using tokenizer from 02)
2. Match keywords against **topic tags** (`ontology-data/topic-tags.json` — 10 cross-cutting topics like "Identity & Access", "Data Protection", etc.)
3. From matched topics, identify which frameworks have concepts tagged with those topics
4. Also match against framework names and descriptions directly (e.g., document mentions "NIST" → include NIST CSF)
5. Return list of relevant framework IDs, ordered by match strength

#### Stage 2: Concept Matching (FTS5 Retrieval)

For each relevant framework, find candidate concepts:

1. Query `concepts_fts` full-text search table with document keywords
2. Filter results to the relevant framework(s)
3. Also query `concepts` table for exact keyword matches in `name_en`, `definition_en`, `code` fields
4. Union candidates, deduplicate

#### Stage 3: Concept Scoring (TF-IDF-like)

Score each candidate concept against the document:

1. Compute **term frequency** of concept keywords in document text
2. Compute **inverse document frequency** across all concept definitions (rarer terms score higher)
3. Calculate **overlap score** between concept definition keywords and document text keywords
4. Combine into a confidence_score (0.0 - 1.0)

#### Stage 4: Gap Classification

Classify each concept based on confidence score:

| Score Range | Classification | Meaning |
|-------------|---------------|---------|
| >= 0.6 | `addressed` | Document clearly covers this concept |
| 0.3 - 0.59 | `partially_addressed` | Document touches on this but incompletely |
| 0.0 - 0.29 | `gap` | Framework concept not addressed in document |
| (excluded) | `not_applicable` | Reserved for user override / LLM Phase 2 |

For gap findings: include concepts that are in relevant frameworks but have NO match in the document — these are the most actionable gaps.

#### Stage 5: Reference Validation

For every finding, verify that `concept_id` exists in the `concepts` table. Drop any finding with an invalid reference. Log warnings for dropped findings.

#### Stage 6: Priority Ranking

Assign priority (1-4) based on:
- **Priority 1 (Critical):** Gap in a root-level concept (no parent_id) of a highly-matched framework
- **Priority 2 (High):** Gap in a second-level concept, or partial coverage of a root concept
- **Priority 3 (Medium):** Gap in a deeper concept, or partial coverage of second-level
- **Priority 4 (Low):** Partial coverage of deeper concepts

#### Stage 7: Recommendation Generation

For each finding, generate a recommendation string:
- **Addressed:** "Document adequately covers [concept name]. Reference: [source_reference]"
- **Partially addressed:** "Document partially addresses [concept name]. Consider expanding coverage of [definition excerpt]. Reference: [source_reference]"
- **Gap:** "Document does not address [concept name]: [definition excerpt]. Recommended action: review and implement controls per [source_reference]"

### Prompt Template System

The JSON-based prompt template configures matching behavior:

```json
{
  "version": 1,
  "description": "Default deterministic matching configuration",
  "settings": {
    "min_confidence_threshold": 0.1,
    "addressed_threshold": 0.6,
    "partial_threshold": 0.3,
    "max_findings_per_framework": 50,
    "include_addressed_findings": true,
    "stopwords": ["the", "and", "or", "is", "in", "to", "of", ...],
    "boost_terms": {
      "security": 1.5,
      "risk": 1.5,
      "compliance": 1.3,
      "control": 1.2
    }
  }
}
```

This template is stored with each analysis and can be edited by the user before running. When LLM integration arrives in Phase 2, this JSON evolves into a natural language prompt while preserving the same settings structure.

## Key Decisions

- **Two-stage pipeline** — FTS5 for fast candidate retrieval (leverages existing infrastructure), then keyword scoring for ranking. User explicitly chose this approach.
- **Trait-based design** — `DeterministicMatcher` implements `MatchingEngine`. Phase 2 `LlmMatcher` swaps in without changing the pipeline orchestration in 04-backend-api-export.
- **Gap detection via absence** — Concepts in relevant frameworks that have zero overlap with the document are the most valuable gaps to surface.
- **English only** — Match against `name_en`, `definition_en` fields. Norwegian matching deferred.
- **Configurable thresholds** — Via prompt template JSON, so users can tune sensitivity without code changes.

## Dependencies

- **Needs from 01-db-models:** `MatchingEngine` trait, `MatchingResult`, `NewFinding`, `FindingType`
- **Needs from 02-document-parsing:** `ParsedDocument` struct (text, keywords, token data)
- **Reads from DB:** `concepts`, `concepts_fts`, `frameworks`, `relationships`, `concept_properties` tables
- **Reads from disk:** `ontology-data/topic-tags.json` (or query topics endpoint)
- **Provides to 04-backend-api-export:** `DeterministicMatcher` implementing `MatchingEngine`

## Existing Infrastructure to Leverage

- **FTS5 table:** `concepts_fts` virtual table already exists with triggers for auto-sync (see migration 001)
- **Topic tags:** Already loaded via `/api/ontology/topics` endpoint — maps topics to concept_ids
- **Relationships:** Cross-framework `maps_to`/`implements` relationships can boost related concepts
- **Concept hierarchy:** `parent_id` field enables priority ranking by concept depth

## Testing Strategy

- Unit test scoring algorithm with known input/output pairs
- Test framework detection with a sample security policy document
- Test gap detection: verify concepts in relevant frameworks with no document match appear as gaps
- Test reference validation: inject fake concept_id, verify it's filtered
- Test threshold sensitivity: same document with different thresholds produces different classifications
