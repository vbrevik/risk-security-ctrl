diff --git a/backend/migrations/003_analysis_schema.sql b/backend/migrations/003_analysis_schema.sql
new file mode 100644
index 0000000..1a2acb1
--- /dev/null
+++ b/backend/migrations/003_analysis_schema.sql
@@ -0,0 +1,62 @@
+-- Document Analysis Engine - Analysis Schema
+-- Split 01: Database Models Foundation
+
+-- ============================================================================
+-- ANALYSES TABLE
+-- ============================================================================
+
+CREATE TABLE IF NOT EXISTS analyses (
+    id TEXT PRIMARY KEY,
+    name TEXT NOT NULL,
+    description TEXT,
+    input_type TEXT NOT NULL CHECK(input_type IN ('text', 'pdf', 'docx')),
+    input_text TEXT,
+    original_filename TEXT,
+    file_path TEXT,
+    extracted_text TEXT,
+    status TEXT NOT NULL DEFAULT 'pending'
+        CHECK(status IN ('pending', 'processing', 'completed', 'failed', 'deleted')),
+    error_message TEXT,
+    prompt_template TEXT,
+    matched_framework_ids TEXT,  -- JSON array, e.g. '["nist-csf","iso31000"]'
+    processing_time_ms INTEGER,
+    token_count INTEGER,
+    created_by TEXT,
+    created_at TEXT NOT NULL DEFAULT (datetime('now')),
+    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
+);
+
+-- ============================================================================
+-- ANALYSIS FINDINGS TABLE
+-- ============================================================================
+
+CREATE TABLE IF NOT EXISTS analysis_findings (
+    id TEXT PRIMARY KEY,
+    analysis_id TEXT NOT NULL REFERENCES analyses(id) ON DELETE CASCADE,
+    concept_id TEXT NOT NULL REFERENCES concepts(id),
+    framework_id TEXT NOT NULL REFERENCES frameworks(id),
+    finding_type TEXT NOT NULL
+        CHECK(finding_type IN ('addressed', 'partially_addressed', 'gap', 'not_applicable')),
+    confidence_score REAL NOT NULL CHECK(confidence_score BETWEEN 0.0 AND 1.0),
+    evidence_text TEXT,
+    recommendation TEXT,
+    priority INTEGER NOT NULL CHECK(priority BETWEEN 1 AND 4),  -- 1=critical, 2=high, 3=medium, 4=low
+    sort_order INTEGER NOT NULL DEFAULT 0,
+    created_at TEXT NOT NULL DEFAULT (datetime('now'))
+);
+
+-- ============================================================================
+-- INDEXES
+-- ============================================================================
+
+-- Analysis indexes
+CREATE INDEX IF NOT EXISTS idx_analyses_status ON analyses(status);
+CREATE INDEX IF NOT EXISTS idx_analyses_created_by ON analyses(created_by);
+CREATE INDEX IF NOT EXISTS idx_analyses_created_at ON analyses(created_at);
+
+-- Finding indexes
+CREATE INDEX IF NOT EXISTS idx_analysis_findings_analysis ON analysis_findings(analysis_id);
+CREATE INDEX IF NOT EXISTS idx_analysis_findings_framework ON analysis_findings(framework_id);
+CREATE INDEX IF NOT EXISTS idx_analysis_findings_type ON analysis_findings(finding_type);
+CREATE INDEX IF NOT EXISTS idx_analysis_findings_priority ON analysis_findings(priority);
+CREATE INDEX IF NOT EXISTS idx_analysis_findings_analysis_type_priority ON analysis_findings(analysis_id, finding_type, priority);
diff --git a/backend/src/main.rs b/backend/src/main.rs
index 257e9fb..7ec0a9c 100644
--- a/backend/src/main.rs
+++ b/backend/src/main.rs
@@ -44,9 +44,17 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
     // Load configuration
     let config = Config::from_env();
 
-    // Create database connection pool
+    // Create database connection pool with foreign key enforcement
     let db = SqlitePoolOptions::new()
         .max_connections(5)
+        .after_connect(|conn, _meta| {
+            Box::pin(async move {
+                sqlx::query("PRAGMA foreign_keys = ON")
+                    .execute(&mut *conn)
+                    .await?;
+                Ok(())
+            })
+        })
         .connect(&config.database_url)
         .await?;
 
@@ -55,6 +63,27 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
 
     tracing::info!("Database migrations completed");
 
+    // Check for foreign key violations in existing data
+    let violations: Vec<(String, i64, String, i64)> =
+        sqlx::query_as("PRAGMA foreign_key_check")
+            .fetch_all(&db)
+            .await?;
+
+    if !violations.is_empty() {
+        tracing::warn!(
+            "Found {} foreign key violations in existing data",
+            violations.len()
+        );
+        for (table, rowid, parent, fkid) in &violations {
+            tracing::warn!(
+                "FK violation: table={}, rowid={}, parent={}, fkid={}",
+                table, rowid, parent, fkid
+            );
+        }
+    } else {
+        tracing::info!("Foreign key integrity check passed");
+    }
+
     // Import ontology data (if not already imported or if new frameworks available)
     let ontology_data_dir = std::path::Path::new("../ontology-data");
     if ontology_data_dir.exists() {
