# Combined Spec: 01-db-models

## What We're Building

Foundation layer for the Document Analysis Engine: a new `analysis` feature module with database migration, Rust model types, and the `MatchingEngine` trait interface.

## Source Documents

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md`
- Split spec: `docs/specs/01-db-models/spec.md`
- Research: `docs/specs/01-db-models/claude-research.md`
- Interview: `docs/specs/01-db-models/claude-interview.md`

## Consolidated Requirements

### Database Migration (003_analysis_schema.sql)

Two tables:

**analyses** — Each analysis run.
- TEXT PK (UUID), name, description, input_type (CHECK: text/pdf/docx), input_text, original_filename, file_path, extracted_text, status (CHECK: pending/processing/completed/failed/deleted), error_message, prompt_template, matched_framework_ids (TEXT, JSON array, deserialized in Rust), processing_time_ms, token_count, created_by, created_at, updated_at
- Indexes: status, created_by, created_at
- Style: `CREATE TABLE IF NOT EXISTS`, `TEXT DEFAULT (datetime('now'))`

**analysis_findings** — Findings linking analysis → concept.
- TEXT PK (UUID), analysis_id (FK → analyses ON DELETE CASCADE), concept_id (FK → concepts), framework_id (FK → frameworks), finding_type (CHECK: addressed/partially_addressed/gap/not_applicable), confidence_score (REAL, CHECK 0.0-1.0), evidence_text, recommendation, priority (INTEGER CHECK 1-4), sort_order, created_at
- Indexes: analysis_id, framework_id, finding_type, priority
- Composite index: (analysis_id, finding_type, priority) for common queries

### PRAGMA foreign_keys

Enable `PRAGMA foreign_keys = ON` in SQLite pool `after_connect` callback. This affects all tables globally — safe since existing tables already define FK constraints.

### Rust Models (features/analysis/models.rs)

**Enums** (with `#[serde(rename_all = "snake_case")]`, manual `From<String>` / `Into<String>`):
- `InputType`: Text, Pdf, Docx
- `AnalysisStatus`: Pending, Processing, Completed, Failed, Deleted
- `FindingType`: Addressed, PartiallyAddressed, Gap, NotApplicable

**Structs** (with `Serialize, Deserialize, FromRow, ToSchema` derives as appropriate):
- `Analysis` — Full entity (FromRow, with matched_framework_ids as `Vec<String>` deserialized from JSON TEXT)
- `CreateAnalysisRequest` — API input (name, description, input_text, prompt_template)
- `AnalysisFinding` — Single finding row (FromRow)
- `AnalysisFindingWithConcept` — Finding + concept metadata (joined query result)
- `AnalysisSummary` — Analysis + aggregated stats (computed on-the-fly with SQL)
- `FrameworkFindingSummary` — Per-framework breakdown

### MatchingEngine Trait (features/analysis/engine.rs)

Using `async-trait` crate for dyn dispatch support:

```rust
#[async_trait]
pub trait MatchingEngine: Send + Sync {
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError>;
}
```

Supporting types: `MatchingResult`, `NewFinding`, `AnalysisError`.

### Module Wiring

- `backend/src/features/analysis/mod.rs` — `pub mod models; pub mod engine;`
- `backend/src/features/mod.rs` — add `pub mod analysis;`
- `backend/src/lib.rs` — no router changes yet (routes come in split 04)
- `Cargo.toml` — add `async-trait` dependency

### Key Decisions

1. **async-trait crate** over enum dispatch — user chose extensibility over simplicity
2. **PRAGMA foreign_keys ON globally** — enables CASCADE, safe for existing schema
3. **Rust-side JSON deserialization** — matched_framework_ids stored as TEXT, deserialized with serde
4. **On-the-fly aggregation** — no summary table, use SQL COUNT/GROUP BY
5. **Follow existing patterns** — derive order, enum conversion, Row suffix for FromRow structs, timestamp format
