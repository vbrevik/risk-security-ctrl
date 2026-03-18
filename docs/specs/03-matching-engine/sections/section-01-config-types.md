# Section 1: Prompt Template Configuration Types

## Goal

Create the foundational types for the matching engine: `MatcherConfig` (with JSON parsing and defaults), `Topic` (for deserialized topic tags), `ConceptCandidate` (raw retrieval result), and `ScoredCandidate` (scored retrieval result). These types are used by every subsequent section.

## File to Create

**`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs`**

This is a new file. It will eventually contain all matcher logic, but this section only defines the configuration and data types at the top of the file.

## Existing Code Context

The matcher module lives alongside existing analysis modules:

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs` -- currently exports `engine`, `models`, `parser`, `tokenizer`. Section 07 will add `pub mod matcher;` here.
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/engine.rs` -- defines `MatchingEngine` trait, `MatchingResult`, `NewFinding`, `AnalysisError`. The matcher will eventually implement `MatchingEngine`.
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/models.rs` -- defines `FindingType` enum (Addressed, PartiallyAddressed, Gap, NotApplicable).
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/tokenizer.rs` -- provides `extract_keywords()`, `term_frequency()`, `sentence_split()`, `generate_ngrams()`.

All dependencies (`serde`, `serde_json`, `tracing`, `std::collections::HashMap`) are already in `Cargo.toml`.

## Tests (Write First)

