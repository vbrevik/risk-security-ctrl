use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::warn;

use super::engine::NewFinding;
use super::models::FindingType;
use super::tokenizer::{extract_keywords, sentence_split};

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

// ============================================================================
// TF-IDF Scoring
// ============================================================================

/// Score candidates against document keywords using TF-IDF with boost terms.
///
/// Each candidate gets a confidence_score in [0.0, 1.0] based on keyword
/// overlap with the document. Gap candidates (zero overlap) score 0.0.
pub fn score_candidates(
    candidates: &[ConceptCandidate],
    doc_keywords: &[String],
    doc_term_freq: &HashMap<String, usize>,
    config: &MatcherConfig,
) -> Vec<ScoredCandidate> {
    if candidates.is_empty() {
        return Vec::new();
    }

    let doc_kw_set: HashSet<&str> = doc_keywords.iter().map(|s| s.as_str()).collect();

    // Precompute: extract keywords for each candidate
    let candidate_keywords: Vec<Vec<String>> = candidates
        .iter()
        .map(|c| {
            let text = format!("{} {}", c.name_en, c.definition_en);
            extract_keywords(&text)
        })
        .collect();

    // Precompute document frequency (df): how many candidates contain each keyword
    let total_candidates = candidates.len() as f64;
    let mut df: HashMap<&str, usize> = HashMap::new();
    for kws in &candidate_keywords {
        let unique: HashSet<&str> = kws.iter().map(|s| s.as_str()).collect();
        for kw in unique {
            *df.entry(kw).or_insert(0) += 1;
        }
    }

    // Compute IDF for each keyword
    // Floor: when total_candidates == df (all candidates contain the term), IDF would be
    // ln(1) = 0. Use max(ln(...), 0.1) to ensure single-candidate sets still produce scores.
    let idf: HashMap<&str, f64> = df
        .iter()
        .map(|(&kw, &count)| {
            let raw_idf = (total_candidates / count as f64).ln();
            (kw, raw_idf.max(0.1))
        })
        .collect();

    // Max boost value for normalization
    let max_boost = config
        .boost_terms
        .values()
        .copied()
        .fold(1.0_f64, f64::max);

    // Max TF value for normalization
    let max_tf = doc_term_freq
        .values()
        .copied()
        .max()
        .unwrap_or(1) as f64;

    // Score each candidate
    candidates
        .iter()
        .zip(candidate_keywords.iter())
        .map(|(candidate, concept_kws)| {
            let overlapping: Vec<&str> = concept_kws
                .iter()
                .filter(|kw| doc_kw_set.contains(kw.as_str()))
                .map(|s| s.as_str())
                .collect();

            let raw_score: f64 = overlapping
                .iter()
                .map(|&kw| {
                    let tf = *doc_term_freq.get(kw).unwrap_or(&1) as f64;
                    let kw_idf = idf.get(kw).copied().unwrap_or(0.0);
                    let boost = config.boost_terms.get(kw).copied().unwrap_or(1.0);
                    tf * kw_idf * boost
                })
                .sum();

            // Max possible score: sum over ALL concept keywords
            let max_possible: f64 = concept_kws
                .iter()
                .map(|kw| {
                    let kw_idf = idf.get(kw.as_str()).copied().unwrap_or(0.0);
                    max_tf * kw_idf * max_boost
                })
                .sum();

            let normalized = if max_possible > 0.0 {
                (raw_score / max_possible).clamp(0.0, 1.0)
            } else {
                0.0
            };

            ScoredCandidate {
                candidate: candidate.clone(),
                confidence_score: normalized,
            }
        })
        .collect()
}

// ============================================================================
// Gap Classification and Findings
// ============================================================================

