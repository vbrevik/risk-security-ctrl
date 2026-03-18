use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::warn;

// ============================================================================
// Configuration Types
// ============================================================================

/// Configuration for the deterministic matching engine.
///
/// Deserialized from the prompt template JSON. Falls back to sensible defaults
/// for any missing fields via `serde(default)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MatcherConfig {
    pub version: u32,
    pub min_confidence_threshold: f64,
    pub addressed_threshold: f64,
    pub partial_threshold: f64,
    pub max_findings_per_framework: usize,
    pub include_addressed_findings: bool,
    pub boost_terms: HashMap<String, f64>,
}

impl Default for MatcherConfig {
    fn default() -> Self {
        Self {
            version: 1,
            min_confidence_threshold: 0.1,
            addressed_threshold: 0.6,
            partial_threshold: 0.3,
            max_findings_per_framework: 50,
            include_addressed_findings: true,
            boost_terms: HashMap::from([
                ("security".into(), 1.5),
                ("risk".into(), 1.5),
                ("compliance".into(), 1.3),
                ("control".into(), 1.2),
            ]),
        }
    }
}

impl MatcherConfig {
    /// Parse a `MatcherConfig` from an optional JSON string.
    ///
    /// Returns `Self::default()` if `input` is `None` or the JSON is malformed.
    /// Logs warnings for malformed JSON and for threshold invariant violations.
    pub fn from_json(input: Option<&str>) -> Self {
        let config = match input {
            None => return Self::default(),
            Some(s) => serde_json::from_str::<Self>(s).unwrap_or_else(|e| {
                warn!("Failed to parse MatcherConfig JSON, using defaults: {e}");
                Self::default()
            }),
        };
        config.validate_thresholds();
        config
    }

    fn validate_thresholds(&self) {
        if self.partial_threshold >= self.addressed_threshold {
            warn!(
                "MatcherConfig: partial_threshold ({}) >= addressed_threshold ({}); \
                 classification may produce unexpected results",
                self.partial_threshold, self.addressed_threshold
            );
        }
        if self.addressed_threshold < 0.0 || self.addressed_threshold > 1.0 {
            warn!(
                "MatcherConfig: addressed_threshold ({}) outside [0.0, 1.0]",
                self.addressed_threshold
            );
        }
        if self.partial_threshold < 0.0 || self.partial_threshold > 1.0 {
            warn!(
                "MatcherConfig: partial_threshold ({}) outside [0.0, 1.0]",
                self.partial_threshold
            );
        }
        if self.min_confidence_threshold < 0.0 || self.min_confidence_threshold > 1.0 {
            warn!(
                "MatcherConfig: min_confidence_threshold ({}) outside [0.0, 1.0]",
                self.min_confidence_threshold
            );
        }
    }
}

// ============================================================================
// Topic Type
// ============================================================================

/// A topic tag loaded from `ontology-data/topic-tags.json`.
///
/// Topics map cross-cutting themes (e.g. "Access Control") to specific
/// ontology concept IDs across multiple frameworks.
#[derive(Debug, Clone, Deserialize)]
pub struct Topic {
    pub id: String,
    pub name_en: String,
    pub concept_ids: Vec<String>,
}

// ============================================================================
// Candidate Types
// ============================================================================

/// A raw concept candidate retrieved from the database during FTS5 retrieval.
///
/// Maps directly to columns from the `concepts` table.
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptCandidate {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub name_en: String,
    pub definition_en: String,
    pub code: Option<String>,
    pub source_reference: Option<String>,
    pub concept_type: String,
}

/// A scored candidate after TF-IDF scoring.
///
/// Wraps a `ConceptCandidate` with a `confidence_score` in the range [0.0, 1.0].
#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub candidate: ConceptCandidate,
    pub confidence_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_config_default_thresholds() {
        let config = MatcherConfig::default();
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
        assert!((config.partial_threshold - 0.3).abs() < f64::EPSILON);
        assert!((config.min_confidence_threshold - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_default_other_fields() {
        let config = MatcherConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.max_findings_per_framework, 50);
        assert!(config.include_addressed_findings);
    }

    #[test]
    fn test_matcher_config_default_boost_terms() {
        let config = MatcherConfig::default();
        assert!((config.boost_terms["security"] - 1.5).abs() < f64::EPSILON);
        assert!((config.boost_terms["risk"] - 1.5).abs() < f64::EPSILON);
        assert!((config.boost_terms["compliance"] - 1.3).abs() < f64::EPSILON);
        assert!((config.boost_terms["control"] - 1.2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_from_json_none_returns_default() {
        let config = MatcherConfig::from_json(None);
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_matcher_config_from_json_valid() {
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
        let config = MatcherConfig::from_json(Some("not valid json {{{"));
        assert!((config.addressed_threshold - 0.6).abs() < f64::EPSILON);
        assert_eq!(config.version, 1);
    }

    #[test]
    fn test_topic_deserialize() {
        let json = r#"{"id":"t1","name_en":"Access Control","concept_ids":["c1","c2"]}"#;
        let topic: Topic = serde_json::from_str(json).unwrap();
        assert_eq!(topic.id, "t1");
        assert_eq!(topic.name_en, "Access Control");
        assert_eq!(topic.concept_ids, vec!["c1", "c2"]);
    }

    #[test]
    fn test_concept_candidate_construction() {
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
        let candidate = ConceptCandidate {
            id: "c-1".into(),
            framework_id: "fw-1".into(),
            parent_id: Some("parent-1".into()),
            name_en: "Control".into(),
            definition_en: "A measure".into(),
            code: None,
            source_reference: None,
            concept_type: "control".into(),
        };
        let sc = ScoredCandidate {
            candidate,
            confidence_score: 0.85,
        };
        assert!((sc.confidence_score - 0.85).abs() < f64::EPSILON);
        assert_eq!(sc.candidate.parent_id, Some("parent-1".into()));
    }
}
