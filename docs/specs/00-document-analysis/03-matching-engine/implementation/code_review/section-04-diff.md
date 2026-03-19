diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index 363ec32..ad6e1e2 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -368,6 +368,109 @@ pub async fn retrieve_candidates(
     Ok(candidates)
 }
 
+// ============================================================================
+// TF-IDF Scoring
+// ============================================================================
+
+/// Score candidates against document keywords using TF-IDF with boost terms.
+///
+/// Each candidate gets a confidence_score in [0.0, 1.0] based on keyword
+/// overlap with the document. Gap candidates (zero overlap) score 0.0.
+pub fn score_candidates(
+    candidates: &[ConceptCandidate],
+    doc_keywords: &[String],
+    doc_term_freq: &HashMap<String, usize>,
+    config: &MatcherConfig,
+) -> Vec<ScoredCandidate> {
+    if candidates.is_empty() {
+        return Vec::new();
+    }
+
+    let doc_kw_set: HashSet<&str> = doc_keywords.iter().map(|s| s.as_str()).collect();
+
+    // Precompute: extract keywords for each candidate
+    let candidate_keywords: Vec<Vec<String>> = candidates
+        .iter()
+        .map(|c| {
+            let text = format!("{} {}", c.name_en, c.definition_en);
+            extract_keywords(&text)
+        })
+        .collect();
+
+    // Precompute document frequency (df): how many candidates contain each keyword
+    let total_candidates = candidates.len() as f64;
+    let mut df: HashMap<&str, usize> = HashMap::new();
+    for kws in &candidate_keywords {
+        let unique: HashSet<&str> = kws.iter().map(|s| s.as_str()).collect();
+        for kw in unique {
+            *df.entry(kw).or_insert(0) += 1;
+        }
+    }
+
+    // Compute IDF for each keyword
+    let idf: HashMap<&str, f64> = df
+        .iter()
+        .map(|(&kw, &count)| (kw, (total_candidates / count as f64).ln()))
+        .collect();
+
+    // Max boost value for normalization
+    let max_boost = config
+        .boost_terms
+        .values()
+        .copied()
+        .fold(1.0_f64, f64::max);
+
+    // Max TF value for normalization
+    let max_tf = doc_term_freq
+        .values()
+        .copied()
+        .max()
+        .unwrap_or(1) as f64;
+
+    // Score each candidate
+    candidates
+        .iter()
+        .zip(candidate_keywords.iter())
+        .map(|(candidate, concept_kws)| {
+            let overlapping: Vec<&str> = concept_kws
+                .iter()
+                .filter(|kw| doc_kw_set.contains(kw.as_str()))
+                .map(|s| s.as_str())
+                .collect();
+
+            let raw_score: f64 = overlapping
+                .iter()
+                .map(|&kw| {
+                    let tf = *doc_term_freq.get(kw).unwrap_or(&1) as f64;
+                    let kw_idf = idf.get(kw).copied().unwrap_or(0.0);
+                    let boost = config.boost_terms.get(kw).copied().unwrap_or(1.0);
+                    tf * kw_idf * boost
+                })
+                .sum();
+
+            // Max possible score: sum over ALL concept keywords
+            let max_possible: f64 = concept_kws
+                .iter()
+                .map(|kw| {
+                    let kw_idf = idf.get(kw.as_str()).copied().unwrap_or(0.0);
+                    max_tf * kw_idf * max_boost
+                })
+                .sum();
+
+            let normalized = if max_possible > 0.0 {
+                (raw_score / max_possible).clamp(0.0, 1.0)
+            } else {
+                0.0
+            };
+
+            ScoredCandidate {
+                candidate: candidate.clone(),
+                confidence_score: normalized,
+            }
+        })
+        .collect()
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -667,4 +770,93 @@ mod tests {
         let id_count = result.iter().filter(|c| c.id == "nist-csf-id").count();
         assert_eq!(id_count, 1, "Concept should appear exactly once despite FTS5 + exact match");
     }
+
+    // ========================================================================
+    // Section 04: TF-IDF Scoring Tests
+    // ========================================================================
+
+    fn make_candidate(id: &str, name: &str, definition: &str) -> ConceptCandidate {
+        ConceptCandidate {
+            id: id.into(),
+            framework_id: "fw-1".into(),
+            parent_id: None,
+            name_en: name.into(),
+            definition_en: definition.into(),
+            code: None,
+            source_reference: None,
+            concept_type: "concept".into(),
+        }
+    }
+
+    #[test]
+    fn score_candidates_high_overlap_scores_high() {
+        let c1 = make_candidate("c1", "Risk Assessment", "Process of identifying and evaluating risks");
+        let c2 = make_candidate("c2", "Quantum Physics", "Study of subatomic particles and waves");
+        let candidates = vec![c1, c2];
+
+        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "identifying".into(), "evaluating".into(), "risks".into()];
+        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 3)).collect();
+        let config = MatcherConfig::default();
+
+        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
+        assert_eq!(scored.len(), 2);
+        assert!(scored[0].confidence_score > 0.5, "High-overlap candidate should score > 0.5, got {}", scored[0].confidence_score);
+        assert!(scored[0].confidence_score > scored[1].confidence_score, "Matching candidate should score higher");
+    }
+
+    #[test]
+    fn score_candidates_no_overlap_scores_zero() {
+        let c1 = make_candidate("c1", "Quantum Entanglement", "Photon wave particle duality");
+        let candidates = vec![c1];
+
+        let doc_keywords: Vec<String> = vec!["risk".into(), "security".into(), "compliance".into()];
+        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 1)).collect();
+        let config = MatcherConfig::default();
+
+        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
+        assert_eq!(scored.len(), 1);
+        assert!((scored[0].confidence_score - 0.0).abs() < f64::EPSILON, "No-overlap candidate should score 0.0");
+    }
+
+    #[test]
+    fn score_candidates_boost_terms_increase_score() {
+        // Both candidates have exactly one keyword that overlaps with doc_keywords
+        // c1 overlaps on "security" (boosted 1.5x), c2 overlaps on "banana" (no boost)
+        // Single-keyword concepts avoid normalization skew from max_boost denominator
+        let c1 = make_candidate("c1", "Security", "");
+        let c2 = make_candidate("c2", "Banana", "");
+        let candidates = vec![c1, c2];
+
+        let doc_keywords: Vec<String> = vec!["security".into(), "banana".into()];
+        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 1)).collect();
+        let config = MatcherConfig::default(); // security=1.5 boost
+
+        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
+        let s1 = scored.iter().find(|s| s.candidate.id == "c1").unwrap();
+        let s2 = scored.iter().find(|s| s.candidate.id == "c2").unwrap();
+        assert!(s1.confidence_score > s2.confidence_score,
+            "Boosted 'security' candidate ({}) should score higher than unboosted 'banana' ({})",
+            s1.confidence_score, s2.confidence_score
+        );
+    }
+
+    #[test]
+    fn score_candidates_all_scores_in_valid_range() {
+        let candidates = vec![
+            make_candidate("c1", "Risk Assessment", "Evaluating risk likelihood"),
+            make_candidate("c2", "Access Control", "Managing user permissions"),
+            make_candidate("c3", "Incident Response", "Handling security incidents"),
+            make_candidate("c4", "Unrelated Topic", "Something completely different"),
+        ];
+
+        let doc_keywords: Vec<String> = vec!["risk".into(), "assessment".into(), "security".into(), "control".into(), "access".into()];
+        let doc_tf: HashMap<String, usize> = doc_keywords.iter().map(|k| (k.clone(), 2)).collect();
+        let config = MatcherConfig::default();
+
+        let scored = score_candidates(&candidates, &doc_keywords, &doc_tf, &config);
+        for sc in &scored {
+            assert!(sc.confidence_score >= 0.0 && sc.confidence_score <= 1.0,
+                "Score {} for {} is outside [0.0, 1.0]", sc.confidence_score, sc.candidate.id);
+        }
+    }
 }
