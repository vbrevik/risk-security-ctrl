# Implementation Plan: 01-db-models

## Overview

This plan covers the foundation layer for the Document Analysis Engine feature in the risk-security-ctrl project. We are adding a new `analysis` feature module to the existing Rust/Axum backend with:

1. A SQLite migration creating two new tables (`analyses` and `analysis_findings`)
2. Rust model structs and enums for the analysis domain
3. A `MatchingEngine` async trait that defines the interface for pluggable analysis implementations
4. A global `PRAGMA foreign_keys = ON` fix for the SQLite connection pool
5. Module wiring into the existing feature-based architecture

The project is a risk management framework explorer with a Rust+Axum+SQLx backend using SQLite. It follows a feature-module pattern (`backend/src/features/{name}/`) with each feature containing `mod.rs`, `models.rs`, and `routes.rs`. This split only creates models and the trait — routes come in a later split (04-backend-api-export).

## Why This Matters

The analysis feature lets users upload documents (PDF/DOCX) or type scenarios and have them evaluated against 10+ risk/security frameworks in the ontology. This foundation split establishes the data model and trait interface that all subsequent splits depend on:

- **02-document-parsing** needs `InputType` enum
- **03-matching-engine** needs `MatchingEngine` trait, `FindingType`, `NewFinding`, `MatchingResult`
- **04-backend-api-export** needs all model structs for API serialization
- **05-frontend-dashboard** consumes API responses shaped by these models

Getting the data model right here prevents cascading changes across all downstream splits.

---

## Section 1: Database Migration

### File: `backend/migrations/003_analysis_schema.sql`

Create two tables following the existing migration style (see `001_initial_schema.sql` for conventions).

#### Table: `analyses`

Stores each analysis run. Key design points:

- `id` is TEXT PRIMARY KEY (UUID v4 string, matching existing pattern)
- `input_type` uses a CHECK constraint limiting to `'text'`, `'pdf'`, `'docx'`
- `status` uses a CHECK constraint: `'pending'`, `'processing'`, `'completed'`, `'failed'`, `'deleted'` — soft delete via status, not row removal
- `matched_framework_ids` is TEXT storing a JSON array (e.g., `["nist-csf","iso31000"]`). Deserialized in Rust, not queried with SQL json functions
- `input_text` and `extracted_text` are nullable TEXT — `input_text` is set for text-type input, `extracted_text` is populated after parsing a file
- `prompt_template` stores the JSON matching configuration used for this analysis
- `processing_time_ms` tracks wall-clock analysis duration in milliseconds
- `token_count` is a document word/token estimate for MVP (word_count * 1.3 approximation). Phase 2 will add separate fields for LLM API input/output tokens
- `created_by` is nullable TEXT (auth is not implemented yet, Sprint 7)
- Timestamps use `TEXT DEFAULT (datetime('now'))` matching existing pattern

Fields (in order): id, name, description, input_type, input_text, original_filename, file_path, extracted_text, status, error_message, prompt_template, matched_framework_ids, processing_time_ms, token_count, created_by, created_at, updated_at.

Indexes: `idx_analyses_status`, `idx_analyses_created_by`, `idx_analyses_created_at`.

#### Table: `analysis_findings`

Links an analysis to ontology concepts with scoring. Key design points:

- `analysis_id` FK to `analyses(id)` with `ON DELETE CASCADE` — deleting an analysis removes all its findings
- `concept_id` FK to `concepts(id)` — must reference a real ontology concept (reference validation happens in the matching engine, but the FK provides DB-level integrity)
- `framework_id` FK to `frameworks(id)` — denormalized from the concept for efficient filtering
- `finding_type` CHECK: `'addressed'`, `'partially_addressed'`, `'gap'`, `'not_applicable'`
- `confidence_score` is REAL with CHECK constraint `BETWEEN 0.0 AND 1.0`
- `priority` is INTEGER with CHECK `BETWEEN 1 AND 4` (1=critical, 4=low)
- `evidence_text` stores the document excerpt that matched this concept (nullable)
- `recommendation` stores the generated action item text (nullable)

