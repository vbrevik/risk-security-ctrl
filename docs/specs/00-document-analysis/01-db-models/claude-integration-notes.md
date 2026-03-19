# Integration Notes: Opus Review Feedback

## Integrating

### Item 6: AnalysisFindingWithConcept i18n (MUST FIX)
**Integrating.** The review correctly identifies that `concepts` has `name_en`/`name_nb`/`definition_en`/`definition_nb`. Using single `concept_name` will break FromRow. Will update the struct to include `concept_name_en`, `concept_name_nb`, `concept_definition_en`, following the compliance pattern.

### Item 3: framework_id consistency
**Integrating as documentation.** Adding a note that the matching engine (split 03) must ensure `framework_id` matches `concepts.framework_id` for the given `concept_id`. Not adding a DB trigger — validation belongs in application logic.

### Item 2: foreign_key_check before enabling
**Integrating.** Adding a `PRAGMA foreign_key_check` step in the migration or startup to log violations before enforcement. Simple safety net.

### Item 9: AnalysisListQuery.status type
**Integrating.** Changing to `Option<AnalysisStatus>` for type safety, matching compliance pattern.

### Item 11: IF NOT EXISTS convention
**Integrating.** Already intended, making it explicit.

### Item 13: PaginatedResponse reuse
**Integrating.** Will document that the analysis feature reuses `PaginatedResponse<T>` from ontology models (it's already generic and accessible).

### Item 12: token_count semantics
**Integrating.** Clarifying: for MVP it's document word/token estimate. Phase 2 will track LLM API tokens separately.

## NOT Integrating

### Item 4: updated_at trigger
**Not integrating.** No existing table has an update trigger — the codebase consistently sets `updated_at` in application code. Adding a trigger would be inconsistent. Will follow existing pattern.

### Item 5: async-trait alternatives
**Not integrating.** User explicitly chose async-trait for dyn dispatch. The compile-time cost of one proc macro is negligible.

### Item 7: Input validation
**Not integrating in this split.** Validation belongs in split 04 (routes). Will add a brief note that constraints should be documented there.

### Item 8: file_path security
**Not integrating in this split.** File handling is split 02's responsibility. The schema just stores the path.

### Item 10: findings updated_at
**Not integrating.** Findings are immutable in the MVP — generated once by the matching engine, never modified. If we need mutability later, a migration can add it.

### Item 1: Migration numbering
**Not integrating.** No other branch is claiming 003. Current work is on master.
