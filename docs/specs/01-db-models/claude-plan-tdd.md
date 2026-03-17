# TDD Plan: 01-db-models

Companion to `claude-plan.md`. Defines tests to write BEFORE each implementation section.

Testing framework: Rust built-in `#[cfg(test)]` modules with `#[test]` attributes. Dev deps already include `tokio-test` and `tower`.

---

## Section 1: Database Migration

No unit tests. Validated by:
- `sqlx::migrate!()` at startup (existing pattern)
- Backend starts without errors
- Tables exist and accept inserts

---

## Section 2: PRAGMA foreign_keys

### Tests (integration-level, in main.rs or a test helper)

- Test: After pool creation, `PRAGMA foreign_keys` returns 1 (enabled)
- Test: `PRAGMA foreign_key_check` runs without panic, logs violations if any
- Test: Deleting an analysis row cascades to delete its findings (requires Section 1 migration)

---

## Section 3: Rust Enums

### Tests (in `models.rs #[cfg(test)]`)

**InputType:**
- Test: `InputType::from("text".to_string())` == `InputType::Text`
- Test: `InputType::from("pdf".to_string())` == `InputType::Pdf`
- Test: `InputType::from("docx".to_string())` == `InputType::Docx`
- Test: `InputType::from("unknown".to_string())` == `InputType::Text` (default)
- Test: `String::from(InputType::Text)` == `"text"`
- Test: `String::from(InputType::Pdf)` == `"pdf"`
- Test: `String::from(InputType::Docx)` == `"docx"`
- Test: serde round-trip (serialize to JSON, deserialize back, assert equal)

**AnalysisStatus:**
- Test: All 5 variants round-trip through String
- Test: Unknown string defaults to Pending
- Test: serde round-trip for each variant

**FindingType:**
- Test: All 4 variants round-trip through String
- Test: `"partially_addressed"` maps to `PartiallyAddressed` (underscore handling)
- Test: Unknown string defaults to Gap
- Test: serde round-trip

---

## Section 4: Rust Structs — Core Models

### Tests (in `models.rs #[cfg(test)]`)

**From<AnalysisRow> for Analysis:**
- Test: All fields populated, conversion succeeds with correct enum types
- Test: `matched_framework_ids` valid JSON `["nist-csf","iso31000"]` deserializes to `Vec<String>` with 2 elements
- Test: `matched_framework_ids` is `None` → empty Vec
- Test: `matched_framework_ids` is malformed JSON → empty Vec (graceful fallback)
- Test: `matched_framework_ids` is empty string → empty Vec
- Test: `input_type` string maps to correct `InputType` enum
- Test: `status` string maps to correct `AnalysisStatus` enum

**From<AnalysisFindingRow> for AnalysisFinding:**
- Test: All fields populated, `finding_type` string maps to `FindingType` enum
- Test: `confidence_score` preserves precision (0.75 stays 0.75)

---

## Section 5: Rust Structs — Summary Types

### Tests (in `models.rs #[cfg(test)]`)

**AnalysisListQuery defaults:**
- Test: Default deserialization produces page=1, limit=50, status=None

**FindingsListQuery defaults:**
- Test: Default deserialization produces page=1, limit=50, all filters None

---

## Section 6: MatchingEngine Trait

### Tests (in `engine.rs #[cfg(test)]`)

**Dyn compatibility:**
- Test: Compile-time check that `MatchingEngine` is dyn-compatible: `fn _assert(_: &dyn MatchingEngine) {}`

**AnalysisError:**
- Test: `AnalysisError::NoFrameworksDetected` displays meaningful message
- Test: `AnalysisError::from(sqlx::Error)` works (From impl)

**NewFinding construction:**
- Test: Can construct `NewFinding` with all required fields
- Test: `MatchingResult` can hold empty `findings` vec and zero framework IDs

---

## Section 7: Module Wiring & Dependencies

No unit tests. Validated by:
- `cargo check` compiles without errors
- `cargo test` runs all module tests
- Feature module is accessible from other features (e.g., `use crate::features::analysis::models::*`)