Fields (in order): id, analysis_id, concept_id, framework_id, finding_type, confidence_score, evidence_text, recommendation, priority, sort_order, created_at.

Indexes: `idx_analysis_findings_analysis` (analysis_id), `idx_analysis_findings_framework` (framework_id), `idx_analysis_findings_type` (finding_type), `idx_analysis_findings_priority` (priority). Also a composite index `idx_analysis_findings_analysis_type_priority` on (analysis_id, finding_type, priority) for the most common query pattern (list findings for an analysis, filtered by type, sorted by priority).

Use section separators (`-- ====`) between table definitions, matching migration 001 style.

---

## Section 2: PRAGMA foreign_keys

### File: `backend/src/main.rs`

The existing SQLite pool creation in `main.rs` does NOT enable foreign key enforcement. Without `PRAGMA foreign_keys = ON`, the CASCADE delete on `analysis_findings` will silently do nothing.

Modify the `SqlitePoolOptions` chain to add an `after_connect` callback that executes `PRAGMA foreign_keys = ON` on every new connection.

**Safety check:** Before enabling enforcement, run `PRAGMA foreign_key_check` at startup (after migrations, before serving requests) and log any violations. This catches orphaned rows from before enforcement was on. If violations exist, log them as warnings but still enable FK enforcement — new operations will be constrained even if legacy data has gaps.

This change affects all tables globally, which is safe — the existing `compliance_items`, `evidence`, and other tables already define FK constraints in their DDL but CASCADE was never enforced. Enabling it now makes the existing schema work as originally intended.

The pool creation currently looks like:

```rust
let db = SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await?;
```

Add `.after_connect(...)` between `.max_connections(5)` and `.connect(...)`.

---

## Section 3: Rust Enums

### File: `backend/src/features/analysis/models.rs`

Three enums following the existing pattern from `compliance/models.rs`:

#### InputType

Variants: `Text`, `Pdf`, `Docx`. Serializes to/from `"text"`, `"pdf"`, `"docx"`.

Derives: `Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq`. Use `#[serde(rename_all = "snake_case")]`.

Implement `From<String> for InputType` (with default fallback to `Text` for unknown values) and `From<InputType> for String`.

#### AnalysisStatus

Variants: `Pending`, `Processing`, `Completed`, `Failed`, `Deleted`. Serializes to/from snake_case strings.

Same derive pattern and From impls. Default fallback: `Pending`.

#### FindingType

Variants: `Addressed`, `PartiallyAddressed`, `Gap`, `NotApplicable`. Serializes to `"addressed"`, `"partially_addressed"`, `"gap"`, `"not_applicable"`.

Same derive pattern and From impls. Default fallback: `Gap`.

Each enum should have unit tests in a `#[cfg(test)] mod tests` block at the bottom of the file, testing round-trip String conversion for every variant.

---

## Section 4: Rust Structs — Core Models

### File: `backend/src/features/analysis/models.rs` (continued)

#### AnalysisRow

The raw database row struct. Derives `Debug, FromRow`. All fields match the `analyses` table columns exactly, with types:

- `id`, `name`: `String`
- `description`, `input_text`, `original_filename`, `file_path`, `extracted_text`, `error_message`, `prompt_template`, `matched_framework_ids`, `created_by`: `Option<String>`
- `input_type`, `status`: `String` (converted to enums in the `From` impl)
- `processing_time_ms`, `token_count`: `Option<i64>`
- `created_at`, `updated_at`: `String`

#### Analysis

The API response struct. Derives `Debug, Serialize, Deserialize, ToSchema`. Same fields as AnalysisRow but:

- `input_type` is `InputType` enum
- `status` is `AnalysisStatus` enum
- `matched_framework_ids` is `Vec<String>` (deserialized from the JSON TEXT column)

Implement `From<AnalysisRow> for Analysis`. The conversion parses `input_type` and `status` from strings to enums, and deserializes `matched_framework_ids` from JSON string to `Vec<String>` (falling back to empty vec on parse failure).

#### CreateAnalysisRequest

