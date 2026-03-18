use axum::{
    extract::{DefaultBodyLimit, Json, Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::error::AppResult;
use crate::AppState;
use super::models::{AnalysisListQuery, CreateAnalysisRequest, FindingsListQuery};

pub fn router() -> Router<AppState> {
    let upload_routes = Router::new()
        .route("/upload", post(upload_analysis))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));

    Router::new()
        .route("/", get(list_analyses).post(create_analysis))
        .route("/{id}", get(get_analysis).delete(delete_analysis))
        .route("/{id}/findings", get(get_findings))
        .route("/{id}/export/{format}", get(export_analysis))
        .route("/prompt-template", get(get_prompt_template).put(update_prompt_template))
        .merge(upload_routes)
}

#[utoipa::path(
    post,
    path = "/api/analyses",
    tag = "analysis",
    request_body = CreateAnalysisRequest,
    responses(
        (status = 201, description = "Analysis created"),
        (status = 400, description = "Invalid request"),
    )
)]
pub async fn create_analysis(
    State(_state): State<AppState>,
    Json(_body): Json<CreateAnalysisRequest>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::CREATED, Json(serde_json::json!({}))))
}

#[utoipa::path(
    post,
    path = "/api/analyses/upload",
    tag = "analysis",
    responses(
        (status = 201, description = "Analysis created from uploaded file"),
        (status = 400, description = "Invalid file"),
    )
)]
pub async fn upload_analysis(
    State(_state): State<AppState>,
    _multipart: Multipart,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::CREATED, Json(serde_json::json!({}))))
}

#[utoipa::path(
    get,
    path = "/api/analyses",
    tag = "analysis",
    params(AnalysisListQuery),
    responses(
        (status = 200, description = "List of analyses")
    )
)]
pub async fn list_analyses(
    State(_state): State<AppState>,
    Query(_query): Query<AnalysisListQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "items": [],
        "total": 0,
        "page": 1,
        "limit": 50,
        "total_pages": 0
    })))
}

#[utoipa::path(
    get,
    path = "/api/analyses/{id}",
    tag = "analysis",
    params(("id" = String, Path, description = "Analysis ID")),
    responses(
        (status = 200, description = "Analysis details"),
        (status = 404, description = "Analysis not found"),
    )
)]
pub async fn get_analysis(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

#[utoipa::path(
    get,
    path = "/api/analyses/{id}/findings",
    tag = "analysis",
    params(
        ("id" = String, Path, description = "Analysis ID"),
        FindingsListQuery,
    ),
    responses(
        (status = 200, description = "List of findings for analysis")
    )
)]
pub async fn get_findings(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
    Query(_query): Query<FindingsListQuery>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "items": [],
        "total": 0,
        "page": 1,
        "limit": 50,
        "total_pages": 0
    })))
}

#[utoipa::path(
    delete,
    path = "/api/analyses/{id}",
    tag = "analysis",
    params(("id" = String, Path, description = "Analysis ID")),
    responses(
        (status = 204, description = "Analysis deleted"),
        (status = 404, description = "Analysis not found"),
    )
)]
pub async fn delete_analysis(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> AppResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/analyses/{id}/export/{format}",
    tag = "analysis",
    params(
        ("id" = String, Path, description = "Analysis ID"),
        ("format" = String, Path, description = "Export format (pdf or docx)"),
    ),
    responses(
        (status = 200, description = "Exported document"),
        (status = 501, description = "Not yet implemented"),
    )
)]
pub async fn export_analysis(
    State(_state): State<AppState>,
    Path((_id, _format)): Path<(String, String)>,
) -> AppResult<impl IntoResponse> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

#[utoipa::path(
    get,
    path = "/api/analyses/prompt-template",
    tag = "analysis",
    responses(
        (status = 200, description = "Current prompt template")
    )
)]
pub async fn get_prompt_template(
    State(_state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

#[utoipa::path(
    put,
    path = "/api/analyses/prompt-template",
    tag = "analysis",
    responses(
        (status = 200, description = "Prompt template updated")
    )
)]
pub async fn update_prompt_template(
    State(_state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}
