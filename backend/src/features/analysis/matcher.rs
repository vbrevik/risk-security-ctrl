use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
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

// ============================================================================
// FTS5 Candidate Retrieval
// ============================================================================

/// FTS5 reserved words that must be filtered from keyword lists.
const FTS5_RESERVED: &[&str] = &["and", "or", "not", "near"];

/// Strip FTS5 special operators and reserved words from keywords to prevent
/// MATCH syntax errors.
///
/// Removes: `"`, `*`, `(`, `)`, `+`, `-`, `^`, `{`, `}`, `:`, `~`, `\`
/// Filters out FTS5 reserved words: AND, OR, NOT, NEAR
/// Discards keywords that become empty after sanitization.
fn sanitize_fts_keywords(keywords: &[String]) -> Vec<String> {
    const FTS5_SPECIAL: &[char] = &['"', '*', '(', ')', '+', '-', '^', '{', '}', ':', '~', '\\'];
    keywords
        .iter()
        .filter_map(|kw| {
            let cleaned: String = kw.chars().filter(|c| !FTS5_SPECIAL.contains(c)).collect();
            let trimmed = cleaned.trim().to_string();
            if trimmed.is_empty() {
                return None;
            }
            // Filter out FTS5 reserved words
            if FTS5_RESERVED.contains(&trimmed.to_lowercase().as_str()) {
                return None;
            }
            Some(trimmed)
        })
        .collect()
}

/// Escape LIKE wildcards in a keyword for safe use in SQL LIKE patterns.
fn escape_like(keyword: &str) -> String {
    keyword.replace('%', "\\%").replace('_', "\\_")
}

/// Row type for reading concept candidates from SQLite queries.
#[derive(sqlx::FromRow)]
struct ConceptRow {
    id: String,
    framework_id: String,
    parent_id: Option<String>,
    name_en: String,
    definition_en: String,
    code: Option<String>,
    source_reference: Option<String>,
    concept_type: String,
}

impl From<ConceptRow> for ConceptCandidate {
    fn from(row: ConceptRow) -> Self {
        Self {
            id: row.id,
            framework_id: row.framework_id,
            parent_id: row.parent_id,
            name_en: row.name_en,
            definition_en: row.definition_en,
            code: row.code,
            source_reference: row.source_reference,
            concept_type: row.concept_type,
        }
    }
}