Place tests in a `#[cfg(test)] mod tests` block at the bottom of `matcher.rs`. The following tests validate the configuration types:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_config_default_thresholds() {
        /// Default config must return addressed_threshold=0.6, partial_threshold=0.3,
        /// min_confidence_threshold=0.1.
        let config = MatcherConfig::default();
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
        assert!((config.partial_threshold - 0.3).abs() < f64::EPSILON);
        assert!((config.min_confidence_threshold - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_default_other_fields() {
        /// version=1, max_findings_per_framework=50, include_addressed_findings=true.
        let config = MatcherConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.max_findings_per_framework, 50);
        assert!(config.include_addressed_findings);
    }

    #[test]
    fn test_matcher_config_default_boost_terms() {
        /// Default boost_terms must contain security=1.5, risk=1.5, compliance=1.3, control=1.2.
        let config = MatcherConfig::default();
        assert!((config.boost_terms["security"] - 1.5).abs() < f64::EPSILON);
        assert!((config.boost_terms["risk"] - 1.5).abs() < f64::EPSILON);
        assert!((config.boost_terms["compliance"] - 1.3).abs() < f64::EPSILON);
        assert!((config.boost_terms["control"] - 1.2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_from_json_none_returns_default() {
        /// Passing None yields the same as Default.
        let config = MatcherConfig::from_json(None);
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_from_json_valid() {
        /// A valid JSON string with custom thresholds should parse correctly.
        let json = r#"{"version":1,"min_confidence_threshold":0.2,"addressed_threshold":0.7,"partial_threshold":0.4,"max_findings_per_framework":100,"include_addressed_findings":false,"boost_terms":{"risk":2.0}}"#;
        let config = MatcherConfig::from_json(Some(json));
        assert!((config.addressed_threshold - 0.7).abs() < f64::EPSILON);
        assert!((config.partial_threshold - 0.4).abs() < f64::EPSILON);
        assert!((config.min_confidence_threshold - 0.2).abs() < f64::EPSILON);
        assert_eq!(config.max_findings_per_framework, 100);
        assert!(!config.include_addressed_findings);
        assert!((config.boost_terms["risk"] - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_from_json_malformed_returns_default() {
        /// Malformed JSON falls back to default (with a tracing warning logged).
        let config = MatcherConfig::from_json(Some("not valid json {{{"));
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
        assert_eq!(config.version, 1);
    }

    #[test]
    fn test_topic_deserialize() {
        /// Topic struct deserializes from JSON with id, name_en, concept_ids.
        let json = r#"{"id":"t1","name_en":"Access Control","concept_ids":["c1","c2"]}"#;
        let topic: Topic = serde_json::from_str(json).unwrap();
        assert_eq!(topic.id, "t1");
        assert_eq!(topic.name_en, "Access Control");
        assert_eq!(topic.concept_ids, vec!["c1", "c2"]);
    }

    #[test]
    fn test_concept_candidate_construction() {
        /// ConceptCandidate can be constructed with all fields.
        let c = ConceptCandidate {
            id: "c-1".into(),
            framework_id: "fw-1".into(),
            parent_id: None,
            name_en: "Risk Assessment".into(),
            definition_en: "Process of identifying risks".into(),
            code: Some("RA-1".into()),
            source_reference: Some("ISO 31000:2018 6.4".into()),
            concept_type: "process".into(),
        };
        assert_eq!(c.id, "c-1");
        assert!(c.parent_id.is_none());
    }

    #[test]
    fn test_scored_candidate_construction() {
        /// ScoredCandidate wraps ConceptCandidate with a confidence_score.
        let sc = ScoredCandidate {
            id: "c-1".into(),
            framework_id: "fw-1".into(),
            parent_id: Some("parent-1".into()),
            name_en: "Control".into(),
            definition_en: "A measure".into(),
            code: None,
            source_reference: None,
            concept_type: "control".into(),
            confidence_score: 0.85,
        };
        assert!((sc.confidence_score - 0.85).abs() < f64::EPSILON);
        assert_eq!(sc.parent_id, Some("parent-1".into()));
    }
}
```

## Implementation Details

### MatcherConfig struct

Derive `Debug, Clone, Serialize, Deserialize` with struct-level `#[serde(default)]`. Partial JSON input falls back to defaults for missing fields.

Fields:
- `version: u32` -- schema version, default 1
- `min_confidence_threshold: f64` -- minimum score to include a finding, default 0.1
- `addressed_threshold: f64` -- score at or above means "addressed", default 0.6
- `partial_threshold: f64` -- score at or above means "partially addressed", default 0.3
- `max_findings_per_framework: usize` -- cap per framework, default 50
- `include_addressed_findings: bool` -- whether to include high-confidence matches, default true
- `boost_terms: HashMap<String, f64>` -- domain terms with score multipliers, default: `{"security": 1.5, "risk": 1.5, "compliance": 1.3, "control": 1.2}`

Implement `Default` manually to set all the defaults listed above.

Implement `from_json(input: Option<&str>) -> Self`:
- If `input` is `None`, return `Self::default()`.
- If `input` is `Some(s)`, attempt `serde_json::from_str(s)`. On success return the parsed config. On failure, log a warning via `tracing::warn!` with the parse error and return `Self::default()`.
- After parsing, call `validate_thresholds()` which logs `tracing::warn!` if `partial_threshold >= addressed_threshold` or any threshold is outside [0.0, 1.0]. (Added per code review — prevents silent misclassification.)

### Topic struct

Derive `Debug, Clone, Deserialize`. Fields:
- `id: String`
- `name_en: String`
- `concept_ids: Vec<String>`

This is a local type used to deserialize topic tags passed from the route handler. Topics come from `ontology-data/topic-tags.json` and are loaded by the API layer before constructing the matcher.

### ConceptCandidate struct

Derive `Debug, Clone, PartialEq`. Fields:
- `id: String`
- `framework_id: String`
- `parent_id: Option<String>`
- `name_en: String`
- `definition_en: String`
- `code: Option<String>`
- `source_reference: Option<String>`
- `concept_type: String`

This represents a raw candidate concept retrieved from the database by the FTS5 retrieval stage (Section 03). It maps directly to columns from the `concepts` table.

### ScoredCandidate struct

Derive `Debug, Clone`. Uses composition (not field duplication):
- `candidate: ConceptCandidate`
- `confidence_score: f64`

This is the output of the scoring stage (Section 04). Access concept fields via `sc.candidate.name_en`. Changed from flat duplication per code review to avoid maintenance hazard.

### Imports needed at top of file

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::warn;
```

## Dependencies

- **No dependencies on other sections.** This section is the foundation that all other sections build on.
- All crate dependencies (`serde`, `serde_json`, `tracing`) are already present in `Cargo.toml`.

## Verification

After implementation, temporarily add `pub mod matcher;` to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs` (Section 07 does this permanently) and run:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo test --lib matcher
```

All 9 tests listed above should pass. Remove the `pub mod matcher;` line after verifying if you want to keep the module unwired until Section 07.