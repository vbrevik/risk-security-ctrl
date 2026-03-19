Good, I have enough context now.

# Section 02: Rust Enums — InputType, AnalysisStatus, FindingType

## Overview

This section creates three domain enums in a new file `backend/src/features/analysis/models.rs`. Each enum models a constrained set of values stored as TEXT in the SQLite database and needs bidirectional `String` conversion plus serde serialization. The enums follow the exact pattern established by `AssessmentStatus` in `backend/src/features/compliance/models.rs`.

**No dependencies** on other sections. This section **blocks** section-03-models (which adds structs to the same file) and section-04-engine-trait (which imports `FindingType`).

## File to Create

**`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/models.rs`**

This file will be extended by section-03-models with structs. For now it contains only the three enums, their trait impls, and a test module.

## Tests FIRST

Write these tests in a `#[cfg(test)] mod tests` block at the bottom of `models.rs` before implementing the enums. All tests use standard `#[test]` attributes (no async, no external test deps).

### InputType tests

- `input_type_from_text`: `InputType::from("text".to_string())` equals `InputType::Text`
- `input_type_from_pdf`: `InputType::from("pdf".to_string())` equals `InputType::Pdf`
- `input_type_from_docx`: `InputType::from("docx".to_string())` equals `InputType::Docx`
- `input_type_from_unknown_defaults_to_text`: `InputType::from("unknown".to_string())` equals `InputType::Text`
- `input_type_to_string_text`: `String::from(InputType::Text)` equals `"text"`
- `input_type_to_string_pdf`: `String::from(InputType::Pdf)` equals `"pdf"`
- `input_type_to_string_docx`: `String::from(InputType::Docx)` equals `"docx"`
- `input_type_serde_roundtrip`: Serialize each variant to JSON with `serde_json::to_string`, deserialize back, assert equality. For example `InputType::Pdf` serializes to `"pdf"` and deserializes back to `InputType::Pdf`.

### AnalysisStatus tests

- `analysis_status_roundtrip_all_variants`: For each of the 5 variants (`Pending`, `Processing`, `Completed`, `Failed`, `Deleted`), convert to `String` and back, assert equal to original.
- `analysis_status_from_unknown_defaults_to_pending`: `AnalysisStatus::from("garbage".to_string())` equals `AnalysisStatus::Pending`
- `analysis_status_serde_roundtrip`: Serialize each variant, deserialize, assert equality.

### FindingType tests

- `finding_type_roundtrip_all_variants`: For each of the 4 variants, convert to `String` and back, assert equal.
- `finding_type_partially_addressed_underscore`: `FindingType::from("partially_addressed".to_string())` equals `FindingType::PartiallyAddressed` — verifies the snake_case underscore is handled correctly.
- `finding_type_from_unknown_defaults_to_gap`: `FindingType::from("nonsense".to_string())` equals `FindingType::Gap`
- `finding_type_serde_roundtrip`: Serialize/deserialize round-trip for each variant.

### Test structure (stub)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_type_from_text() {
        assert_eq!(InputType::from("text".to_string()), InputType::Text);
    }

    // ... remaining tests follow the same pattern
}
```

## Implementation Details

### Imports

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
```

`sqlx::FromRow` and `utoipa::IntoParams` are NOT needed yet — those are for the structs added in section-03.

### Enum 1: InputType

Variants: `Text`, `Pdf`, `Docx`

Derives: `Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq`

Attribute: `#[serde(rename_all = "snake_case")]`

Conversion impls:
- `From<String> for InputType` — match on `"text"`, `"pdf"`, `"docx"`, default to `InputType::Text`
- `From<InputType> for String` — map each variant to its lowercase string

Follow the exact match-arm pattern from `AssessmentStatus` in `compliance/models.rs` (lines 20-43 of that file).

### Enum 2: AnalysisStatus

Variants: `Pending`, `Processing`, `Completed`, `Failed`, `Deleted`

Same derive set and attribute as `InputType`.

Conversion strings: `"pending"`, `"processing"`, `"completed"`, `"failed"`, `"deleted"`.

Default fallback for unknown strings: `AnalysisStatus::Pending`.

### Enum 3: FindingType

Variants: `Addressed`, `PartiallyAddressed`, `Gap`, `NotApplicable`

Same derive set and attribute as `InputType`.

Conversion strings: `"addressed"`, `"partially_addressed"`, `"gap"`, `"not_applicable"`.

Default fallback for unknown strings: `FindingType::Gap`.

### Design Rationale

- **Fallback defaults instead of panicking**: Unknown database values map to safe defaults (`Text`, `Pending`, `Gap`) rather than causing a panic. This matches the existing compliance module pattern and is defensive against future schema evolution or data migration edge cases.
- **`serde(rename_all = "snake_case")`**: Ensures JSON serialization matches the database CHECK constraint values exactly (e.g., `PartiallyAddressed` serializes as `"partially_addressed"`).
- **`PartialEq, Eq`**: Required for test assertions with `assert_eq!` and for downstream use in match guards and filters.
- **`ToSchema`**: Required by utoipa for OpenAPI documentation generation.

## Verification

After implementation, run from `/Users/vidarbrevik/projects/risk-security-ctrl/backend/`:

```bash
cargo test --lib features::analysis::models::tests
```

Note: This will only compile once section-05-wiring has added `pub mod analysis;` to `features/mod.rs` and created `features/analysis/mod.rs` with `pub mod models;`. If working on this section in isolation, you can temporarily add those module declarations to verify tests pass, then revert.