/// Retrieve candidate concepts from the database using FTS5 and exact matching.
///
/// Returns all matched concepts plus gap candidates (unmatched concepts from
/// detected frameworks) for comprehensive gap analysis.
pub async fn retrieve_candidates(
    keywords: &[String],
    framework_ids: &[String],
    db: &SqlitePool,
) -> Result<Vec<ConceptCandidate>, sqlx::Error> {
    if framework_ids.is_empty() {
        return Ok(Vec::new());
    }

    let fw_json = serde_json::to_string(framework_ids).unwrap_or_else(|_| "[]".into());
    let sanitized = sanitize_fts_keywords(keywords);
    let capped: Vec<&String> = sanitized.iter().take(20).collect();

    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut candidates: Vec<ConceptCandidate> = Vec::new();

    // Step 1: FTS5 MATCH query
    if !capped.is_empty() {
        let match_expr = capped.iter().map(|k| k.as_str()).collect::<Vec<_>>().join(" OR ");
        let fts_rows: Vec<ConceptRow> = sqlx::query_as(
            r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                      COALESCE(c.definition_en, '') as definition_en,
                      c.code, c.source_reference, c.concept_type
               FROM concepts c
               JOIN concepts_fts ON concepts_fts.rowid = c.rowid
               WHERE concepts_fts MATCH ?1
               AND c.framework_id IN (SELECT value FROM json_each(?2))"#,
        )
        .bind(&match_expr)
        .bind(&fw_json)
        .fetch_all(db)
        .await
        .unwrap_or_else(|e| {
            warn!("FTS5 MATCH query failed, continuing with exact matches: {e}");
            Vec::new()
        });

        for row in fts_rows {
            let candidate: ConceptCandidate = row.into();
            if seen_ids.insert(candidate.id.clone()) {
                candidates.push(candidate);
            }
        }
    }

    // Step 2: Exact match on name_en and code (keywords > 4 chars)
    for kw in capped.iter().filter(|k| k.len() > 4) {
        let kw_lower = escape_like(&kw.to_lowercase());
        let exact_rows: Vec<ConceptRow> = sqlx::query_as(
            r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                      COALESCE(c.definition_en, '') as definition_en,
                      c.code, c.source_reference, c.concept_type
               FROM concepts c
               WHERE c.framework_id IN (SELECT value FROM json_each(?1))
               AND (LOWER(c.name_en) LIKE '%' || ?2 || '%' ESCAPE '\'
                    OR LOWER(c.code) LIKE '%' || ?2 || '%' ESCAPE '\')"#,
        )
        .bind(&fw_json)
        .bind(&kw_lower)
        .fetch_all(db)
        .await?;

        for row in exact_rows {
            let candidate: ConceptCandidate = row.into();
            if seen_ids.insert(candidate.id.clone()) {
                candidates.push(candidate);
            }
        }
    }

    // Step 3: Load gap candidates (concepts not yet matched)
    let matched_json = serde_json::to_string(&seen_ids.iter().collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    let gap_rows: Vec<ConceptRow> = sqlx::query_as(
        r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                  COALESCE(c.definition_en, '') as definition_en,
                  c.code, c.source_reference, c.concept_type
           FROM concepts c
           WHERE c.framework_id IN (SELECT value FROM json_each(?1))
           AND c.id NOT IN (SELECT value FROM json_each(?2))"#,
    )
    .bind(&fw_json)
    .bind(&matched_json)
    .fetch_all(db)
    .await?;

    for row in gap_rows {
        candidates.push(row.into());
    }

    Ok(candidates)
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

    // ========================================================================
    // Section 03: FTS5 Candidate Retrieval Tests
    // ========================================================================

    #[test]
    fn test_sanitize_fts_keywords_removes_special_chars() {
        let input: Vec<String> = vec![
            "risk*".into(),
            "\"assessment\"".into(),
            "(control)".into(),
            "normal".into(),
            "***".into(),  // discarded entirely
            "NOT".into(),  // FTS5 reserved word
            "near".into(), // FTS5 reserved word (case-insensitive)
        ];
        let result = sanitize_fts_keywords(&input);
        assert_eq!(result, vec!["risk", "assessment", "control", "normal"]);
    }

    #[test]
    fn test_sanitize_fts_keywords_empty_input() {
        let result = sanitize_fts_keywords(&[]);
        assert!(result.is_empty());
    }

    /// Helper to create an in-memory SQLite database with schema and test data.
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        // Create minimal schema (frameworks, concepts, FTS5)
        sqlx::raw_sql(
            r#"
            CREATE TABLE frameworks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE concepts (
                id TEXT PRIMARY KEY,
                framework_id TEXT NOT NULL REFERENCES frameworks(id),
                parent_id TEXT,
                concept_type TEXT NOT NULL,
                code TEXT,
                name_en TEXT NOT NULL,
                name_nb TEXT,
                definition_en TEXT,
                definition_nb TEXT,
                source_reference TEXT,
                sort_order INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );
            CREATE VIRTUAL TABLE concepts_fts USING fts5(
                name_en, name_nb, definition_en, definition_nb,
                content='concepts', content_rowid='rowid'
            );
            CREATE TRIGGER concepts_ai AFTER INSERT ON concepts BEGIN
                INSERT INTO concepts_fts(rowid, name_en, name_nb, definition_en, definition_nb)
                VALUES (NEW.rowid, NEW.name_en, NEW.name_nb, NEW.definition_en, NEW.definition_nb);
            END;
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        // Insert test data
        sqlx::raw_sql(
            r#"
            INSERT INTO frameworks (id, name) VALUES ('nist-csf', 'NIST Cybersecurity Framework');
            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
                VALUES ('nist-csf-id', 'nist-csf', NULL, 'function', 'ID', 'Identify', 'Develop understanding of cybersecurity risk');
            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
                VALUES ('nist-csf-pr', 'nist-csf', NULL, 'function', 'PR', 'Protect', 'Implement safeguards for critical services');
            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
                VALUES ('nist-csf-de', 'nist-csf', NULL, 'function', 'DE', 'Detect', 'Identify cybersecurity events');
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_retrieve_candidates_returns_fts_matches() {
        let pool = setup_test_db().await;
        let keywords: Vec<String> = vec!["identify".into(), "cybersecurity".into(), "risk".into()];
        let framework_ids: Vec<String> = vec!["nist-csf".into()];

        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
        let ids: Vec<&str> = result.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"nist-csf-id"), "Should find 'Identify' concept via FTS5");
    }

    #[tokio::test]
    async fn test_retrieve_candidates_includes_gap_candidates() {
        let pool = setup_test_db().await;
        // Only "identify" matches — "Protect" and "Detect" should appear as gap candidates
        let keywords: Vec<String> = vec!["identify".into()];
        let framework_ids: Vec<String> = vec!["nist-csf".into()];

        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
        assert_eq!(result.len(), 3, "Should include 1 FTS match + 2 gap candidates");
        let ids: Vec<&str> = result.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"nist-csf-id"));
        assert!(ids.contains(&"nist-csf-pr"));
        assert!(ids.contains(&"nist-csf-de"));
    }

    #[tokio::test]
    async fn test_retrieve_candidates_deduplicates() {
        let pool = setup_test_db().await;
        // "identify" matches via FTS5 AND exact name match (>4 chars)
        let keywords: Vec<String> = vec!["identify".into()];
        let framework_ids: Vec<String> = vec!["nist-csf".into()];

        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
        let id_count = result.iter().filter(|c| c.id == "nist-csf-id").count();
        assert_eq!(id_count, 1, "Concept should appear exactly once despite FTS5 + exact match");
    }
}
