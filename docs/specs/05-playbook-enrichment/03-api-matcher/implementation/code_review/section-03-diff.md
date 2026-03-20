diff --git a/backend/src/features/analysis/matcher.rs b/backend/src/features/analysis/matcher.rs
index a51f5d1..154aed4 100644
--- a/backend/src/features/analysis/matcher.rs
+++ b/backend/src/features/analysis/matcher.rs
@@ -127,6 +127,8 @@ pub struct ConceptCandidate {
     pub code: Option<String>,
     pub source_reference: Option<String>,
     pub concept_type: String,
+    pub about_en: Option<String>,
+    pub actions_text: Option<String>,
 }
 
 /// A scored candidate after TF-IDF scoring.
@@ -248,7 +250,7 @@ fn escape_like(keyword: &str) -> String {
     keyword.replace('%', "\\%").replace('_', "\\_")
 }
 
-/// Row type for reading concept candidates from SQLite queries.
+/// Row type for reading concept candidates from SQLite queries (with guidance fields).
 #[derive(sqlx::FromRow)]
 struct ConceptRow {
     id: String,
@@ -259,6 +261,8 @@ struct ConceptRow {
     code: Option<String>,
     source_reference: Option<String>,
     concept_type: String,
+    about_en: Option<String>,
+    actions_text: Option<String>,
 }
 
 impl From<ConceptRow> for ConceptCandidate {
@@ -272,6 +276,38 @@ impl From<ConceptRow> for ConceptCandidate {
             code: row.code,
             source_reference: row.source_reference,
             concept_type: row.concept_type,
+            about_en: row.about_en,
+            actions_text: row.actions_text,
+        }
+    }
+}
+
+/// Row type for gap candidates (no guidance data needed).
+#[derive(sqlx::FromRow)]
+struct GapConceptRow {
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
+impl From<GapConceptRow> for ConceptCandidate {
+    fn from(row: GapConceptRow) -> Self {
+        Self {
+            id: row.id,
+            framework_id: row.framework_id,
+            parent_id: row.parent_id,
+            name_en: row.name_en,
+            definition_en: row.definition_en,
+            code: row.code,
+            source_reference: row.source_reference,
+            concept_type: row.concept_type,
+            about_en: None,
+            actions_text: None,
         }
     }
 }
@@ -299,12 +335,19 @@ pub async fn retrieve_candidates(
     // Step 1: FTS5 MATCH query
     if !capped.is_empty() {
         let match_expr = capped.iter().map(|k| k.as_str()).collect::<Vec<_>>().join(" OR ");
+
+        // Step 1a: FTS5 MATCH against concepts_fts (with guidance LEFT JOIN)
         let fts_rows: Vec<ConceptRow> = sqlx::query_as(
             r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                       COALESCE(c.definition_en, '') as definition_en,
-                      c.code, c.source_reference, c.concept_type
+                      c.code, c.source_reference, c.concept_type,
+                      cg.about_en,
+                      (SELECT GROUP_CONCAT(action_text_en, char(10))
+                       FROM (SELECT action_text_en FROM concept_actions
+                             WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
                FROM concepts c
                JOIN concepts_fts ON concepts_fts.rowid = c.rowid
+               LEFT JOIN concept_guidance cg ON cg.concept_id = c.id
                WHERE concepts_fts MATCH ?1
                AND c.framework_id IN (SELECT value FROM json_each(?2))"#,
         )
@@ -323,6 +366,39 @@ pub async fn retrieve_candidates(
                 candidates.push(candidate);
             }
         }
+
+        // Step 1b: FTS5 MATCH against concept_guidance_fts (broader recall)
+        let guidance_fts_rows: Vec<ConceptRow> = sqlx::query_as(
+            r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
+                      COALESCE(c.definition_en, '') as definition_en,
+                      c.code, c.source_reference, c.concept_type,
+                      cg.about_en,
+                      (SELECT GROUP_CONCAT(action_text_en, char(10))
+                       FROM (SELECT action_text_en FROM concept_actions
+                             WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
+               FROM concept_guidance_fts gf
+               JOIN concept_guidance cg ON cg.rowid = gf.rowid
+               JOIN concepts c ON c.id = cg.concept_id
+               WHERE concept_guidance_fts MATCH ?1
+               AND c.framework_id IN (SELECT value FROM json_each(?2))
+               ORDER BY bm25(concept_guidance_fts, 10.0, 3.0, 5.0)
+               LIMIT 50"#,
+        )
+        .bind(&match_expr)
+        .bind(&fw_json)
+        .fetch_all(db)
+        .await
+        .unwrap_or_else(|e| {
+            warn!("Guidance FTS5 query failed, continuing without guidance matches: {e}");
+            Vec::new()
+        });
+
+        for row in guidance_fts_rows {
+            let candidate: ConceptCandidate = row.into();
+            if seen_ids.insert(candidate.id.clone()) {
+                candidates.push(candidate);
+            }
+        }
     }
 
     // Step 2: Exact match on name_en and code (keywords > 4 chars)
@@ -331,8 +407,13 @@ pub async fn retrieve_candidates(
         let exact_rows: Vec<ConceptRow> = sqlx::query_as(
             r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                       COALESCE(c.definition_en, '') as definition_en,
-                      c.code, c.source_reference, c.concept_type
+                      c.code, c.source_reference, c.concept_type,
+                      cg.about_en,
+                      (SELECT GROUP_CONCAT(action_text_en, char(10))
+                       FROM (SELECT action_text_en FROM concept_actions
+                             WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
                FROM concepts c
+               LEFT JOIN concept_guidance cg ON cg.concept_id = c.id
                WHERE c.framework_id IN (SELECT value FROM json_each(?1))
                AND (LOWER(c.name_en) LIKE '%' || ?2 || '%' ESCAPE '\'
                     OR LOWER(c.code) LIKE '%' || ?2 || '%' ESCAPE '\')"#,
@@ -350,9 +431,9 @@ pub async fn retrieve_candidates(
         }
     }
 
-    // Step 3: Load gap candidates (concepts not yet matched)
+    // Step 3: Load gap candidates (concepts not yet matched — no guidance JOIN needed)
     let matched_json = serde_json::to_string(&seen_ids.iter().collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
-    let gap_rows: Vec<ConceptRow> = sqlx::query_as(
+    let gap_rows: Vec<GapConceptRow> = sqlx::query_as(
         r#"SELECT c.id, c.framework_id, c.parent_id, c.name_en,
                   COALESCE(c.definition_en, '') as definition_en,
                   c.code, c.source_reference, c.concept_type
@@ -804,6 +885,8 @@ mod tests {
             code: Some("RA-1".into()),
             source_reference: Some("ISO 31000:2018 6.4".into()),
             concept_type: "process".into(),
+            about_en: None,
+            actions_text: None,
         };
         assert_eq!(c.id, "c-1");
         assert!(c.parent_id.is_none());
@@ -820,6 +903,8 @@ mod tests {
             code: None,
             source_reference: None,
             concept_type: "control".into(),
+            about_en: None,
+            actions_text: None,
         };
         let sc = ScoredCandidate {
             candidate,
@@ -967,6 +1052,65 @@ mod tests {
                 INSERT INTO concepts_fts(rowid, name_en, name_nb, definition_en, definition_nb)
                 VALUES (NEW.rowid, NEW.name_en, NEW.name_nb, NEW.definition_en, NEW.definition_nb);
             END;
+
+            CREATE TABLE concept_guidance (
+                id TEXT PRIMARY KEY,
+                concept_id TEXT NOT NULL UNIQUE REFERENCES concepts(id) ON DELETE CASCADE,
+                source_pdf TEXT NOT NULL,
+                source_page INTEGER NOT NULL,
+                about_en TEXT,
+                about_nb TEXT,
+                created_at TEXT DEFAULT (datetime('now')),
+                updated_at TEXT DEFAULT (datetime('now'))
+            );
+
+            CREATE TABLE concept_actions (
+                id TEXT PRIMARY KEY,
+                concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
+                action_text_en TEXT NOT NULL,
+                action_text_nb TEXT,
+                sort_order INTEGER NOT NULL,
+                created_at TEXT DEFAULT (datetime('now')),
+                UNIQUE(concept_id, sort_order)
+            );
+
+            CREATE TABLE concept_transparency_questions (
+                id TEXT PRIMARY KEY,
+                concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
+                question_text_en TEXT NOT NULL,
+                question_text_nb TEXT,
+                sort_order INTEGER NOT NULL,
+                created_at TEXT DEFAULT (datetime('now')),
+                UNIQUE(concept_id, sort_order)
+            );
+
+            CREATE TABLE concept_references (
+                id TEXT PRIMARY KEY,
+                concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
+                reference_type TEXT NOT NULL CHECK(reference_type IN ('academic', 'transparency_resource')),
+                title TEXT NOT NULL,
+                authors TEXT,
+                year INTEGER,
+                venue TEXT,
+                url TEXT,
+                sort_order INTEGER NOT NULL,
+                created_at TEXT DEFAULT (datetime('now'))
+            );
+
+            CREATE VIEW concept_guidance_search_v AS
+            SELECT
+                cg.rowid AS rowid,
+                c.name_en,
+                c.definition_en,
+                cg.about_en
+            FROM concept_guidance cg
+            JOIN concepts c ON c.id = cg.concept_id;
+
+            CREATE VIRTUAL TABLE concept_guidance_fts USING fts5(
+                name_en, definition_en, about_en,
+                content='concept_guidance_search_v',
+                content_rowid='rowid'
+            );
             "#,
         )
         .execute(&pool)
@@ -1030,6 +1174,165 @@ mod tests {
         assert_eq!(id_count, 1, "Concept should appear exactly once despite FTS5 + exact match");
     }
 
+    // ========================================================================
+    // Section 03b: Guidance Enrichment Tests
+    // ========================================================================
+
+    /// Helper: seed guidance data into the test DB and rebuild FTS
+    async fn seed_guidance(pool: &SqlitePool) {
+        sqlx::raw_sql(
+            r#"
+            INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en)
+                VALUES ('g1', 'nist-csf-id', 'playbook.pdf', 10, 'About identifying risks with measurement approaches');
+            INSERT INTO concept_actions (id, concept_id, action_text_en, sort_order)
+                VALUES ('a1', 'nist-csf-id', 'Establish governance board', 1);
+            INSERT INTO concept_actions (id, concept_id, action_text_en, sort_order)
+                VALUES ('a2', 'nist-csf-id', 'Define risk appetite', 2);
+            INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
+            "#,
+        )
+        .execute(pool)
+        .await
+        .unwrap();
+    }
+
+    #[tokio::test]
+    async fn test_candidate_with_guidance_has_about_en() {
+        let pool = setup_test_db().await;
+        seed_guidance(&pool).await;
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let id_candidate = result.iter().find(|c| c.id == "nist-csf-id").unwrap();
+        assert_eq!(
+            id_candidate.about_en.as_deref(),
+            Some("About identifying risks with measurement approaches")
+        );
+    }
+
+    #[tokio::test]
+    async fn test_candidate_with_guidance_has_actions_text() {
+        let pool = setup_test_db().await;
+        seed_guidance(&pool).await;
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let id_candidate = result.iter().find(|c| c.id == "nist-csf-id").unwrap();
+        assert_eq!(
+            id_candidate.actions_text.as_deref(),
+            Some("Establish governance board\nDefine risk appetite")
+        );
+    }
+
+    #[tokio::test]
+    async fn test_candidate_without_guidance_has_none_fields() {
+        let pool = setup_test_db().await;
+        // nist-csf-pr has no guidance data
+        let keywords: Vec<String> = vec!["protect".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let pr_candidate = result.iter().find(|c| c.id == "nist-csf-pr").unwrap();
+        assert!(pr_candidate.about_en.is_none());
+        assert!(pr_candidate.actions_text.is_none());
+    }
+
+    #[tokio::test]
+    async fn test_guidance_fts_returns_about_en_matches() {
+        let pool = setup_test_db().await;
+        // Add a concept whose name/definition don't contain "measurement"
+        // but whose guidance about_en does
+        sqlx::raw_sql(
+            r#"
+            INSERT INTO concepts (id, framework_id, concept_type, code, name_en, definition_en)
+                VALUES ('nist-csf-gov', 'nist-csf', 'function', 'GV', 'Govern', 'Establish governance structure');
+            INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en)
+                VALUES ('g-gov', 'nist-csf-gov', 'playbook.pdf', 20, 'measurement approaches for governance');
+            INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
+            "#,
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+
+        let keywords: Vec<String> = vec!["measurement".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let ids: Vec<&str> = result.iter().map(|c| c.id.as_str()).collect();
+        assert!(
+            ids.contains(&"nist-csf-gov"),
+            "Should find concept via guidance FTS about_en match"
+        );
+    }
+
+    #[tokio::test]
+    async fn test_fts_union_broader_recall() {
+        let pool = setup_test_db().await;
+        // nist-csf-id matches "identify" via concepts_fts
+        // Add a concept found only via guidance FTS
+        sqlx::raw_sql(
+            r#"
+            INSERT INTO concepts (id, framework_id, concept_type, code, name_en, definition_en)
+                VALUES ('nist-csf-rs', 'nist-csf', 'function', 'RS', 'Respond', 'Response planning activities');
+            INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en)
+                VALUES ('g-rs', 'nist-csf-rs', 'playbook.pdf', 30, 'identify response procedures');
+            INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
+            "#,
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let matched_ids: Vec<&str> = result.iter()
+            .filter(|c| c.id == "nist-csf-id" || c.id == "nist-csf-rs")
+            .map(|c| c.id.as_str())
+            .collect();
+        assert!(matched_ids.contains(&"nist-csf-id"), "concepts_fts match");
+        assert!(matched_ids.contains(&"nist-csf-rs"), "guidance_fts match");
+    }
+
+    #[tokio::test]
+    async fn test_gap_candidates_no_guidance_fields() {
+        let pool = setup_test_db().await;
+        seed_guidance(&pool).await;
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        // nist-csf-pr and nist-csf-de are gap candidates
+        for candidate in result.iter().filter(|c| c.id == "nist-csf-pr" || c.id == "nist-csf-de") {
+            assert!(candidate.about_en.is_none(), "Gap candidate {} should not have about_en", candidate.id);
+            assert!(candidate.actions_text.is_none(), "Gap candidate {} should not have actions_text", candidate.id);
+        }
+    }
+
+    #[tokio::test]
+    async fn test_dedup_across_fts_tables() {
+        let pool = setup_test_db().await;
+        seed_guidance(&pool).await;
+        // "identify" matches nist-csf-id via both concepts_fts AND concept_guidance_fts
+        let keywords: Vec<String> = vec!["identify".into()];
+        let framework_ids: Vec<String> = vec!["nist-csf".into()];
+
+        let result = retrieve_candidates(&keywords, &framework_ids, &pool).await.unwrap();
+        let id_count = result.iter().filter(|c| c.id == "nist-csf-id").count();
+        assert_eq!(id_count, 1, "Concept should appear exactly once despite matching both FTS tables");
+    }
+
+    #[test]
+    fn test_make_candidate_includes_guidance_fields() {
+        let c = make_candidate("c1", "Test", "Def");
+        assert!(c.about_en.is_none());
+        assert!(c.actions_text.is_none());
+    }
+
     // ========================================================================
     // Section 04: TF-IDF Scoring Tests
     // ========================================================================
@@ -1044,6 +1347,8 @@ mod tests {
             code: None,
             source_reference: None,
             concept_type: "concept".into(),
+            about_en: None,
+            actions_text: None,
         }
     }
 
@@ -1137,6 +1442,8 @@ mod tests {
                 code: None,
                 source_reference: src.map(String::from),
                 concept_type: "concept".into(),
+                about_en: None,
+                actions_text: None,
             },
             confidence_score: score,
         }
