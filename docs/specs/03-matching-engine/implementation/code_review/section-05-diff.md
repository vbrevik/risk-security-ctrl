diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index fd783ef..a77f12d 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -4,7 +4,9 @@ use serde::{Deserialize, Serialize};
 use sqlx::SqlitePool;
 use tracing::warn;
 
-use super::tokenizer::extract_keywords;
+use super::engine::NewFinding;
+use super::models::FindingType;
+use super::tokenizer::{extract_keywords, sentence_split};
 
 // ============================================================================
 // Configuration Types
@@ -476,6 +478,167 @@ pub fn score_candidates(
         .collect()
 }
 
+// ============================================================================
+// Gap Classification and Findings
+// ============================================================================
+
+/// Classify scored candidates into findings with threshold-based types,
+/// priority rankings, recommendation text, and evidence extraction.
+pub fn classify_findings(
+    scored_candidates: Vec<ScoredCandidate>,
+    config: &MatcherConfig,
+    document_text: &str,
+) -> Vec<NewFinding> {
+    // Pre-compute sentence keywords for evidence extraction
+    let sentences = sentence_split(document_text);
+    let sentence_kw_sets: Vec<(&str, HashSet<String>)> = sentences
+        .iter()
+        .map(|s| {
+            let kws: HashSet<String> = extract_keywords(s).into_iter().collect();
+            (s.as_str(), kws)
+        })
+        .collect();
+
+    // Step 1-3: Classify, filter, and collect
+    let mut findings: Vec<(ScoredCandidate, FindingType)> = Vec::new();
+
+    for sc in scored_candidates {
+        // Step 1: Classify by threshold
+        let finding_type = if sc.confidence_score >= config.addressed_threshold {
+            FindingType::Addressed
+        } else if sc.confidence_score >= config.partial_threshold {
+            FindingType::PartiallyAddressed
+        } else {
+            FindingType::Gap
+        };
+
+        // Step 2: Filter by minimum confidence (keep zero-score gaps)
+        if sc.confidence_score < config.min_confidence_threshold
+            && !(sc.confidence_score == 0.0 && finding_type == FindingType::Gap)
+        {
+            continue;
+        }
+
+        // Step 3: Filter addressed findings if configured
+        if !config.include_addressed_findings && finding_type == FindingType::Addressed {
+            continue;
+        }
+
+        findings.push((sc, finding_type));
+    }
+
+    // Step 4: Cap per framework (gaps prioritized within cap)
+    let mut grouped: HashMap<String, Vec<(ScoredCandidate, FindingType)>> = HashMap::new();
+    for (sc, ft) in findings {
+        grouped
+            .entry(sc.candidate.framework_id.clone())
+            .or_default()
+            .push((sc, ft));
+    }
+
+    let mut capped: Vec<(ScoredCandidate, FindingType)> = Vec::new();
+    for (_fw_id, mut group) in grouped {
+        // Sort: gaps first (score 0.0), then by score descending
+        group.sort_by(|a, b| {
+            let a_is_gap = (a.1 == FindingType::Gap && a.0.confidence_score == 0.0) as u8;
+            let b_is_gap = (b.1 == FindingType::Gap && b.0.confidence_score == 0.0) as u8;
+            b_is_gap
+                .cmp(&a_is_gap)
+                .then(b.0.confidence_score.partial_cmp(&a.0.confidence_score).unwrap_or(std::cmp::Ordering::Equal))
+        });
+        capped.extend(group.into_iter().take(config.max_findings_per_framework));
+    }
+
+    // Steps 5-7: Assign priority, generate recommendation, extract evidence
+    capped
+        .into_iter()
+        .map(|(sc, finding_type)| {
+            // Step 5: Priority
+            let priority = match finding_type {
+                FindingType::Addressed => 4,
+                FindingType::Gap => {
+                    if sc.candidate.parent_id.is_none() { 1 } else { 2 }
+                }
+                FindingType::PartiallyAddressed => {
+                    if sc.candidate.parent_id.is_none() { 2 } else { 3 }
+                }
+                FindingType::NotApplicable => 4,
+            };
+
+            // Step 6: Recommendation
+            let def_excerpt = if sc.candidate.definition_en.len() > 100 {
+                &sc.candidate.definition_en[..100]
+            } else {
+                &sc.candidate.definition_en
+            };
+            let ref_clause = sc
+                .candidate
+                .source_reference
+                .as_deref()
+                .map(|r| format!(". Reference: {r}"))
+                .unwrap_or_default();
+
+            let recommendation = match finding_type {
+                FindingType::Addressed => {
+                    Some(format!("Document adequately covers {}{ref_clause}", sc.candidate.name_en))
+                }
+                FindingType::PartiallyAddressed => {
+                    Some(format!(
+                        "Document partially addresses {}. Consider expanding coverage of {def_excerpt}{ref_clause}",
+                        sc.candidate.name_en
+                    ))
+                }
+                FindingType::Gap => {
+                    let action = sc
+                        .candidate
+                        .source_reference
+                        .as_deref()
+                        .map(|r| format!("review and implement controls per {r}"))
+                        .unwrap_or_else(|| "review and implement appropriate controls.".into());
+                    Some(format!(
+                        "Document does not address {}: {def_excerpt}. Recommended action: {action}",
+                        sc.candidate.name_en
+                    ))
+                }
+                FindingType::NotApplicable => None,
+            };
+
+            // Step 7: Evidence extraction (non-gap only)
+            let evidence_text = if finding_type != FindingType::Gap {
+                let concept_kws: HashSet<String> = extract_keywords(
+                    &format!("{} {}", sc.candidate.name_en, sc.candidate.definition_en),
+                )
+                .into_iter()
+                .collect();
+
+                sentence_kw_sets
+                    .iter()
+                    .max_by_key(|(_, s_kws)| s_kws.intersection(&concept_kws).count())
+                    .and_then(|(sentence, s_kws)| {
+                        let overlap = s_kws.intersection(&concept_kws).count();
+                        if overlap > 0 {
+                            Some(sentence.to_string())
+                        } else {
+                            None
+                        }
+                    })
+            } else {
+                None
+            };
+
+            NewFinding {
+                concept_id: sc.candidate.id,
+                framework_id: sc.candidate.framework_id,
+                finding_type,
+                confidence_score: sc.confidence_score,
+                evidence_text,
+                recommendation,
+                priority,
+            }
+        })
+        .collect()
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -867,4 +1030,97 @@ mod tests {
                 "Score {} for {} is outside [0.0, 1.0]", sc.confidence_score, sc.candidate.id);
         }
     }
