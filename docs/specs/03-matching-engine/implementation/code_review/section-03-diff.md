diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index 621f0ac..45e2761 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -1,6 +1,7 @@
 use std::collections::{HashMap, HashSet};
 
 use serde::{Deserialize, Serialize};
+use sqlx::SqlitePool;
 use tracing::warn;
 
 use super::tokenizer::extract_keywords;
@@ -206,6 +207,153 @@ pub fn detect_frameworks(
     ranked.into_iter().map(|(id, _)| id.to_string()).collect()
 }
 
+// ============================================================================
+// FTS5 Candidate Retrieval
+// ============================================================================
+
+/// Strip FTS5 special operators from keywords to prevent MATCH syntax errors.
+///
+/// Removes: `"`, `*`, `(`, `)`, `+`, `-`, `^`, `{`, `}`, `:`, `~`
+/// Discards keywords that become empty after sanitization.
+fn sanitize_fts_keywords(keywords: &[String]) -> Vec<String> {
+    const FTS5_SPECIAL: &[char] = &['"', '*', '(', ')', '+', '-', '^', '{', '}', ':', '~'];
+    keywords
+        .iter()
+        .filter_map(|kw| {
+            let cleaned: String = kw.chars().filter(|c| !FTS5_SPECIAL.contains(c)).collect();
+            let trimmed = cleaned.trim().to_string();
+            if trimmed.is_empty() {
+                None
+            } else {
+                Some(trimmed)
+            }
+        })
+        .collect()
+}
+
+/// Row type for reading concept candidates from SQLite queries.
+#[derive(sqlx::FromRow)]
+struct ConceptRow {
+    id: String,
+    framework_id: String,
+    parent_id: Option<String>,
+    name_en: String,
+    definition_en: String,
+    code: Option<String>,
+    source_reference: Option<String>,
+    concept_type: String,
+}
+
+impl From<ConceptRow> for ConceptCandidate {
+    fn from(row: ConceptRow) -> Self {
+        Self {
+            id: row.id,
+            framework_id: row.framework_id,
+            parent_id: row.parent_id,
+            name_en: row.name_en,
+            definition_en: row.definition_en,
+            code: row.code,
+            source_reference: row.source_reference,
+            concept_type: row.concept_type,
+        }
+    }
+}
+
+/// Retrieve candidate concepts from the database using FTS5 and exact matching.
+///
+/// Returns all matched concepts plus gap candidates (unmatched concepts from
+/// detected frameworks) for comprehensive gap analysis.
+pub async fn retrieve_candidates(
+    keywords: &[String],
+    framework_ids: &[String],
+    db: &SqlitePool,
+) -> Result<Vec<ConceptCandidate>, sqlx::Error> {
+    if framework_ids.is_empty() {
+        return Ok(Vec::new());
+    }
+
+    let fw_json = serde_json::to_string(framework_ids).unwrap_or_else(|_| "[]".into());
+    let sanitized = sanitize_fts_keywords(keywords);
+    let capped: Vec<&String> = sanitized.iter().take(20).collect();
+
+    let mut seen_ids: HashSet<String> = HashSet::new();
+    let mut candidates: Vec<ConceptCandidate> = Vec::new();
+
+    // Step 1: FTS5 MATCH query
+    if !capped.is_empty() {
+        let match_expr = capped.iter().map(|k| format!("\"{}\"", k)).collect::<Vec<_>>().join(" OR ");
+        let fts_rows: Vec<ConceptRow> = sqlx::query_as(
+            r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
+                      COALESCE(c.definition_en, '') as definition_en,
+                      c.code, c.source_reference, c.concept_type
+               FROM concepts c
+               JOIN concepts_fts ON concepts_fts.rowid = c.rowid
+               WHERE concepts_fts MATCH ?1
+               AND c.framework_id IN (SELECT value FROM json_each(?2))"#,
+        )
+        .bind(&match_expr)
+        .bind(&fw_json)
+        .fetch_all(db)
+        .await
+        .unwrap_or_else(|e| {
+            warn!("FTS5 MATCH query failed, continuing with exact matches: {e}");
+            Vec::new()
+        });
+
+        for row in fts_rows {
+            let candidate: ConceptCandidate = row.into();
+            if seen_ids.insert(candidate.id.clone()) {
+                candidates.push(candidate);
+            }
+        }
+    }
+
+    // Step 2: Exact match on name_en and code (keywords > 4 chars)
+    for kw in capped.iter().filter(|k| k.len() > 4) {
+        let exact_rows: Vec<ConceptRow> = sqlx::query_as(
+            r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
+                      COALESCE(c.definition_en, '') as definition_en,
+                      c.code, c.source_reference, c.concept_type
+               FROM concepts c
+               WHERE c.framework_id IN (SELECT value FROM json_each(?1))
+               AND (LOWER(c.name_en) LIKE '%' || ?2 || '%'
+                    OR LOWER(c.code) LIKE '%' || ?2 || '%')"#,
+        )
+        .bind(&fw_json)
+        .bind(kw.as_str())
+        .fetch_all(db)
+        .await?;
+
+        for row in exact_rows {
+            let candidate: ConceptCandidate = row.into();
+            if seen_ids.insert(candidate.id.clone()) {
+                candidates.push(candidate);
+            }
+        }
+    }
+
+    // Step 3: Load gap candidates (concepts not yet matched)
+    let matched_json = serde_json::to_string(&seen_ids.iter().collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
+    let gap_rows: Vec<ConceptRow> = sqlx::query_as(
+        r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
+                  COALESCE(c.definition_en, '') as definition_en,
+                  c.code, c.source_reference, c.concept_type
+           FROM concepts c
+           WHERE c.framework_id IN (SELECT value FROM json_each(?1))
+           AND c.id NOT IN (SELECT value FROM json_each(?2))"#,
+    )
+    .bind(&fw_json)
+    .bind(&matched_json)
+    .fetch_all(db)
+    .await?;
+
+    for row in gap_rows {
+        candidates.push(row.into());
+    }
+
+    Ok(candidates)
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -383,4 +531,124 @@ mod tests {
         assert_eq!(result[0], "iso31000");
         assert_eq!(result[1], "nist-csf");
     }
+
+    // ========================================================================
+    // Section 03: FTS5 Candidate Retrieval Tests
+    // ========================================================================
+
+    #[test]
+    fn test_sanitize_fts_keywords_removes_special_chars() {
+        let input: Vec<String> = vec![
+            "risk*".into(),
+            "\"assessment\"".into(),
+            "(control)".into(),
+            "normal".into(),
+            "***".into(), // should be discarded entirely
+        ];
+        let result = sanitize_fts_keywords(&input);
+        assert_eq!(result, vec!["risk", "assessment", "control", "normal"]);
+    }
+
+    #[test]
+    fn test_sanitize_fts_keywords_empty_input() {
+        let result = sanitize_fts_keywords(&[]);
+        assert!(result.is_empty());
+    }
+
+    /// Helper to create an in-memory SQLite database with schema and test data.
+    async fn setup_test_db() -> SqlitePool {
+        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
+
+        // Create minimal schema (frameworks, concepts, FTS5)
+        sqlx::raw_sql(
+            r#"
+            CREATE TABLE frameworks (
+                id TEXT PRIMARY KEY,
+                name TEXT NOT NULL
+            );
+            CREATE TABLE concepts (
+                id TEXT PRIMARY KEY,
+                framework_id TEXT NOT NULL REFERENCES frameworks(id),
+                parent_id TEXT,
+                concept_type TEXT NOT NULL,
+                code TEXT,
+                name_en TEXT NOT NULL,
+                name_nb TEXT,
+                definition_en TEXT,
+                definition_nb TEXT,
+                source_reference TEXT,
+                sort_order INTEGER DEFAULT 0,
+                created_at TEXT DEFAULT (datetime('now')),
+                updated_at TEXT DEFAULT (datetime('now'))
+            );
+            CREATE VIRTUAL TABLE concepts_fts USING fts5(
+                name_en, name_nb, definition_en, definition_nb,
+                content='concepts', content_rowid='rowid'
+            );
+            CREATE TRIGGER concepts_ai AFTER INSERT ON concepts BEGIN
+                INSERT INTO concepts_fts(rowid, name_en, name_nb, definition_en, definition_nb)
+                VALUES (NEW.rowid, NEW.name_en, NEW.name_nb, NEW.definition_en, NEW.definition_nb);
+            END;
+            "#,
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+
+        // Insert test data
+        sqlx::raw_sql(
+            r#"
+            INSERT INTO frameworks (id, name) VALUES ('nist-csf', 'NIST Cybersecurity Framework');
+            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
+                VALUES ('nist-csf-id', 'nist-csf', NULL, 'function', 'ID', 'Identify', 'Develop understanding of cybersecurity risk');
+            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
+                VALUES ('nist-csf-pr', 'nist-csf', NULL, 'function', 'PR', 'Protect', 'Implement safeguards for critical services');
+            INSERT INTO concepts (id, framework_id, parent_id, concept_type, code, name_en, definition_en)
+                VALUES ('nist-csf-de', 'nist-csf', NULL, 'function', 'DE', 'Detect', 'Identify cybersecurity events');
+            "#,
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+
+        pool
+    }
+
+    #[tokio::test]
+    async fn test_retrieve_candidates_returns_fts_matches() {
+        let pool = setup_test_db().await;
+        let keywords: Vec<String> = vec!["identify".into(), "cybersecurity".into(), "risk".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let ids: Vec<&str> = result.iter().map(|c| c.id.as_str()).collect();
+        assert!(ids.contains(&"nist-csf-id"), "Should find 'Identify' concept via FTS5");
+    }
+
+    #[tokio::test]
+    async fn test_retrieve_candidates_includes_gap_candidates() {
+        let pool = setup_test_db().await;
+        // Only "identify" matches — "Protect" and "Detect" should appear as gap candidates
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        assert_eq!(result.len(), 3, "Should include 1 FTS match + 2 gap candidates");
+        let ids: Vec<&str> = result.iter().map(|c| c.id.as_str()).collect();
+        assert!(ids.contains(&"nist-csf-id"));
+        assert!(ids.contains(&"nist-csf-pr"));
+        assert!(ids.contains(&"nist-csf-de"));
+    }
+
+    #[tokio::test]
+    async fn test_retrieve_candidates_deduplicates() {
+        let pool = setup_test_db().await;
+        // "identify" matches via FTS5 AND exact name match (>4 chars)
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let id_count = result.iter().filter(|c| c.id == "nist-csf-id").count();
+        assert_eq!(id_count, 1, "Concept should appear exactly once despite FTS5 + exact match");
+    }
 }
