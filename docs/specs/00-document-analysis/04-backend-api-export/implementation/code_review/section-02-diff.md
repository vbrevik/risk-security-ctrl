diff --git a/backend/src/features/analysis/mod.rs b/backend/src/features/analysis/mod.rs
index 96ee99e..8bde393 100644
--- a/backend/src/features/analysis/mod.rs
+++ b/backend/src/features/analysis/mod.rs
@@ -2,4 +2,5 @@ pub mod engine;
 pub mod matcher;
 pub mod models;
 pub mod parser;
+pub mod routes;
 pub mod tokenizer;
diff --git a/backend/src/features/analysis/routes.rs b/backend/src/features/analysis/routes.rs
new file mode 100644
index 0000000..6975769
--- /dev/null
+++ b/backend/src/features/analysis/routes.rs
@@ -0,0 +1,189 @@
+use axum::{
+    extract::{DefaultBodyLimit, Json, Multipart, Path, Query, State},
+    http::StatusCode,
+    response::IntoResponse,
+    routing::{get, post},
+    Router,
+};
+
+use crate::error::AppResult;
+use crate::AppState;
+use super::models::{AnalysisListQuery, CreateAnalysisRequest, FindingsListQuery};
+
+pub fn router() -> Router<AppState> {
+    let upload_routes = Router::new()
+        .route("/upload", post(upload_analysis))
+        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));
+
+    Router::new()
+        .route("/", get(list_analyses).post(create_analysis))
+        .route("/{id}", get(get_analysis).delete(delete_analysis))
+        .route("/{id}/findings", get(get_findings))
+        .route("/{id}/export/{format}", get(export_analysis))
+        .route("/prompt-template", get(get_prompt_template).put(update_prompt_template))
+        .merge(upload_routes)
+}
+
+#[utoipa::path(
+    post,
+    path = "/api/analyses",
+    tag = "analysis",
+    request_body = CreateAnalysisRequest,
+    responses(
+        (status = 201, description = "Analysis created"),
+        (status = 400, description = "Invalid request"),
+    )
+)]
+async fn create_analysis(
+    State(_state): State<AppState>,
+    Json(_body): Json<CreateAnalysisRequest>,
+) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
+    Ok((StatusCode::CREATED, Json(serde_json::json!({}))))
+}
+
+#[utoipa::path(
+    post,
+    path = "/api/analyses/upload",
+    tag = "analysis",
+    responses(
+        (status = 201, description = "Analysis created from uploaded file"),
+        (status = 400, description = "Invalid file"),
+    )
+)]
+async fn upload_analysis(
+    State(_state): State<AppState>,
+    _multipart: Multipart,
+) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
+    Ok((StatusCode::CREATED, Json(serde_json::json!({}))))
+}
+
+#[utoipa::path(
+    get,
+    path = "/api/analyses",
+    tag = "analysis",
+    params(AnalysisListQuery),
+    responses(
+        (status = 200, description = "List of analyses")
+    )
+)]
+async fn list_analyses(
+    State(_state): State<AppState>,
+    Query(_query): Query<AnalysisListQuery>,
+) -> AppResult<Json<serde_json::Value>> {
+    Ok(Json(serde_json::json!({
+        "items": [],
+        "total": 0,
+        "page": 1,
+        "limit": 50,
+        "total_pages": 0
+    })))
+}
+
+#[utoipa::path(
+    get,
+    path = "/api/analyses/{id}",
+    tag = "analysis",
+    params(("id" = String, Path, description = "Analysis ID")),
+    responses(
+        (status = 200, description = "Analysis details"),
+        (status = 404, description = "Analysis not found"),
+    )
+)]
+async fn get_analysis(
+    State(_state): State<AppState>,
+    Path(_id): Path<String>,
+) -> AppResult<Json<serde_json::Value>> {
+    Ok(Json(serde_json::json!({})))
+}
+
+#[utoipa::path(
+    get,
+    path = "/api/analyses/{id}/findings",
+    tag = "analysis",
+    params(
+        ("id" = String, Path, description = "Analysis ID"),
+        FindingsListQuery,
+    ),
+    responses(
+        (status = 200, description = "List of findings for analysis")
+    )
+)]
+async fn get_findings(
+    State(_state): State<AppState>,
+    Path(_id): Path<String>,
+    Query(_query): Query<FindingsListQuery>,
+) -> AppResult<Json<serde_json::Value>> {
+    Ok(Json(serde_json::json!({
+        "items": [],
+        "total": 0,
+        "page": 1,
+        "limit": 50,
+        "total_pages": 0
+    })))
+}
+
+#[utoipa::path(
+    delete,
+    path = "/api/analyses/{id}",
+    tag = "analysis",
+    params(("id" = String, Path, description = "Analysis ID")),
+    responses(
+        (status = 204, description = "Analysis deleted"),
+        (status = 404, description = "Analysis not found"),
+    )
+)]
+async fn delete_analysis(
+    State(_state): State<AppState>,
+    Path(_id): Path<String>,
+) -> AppResult<StatusCode> {
+    Ok(StatusCode::NO_CONTENT)
+}
+
+#[utoipa::path(
+    get,
+    path = "/api/analyses/{id}/export/{format}",
+    tag = "analysis",
+    params(
+        ("id" = String, Path, description = "Analysis ID"),
+        ("format" = String, Path, description = "Export format (pdf or docx)"),
+    ),
+    responses(
+        (status = 200, description = "Exported document"),
+        (status = 501, description = "Not yet implemented"),
+    )
+)]
+async fn export_analysis(
+    State(_state): State<AppState>,
+    Path((_id, _format)): Path<(String, String)>,
+) -> AppResult<impl IntoResponse> {
+    Ok(StatusCode::NOT_IMPLEMENTED)
+}
+
+#[utoipa::path(
+    get,
+    path = "/api/analyses/prompt-template",
+    tag = "analysis",
+    responses(
+        (status = 200, description = "Current prompt template")
+    )
+)]
+async fn get_prompt_template(
+    State(_state): State<AppState>,
+) -> AppResult<Json<serde_json::Value>> {
+    Ok(Json(serde_json::json!({})))
+}
+
+#[utoipa::path(
+    put,
+    path = "/api/analyses/prompt-template",
+    tag = "analysis",
+    responses(
+        (status = 200, description = "Prompt template updated")
+    )
+)]
+async fn update_prompt_template(
+    State(_state): State<AppState>,
+    Json(_body): Json<serde_json::Value>,
+) -> AppResult<Json<serde_json::Value>> {
+    Ok(Json(serde_json::json!({})))
+}
diff --git a/backend/src/lib.rs b/backend/src/lib.rs
index bb45aa5..3e7cadc 100644
--- a/backend/src/lib.rs
+++ b/backend/src/lib.rs
@@ -86,4 +86,5 @@ fn api_routes() -> Router<AppState> {
         .nest("/compliance", features::compliance::routes::router())
         .nest("/reports", features::reports::routes::router())
         .nest("/auth", features::auth::routes::router())
+        .nest("/analyses", features::analysis::routes::router())
 }
