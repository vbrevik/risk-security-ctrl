# Research: 03-matching-engine

## FTS5 Infrastructure
- `concepts_fts` FTS5 table indexes: name_en, name_nb, definition_en, definition_nb
- Auto-synced via INSERT/UPDATE/DELETE triggers on `concepts` table
- **No existing FTS5 MATCH queries in codebase** — current search uses LIKE. We'll be first to use FTS5.

## Topic Tags
- Loaded from `ontology-data/topic-tags.json` per request (no DB cache)
- Structure: `{ topics: [{ id, name_en, description_en, concept_ids: [...] }] }`
- ~10 topics mapping concepts across frameworks

## Concept Scale
- ~1000 unique concepts across 24 frameworks
- Largest: NIST SP 800-53 (345 concepts), ISO 27000 (107), NIST AI RMF (130)
- Hierarchy via `parent_id` (root → children → grandchildren)
- ~442 cross-framework relationships

## Tokenizer (from split 02)
- `extract_keywords(text)` → deduplicated lowercase keywords, stopwords removed
- `term_frequency(text)` → HashMap<String, usize>
- `generate_ngrams(words, n)` → n-gram strings
- `sentence_split(text)` → sentence boundaries

## MatchingEngine Trait (from split 01)
- `async fn analyze(&self, text, prompt_template, db) -> Result<MatchingResult, AnalysisError>`
- Returns `MatchingResult { matched_framework_ids, findings: Vec<NewFinding>, processing_time_ms, token_count }`
- `NewFinding { concept_id, framework_id, finding_type, confidence_score, evidence_text, recommendation, priority }`

## Key Implementation Notes
- FTS5 MATCH syntax: `SELECT * FROM concepts_fts WHERE concepts_fts MATCH ?`
- Topics loaded from disk, not DB — need to read JSON file in matcher
- `parent_id IS NULL` identifies root concepts (for priority ranking)
- Concept types: principle, framework_component, process, technique, function, category, subcategory
