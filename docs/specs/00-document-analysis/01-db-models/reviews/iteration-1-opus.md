# Opus Review

**Model:** claude-opus-4
**Generated:** 2026-03-17T09:30:00Z

---

## Review: 01-db-models Implementation Plan

### 1. Migration Numbering Gap
The plan specifies `003_analysis_schema.sql`. Confirm no other branch claims `003`. SQLx migrations are ordered lexicographically — collision causes startup failures.

### 2. PRAGMA foreign_keys -- Retroactive Risk
Enabling globally is necessary, but existing data may violate FK constraints (orphaned rows). **Recommendation:** Add `PRAGMA foreign_key_check` before enabling enforcement.

### 3. framework_id Consistency
Finding's `framework_id` could mismatch concept's actual `framework_id`. **Recommendation:** Document that matching engine must enforce consistency, or add validation.

### 4. Soft Delete updated_at Gap
`DEFAULT (datetime('now'))` only fires on INSERT. UPDATEs leave `updated_at` stale. **Recommendation:** Add trigger or document that UPDATE queries must set it manually.

### 5. async-trait May Be Unnecessary
Native async fn in traits (Rust 1.75+) works for generics. Only needed for dyn dispatch. Plan correctly identifies dyn dispatch motivation. Not a blocker.

### 6. AnalysisFindingWithConcept i18n Mismatch (MUST FIX)
`concepts` table uses `name_en`, `name_nb`, `definition_en`, `definition_nb`. Plan has single `concept_name` and `concept_definition`. Will cause FromRow failures. **Follow compliance pattern with `_en` and `_nb` variants.**

### 7. No Input Validation on CreateAnalysisRequest
No length constraints on name or input_text. **Document expected constraints even if validation happens in split 04.**

### 8. file_path Security Concern
Storing filesystem paths — document that it should be relative, never returned to client without sanitization.

### 9. AnalysisListQuery.status Type
Uses `Option<String>` but compliance uses `Option<AssessmentStatus>`. **Use `Option<AnalysisStatus>` for type safety.**

### 10. No updated_at on analysis_findings
If findings are immutable, fine. If not, add `updated_at` to avoid migration later.

### 11. IF NOT EXISTS Convention
Follow existing convention of using `CREATE TABLE IF NOT EXISTS`.

### 12. token_count Semantics
Ambiguous — document token count vs LLM API tokens. Clarify purpose.

### 13. PaginatedResponse Reuse
Should reuse existing `PaginatedResponse<T>` from ontology models or extract to shared location.

### Summary of Priorities

**Must fix:** Items 6, 3
**Should fix:** Items 2, 7, 8, 9
**Nice to have:** Items 4, 5, 10, 12, 13
