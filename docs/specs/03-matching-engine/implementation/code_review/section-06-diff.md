diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index 3c0e228..a51f5d1 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -1,12 +1,14 @@
 use std::collections::{HashMap, HashSet};
+use std::time::Instant;
 
+use async_trait::async_trait;
 use serde::{Deserialize, Serialize};
 use sqlx::SqlitePool;
 use tracing::warn;
 
-use super::engine::NewFinding;
+use super::engine::{AnalysisError, MatchingEngine, MatchingResult, NewFinding};
 use super::models::FindingType;
-use super::tokenizer::{extract_keywords, sentence_split};
+use super::tokenizer::{extract_keywords, sentence_split, term_frequency};
 
 // ============================================================================
 // Configuration Types
@@ -635,6 +637,99 @@ pub fn classify_findings(
         .collect()
 }
 
+// ============================================================================
+// DeterministicMatcher
+// ============================================================================
+
+/// Deterministic matching engine using FTS5 + TF-IDF scoring.
+///
+/// Orchestrates the full analysis pipeline: framework detection, candidate
+/// retrieval, scoring, classification, and reference validation.
+pub struct DeterministicMatcher {
+    topics: Vec<Topic>,
+}
+
+impl DeterministicMatcher {
+    pub fn new(topics: Vec<Topic>) -> Self {
+        Self { topics }
+    }
+}
+
+#[async_trait]
+impl MatchingEngine for DeterministicMatcher {
+    async fn analyze(
+        &self,
+        text: &str,
+        prompt_template: Option<&str>,
+        db: &SqlitePool,
+    ) -> Result<MatchingResult, AnalysisError> {
+        let start = Instant::now();
+
+        // Stage 1: Parse config
+        let config = MatcherConfig::from_json(prompt_template);
+
+        // Stage 2: Extract keywords and term frequencies
+        let doc_keywords = extract_keywords(text);
+        let doc_tf = term_frequency(text);
+
+        // Stage 3: Load frameworks and detect relevant ones
+        let frameworks: Vec<(String, String)> = sqlx::query_as(
+            "SELECT id, name FROM frameworks",
+        )
+        .fetch_all(db)
+        .await?;
+
+        let framework_ids = detect_frameworks(&doc_keywords, &self.topics, &frameworks, &config);
+        if framework_ids.is_empty() {
+            return Err(AnalysisError::NoFrameworksDetected);
+        }
+
+        // Stage 4: Retrieve candidates via FTS5
+        let candidates = retrieve_candidates(&doc_keywords, &framework_ids, db).await?;
+
+        // Stage 5: Score candidates
+        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
+
+        // Stage 6: Classify into findings
+        let findings = classify_findings(scored, &config, text);
+
+        // Stage 7: Reference validation (batch query)
+        let finding_ids: Vec<&str> = findings.iter().map(|f| f.concept_id.as_str()).collect();
+        let ids_json = serde_json::to_string(&finding_ids).unwrap_or_else(|_| "[]".into());
+        let valid_ids: HashSet<String> = sqlx::query_scalar::<_, String>(
+            "SELECT id FROM concepts WHERE id IN (SELECT value FROM json_each(?1))",
+        )
+        .bind(&ids_json)
+        .fetch_all(db)
+        .await?
+        .into_iter()
+        .collect();
+
+        let validated_findings: Vec<NewFinding> = findings
+            .into_iter()
+            .filter(|f| {
+                if valid_ids.contains(&f.concept_id) {
+                    true
+                } else {
+                    warn!("Dropping finding with non-existent concept_id: {}", f.concept_id);
+                    false
+                }
+            })
+            .collect();
+
+        // Stage 8: Build result
+        let processing_time_ms = start.elapsed().as_millis() as i64;
+        let token_count = (text.split_whitespace().count() as f64 * 1.33) as i64;
+
+        Ok(MatchingResult {
+            matched_framework_ids: framework_ids,
+            findings: validated_findings,
+            processing_time_ms,
+            token_count,
+        })
+    }
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -1119,4 +1214,60 @@ mod tests {
             "No Addressed findings should be present");
         assert_eq!(findings.len(), 2);
     }
+
+    // ========================================================================
+    // Section 06: DeterministicMatcher Integration Tests
+    // ========================================================================
+
+    fn sample_topics() -> Vec<Topic> {
+        vec![
+            make_topic("risk-mgmt", "Risk Management and Assessment", vec!["nist-csf-id"]),
+            make_topic("security", "Security Controls and Protection", vec!["nist-csf-pr"]),
+        ]
+    }
+
+    #[tokio::test]
+    async fn test_analyze_security_text_returns_findings() {
+        let db = setup_test_db().await;
+        let matcher = DeterministicMatcher::new(sample_topics());
+        let text = "Our organization implements risk assessment procedures and security controls for identifying threats and vulnerabilities.";
+        let result = matcher.analyze(text, None, &db).await.unwrap();
+        assert!(!result.findings.is_empty(), "Should produce findings for security-related text");
+        assert!(!result.matched_framework_ids.is_empty(), "Should detect at least one framework");
+    }
+
+    #[tokio::test]
+    async fn test_analyze_irrelevant_text_returns_no_frameworks_error() {
+        let db = setup_test_db().await;
+        let matcher = DeterministicMatcher::new(sample_topics());
+        let text = "The weather today is sunny with a chance of rain in the afternoon.";
+        let result = matcher.analyze(text, None, &db).await;
+        assert!(matches!(result, Err(AnalysisError::NoFrameworksDetected)));
+    }
+
+    #[tokio::test]
+    async fn test_analyze_validates_references() {
+        let db = setup_test_db().await;
+        let matcher = DeterministicMatcher::new(sample_topics());
+        let text = "Risk management and security controls are critical for compliance.";
+        let result = matcher.analyze(text, None, &db).await.unwrap();
+        for finding in &result.findings {
+            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concepts WHERE id = ?")
+                .bind(&finding.concept_id)
+                .fetch_one(&db)
+                .await
+                .unwrap();
+            assert!(count.0 > 0, "Finding references non-existent concept: {}", finding.concept_id);
+        }
+    }
+
+    #[tokio::test]
+    async fn test_analyze_returns_timing_and_token_count() {
+        let db = setup_test_db().await;
+        let matcher = DeterministicMatcher::new(sample_topics());
+        let text = "Security risk assessment identifies threats and vulnerabilities in systems.";
+        let result = matcher.analyze(text, None, &db).await.unwrap();
+        assert!(result.processing_time_ms >= 0);
+        assert!(result.token_count > 0);
+    }
 }
