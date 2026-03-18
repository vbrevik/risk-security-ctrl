diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index e517401..d380d57 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -1,8 +1,10 @@
-use std::collections::HashMap;
+use std::collections::{HashMap, HashSet};
 
 use serde::{Deserialize, Serialize};
 use tracing::warn;
 
+use super::tokenizer::extract_keywords;
+
 // ============================================================================
 // Configuration Types
 // ============================================================================
@@ -131,6 +133,76 @@ pub struct ScoredCandidate {
     pub confidence_score: f64,
 }
 
+// ============================================================================
+// Framework Detection
+// ============================================================================
+
+/// Detect which frameworks are relevant to a document based on keyword matching.
+///
+/// Uses a two-pronged approach:
+/// 1. Topic matching: overlap between document keywords and topic name tokens,
+///    then map matched topics to their concept_ids to identify frameworks.
+/// 2. Direct name matching: check if document keywords contain framework names
+///    or common abbreviations (e.g., "nist", "iso", "gdpr").
+///
+/// Returns framework IDs ordered by match strength (highest first).
+pub fn detect_frameworks(
+    doc_keywords: &[String],
+    topics: &[Topic],
+    frameworks: Vec<(String, String)>, // (id, name) pairs
+    _config: &MatcherConfig,
+) -> Vec<String> {
+    let doc_kw_set: HashSet<&str> = doc_keywords.iter().map(|s| s.as_str()).collect();
+
+    // Step 1: Topic matching — find topics whose name tokens overlap with doc keywords
+    let mut matched_concept_ids: Vec<&str> = Vec::new();
+    for topic in topics {
+        let topic_tokens = extract_keywords(&topic.name_en);
+        let overlap = topic_tokens
+            .iter()
+            .filter(|t| doc_kw_set.contains(t.as_str()))
+            .count();
+        if overlap > 0 {
+            for cid in &topic.concept_ids {
+                matched_concept_ids.push(cid.as_str());
+            }
+        }
+    }
+
+    // Step 2: Score each framework
+    let mut scores: HashMap<&str, f64> = HashMap::new();
+
+    for (fw_id, fw_name) in &frameworks {
+        let mut score = 0.0_f64;
+
+        // Topic-based score: count concept_ids that belong to this framework
+        let topic_count = matched_concept_ids
+            .iter()
+            .filter(|cid| cid.starts_with(fw_id.as_str()))
+            .count();
+        score += topic_count as f64;
+
+        // Direct name match: tokenize framework name, check overlap with doc keywords
+        let fw_tokens = extract_keywords(fw_name);
+        let name_overlap = fw_tokens
+            .iter()
+            .filter(|t| doc_kw_set.contains(t.as_str()))
+            .count();
+        if name_overlap > 0 {
+            score += 2.0;
+        }
+
+        if score > 0.0 {
+            scores.insert(fw_id.as_str(), score);
+        }
+    }
+
+    // Step 3: Sort by score descending and return IDs
+    let mut ranked: Vec<(&str, f64)> = scores.into_iter().collect();
+    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
+    ranked.into_iter().map(|(id, _)| id.to_string()).collect()
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -229,4 +301,83 @@ mod tests {
         assert!((sc.confidence_score - 0.85).abs() < f64::EPSILON);
         assert_eq!(sc.candidate.parent_id, Some("parent-1".into()));
     }
+
+    // ========================================================================
+    // Section 02: Framework Detection Tests
+    // ========================================================================
+
+    fn make_topic(id: &str, name: &str, concept_ids: Vec<&str>) -> Topic {
+        Topic {
+            id: id.into(),
+            name_en: name.into(),
+            concept_ids: concept_ids.into_iter().map(String::from).collect(),
+        }
+    }
+
+    #[test]
+    fn test_detect_frameworks_risk_keywords_match_iso31000() {
+        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "management".into()];
+        let topics = vec![make_topic(
+            "risk-mgmt",
+            "Risk Management and Assessment",
+            vec!["iso31000-risk", "iso31000-assessment"],
+        )];
+        let frameworks = vec![
+            ("iso31000".into(), "ISO 31000".into()),
+            ("nist-csf".into(), "NIST Cybersecurity Framework".into()),
+        ];
+        let config = MatcherConfig::default();
+
+        let result = detect_frameworks(&doc_keywords, &topics, frameworks, &config);
+        assert!(result.contains(&"iso31000".to_string()));
+    }
+
+    #[test]
+    fn test_detect_frameworks_direct_name_match_nist() {
+        let doc_keywords: Vec<String> = vec!["nist".into(), "cybersecurity".into(), "framework".into()];
+        let topics: Vec<Topic> = vec![];
+        let frameworks = vec![("nist-csf".into(), "NIST Cybersecurity Framework".into())];
+        let config = MatcherConfig::default();
+
+        let result = detect_frameworks(&doc_keywords, &topics, frameworks, &config);
+        assert!(result.contains(&"nist-csf".to_string()));
+    }
+
+    #[test]
+    fn test_detect_frameworks_unrelated_keywords_empty() {
+        let doc_keywords: Vec<String> = vec!["banana".into(), "tropical".into(), "fruit".into()];
+        let topics = vec![make_topic(
+            "risk-mgmt",
+            "Risk Management",
+            vec!["iso31000-risk"],
+        )];
+        let frameworks = vec![("iso31000".into(), "ISO 31000".into())];
+        let config = MatcherConfig::default();
+
+        let result = detect_frameworks(&doc_keywords, &topics, frameworks, &config);
+        assert!(result.is_empty());
+    }
+
+    #[test]
+    fn test_detect_frameworks_ordered_by_strength() {
+        // "risk" and "assessment" match the risk topic strongly (2 concept_ids for iso31000)
+        // "nist" matches NIST by direct name only (no topic concepts)
+        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "nist".into()];
+        let topics = vec![make_topic(
+            "risk-mgmt",
+            "Risk Management and Assessment",
+            vec!["iso31000-risk", "iso31000-assessment"],
+        )];
+        let frameworks = vec![
+            ("nist-csf".into(), "NIST Cybersecurity Framework".into()),
+            ("iso31000".into(), "ISO 31000".into()),
+        ];
+        let config = MatcherConfig::default();
+
+        let result = detect_frameworks(&doc_keywords, &topics, frameworks, &config);
+        assert_eq!(result.len(), 2);
+        // iso31000 should rank first (2 concept matches + possible name match) vs nist-csf (name match only)
+        assert_eq!(result[0], "iso31000");
+        assert_eq!(result[1], "nist-csf");
+    }
 }
