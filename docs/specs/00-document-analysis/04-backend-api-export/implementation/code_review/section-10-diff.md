diff --git a/backend/src/features/analysis/routes.rs b/backend/src/features/analysis/routes.rs
index 1adc143..7a0e82c 100644
--- a/backend/src/features/analysis/routes.rs
+++ b/backend/src/features/analysis/routes.rs
@@ -1,6 +1,6 @@
 use axum::{
     extract::{DefaultBodyLimit, Json, Multipart, Path, Query, State},
-    http::StatusCode,
+    http::{header, StatusCode},
     response::IntoResponse,
     routing::{get, post},
     Router,
@@ -10,11 +10,20 @@ use uuid::Uuid;
 use crate::error::{AppError, AppResult};
 use crate::AppState;
 use super::engine::MatchingEngine;
+use super::export_docx;
+use super::export_pdf;
 use super::matcher::DeterministicMatcher;
-use super::models::{AnalysisListQuery, CreateAnalysisRequest, FindingsListQuery, InputType};
+use super::models::{
+    Analysis, AnalysisFindingWithConcept, AnalysisListQuery, AnalysisRow, CreateAnalysisRequest,
+    FindingType, FindingsListQuery, InputType,
+};
 use super::parser::parse_async;
 use super::upload::{validate_upload, save_upload};
 
+const PDF_CONTENT_TYPE: &str = "application/pdf";
+const DOCX_CONTENT_TYPE: &str =
+    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
+
 pub fn router() -> Router<AppState> {
     let upload_routes = Router::new()
         .route("/upload", post(upload_analysis))
@@ -446,24 +455,158 @@ pub async fn delete_analysis(
     Ok(StatusCode::NO_CONTENT)
 }
 
+/// Export an analysis report in the specified format (pdf or docx).
 #[utoipa::path(
     get,
     path = "/api/analyses/{id}/export/{format}",
     tag = "analysis",
     params(
         ("id" = String, Path, description = "Analysis ID"),
-        ("format" = String, Path, description = "Export format (pdf or docx)"),
+        ("format" = String, Path, description = "Export format: pdf or docx"),
     ),
     responses(
-        (status = 200, description = "Exported document"),
-        (status = 501, description = "Not yet implemented"),
+        (status = 200, description = "Exported document bytes"),
+        (status = 400, description = "Invalid export format"),
+        (status = 404, description = "Analysis not found"),
+        (status = 500, description = "Export generation failed"),
     )
 )]
 pub async fn export_analysis(
-    State(_state): State<AppState>,
-    Path((_id, _format)): Path<(String, String)>,
+    State(state): State<AppState>,
+    Path((id, format)): Path<(String, String)>,
 ) -> AppResult<impl IntoResponse> {
-    Ok(StatusCode::NOT_IMPLEMENTED)
+    // 1. Validate format
+    if format != "pdf" && format != "docx" {
+        return Err(AppError::BadRequest(
+            "Invalid export format. Must be 'pdf' or 'docx'".into(),
+        ));
+    }
+
+    // 2. Load analysis
+    let row: AnalysisRow = sqlx::query_as(
+        "SELECT id, name, description, input_type, input_text, original_filename, file_path, extracted_text, status, error_message, prompt_template, matched_framework_ids, processing_time_ms, token_count, created_by, created_at, updated_at FROM analyses WHERE id = ? AND status != 'deleted'"
+    )
+    .bind(&id)
+    .fetch_optional(&state.db)
+    .await?
+    .ok_or_else(|| AppError::NotFound(format!("Analysis {id} not found")))?;
+
+    let analysis = Analysis::from(row);
+
+    // 3. Load findings with concept metadata
+    let finding_rows = sqlx::query(
+        "SELECT f.id, f.analysis_id, f.concept_id, f.framework_id, f.finding_type, f.confidence_score, f.evidence_text, f.recommendation, f.priority, f.sort_order, f.created_at, c.code, c.name_en, COALESCE(c.name_nb, '') as name_nb, COALESCE(c.definition_en, '') as definition_en, c.definition_nb, c.source_reference FROM analysis_findings f LEFT JOIN concepts c ON f.concept_id = c.id WHERE f.analysis_id = ? ORDER BY f.sort_order ASC"
+    )
+    .bind(&id)
+    .fetch_all(&state.db)
+    .await?;
+
+    let findings: Vec<AnalysisFindingWithConcept> = finding_rows
+        .into_iter()
+        .map(|r| {
+            use sqlx::Row;
+            AnalysisFindingWithConcept {
+                id: r.get("id"),
+                analysis_id: r.get("analysis_id"),
+                concept_id: r.get("concept_id"),
+                framework_id: r.get("framework_id"),
+                finding_type: FindingType::from(r.get::<String, _>("finding_type")),
+                confidence_score: r.get("confidence_score"),
+                evidence_text: r.get("evidence_text"),
+                recommendation: r.get("recommendation"),
+                priority: r.get("priority"),
+                sort_order: r.get("sort_order"),
+                created_at: r.get("created_at"),
+                concept_code: r.get("code"),
+                concept_name_en: r.get("name_en"),
+                concept_name_nb: r.get("name_nb"),
+                concept_definition_en: r.get("definition_en"),
+                concept_definition_nb: r.get("definition_nb"),
+                source_reference: r.get("source_reference"),
+            }
+        })
+        .collect();
+
+    // 4. Build framework list (id, display name)
+    let frameworks: Vec<(String, String)> = {
+        let mut fws = Vec::new();
+        for fw_id in &analysis.matched_framework_ids {
+            let name: Option<(String,)> =
+                sqlx::query_as("SELECT name FROM frameworks WHERE id = ?")
+                    .bind(fw_id)
+                    .fetch_optional(&state.db)
+                    .await?;
+            let display_name = name.map(|n| n.0).unwrap_or_else(|| fw_id.clone());
+            fws.push((fw_id.clone(), display_name));
+        }
+        fws
+    };
+
+    // 5. Save name for filename before moving analysis into closure
+    let analysis_name = analysis.name.clone();
+
+    // Generate export (CPU-bound, use spawn_blocking)
+    let format_clone = format.clone();
+    let bytes = tokio::task::spawn_blocking(move || match format_clone.as_str() {
+        "pdf" => export_pdf::generate_pdf(&analysis, &findings, &frameworks)
+            .map_err(|e| AppError::Internal(format!("Export generation failed: {e}"))),
+        "docx" => export_docx::generate_docx(&analysis, &findings, &frameworks)
+            .map_err(|e| AppError::Internal(format!("Export generation failed: {e}"))),
+        _ => unreachable!(),
+    })
+    .await
+    .map_err(|e| AppError::Internal(format!("Export task failed: {e}")))?
+    ?;
+
+    // 6. Build response headers
+    let (content_type, ext) = match format.as_str() {
+        "pdf" => (PDF_CONTENT_TYPE, "pdf"),
+        "docx" => (DOCX_CONTENT_TYPE, "docx"),
+        _ => unreachable!(),
+    };
+
+    let safe_name = sanitize_filename(&analysis_name);
+    let date = chrono::Utc::now().format("%Y-%m-%d");
+    let content_disposition = format!("attachment; filename=\"{safe_name}_{date}.{ext}\"");
+
+    // 7. Audit log
+    sqlx::query(
+        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
+           VALUES (?, NULL, 'analysis_exported', 'analysis', ?, ?, datetime('now'))"#,
+    )
+    .bind(Uuid::new_v4().to_string())
+    .bind(&id)
+    .bind(&format)
+    .execute(&state.db)
+    .await?;
+
+    Ok((
+        [
+            (header::CONTENT_TYPE, content_type.to_string()),
+            (header::CONTENT_DISPOSITION, content_disposition),
+        ],
+        bytes,
+    ))
+}
+
+/// Sanitize a string for use in a filename: keep alphanumeric, hyphens, underscores.
+fn sanitize_filename(name: &str) -> String {
+    let sanitized: String = name
+        .chars()
+        .map(|c| {
+            if c.is_alphanumeric() || c == '-' || c == '_' {
+                c
+            } else {
+                '_'
+            }
+        })
+        .collect();
+    let truncated: String = sanitized.chars().take(100).collect();
+    if truncated.is_empty() {
+        "export".to_string()
+    } else {
+        truncated
+    }
 }
 
 #[utoipa::path(