+
+    // ========================================================================
+    // Section 05: Classification Tests
+    // ========================================================================
+
+    fn make_scored(id: &str, fw: &str, parent: Option<&str>, name: &str, def: &str, src: Option<&str>, score: f64) -> ScoredCandidate {
+        ScoredCandidate {
+            candidate: ConceptCandidate {
+                id: id.into(),
+                framework_id: fw.into(),
+                parent_id: parent.map(String::from),
+                name_en: name.into(),
+                definition_en: def.into(),
+                code: None,
+                source_reference: src.map(String::from),
+                concept_type: "concept".into(),
+            },
+            confidence_score: score,
+        }
+    }
+
+    #[test]
+    fn classify_score_08_produces_addressed() {
+        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.8);
+        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
+        assert_eq!(findings[0].finding_type, FindingType::Addressed);
+    }
+
+    #[test]
+    fn classify_score_04_produces_partially_addressed() {
+        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.4);
+        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
+        assert_eq!(findings[0].finding_type, FindingType::PartiallyAddressed);
+    }
+
+    #[test]
+    fn classify_score_01_produces_gap() {
+        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.1);
+        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
+        assert_eq!(findings[0].finding_type, FindingType::Gap);
+    }
+
+    #[test]
+    fn classify_score_00_produces_gap() {
+        let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.0);
+        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
+        assert_eq!(findings.len(), 1, "Zero-score gap should be retained");
+        assert_eq!(findings[0].finding_type, FindingType::Gap);
+    }
+
+    #[test]
+    fn classify_priority_root_gap_p1_child_gap_p2() {
+        let root = make_scored("c1", "fw", None, "Root", "Def", None, 0.0);
+        let child = make_scored("c2", "fw", Some("c1"), "Child", "Def", None, 0.0);
+        let findings = classify_findings(vec![root, child], &MatcherConfig::default(), "");
+        let root_f = findings.iter().find(|f| f.concept_id == "c1").unwrap();
+        let child_f = findings.iter().find(|f| f.concept_id == "c2").unwrap();
+        assert_eq!(root_f.priority, 1);
+        assert_eq!(child_f.priority, 2);
+    }
+
+    #[test]
+    fn classify_recommendation_contains_name_and_reference() {
+        let sc = make_scored("c1", "fw", None, "Access Control", "Managing access", Some("NIST SP 800-53 AC-1"), 0.0);
+        let findings = classify_findings(vec![sc], &MatcherConfig::default(), "");
+        let rec = findings[0].recommendation.as_ref().unwrap();
+        assert!(rec.contains("Access Control"), "Recommendation should contain concept name");
+        assert!(rec.contains("NIST SP 800-53 AC-1"), "Recommendation should contain source reference");
+    }
+
+    #[test]
+    fn classify_max_findings_per_framework_caps_output() {
+        let candidates: Vec<ScoredCandidate> = (0..60)
+            .map(|i| make_scored(&format!("c{i}"), "fw", None, "Concept", "Def", None, 0.0))
+            .collect();
+        let findings = classify_findings(candidates, &MatcherConfig::default(), "");
+        assert!(findings.len() <= 50, "Should cap at max_findings_per_framework=50, got {}", findings.len());
+    }
+
+    #[test]
+    fn classify_exclude_addressed_findings() {
+        let candidates = vec![
+            make_scored("c1", "fw", None, "High", "Def", None, 0.8),
+            make_scored("c2", "fw", None, "Medium", "Def", None, 0.4),
+            make_scored("c3", "fw", None, "Low", "Def", None, 0.1),
+        ];
+        let mut config = MatcherConfig::default();
+        config.include_addressed_findings = false;
+        let findings = classify_findings(candidates, &config, "");
+        assert!(findings.iter().all(|f| f.finding_type != FindingType::Addressed),
+            "No Addressed findings should be present");
+        assert_eq!(findings.len(), 2);
+    }
 }
