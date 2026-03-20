# Plan Contract: 03-api-matcher

## GOAL
Deliver a self-contained prose blueprint for extending the concept detail API with nested guidance data and enhancing the analysis matcher with guidance-enriched TF-IDF scoring.

## CONTEXT
This plan drives all downstream section files and implementation via deep-implement. It must be complete enough for an unfamiliar engineer or LLM to implement without guessing. The API response shape defined here is consumed by split 04 (frontend).

## CONSTRAINTS
- Plans are prose documents, zero full function implementations
- Follow existing codebase patterns: Axum + SQLx + utoipa, feature-based architecture
- Extend `get_concept_relationships` handler (not plain `get_concept`)
- Union both FTS tables for candidate retrieval
- Custom BM25 weights: name=10, definition=3, about=5
- Add about_en + actions_text to ConceptCandidate struct
- Include all suggested actions in recommendations
- Use `skip_serializing_if = "Option::is_none"` for guidance field
- All SQL uses parameterized binds (STIG V-222607)
- Validate/sanitize FTS5 input keywords (STIG V-222602)

## FORMAT
Single file `claude-plan.md` with sections that map to implementable units.

## STIG Constraints
- V-222607 (CAT I): All SQL must use parameterized binds — no string concatenation
- V-222602 (CAT I): FTS5 keywords must be sanitized before MATCH expressions
- V-222606 (CAT I): Validate all input — concept IDs, search terms, path parameters
- V-222609 (CAT I): Handle malformed input gracefully without crashing
- V-222610 (CAT II): Error messages must not reveal SQL errors, internal paths, or stack traces
- V-222585 (CAT I): Fail to secure state — error responses must not expose data

## FAILURE CONDITIONS
- SHALL NOT contain full function bodies
- SHALL NOT assume reader has prior context
- SHALL NOT omit testing strategy
- SHALL NOT add features beyond the spec
