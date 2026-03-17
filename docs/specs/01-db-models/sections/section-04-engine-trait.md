Good, the file doesn't exist yet - this is greenfield. Now I have all the context needed.

# Section 4: MatchingEngine Trait

## Overview

This section creates the `MatchingEngine` async trait, along with supporting types `MatchingResult`, `NewFinding`, and `AnalysisError`. These are defined in a new file `backend/src/features/analysis/engine.rs`. The trait defines the interface for pluggable analysis implementations that evaluate document text against ontology frameworks.

## Dependencies

- **section-02-enums** must be completed first: this section imports `FindingType` from `features/analysis/models.rs`.
- **section-05-wiring** will register this module and add the `async-trait` crate to `Cargo.toml`. Until then, the file can be written but will not compile as part of the project.

## File to Create

**`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/engine.rs`**

## Tests First

Write these tests in a `#[cfg(test)] mod tests` block at the bottom of `engine.rs`. They validate dyn-compatibility, error construction, and basic struct construction.

### Test: Dyn compatibility compile check

A function that accepts `&dyn MatchingEngine` proves the trait is object-safe (dyn-compatible). If it compiles, the test passes. No `#[test]` attribute needed -- it is a compile-time assertion.

```rust
fn _assert_dyn_compatible(_: &dyn MatchingEngine) {}
```

### Test: AnalysisError displays meaningful messages

```rust
#[test]
fn test_analysis_error_no_frameworks_display() {
    let err = AnalysisError::NoFrameworksDetected;
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}
```

### Test: AnalysisError From<sqlx::Error> works

Construct a `sqlx::Error` (e.g., `sqlx::Error::RowNotFound`) and verify it converts into `AnalysisError::DatabaseError` via the `#[from]` attribute.

```rust
#[test]
fn test_analysis_error_from_sqlx() {
    let sqlx_err = sqlx::Error::RowNotFound;
    let err: AnalysisError = sqlx_err.into();
    assert!(matches!(err, AnalysisError::DatabaseError(_)));
}
```

### Test: NewFinding construction

Verify a `NewFinding` can be constructed with all required fields.

```rust
#[test]
fn test_new_finding_construction() {
    let finding = NewFinding {
        concept_id: "concept-1".to_string(),
        framework_id: "nist-csf".to_string(),
        finding_type: FindingType::Gap,
        confidence_score: 0.85,
        evidence_text: Some("evidence here".to_string()),
        recommendation: None,
        priority: 2,
    };
    assert_eq!(finding.concept_id, "concept-1");
    assert_eq!(finding.priority, 2);
}
```

### Test: MatchingResult with empty data

Verify `MatchingResult` can hold empty vecs and zero counts.

```rust
#[test]
fn test_matching_result_empty() {
    let result = MatchingResult {
        matched_framework_ids: vec![],
        findings: vec![],
        processing_time_ms: 0,
        token_count: 0,
    };
    assert!(result.findings.is_empty());
    assert!(result.matched_framework_ids.is_empty());
}
```

## Implementation Details

### Imports

The file needs these imports:

```rust
use async_trait::async_trait;
use sqlx::SqlitePool;
use super::models::FindingType;
```

Note: `async_trait` comes from the `async-trait` crate (version `0.1`), which must be added to `Cargo.toml` (handled in section-05-wiring). `thiserror` version `2` is already in `Cargo.toml`.

### AnalysisError enum

Define using `thiserror::Error` derive (the project uses `thiserror = "2"`, so the derive path is `thiserror::Error`):

- `DatabaseError(sqlx::Error)` with `#[from]` attribute for automatic conversion and display message like `"Database error: {0}"`
- `NoFrameworksDetected` with display `"No relevant frameworks detected in document"`
- `ProcessingFailed(String)` with display `"Processing failed: {0}"`
- `InvalidPromptTemplate(String)` with display `"Invalid prompt template: {0}"`

Derive `Debug` on the enum.

### MatchingResult struct

A plain data struct with derives `Debug, Clone`:

| Field | Type |
|-------|------|
| `matched_framework_ids` | `Vec<String>` |
| `findings` | `Vec<NewFinding>` |
| `processing_time_ms` | `i64` |
| `token_count` | `i64` |

### NewFinding struct

A finding before persistence (no `id` or `analysis_id`). Derives `Debug, Clone`:

| Field | Type |
|-------|------|
| `concept_id` | `String` |
| `framework_id` | `String` |
| `finding_type` | `FindingType` |
| `confidence_score` | `f64` |
| `evidence_text` | `Option<String>` |
| `recommendation` | `Option<String>` |
| `priority` | `i32` |

### MatchingEngine trait

The trait uses the `#[async_trait]` attribute macro and requires `Send + Sync` bounds for compatibility with Axum's async handlers and `Arc<dyn MatchingEngine>` usage.

Single method signature:

```rust
#[async_trait]
pub trait MatchingEngine: Send + Sync {
    /// Analyze extracted text against ontology frameworks.
    ///
    /// - `text`: the document text to analyze
    /// - `prompt_template`: optional JSON config overriding default matching behavior
    /// - `db`: database pool for querying ontology concepts during analysis
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError>;
}
```

The actual implementation (`DeterministicMatcher`) will be created in a later split (03-matching-engine). This file only defines the trait interface.

## Key Design Decisions

- **`async-trait` crate** is used rather than native async traits (stable in Rust 1.75+) because dyn dispatch with `Box<dyn MatchingEngine>` requires it. Native async traits do not support dyn dispatch without boxing.
- **`Send + Sync` bounds** on the trait enable storing the engine in Axum's shared state via `Arc<dyn MatchingEngine>`.
- **`&SqlitePool`** is passed to `analyze` rather than stored in the trait impl, keeping the engine stateless and testable with different database pools.

## Verification

After implementation (and after section-05-wiring completes module registration and adds `async-trait` to `Cargo.toml`):

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo test --lib features::analysis::engine
```

All five tests above should pass, confirming the trait is dyn-compatible, errors work as expected, and the data structs are constructible.