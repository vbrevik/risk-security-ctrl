# 01-db-models: Database Schema & Core Types

## Summary

Foundation layer for the Document Analysis Engine. Creates the database migration, Rust model structs/enums, and the `MatchingEngine` trait that defines the interface for pluggable matching implementations (deterministic MVP, LLM Phase 2).

## Requirements Source

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md` (Data Model section)
- Interview: `docs/specs/deep_project_interview.md`

## What to Build

### Database Migration (`backend/migrations/003_analysis_schema.sql`)

Two new tables:

**`analyses`** — Stores each analysis run with its input, status, and metadata.
- Fields: id (TEXT PK), name, description, input_type (CHECK: text/pdf/docx), input_text, original_filename, file_path, extracted_text, status (CHECK: pending/processing/completed/failed/deleted), error_message, prompt_template, matched_framework_ids (JSON array), processing_time_ms, token_count, created_by, created_at, updated_at
- Indexes on: status, created_by, created_at

**`analysis_findings`** — Individual findings linking an analysis to ontology concepts.
- Fields: id (TEXT PK), analysis_id (FK → analyses ON DELETE CASCADE), concept_id (FK → concepts), framework_id (FK → frameworks), finding_type (CHECK: addressed/partially_addressed/gap/not_applicable), confidence_score (REAL 0.0-1.0), evidence_text, recommendation, priority (INTEGER 1-4), sort_order, created_at
- Indexes on: analysis_id, framework_id, finding_type, priority

See the spec's Data Model section for complete SQL.

### Rust Models (`backend/src/features/analysis/models.rs`)

Enums:
- `InputType` — Text, Pdf, Docx (with serde serialization)
- `AnalysisStatus` — Pending, Processing, Completed, Failed, Deleted
- `FindingType` — Addressed, PartiallyAddressed, Gap, NotApplicable

Structs:
- `Analysis` — Full analysis entity with all fields
- `CreateAnalysisRequest` — For text-based creation (name, description, input_text, prompt_template)
- `AnalysisFinding` — Single finding row
- `AnalysisFindingWithConcept` — Finding enriched with concept metadata (code, name, definition, source_reference)
- `AnalysisSummary` — Analysis + aggregated finding counts + framework breakdown
- `FrameworkFindingSummary` — Per-framework stats (total, gap_count, addressed_count, etc.)

### MatchingEngine Trait (`backend/src/features/analysis/engine.rs`)

```rust
#[async_trait]
pub trait MatchingEngine: Send + Sync {
    /// Analyze extracted text against the ontology and produce findings
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError>;
}

pub struct MatchingResult {
    pub matched_framework_ids: Vec<String>,
    pub findings: Vec<NewFinding>,
    pub processing_time_ms: i64,
    pub token_count: i64,
}

pub struct NewFinding {
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
}
```

This trait is the seam point for LLM integration in Phase 2. The `DeterministicMatcher` (split 03) implements it.

### Module Registration (`backend/src/features/analysis/mod.rs`)

Wire the new feature module into the existing feature-based structure. Export models, engine trait.

## Key Decisions

- **MatchingEngine trait designed upfront** — User explicitly chose LLM-readiness from day one. The trait takes `db: &SqlitePool` because both deterministic and LLM matchers need ontology data access.
- **Soft delete** — `status = 'deleted'` rather than hard delete, preserving audit trail.
- **JSON for matched_framework_ids** — Stored as TEXT containing a JSON array. Deserialized in Rust models.
- **Priority 1-4 scale** — 1=critical, 2=high, 3=medium, 4=low.

## Dependencies

- **Needs:** Nothing (foundational split)
- **Provides to 02-document-parsing:** `InputType` enum
- **Provides to 03-matching-engine:** `MatchingEngine` trait, `FindingType`, `NewFinding`, `MatchingResult`
- **Provides to 04-backend-api-export:** All model structs for API serialization

## Existing Patterns to Follow

- Migration files: `backend/migrations/` (see 001, 002 for style — plain SQL, CREATE TABLE IF NOT EXISTS)
- Models: `backend/src/features/ontology/models.rs` (serde derives, sqlx::FromRow, utoipa::ToSchema)
- Feature modules: `backend/src/features/{name}/mod.rs` pattern
- IDs: UUID v4 strings via `uuid::Uuid::new_v4().to_string()`
- Timestamps: `datetime('now')` default in SQL, String type in Rust
