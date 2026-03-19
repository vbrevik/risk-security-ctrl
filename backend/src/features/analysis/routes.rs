use axum::{
    extract::{DefaultBodyLimit, Json, Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::AppState;
use super::engine::MatchingEngine;
use super::matcher::DeterministicMatcher;
use super::models::{AnalysisListQuery, CreateAnalysisRequest, FindingsListQuery, InputType};
use super::parser::parse_async;
use super::upload::{validate_upload, save_upload};

pub fn router() -> Router<AppState> {
    let upload_routes = Router::new()
        .route("/upload", post(upload_analysis))
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024));

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
    State(state): State<AppState>,
    Json(body): Json<CreateAnalysisRequest>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if body.input_text.trim().is_empty() {
        return Err(AppError::BadRequest("input_text is required".into()));
    }
    let input_text = &body.input_text;

    // Insert analysis with status "processing"
    sqlx::query(
        "INSERT INTO analyses (id, name, description, input_type, input_text, extracted_text, status, prompt_template, created_at, updated_at) VALUES (?, ?, ?, 'text', ?, ?, 'processing', ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.name)
    .bind(&body.description)
    .bind(input_text)
    .bind(input_text)
    .bind(&body.prompt_template)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Run matching engine
    let matcher = DeterministicMatcher::new(state.topics.clone());
    match matcher.analyze(input_text, body.prompt_template.as_deref(), &state.db).await {
        Ok(result) => {
            let fw_ids = serde_json::to_string(&result.matched_framework_ids).unwrap_or_default();
            sqlx::query(
                "UPDATE analyses SET status = 'completed', matched_framework_ids = ?, processing_time_ms = ?, token_count = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&fw_ids)
            .bind(result.processing_time_ms)
            .bind(result.token_count)
            .bind(&now)
            .bind(&id)
            .execute(&state.db)
            .await?;

            // Insert findings
            for (i, finding) in result.findings.iter().enumerate() {
                let finding_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO analysis_findings (id, analysis_id, concept_id, framework_id, finding_type, confidence_score, evidence_text, recommendation, priority, sort_order, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&finding_id)
                .bind(&id)
                .bind(&finding.concept_id)
                .bind(&finding.framework_id)
                .bind(String::from(finding.finding_type.clone()))
                .bind(finding.confidence_score)
                .bind(&finding.evidence_text)
                .bind(&finding.recommendation)
                .bind(finding.priority)
                .bind(i as i64)
                .bind(&now)
                .execute(&state.db)
                .await?;
            }
        }
        Err(e) => {
            sqlx::query("UPDATE analyses SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(e.to_string())
                .bind(&now)
                .bind(&id)
                .execute(&state.db)
                .await?;
        }
    }

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
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
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file_data: Option<(String, Vec<u8>)> = None;
    let mut name = String::from("Uploaded analysis");

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "file" => {
                let filename = field.file_name().unwrap_or("unnamed").to_string();
                let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;
                file_data = Some((filename, data.to_vec()));
            }
            "name" => {
                name = field.text().await.map_err(|e| AppError::BadRequest(e.to_string()))?;
            }
            _ => {}
        }
    }

    let (filename, data) = file_data.ok_or_else(|| AppError::BadRequest("no file field in upload".into()))?;

    // Validate upload
    let header = &data[..data.len().min(8)];
    let input_type = validate_upload(&filename, data.len() as u64, header)?;
    let input_type_str = match input_type {
        InputType::Pdf => "pdf",
        InputType::Docx => "docx",
        InputType::Text => "text",
    };

    // Save file
    let file_path = save_upload(&id, &filename, &data)?;

    // Insert analysis with status "processing"
    sqlx::query(
        "INSERT INTO analyses (id, name, input_type, original_filename, file_path, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'processing', ?, ?)"
    )
    .bind(&id)
    .bind(&name)
    .bind(input_type_str)
    .bind(&filename)
    .bind(file_path.to_str().unwrap_or(""))
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Parse document
    let parsed = match parse_async(file_path).await {
        Ok(doc) => doc,
        Err(e) => {
            let err_msg = e.to_string();
            sqlx::query("UPDATE analyses SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(&err_msg)
                .bind(&now)
                .bind(&id)
                .execute(&state.db)
                .await?;
            return Err(e.into());
        }
    };

    // Update with extracted text
    sqlx::query("UPDATE analyses SET extracted_text = ?, token_count = ?, updated_at = ? WHERE id = ?")
        .bind(&parsed.full_text)
        .bind(parsed.token_count_estimate as i64)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await?;

    // Run matching engine
    let matcher = DeterministicMatcher::new(state.topics.clone());
    match matcher.analyze(&parsed.full_text, None, &state.db).await {
        Ok(result) => {
            let fw_ids = serde_json::to_string(&result.matched_framework_ids).unwrap_or_default();
            sqlx::query(
                "UPDATE analyses SET status = 'completed', matched_framework_ids = ?, processing_time_ms = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&fw_ids)
            .bind(result.processing_time_ms)
            .bind(&now)
            .bind(&id)
            .execute(&state.db)
            .await?;

            for (i, finding) in result.findings.iter().enumerate() {
                let finding_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO analysis_findings (id, analysis_id, concept_id, framework_id, finding_type, confidence_score, evidence_text, recommendation, priority, sort_order, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&finding_id)
                .bind(&id)
                .bind(&finding.concept_id)
                .bind(&finding.framework_id)
                .bind(String::from(finding.finding_type.clone()))
                .bind(finding.confidence_score)
                .bind(&finding.evidence_text)
                .bind(&finding.recommendation)
                .bind(finding.priority)
                .bind(i as i64)
                .bind(&now)
                .execute(&state.db)
                .await?;
            }
        }
        Err(e) => {
            sqlx::query("UPDATE analyses SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(e.to_string())
                .bind(&now)
                .bind(&id)
                .execute(&state.db)
                .await?;
        }
    }

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id, "status": "completed" }))))
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
    State(state): State<AppState>,
    Query(query): Query<AnalysisListQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.max(1);
    let limit = query.limit.min(100);
    let offset = (page - 1) * limit;

    let status_str = query.status.map(|s| String::from(s));

    let (rows, total): (Vec<(String, String, Option<String>, String, String, Option<String>, Option<i64>, String, String)>, i64) = if let Some(ref status) = status_str {
        let rows = sqlx::query_as(
            "SELECT id, name, description, input_type, status, error_message, processing_time_ms, created_at, updated_at FROM analyses WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM analyses WHERE status = ?")
            .bind(status)
            .fetch_one(&state.db)
            .await?;
        (rows, total)
    } else {
        let rows = sqlx::query_as(
            "SELECT id, name, description, input_type, status, error_message, processing_time_ms, created_at, updated_at FROM analyses ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM analyses")
            .fetch_one(&state.db)
            .await?;
        (rows, total)
    };

    let items: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.0, "name": r.1, "description": r.2,
            "input_type": r.3, "status": r.4, "error_message": r.5,
            "processing_time_ms": r.6, "created_at": r.7, "updated_at": r.8
        })
    }).collect();

    let total_pages = ((total as f64) / (limit as f64)).ceil() as i64;

    Ok(Json(serde_json::json!({
        "items": items,
        "total": total,
        "page": page,
        "limit": limit,
        "total_pages": total_pages
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
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let row: Option<(String, String, Option<String>, String, Option<String>, Option<String>, Option<String>, String, Option<String>, Option<String>, Option<i64>, Option<i64>, String, String)> = sqlx::query_as(
        "SELECT id, name, description, input_type, input_text, original_filename, file_path, status, error_message, matched_framework_ids, processing_time_ms, token_count, created_at, updated_at FROM analyses WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;

    let r = row.ok_or_else(|| AppError::NotFound(format!("Analysis {} not found", id)))?;

    Ok(Json(serde_json::json!({
        "id": r.0, "name": r.1, "description": r.2,
        "input_type": r.3, "input_text": r.4,
        "original_filename": r.5, "file_path": r.6,
        "status": r.7, "error_message": r.8,
        "matched_framework_ids": r.9,
        "processing_time_ms": r.10, "token_count": r.11,
        "created_at": r.12, "updated_at": r.13
    })))
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
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<FindingsListQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let page = query.page.max(1);
    let limit = query.limit.min(100);
    let offset = (page - 1) * limit;

    // Build query with optional filters
    let mut sql = String::from(
        "SELECT f.id, f.concept_id, f.framework_id, f.finding_type, f.confidence_score, f.evidence_text, f.recommendation, f.priority, f.sort_order, f.created_at, c.code, c.name_en, c.definition_en FROM analysis_findings f LEFT JOIN concepts c ON f.concept_id = c.id WHERE f.analysis_id = ?"
    );
    let mut count_sql = String::from("SELECT COUNT(*) FROM analysis_findings WHERE analysis_id = ?");

    if let Some(ref fw) = query.framework_id {
        sql.push_str(&format!(" AND f.framework_id = '{}'", fw.replace('\'', "''")));
        count_sql.push_str(&format!(" AND framework_id = '{}'", fw.replace('\'', "''")));
    }
    if let Some(ref ft) = query.finding_type {
        let ft_str = String::from(ft.clone());
        sql.push_str(&format!(" AND f.finding_type = '{}'", ft_str.replace('\'', "''")));
        count_sql.push_str(&format!(" AND finding_type = '{}'", ft_str.replace('\'', "''")));
    }

    sql.push_str(" ORDER BY f.priority ASC, f.sort_order ASC LIMIT ? OFFSET ?");

    let rows: Vec<(String, String, String, String, f64, Option<String>, Option<String>, i64, i64, String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(&sql)
        .bind(&id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?;

    let (total,): (i64,) = sqlx::query_as(&count_sql)
        .bind(&id)
        .fetch_one(&state.db)
        .await?;

    let items: Vec<serde_json::Value> = rows.iter().map(|r| {
        serde_json::json!({
            "id": r.0, "concept_id": r.1, "framework_id": r.2,
            "finding_type": r.3, "confidence_score": r.4,
            "evidence_text": r.5, "recommendation": r.6,
            "priority": r.7, "sort_order": r.8, "created_at": r.9,
            "concept_code": r.10, "concept_name": r.11, "concept_definition": r.12
        })
    }).collect();

    let total_pages = ((total as f64) / (limit as f64)).ceil() as i64;

    Ok(Json(serde_json::json!({
        "items": items, "total": total, "page": page,
        "limit": limit, "total_pages": total_pages
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
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM analyses WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Analysis {} not found", id)));
    }

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
    Ok(Json(serde_json::json!({
        "template": null,
        "description": "Default matching engine configuration"
    })))
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
    Ok(Json(serde_json::json!({ "status": "ok" })))
}
