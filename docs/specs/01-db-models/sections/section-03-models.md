# Section 03: Rust Structs -- Core Models, Summary Types, and From Conversions

## Overview

This section adds all Rust structs to `backend/src/features/analysis/models.rs`: the database row types, API response types, request types, summary/aggregation types, and query parameter structs. It also implements `From<Row>` conversions that bridge the raw database layer to the typed API layer.

This section builds on the enums defined in section-02 (`InputType`, `AnalysisStatus`, `FindingType`) which must already exist in the same file. It does NOT add routes or database queries -- those come in later splits.

## Dependencies

- **section-01-migration**: The struct field names and types must match the `analyses` and `analysis_findings` table columns defined in the migration.
- **section-02-enums**: The three enums (`InputType`, `AnalysisStatus`, `FindingType`) and their `From<String>` / `Into<String>` implementations must already be in `backend/src/features/analysis/models.rs`.

## File

All code goes in: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/models.rs`

This file already contains the three enums from section-02. The structs defined here are appended below those enums, before the `#[cfg(test)]` block.

## Tests First

Write these tests in the `#[cfg(test)] mod tests` block of `models.rs`. They should be added alongside the existing enum tests from section-02.

### From<AnalysisRow> for Analysis tests

```rust
#[test]
fn test_analysis_from_row_all_fields() {
    /// Construct an AnalysisRow with all fields populated.
    /// Convert to Analysis. Assert:
    /// - input_type is the correct InputType enum variant
    /// - status is the correct AnalysisStatus enum variant  
    /// - matched_framework_ids is a Vec<String> with expected elements
    /// - all other fields transferred verbatim
}

#[test]
fn test_analysis_matched_frameworks_valid_json() {
    /// AnalysisRow with matched_framework_ids = Some(r#"["nist-csf","iso31000"]"#.into())
    /// After conversion, Analysis.matched_framework_ids == vec!["nist-csf", "iso31000"]
}

#[test]
fn test_analysis_matched_frameworks_none() {
    /// AnalysisRow with matched_framework_ids = None
    /// After conversion, Analysis.matched_framework_ids == vec![]
}

#[test]
fn test_analysis_matched_frameworks_malformed_json() {
    /// AnalysisRow with matched_framework_ids = Some("not json".into())
    /// After conversion, Analysis.matched_framework_ids == vec![] (graceful fallback)
}

#[test]
fn test_analysis_matched_frameworks_empty_string() {
    /// AnalysisRow with matched_framework_ids = Some("".into())
    /// After conversion, Analysis.matched_framework_ids == vec![]
}
```

### From<AnalysisFindingRow> for AnalysisFinding tests

```rust
#[test]
fn test_finding_from_row() {
    /// Construct an AnalysisFindingRow with finding_type = "gap".
    /// Convert to AnalysisFinding. Assert finding_type == FindingType::Gap.
    /// Assert confidence_score preserves precision (e.g., 0.75 stays 0.75).
}
```

### Query parameter default tests

```rust
#[test]
fn test_analysis_list_query_defaults() {
    /// Deserialize an empty JSON object "{}" into AnalysisListQuery.
    /// Assert page == 1, limit == 50, status == None.
}

#[test]
fn test_findings_list_query_defaults() {
    /// Deserialize an empty JSON object "{}" into FindingsListQuery.
    /// Assert page == 1, limit == 50, all filter fields are None.
}
```

## Implementation Details

### Pattern Reference

Follow the exact pattern used in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/compliance/models.rs`. That file demonstrates:
- `Row` struct with `#[derive(Debug, FromRow)]` and all-String/Option fields
- API response struct with `#[derive(Debug, Serialize, Deserialize, ToSchema)]` and typed enum fields
- `From<Row> for Response` impl that converts string fields to enums
- Query param structs with `#[derive(Debug, Deserialize, IntoParams)]` and `#[serde(default = "default_page")]`

### Required imports