API input struct for text-based analysis creation. Derives `Debug, Deserialize, ToSchema`.

Fields: `name: String`, `description: Option<String>`, `input_text: String`, `prompt_template: Option<String>`.

#### AnalysisFindingRow

Raw DB row for findings. Derives `Debug, FromRow`.

Fields match `analysis_findings` table. `finding_type` as `String`, `confidence_score` as `f64`, `priority` and `sort_order` as `i32`.

#### AnalysisFinding

API response for a single finding. Derives `Debug, Serialize, Deserialize, ToSchema`.

Same fields but `finding_type` is `FindingType` enum. Implement `From<AnalysisFindingRow>`.

#### AnalysisFindingWithConcept

Enriched finding with joined concept data. Derives `Debug, Serialize, ToSchema`.

Includes all `AnalysisFinding` fields plus bilingual concept metadata (following the existing `ComplianceItemWithConcept` pattern from compliance models):

- `concept_code: Option<String>`
- `concept_name_en: String`
- `concept_name_nb: String`
- `concept_definition_en: String`
- `concept_definition_nb: Option<String>`
- `source_reference: Option<String>`

This struct is populated from a JOIN query between `analysis_findings` and `concepts`. The JOIN must alias the concept columns appropriately (e.g., `c.name_en AS concept_name_en`).

**Note:** The `framework_id` on a finding must always match the `framework_id` of the referenced concept. This invariant is enforced by the matching engine (split 03), not by a database constraint.

---

## Section 5: Rust Structs — Summary Types

### File: `backend/src/features/analysis/models.rs` (continued)

#### FrameworkFindingSummary

Per-framework aggregation. Derives `Debug, Serialize, ToSchema`.

Fields: `framework_id: String`, `framework_name: String`, `total_findings: i64`, `addressed_count: i64`, `partially_addressed_count: i64`, `gap_count: i64`, `not_applicable_count: i64`.

These are computed with SQL `COUNT(*) FILTER (WHERE finding_type = ...)` or equivalent `CASE WHEN` expressions, grouped by framework_id, with a JOIN to `frameworks` for the name.

#### AnalysisSummary

Analysis with aggregated stats. Derives `Debug, Serialize, ToSchema`.

Fields: all `Analysis` fields plus `total_findings: i64`, `gap_count: i64`, `addressed_count: i64`, `partially_addressed_count: i64`, `frameworks_matched: Vec<FrameworkFindingSummary>`.

This is NOT a FromRow struct — it's assembled in the route handler by running the analysis query plus aggregate queries. The `frameworks_matched` vector is populated from a separate GROUP BY query.

#### AnalysisListQuery

Query parameters for listing analyses. Derives `Debug, Deserialize, IntoParams`.

Fields: `page: i64` (default 1), `limit: i64` (default 50), `status: Option<AnalysisStatus>` (typed enum for validation, matching compliance pattern).

#### FindingsListQuery

Query parameters for listing findings. Derives `Debug, Deserialize, IntoParams`.

Fields: `framework_id: Option<String>`, `finding_type: Option<String>`, `priority: Option<i32>`, `sort_by: Option<String>` (values: "priority", "confidence", "framework"), `page: i64` (default 1), `limit: i64` (default 50).

---

## Section 6: MatchingEngine Trait

### File: `backend/src/features/analysis/engine.rs`

#### Trait Definition

Uses the `async-trait` crate (add `async-trait = "0.1"` to Cargo.toml).

The `MatchingEngine` trait has a single method `analyze` that takes:
- `&self`
- `text: &str` — the extracted document text
- `prompt_template: Option<&str>` — optional JSON config overriding defaults
- `db: &SqlitePool` — database access for querying ontology concepts

Returns `Result<MatchingResult, AnalysisError>`.

The trait is `Send + Sync` bounded for thread safety with Axum's async handlers.

#### MatchingResult

Struct returned by the trait. Fields:
- `matched_framework_ids: Vec<String>` — frameworks the engine determined are relevant
- `findings: Vec<NewFinding>` — all generated findings
- `processing_time_ms: i64` — wall-clock analysis duration
- `token_count: i64` — document token count estimate

