# Section 01 Code Review Interview

## Decisions

### 1. ScoredCandidate: flat vs composition [ASKED USER]
**Decision:** Use composition (`candidate: ConceptCandidate` + `confidence_score`)
**Rationale:** More idiomatic Rust, avoids field duplication maintenance hazard.
**Applied:** Yes — restructured ScoredCandidate, updated test.

### 2. Threshold invariant validation [ASKED USER]
**Decision:** Add warn-only validation in `from_json`
**Rationale:** Prevents silent misclassification from inverted/OOB thresholds.
**Applied:** Yes — added `validate_thresholds()` with tracing::warn for partial >= addressed and OOB checks.

## Auto-fixes

### 3. Struct-level serde default [AUTO-FIX]
**Change:** Replaced 7 per-field `#[serde(default = "fn")]` attributes with single `#[serde(default)]` at struct level. Removed 7 helper functions.
**Rationale:** Same behavior, less code.

### 4. PartialEq derive on ConceptCandidate [AUTO-FIX]
**Change:** Added `PartialEq` derive to `ConceptCandidate`.
**Rationale:** Enables collection assertions in later section tests.

## Let Go

- confidence_score range enforcement (section 04 concern)
- Topic missing Serialize (not needed for current usage)