/// Classify scored candidates into findings with threshold-based types,
/// priority rankings, recommendation text, and evidence extraction.
pub fn classify_findings(
    scored_candidates: Vec<ScoredCandidate>,
    config: &MatcherConfig,
    document_text: &str,
) -> Vec<NewFinding> {
    // Pre-compute sentence keywords for evidence extraction
    let sentences = sentence_split(document_text);
    let sentence_kw_sets: Vec<(&str, HashSet<String>)> = sentences
        .iter()
        .map(|s| {
            let kws: HashSet<String> = extract_keywords(s).into_iter().collect();
            (s.as_str(), kws)
        })
        .collect();

    // Step 1-3: Classify, filter, and collect
    let mut findings: Vec<(ScoredCandidate, FindingType)> = Vec::new();

    for sc in scored_candidates {
        // Step 1: Classify by threshold
        let finding_type = if sc.confidence_score >= config.addressed_threshold {
            FindingType::Addressed
        } else if sc.confidence_score >= config.partial_threshold {
            FindingType::PartiallyAddressed
        } else {
            FindingType::Gap
        };

        // Step 2: Filter by minimum confidence (keep zero-score gaps)
        if sc.confidence_score < config.min_confidence_threshold
            && !(sc.confidence_score == 0.0 && finding_type == FindingType::Gap)
        {
            continue;
        }

        // Step 3: Filter addressed findings if configured
        if !config.include_addressed_findings && finding_type == FindingType::Addressed {
            continue;
        }

        findings.push((sc, finding_type));
    }

    // Step 4: Cap per framework (gaps prioritized within cap)
    let mut grouped: HashMap<String, Vec<(ScoredCandidate, FindingType)>> = HashMap::new();
    for (sc, ft) in findings {
        grouped
            .entry(sc.candidate.framework_id.clone())
            .or_default()
            .push((sc, ft));
    }

    let mut capped: Vec<(ScoredCandidate, FindingType)> = Vec::new();
    for (_fw_id, mut group) in grouped {
        // Sort: gaps first (score 0.0), then by score descending
        group.sort_by(|a, b| {
            let a_is_gap = (a.1 == FindingType::Gap && a.0.confidence_score == 0.0) as u8;
            let b_is_gap = (b.1 == FindingType::Gap && b.0.confidence_score == 0.0) as u8;
            b_is_gap
                .cmp(&a_is_gap)
                .then(b.0.confidence_score.partial_cmp(&a.0.confidence_score).unwrap_or(std::cmp::Ordering::Equal))
        });
        capped.extend(group.into_iter().take(config.max_findings_per_framework));
    }

    // Steps 5-7: Assign priority, generate recommendation, extract evidence
    capped
        .into_iter()
        .map(|(sc, finding_type)| {
            // Step 5: Priority
            let priority = match finding_type {
                FindingType::Addressed => 4,
                FindingType::Gap => {
                    if sc.candidate.parent_id.is_none() { 1 } else { 2 }
                }
                FindingType::PartiallyAddressed => {
                    if sc.candidate.parent_id.is_none() { 2 } else { 3 }
                }
                FindingType::NotApplicable => 4,
            };

            // Step 6: Recommendation
            let def_excerpt: String = sc.candidate.definition_en.chars().take(100).collect();
            let ref_clause = sc
                .candidate
                .source_reference
                .as_deref()
                .map(|r| format!(". Reference: {r}"))
                .unwrap_or_default();

            let recommendation = match finding_type {
                FindingType::Addressed => {
                    Some(format!("Document adequately covers {}{ref_clause}", sc.candidate.name_en))
                }
                FindingType::PartiallyAddressed => {
                    Some(format!(
                        "Document partially addresses {}. Consider expanding coverage of {def_excerpt}{ref_clause}",
                        sc.candidate.name_en
                    ))
                }
                FindingType::Gap => {
                    let action = sc
                        .candidate
                        .source_reference
                        .as_deref()
                        .map(|r| format!("review and implement controls per {r}."))
                        .unwrap_or_else(|| "review and implement appropriate controls.".into());
                    Some(format!(
                        "Document does not address {}: {def_excerpt}. Recommended action: {action}",
                        sc.candidate.name_en
                    ))
                }
                FindingType::NotApplicable => None,
            };

            // Step 7: Evidence extraction (non-gap only)
            let evidence_text = if finding_type != FindingType::Gap {
                let concept_kws: HashSet<String> = extract_keywords(
                    &format!("{} {}", sc.candidate.name_en, sc.candidate.definition_en),
                )
                .into_iter()
                .collect();

                sentence_kw_sets
                    .iter()
                    .max_by_key(|(_, s_kws)| s_kws.intersection(&concept_kws).count())
                    .and_then(|(sentence, s_kws)| {
                        let overlap = s_kws.intersection(&concept_kws).count();
                        if overlap > 0 {
                            Some(sentence.to_string())
                        } else {
                            None
                        }
                    })
            } else {
                None
            };

            NewFinding {
                concept_id: sc.candidate.id,
                framework_id: sc.candidate.framework_id,
                finding_type,
                confidence_score: sc.confidence_score,
                evidence_text,
                recommendation,
                priority,
            }
        })
        .collect()
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

    // ========================================================================
    // Section 04: TF-IDF Scoring Tests
    // ========================================================================

    fn make_candidate(id: &str, name: &str, definition: &str) -> ConceptCandidate {
        ConceptCandidate {
            id: id.into(),
            framework_id: "fw-1".into(),
            parent_id: None,
            name_en: name.into(),
            definition_en: definition.into(),
            code: None,
            source_reference: None,
            concept_type: "concept".into(),
        }
    }

    #[test]
    fn score_candidates_high_overlap_scores_high() {
        let c1 = make_candidate("c1", "Risk Assessment", "Process of identifying and evaluating risks");
        let c2 = make_candidate("c2", "Quantum Physics", "Study of subatomic particles and waves");
        let candidates = vec![c1, c2];

        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "identifying".into(), "evaluating".into(), "risks".into()];
        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 3)).collect();
        let config = MatcherConfig::default();

        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
        assert_eq!(scored.len(), 2);
        assert!(scored[0].confidence_score > 0.5, "High-overlap candidate should score > 0.5, got {}", scored[0].confidence_score);
        assert!(scored[0].confidence_score > scored[1].confidence_score, "Matching candidate should score higher");
    }

    #[test]
    fn score_candidates_no_overlap_scores_zero() {
        // Use 2 candidates so IDF is nonzero — tests true disjointness, not IDF=0
        let c1 = make_candidate("c1", "Quantum Entanglement", "Photon wave particle duality");
        let c2 = make_candidate("c2", "Risk Management", "Process of identifying risks");
        let candidates = vec![c1, c2];

        let doc_keywords: Vec<String> = vec!["risk".into(), "identifying".into(), "management".into()];
        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 1)).collect();
        let config = MatcherConfig::default();

        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
        let quantum = scored.iter().find(|s| s.candidate.id == "c1").unwrap();
        assert!((quantum.confidence_score - 0.0).abs() < f64::EPSILON,
            "No-overlap candidate should score 0.0, got {}", quantum.confidence_score);
    }

    #[test]
    fn score_candidates_boost_terms_increase_score() {
        // Both candidates have exactly one keyword that overlaps with doc_keywords
        // c1 overlaps on "security" (boosted 1.5x), c2 overlaps on "banana" (no boost)
        // Single-keyword concepts avoid normalization skew from max_boost denominator
        let c1 = make_candidate("c1", "Security", "");
        let c2 = make_candidate("c2", "Banana", "");
        let candidates = vec![c1, c2];

        let doc_keywords: Vec<String> = vec!["security".into(), "banana".into()];
        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 1)).collect();
        let config = MatcherConfig::default(); // security=1.5 boost

        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
        let s1 = scored.iter().find(|s| s.candidate.id == "c1").unwrap();
        let s2 = scored.iter().find(|s| s.candidate.id == "c2").unwrap();
        assert!(s1.confidence_score > s2.confidence_score,
            "Boosted 'security' candidate ({}) should score higher than unboosted 'banana' ({})",
            s1.confidence_score, s2.confidence_score
        );
    }

    #[test]
    fn score_candidates_all_scores_in_valid_range() {
        let candidates = vec![
            make_candidate("c1", "Risk Assessment", "Evaluating risk likelihood"),
            make_candidate("c2", "Access Control", "Managing user permissions"),
            make_candidate("c3", "Incident Response", "Handling security incidents"),
            make_candidate("c4", "Unrelated Topic", "Something completely different"),
        ];

        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "security".into(), "control".into(), "access".into()];
        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 2)).collect();
        let config = MatcherConfig::default();

        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
        for sc in &scored {
            assert!(sc.confidence_score >= 0.0 && sc.confidence_score <= 1.0,
                "Score {} for {} is outside [0.0, 1.0]", sc.confidence_score, sc.candidate.id);
        }
    }

    // ========================================================================
    // Section 05: Classification Tests
    // ========================================================================

    fn make_scored(id: &str, fw: &str, parent: Option<&str>, name: &str, def: &str, src: Option<&str>, score: f64) -> ScoredCandidate {
        ScoredCandidate {
            candidate: ConceptCandidate {
                id: id.into(),
                framework_id: fw.into(),
                parent_id: parent.map(String::from),
                name_en: name.into(),
                definition_en: def.into(),
                code: None,
                source_reference: src.map(String::from),
                concept_type: "concept".into(),
            },
            confidence_score: score,
        }
    }

    #[test]
    fn classify_score_08_produces_addressed() {
        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.8);
        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
        assert_eq!(findings[0].finding_type, FindingType::Addressed);
    }

    #[test]
    fn classify_score_04_produces_partially_addressed() {
        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.4);
        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
        assert_eq!(findings[0].finding_type, FindingType::PartiallyAddressed);
    }

    #[test]
    fn classify_score_01_produces_gap() {
        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.1);
        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
        assert_eq!(findings[0].finding_type, FindingType::Gap);
    }

    #[test]
    fn classify_score_00_produces_gap() {
        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.0);
        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
        assert_eq!(findings.len(), 1, "Zero-score gap should be retained");
        assert_eq!(findings[0].finding_type, FindingType::Gap);
    }

    #[test]
    fn classify_priority_root_gap_p1_child_gap_p2() {
        let root = make_scored("c1", "fw", None, "Root", "Def", None, 0.0);
        let child = make_scored("c2", "fw", Some("c1"), "Child", "Def", None, 0.0);
        let findings = classify_findings(vec![root, child], &MatcherConfig::default(), "");
        let root_f = findings.iter().find(|f| f.concept_id == "c1").unwrap();
        let child_f = findings.iter().find(|f| f.concept_id == "c2").unwrap();
        assert_eq!(root_f.priority, 1);
        assert_eq!(child_f.priority, 2);
    }

    #[test]
    fn classify_recommendation_contains_name_and_reference() {
        let sc = make_scored("c1", "fw", None, "Access Control", "Managing access", Some("NIST SP 800-53 AC-1"), 0.0);
        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
        let rec = findings[0].recommendation.as_ref().unwrap();
        assert!(rec.contains("Access Control"), "Recommendation should contain concept name");
        assert!(rec.contains("NIST SP 800-53 AC-1"), "Recommendation should contain source reference");
    }

    #[test]
    fn classify_max_findings_per_framework_caps_output() {
        let candidates: Vec<ScoredCandidate> = (0..60)
            .map(|i| make_scored(&format!("c{i}"), "fw", None, "Concept", "Def", None, 0.0))
            .collect();
        let findings = classify_findings(candidates, &MatcherConfig::default(), "");
        assert!(findings.len() <= 50, "Should cap at max_findings_per_framework=50, got {}", findings.len());
    }

    #[test]
    fn classify_exclude_addressed_findings() {
        let candidates = vec![
            make_scored("c1", "fw", None, "High", "Def", None, 0.8),
            make_scored("c2", "fw", None, "Medium", "Def", None, 0.4),
            make_scored("c3", "fw", None, "Low", "Def", None, 0.1),
        ];
        let mut config = MatcherConfig::default();
        config.include_addressed_findings = false;
        let findings = classify_findings(candidates, &config, "");
        assert!(findings.iter().all(|f| f.finding_type != FindingType::Addressed),
            "No Addressed findings should be present");
        assert_eq!(findings.len(), 2);
    }
}