At the top of `models.rs`, ensure these imports are present (some may already exist from section-02):

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
```

### Struct: AnalysisRow

Derives `Debug, FromRow`. All fields as raw database types (strings, options, integers). Field list matches the `analyses` table column order:

- `id: String`, `name: String`
- `description: Option<String>`, `input_type: String`, `input_text: Option<String>`
- `original_filename: Option<String>`, `file_path: Option<String>`, `extracted_text: Option<String>`
- `status: String`, `error_message: Option<String>`
- `prompt_template: Option<String>`, `matched_framework_ids: Option<String>`
- `processing_time_ms: Option<i64>`, `token_count: Option<i64>`
- `created_by: Option<String>`
- `created_at: String`, `updated_at: String`

### Struct: Analysis

Derives `Debug, Serialize, Deserialize, ToSchema`. Same fields as `AnalysisRow` except:

- `input_type: InputType` (enum, not String)
- `status: AnalysisStatus` (enum, not String)
- `matched_framework_ids: Vec<String>` (deserialized from JSON text)

### From<AnalysisRow> for Analysis

The conversion must:
1. Call `InputType::from(row.input_type)` and `AnalysisStatus::from(row.status)` to convert string fields to enums.
2. Deserialize `matched_framework_ids` from JSON: use `serde_json::from_str` on the `Option<String>`. If `None`, empty string, or invalid JSON, fall back to an empty `Vec<String>`. This is the critical conversion -- do NOT unwrap or panic.
3. Transfer all other fields directly.

### Struct: CreateAnalysisRequest

Derives `Debug, Deserialize, ToSchema`. Fields:
- `name: String`
- `description: Option<String>`
- `input_text: String`
- `prompt_template: Option<String>`

### Struct: AnalysisFindingRow

Derives `Debug, FromRow`. Fields matching `analysis_findings` table:
- `id: String`, `analysis_id: String`, `concept_id: String`, `framework_id: String`
- `finding_type: String`
- `confidence_score: f64`
- `evidence_text: Option<String>`, `recommendation: Option<String>`
- `priority: i32`, `sort_order: i32`
- `created_at: String`

### Struct: AnalysisFinding

Derives `Debug, Serialize, Deserialize, ToSchema`. Same as `AnalysisFindingRow` but `finding_type: FindingType` (enum).

### From<AnalysisFindingRow> for AnalysisFinding

Convert `finding_type` string to `FindingType` enum. Transfer all other fields directly.

### Struct: AnalysisFindingWithConcept

Derives `Debug, Serialize, ToSchema`. Contains all `AnalysisFinding` fields (flattened, not nested) plus concept metadata from a JOIN query:

- `concept_code: Option<String>`
- `concept_name_en: String`
- `concept_name_nb: String`
- `concept_definition_en: String`
- `concept_definition_nb: Option<String>`
- `source_reference: Option<String>`

This struct is populated from a JOIN query in the routes layer (split 04). It does not need a `FromRow` derive -- it will be constructed manually from a row that aliases concept columns (e.g., `c.name_en AS concept_name_en`). Alternatively, if you use `FromRow`, ensure the column aliases match field names exactly.

### Struct: FrameworkFindingSummary

Derives `Debug, Serialize, ToSchema`. Per-framework aggregation:
- `framework_id: String`
- `framework_name: String`
- `total_findings: i64`
- `addressed_count: i64`
- `partially_addressed_count: i64`
- `gap_count: i64`
- `not_applicable_count: i64`

Populated from a `GROUP BY framework_id` query with conditional counts. Not a `FromRow` struct.

### Struct: AnalysisSummary

Derives `Debug, Serialize, ToSchema`. Analysis with aggregated stats:
- All `Analysis` fields (flattened, not nested)
- `total_findings: i64`
- `gap_count: i64`
- `addressed_count: i64`
- `partially_addressed_count: i64`
- `frameworks_matched: Vec<FrameworkFindingSummary>`

Assembled in the route handler, not from a single DB query.

### Helper functions and Query Parameter Structs

Define default value functions (or reuse from compliance if accessible):

```rust
fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 50 }
```

### Struct: AnalysisListQuery

Derives `Debug, Deserialize, IntoParams`. Fields:
- `page: i64` with `#[serde(default = "default_page")]`
- `limit: i64` with `#[serde(default = "default_limit")]`
- `status: Option<AnalysisStatus>`

### Struct: FindingsListQuery

Derives `Debug, Deserialize, IntoParams`. Fields:
- `framework_id: Option<String>`
- `finding_type: Option<String>`
- `priority: Option<i32>`
- `sort_by: Option<String>` -- accepted values: `"priority"`, `"confidence"`, `"framework"`
- `page: i64` with `#[serde(default = "default_page")]`
- `limit: i64` with `#[serde(default = "default_limit")]`

## Notes on PaginatedResponse

The generic `PaginatedResponse<T>` already exists in both `ontology::models` and `compliance::models`. Reuse one of them via import rather than redefining. When routes are added in split 04, import it as:

```rust
use crate::features::compliance::models::PaginatedResponse;
```

Do NOT define `PaginatedResponse` again in this module.

## Verification

After implementation, run:
```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo test features::analysis::models
```

All tests from both section-02 (enum tests) and this section (struct conversion tests, query default tests) should pass. Also verify `cargo check` succeeds to confirm all types are well-formed and derive macros expand correctly.