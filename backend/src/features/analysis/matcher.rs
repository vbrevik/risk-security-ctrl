use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use tracing::warn;

use super::tokenizer::extract_keywords;

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

// ============================================================================
// Framework Detection
// ============================================================================

/// Detect which frameworks are relevant to a document based on keyword matching.
///
/// Uses a two-pronged approach:
/// 1. Topic matching: overlap between document keywords and topic name tokens,
///    then map matched topics to their concept_ids to identify frameworks.
/// 2. Direct name matching: check if document keywords contain framework names
///    or common abbreviations (e.g., "nist", "iso", "gdpr").
///
/// Returns framework IDs ordered by match strength (highest first).
pub fn detect_frameworks(
    doc_keywords: &[String],
    topics: &[Topic],
    frameworks: &[(String, String)], // (id, name) pairs
    _config: &MatcherConfig,
) -> Vec<String> {
    let doc_kw_set: HashSet<&str> = doc_keywords.iter().map(|s| s.as_str()).collect();

    // Step 1: Topic matching — find topics whose name tokens overlap with doc keywords
    // Deduplicate concept_ids to prevent score inflation from overlapping topics
    let mut matched_concept_ids: HashSet<&str> = HashSet::new();
    for topic in topics {
        let topic_tokens = extract_keywords(&topic.name_en);
        let overlap = topic_tokens
            .iter()
            .filter(|t| doc_kw_set.contains(t.as_str()))
            .count();
        if overlap > 0 {
            for cid in &topic.concept_ids {
                matched_concept_ids.insert(cid.as_str());
            }
        }
    }

    // Step 2: Score each framework
    let mut scores: HashMap<&str, f64> = HashMap::new();

    for (fw_id, fw_name) in frameworks {
        let mut score = 0.0_f64;

        // Topic-based score: count concept_ids that belong to this framework
        // Use delimiter guard to prevent prefix collisions (e.g., "nist" matching "nist-csf-*")
        let fw_prefix = format!("{}-", fw_id);
        let topic_count = matched_concept_ids
            .iter()
            .filter(|cid| cid.starts_with(&fw_prefix) || **cid == fw_id.as_str())
            .count();
        score += topic_count as f64;

        // Direct name match: tokenize framework name, check overlap with doc keywords
        let fw_tokens = extract_keywords(fw_name);
        let name_overlap = fw_tokens
            .iter()
            .filter(|t| doc_kw_set.contains(t.as_str()))
            .count();
        if name_overlap > 0 {
            score += 2.0;
        }

        if score > 0.0 {
            scores.insert(fw_id.as_str(), score);
        }
    }

    // Step 3: Sort by score descending and return IDs
    let mut ranked: Vec<(&str, f64)> = scores.into_iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked.into_iter().map(|(id, _)| id.to_string()).collect()
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

    // ========================================================================
    // Section 02: Framework Detection Tests
    // ========================================================================

    fn make_topic(id: &str, name: &str, concept_ids: Vec<&str>) -> Topic {
        Topic {
            id: id.into(),
            name_en: name.into(),
            concept_ids: concept_ids.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_detect_frameworks_risk_keywords_match_iso31000() {
        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "management".into()];
        let topics = vec![make_topic(
            "risk-mgmt",
            "Risk Management and Assessment",
            vec!["iso31000-risk", "iso31000-assessment"],
        )];
        let frameworks = vec![
            ("iso31000".into(), "ISO 31000".into()),
            ("nist-csf".into(), "NIST Cybersecurity Framework".into()),
        ];
        let config = MatcherConfig::default();

        let result = detect_frameworks(&doc_keywords, &topics, &frameworks, &config);
        assert!(result.contains(&"iso31000".to_string()));
    }

    #[test]
    fn test_detect_frameworks_direct_name_match_nist() {
        let doc_keywords: Vec<String> = vec!["nist".into(), "cybersecurity".into(), "framework".into()];
        let topics: Vec<Topic> = vec![];
        let frameworks = vec![("nist-csf".into(), "NIST Cybersecurity Framework".into())];
        let config = MatcherConfig::default();

        let result = detect_frameworks(&doc_keywords, &topics, &frameworks, &config);
        assert!(result.contains(&"nist-csf".to_string()));
    }

    #[test]
    fn test_detect_frameworks_unrelated_keywords_empty() {
        let doc_keywords: Vec<String> = vec!["banana".into(), "tropical".into(), "fruit".into()];
        let topics = vec![make_topic(
            "risk-mgmt",
            "Risk Management",
            vec!["iso31000-risk"],
        )];
        let frameworks = vec![("iso31000".into(), "ISO 31000".into())];
        let config = MatcherConfig::default();

        let result = detect_frameworks(&doc_keywords, &topics, &frameworks, &config);
        assert!(result.is_empty());
    }

    #[test]
    fn test_detect_frameworks_ordered_by_strength() {
        // iso31000 gets topic score (3 concept matches) + name match (2.0) = 5.0
        // nist-csf gets name match only (2.0)
        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "management".into(), "nist".into()];
        let topics = vec![make_topic(
            "risk-mgmt",
            "Risk Management and Assessment",
            vec!["iso31000-risk", "iso31000-assessment", "iso31000-management"],
        )];
        let frameworks = vec![
            ("nist-csf".into(), "NIST Cybersecurity Framework".into()),
            ("iso31000".into(), "ISO 31000".into()),
        ];
        let config = MatcherConfig::default();

        let result = detect_frameworks(&doc_keywords, &topics, &frameworks, &config);
        assert_eq!(result.len(), 2);
        // iso31000 ranks first (topic concepts + name) vs nist-csf (name only)
        assert_eq!(result[0], "iso31000");
        assert_eq!(result[1], "nist-csf");
    }
}
