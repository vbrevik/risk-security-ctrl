diff --git a/backend/Cargo.toml b/backend/Cargo.toml
index 1e263a1..7bba508 100644
--- a/backend/Cargo.toml
+++ b/backend/Cargo.toml
@@ -45,6 +45,16 @@ pdf-extract = "0.10"
 zip = "2"
 quick-xml = "0.37"
 
+# Chart rendering
+plotters = "0.3"
+image = "0.25"
+
+# PDF export
+genpdf = { version = "0.2", features = ["images"] }
+
+# DOCX export
+docx-rs = "0.4"
+
 [lib]
 name = "ontology_backend"
 path = "src/lib.rs"
diff --git a/backend/fonts/LiberationSans-Bold.ttf b/backend/fonts/LiberationSans-Bold.ttf
new file mode 100644
index 0000000..dc5d57f
Binary files /dev/null and b/backend/fonts/LiberationSans-Bold.ttf differ
diff --git a/backend/fonts/LiberationSans-Italic.ttf b/backend/fonts/LiberationSans-Italic.ttf
new file mode 100644
index 0000000..25970d9
Binary files /dev/null and b/backend/fonts/LiberationSans-Italic.ttf differ
diff --git a/backend/fonts/LiberationSans-Regular.ttf b/backend/fonts/LiberationSans-Regular.ttf
new file mode 100644
index 0000000..e633985
Binary files /dev/null and b/backend/fonts/LiberationSans-Regular.ttf differ
diff --git a/backend/src/lib.rs b/backend/src/lib.rs
index 6175148..466c4da 100644
--- a/backend/src/lib.rs
+++ b/backend/src/lib.rs
@@ -11,11 +11,13 @@ pub mod features;
 pub mod import;
 
 pub use config::Config;
+use features::analysis::matcher::Topic;
 
 #[derive(Clone)]
 pub struct AppState {
     pub db: sqlx::SqlitePool,
     pub config: Config,
+    pub topics: Vec<Topic>,
 }
 
 /// Create the application router (for testing)
diff --git a/backend/src/main.rs b/backend/src/main.rs
index 18e2e20..4e38faf 100644
--- a/backend/src/main.rs
+++ b/backend/src/main.rs
@@ -134,10 +134,42 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
         );
     }
 
+    // Load topics from ontology-data/topic-tags.json
+    let topics = {
+        let path = std::path::Path::new("../ontology-data/topic-tags.json");
+        if path.exists() {
+            let contents = std::fs::read_to_string(path)?;
+            let file: serde_json::Value = serde_json::from_str(&contents)?;
+            file["topics"]
+                .as_array()
+                .map(|arr| {
+                    arr.iter()
+                        .filter_map(|v| {
+                            Some(ontology_backend::features::analysis::matcher::Topic {
+                                id: v["id"].as_str()?.to_string(),
+                                name_en: v["name_en"].as_str()?.to_string(),
+                                concept_ids: v["concept_ids"]
+                                    .as_array()?
+                                    .iter()
+                                    .filter_map(|c| c.as_str().map(String::from))
+                                    .collect(),
+                            })
+                        })
+                        .collect()
+                })
+                .unwrap_or_default()
+        } else {
+            tracing::warn!("topic-tags.json not found, analysis matching will have no topics");
+            vec![]
+        }
+    };
+    tracing::info!("Loaded {} topics for analysis matching", topics.len());
+
     // Create application state
     let state = AppState {
         db,
         config: config.clone(),
+        topics,
     };
 
     // Build router with Swagger UI
diff --git a/backend/tests/common/mod.rs b/backend/tests/common/mod.rs
index dcf0a7a..a3a70e6 100644
--- a/backend/tests/common/mod.rs
+++ b/backend/tests/common/mod.rs
@@ -40,9 +40,40 @@ pub async fn create_test_app() -> Router {
         }
     }
 
+    // Load topics for test state
+    let topics = {
+        let path = std::path::Path::new("../ontology-data/topic-tags.json");
+        if path.exists() {
+            let contents = std::fs::read_to_string(path).expect("Failed to read topic-tags.json");
+            let file: serde_json::Value =
+                serde_json::from_str(&contents).expect("Failed to parse topic-tags.json");
+            file["topics"]
+                .as_array()
+                .map(|arr| {
+                    arr.iter()
+                        .filter_map(|v| {
+                            Some(ontology_backend::features::analysis::matcher::Topic {
+                                id: v["id"].as_str()?.to_string(),
+                                name_en: v["name_en"].as_str()?.to_string(),
+                                concept_ids: v["concept_ids"]
+                                    .as_array()?
+                                    .iter()
+                                    .filter_map(|c| c.as_str().map(String::from))
+                                    .collect(),
+                            })
+                        })
+                        .collect()
+                })
+                .unwrap_or_default()
+        } else {
+            vec![]
+        }
+    };
+
     let state = AppState {
         db: pool,
         config: config.clone(),
+        topics,
     };
 
     ontology_backend::create_router(state)
