diff --git a/backend/src/features/analysis/routes.rs b/backend/src/features/analysis/routes.rs
index 5a4c73e..d499495 100644
--- a/backend/src/features/analysis/routes.rs
+++ b/backend/src/features/analysis/routes.rs
@@ -12,7 +12,7 @@ use crate::AppState;
 use super::engine::MatchingEngine;
 use super::export_docx;
 use super::export_pdf;
-use super::matcher::DeterministicMatcher;
+use super::matcher::{DeterministicMatcher, MatcherConfig};
 use super::models::{
     Analysis, AnalysisFindingWithConcept, AnalysisListQuery, AnalysisRow, CreateAnalysisRequest,
     FindingType, FindingsListQuery, InputType,
@@ -20,6 +20,8 @@ use super::models::{
 use super::parser::parse_async;
 use super::upload::{validate_upload, save_upload};
 
+const PROMPT_TEMPLATE_PATH: &str = "config/default-prompt-template.json";
+
 const PDF_CONTENT_TYPE: &str = "application/pdf";
 const DOCX_CONTENT_TYPE: &str =
     "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
@@ -625,16 +627,27 @@ fn sanitize_filename(name: &str) -> String {
     path = "/api/analyses/prompt-template",
     tag = "analysis",
     responses(
-        (status = 200, description = "Current prompt template")
+        (status = 200, description = "Current prompt template configuration")
     )
 )]
 pub async fn get_prompt_template(
     State(_state): State<AppState>,
-) -> AppResult<Json<serde_json::Value>> {
-    Ok(Json(serde_json::json!({
-        "template": null,
-        "description": "Default matching engine configuration"
-    })))
+) -> AppResult<Json<MatcherConfig>> {
+    match tokio::fs::read_to_string(PROMPT_TEMPLATE_PATH).await {
+        Ok(contents) => {
+            let config: MatcherConfig = serde_json::from_str(&contents).unwrap_or_else(|e| {
+                tracing::warn!("Invalid prompt template file, using defaults: {e}");
+                MatcherConfig::default()
+            });
+            Ok(Json(config))
+        }
+        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
+            Ok(Json(MatcherConfig::default()))
+        }
+        Err(e) => Err(AppError::Internal(format!(
+            "Failed to read prompt template: {e}"
+        ))),
+    }
 }
 
 #[utoipa::path(
@@ -642,12 +655,48 @@ pub async fn get_prompt_template(
     path = "/api/analyses/prompt-template",
     tag = "analysis",
     responses(
-        (status = 200, description = "Prompt template updated")
+        (status = 200, description = "Prompt template updated"),
+        (status = 400, description = "Invalid configuration"),
     )
 )]
 pub async fn update_prompt_template(
-    State(_state): State<AppState>,
-    Json(_body): Json<serde_json::Value>,
-) -> AppResult<Json<serde_json::Value>> {
-    Ok(Json(serde_json::json!({ "status": "ok" })))
+    State(state): State<AppState>,
+    Json(body): Json<serde_json::Value>,
+) -> AppResult<Json<MatcherConfig>> {
+    let config: MatcherConfig = serde_json::from_value(body).map_err(|e| {
+        AppError::BadRequest(format!("Invalid prompt template configuration: {e}"))
+    })?;
+
+    // Ensure config directory exists
+    tokio::fs::create_dir_all("config")
+        .await
+        .map_err(|e| AppError::Internal(format!("Failed to create config directory: {e}")))?;
+
+    // Write atomically: temp file then rename
+    let tmp_path = format!("{PROMPT_TEMPLATE_PATH}.tmp");
+    let json_str = serde_json::to_string_pretty(&config)
+        .map_err(|e| AppError::Internal(format!("Failed to serialize config: {e}")))?;
+
+    tokio::fs::write(&tmp_path, &json_str)
+        .await
+        .map_err(|e| AppError::Internal(format!("Failed to write prompt template: {e}")))?;
+
+    tokio::fs::rename(&tmp_path, PROMPT_TEMPLATE_PATH)
+        .await
+        .map_err(|e| AppError::Internal(format!("Failed to save prompt template: {e}")))?;
+
+    // Audit log (best-effort)
+    if let Err(e) = sqlx::query(
+        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
+           VALUES (?, NULL, 'prompt_template_updated', 'prompt_template', 'default', ?, datetime('now'))"#,
+    )
+    .bind(Uuid::new_v4().to_string())
+    .bind(&json_str)
+    .execute(&state.db)
+    .await
+    {
+        tracing::warn!("Failed to write audit log for prompt template update: {e}");
+    }
+
+    Ok(Json(config))
 }