#### NewFinding

A finding before it's persisted (no `id` or `analysis_id`). Fields:
- `concept_id: String`
- `framework_id: String`
- `finding_type: FindingType`
- `confidence_score: f64`
- `evidence_text: Option<String>`
- `recommendation: Option<String>`
- `priority: i32`

#### AnalysisError

Error enum using `thiserror`. Variants:
- `DatabaseError(sqlx::Error)` — with `#[from]`
- `NoFrameworksDetected` — document has no relevant framework matches
- `ProcessingFailed(String)` — generic analysis failure with message
- `InvalidPromptTemplate(String)` — malformed JSON template

---

## Section 7: Module Wiring & Dependencies

### New files to create

```
backend/src/features/analysis/
  mod.rs      — pub mod models; pub mod engine;
  models.rs   — all enums, structs, From impls, tests
  engine.rs   — MatchingEngine trait, MatchingResult, NewFinding, AnalysisError
```

### Existing files to modify

1. **`backend/src/features/mod.rs`** — Add `pub mod analysis;` (alphabetical order, before `pub mod auth;`)

2. **`backend/Cargo.toml`** — Add `async-trait = "0.1"` to `[dependencies]`

3. **`backend/src/main.rs`** — Add `after_connect` to SQLite pool builder for `PRAGMA foreign_keys = ON`

Note: Do NOT add router nesting in `lib.rs` yet — the `routes.rs` file doesn't exist until split 04.

The analysis feature should reuse the existing `PaginatedResponse<T>` from `ontology::models` (it's already generic). Import it rather than redefining.

### Dependency Changes

| Crate | Version | Purpose |
|-------|---------|---------|
| `async-trait` | `0.1` | Async trait support for `MatchingEngine` dyn dispatch |

No other new dependencies. `sqlx`, `serde`, `utoipa`, `chrono`, `uuid`, `thiserror` are all already in Cargo.toml.

---

## Section 8: Testing Strategy

### Unit Tests (in models.rs `#[cfg(test)]` block)

**Enum conversion tests:**
- Test every variant of `InputType`, `AnalysisStatus`, `FindingType` round-trips through String
- Test unknown string values fall back to defaults (Text, Pending, Gap respectively)

**From<Row> conversion tests:**
- Test `From<AnalysisRow> for Analysis` with all fields populated
- Test `matched_framework_ids` JSON deserialization: valid JSON array, empty string, malformed JSON
- Test `From<AnalysisFindingRow> for AnalysisFinding` with all finding types

**Query parameter defaults:**
- Test that `AnalysisListQuery` defaults to page=1, limit=50
- Test that `FindingsListQuery` defaults similarly

### Integration Tests

Not needed for this split — no routes or database queries to test. The migration will be validated when the backend starts and runs `sqlx::migrate!()`.

### Trait Tests (in engine.rs)

No implementation to test yet (the `DeterministicMatcher` comes in split 03). Include a compile-time check that the trait is dyn-compatible:

```rust
fn _assert_dyn_compatible(_: &dyn MatchingEngine) {}
```

---

## Decision Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Trait approach | `async-trait` crate | User chose extensibility via dyn dispatch over enum dispatch simplicity. Enables `Box<dyn MatchingEngine>` for runtime polymorphism. |
| Foreign keys | Enable globally | CASCADE delete requires it. Safe for existing schema. One-time fix. |
| JSON storage | Rust-side deserialization | `matched_framework_ids` is never queried with SQL json functions. Simpler to store as TEXT and deserialize with serde. |
| Aggregation | On-the-fly SQL | No summary table with triggers. COUNT/GROUP BY at query time is fast enough for MVP scale (hundreds of analyses, not millions). |
| Enum defaults | Fallback to safe values | Unknown DB values map to safe defaults (Pending, Gap, Text) rather than panicking, following existing compliance model pattern. |
| Row/Response split | Separate structs | AnalysisRow for FromRow, Analysis for API response. Matches existing pattern in ontology and compliance features. |