diff --git a/backend/src/main.rs b/backend/src/main.rs
index 9d6b98b..94c4a6d 100644
--- a/backend/src/main.rs
+++ b/backend/src/main.rs
@@ -23,6 +23,7 @@ use utoipa_swagger_ui::SwaggerUi;
         (name = "compliance", description = "Compliance tracking endpoints"),
         (name = "reports", description = "Reporting endpoints"),
         (name = "auth", description = "Authentication endpoints"),
+        (name = "analysis", description = "Document analysis endpoints"),
     )
 )]
 struct ApiDoc;
diff --git a/backend/tests/analysis_tests.rs b/backend/tests/analysis_tests.rs
new file mode 100644
index 0000000..cf9b480
--- /dev/null
+++ b/backend/tests/analysis_tests.rs
@@ -0,0 +1,31 @@
+use axum::{
+    body::Body,
+    http::{Request, StatusCode},
+};
+use tower::ServiceExt;
+
+mod common;
+use common::create_test_app;
+
+/// Proves the analysis router is registered under /api/analyses.
+/// A GET to the list endpoint should return 200 (not 404).
+#[tokio::test]
+async fn test_router_registration() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .method("GET")
+                .uri("/api/analyses")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    // Must not be 404 — that would mean the route is not registered.
+    assert_ne!(response.status(), StatusCode::NOT_FOUND);
+    // The stub handler should return 200 with an empty list or similar.
+    assert_eq!(response.status(), StatusCode::OK);
+}
