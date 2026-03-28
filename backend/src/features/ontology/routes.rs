use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::Row;
use utoipa::ToSchema;

use crate::AppState;

use super::models::{
    ActionResponse, Concept, ConceptGuidanceResponse, ConceptListQuery,
    ConceptWithRelationships, Framework, PaginatedResponse, ProofResponse, QuestionResponse,
    ReferenceResponse, RelatedConcept, Relationship, SearchQuery, Topic, TopicTagsFile,
};

/// Health check response
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// List all frameworks
#[utoipa::path(
    get,
    path = "/api/ontology/frameworks",
    tag = "ontology",
    responses(
        (status = 200, description = "List of frameworks", body = Vec<Framework>)
    )
)]
pub async fn list_frameworks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Framework>>, StatusCode> {
    let frameworks = sqlx::query_as::<_, Framework>(
        r#"SELECT id, name, version, description, source_url, verification_status, verification_date, verification_source, verification_notes, created_at, updated_at FROM frameworks ORDER BY name"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(frameworks))
}

/// Get framework by ID
#[utoipa::path(
    get,
    path = "/api/ontology/frameworks/{id}",
    tag = "ontology",
    params(
        ("id" = String, Path, description = "Framework ID")
    ),
    responses(
        (status = 200, description = "Framework details", body = Framework),
        (status = 404, description = "Framework not found")
    )
)]
pub async fn get_framework(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Framework>, StatusCode> {
    let framework = sqlx::query_as::<_, Framework>(
        r#"SELECT id, name, version, description, source_url, verification_status, verification_date, verification_source, verification_notes, created_at, updated_at FROM frameworks WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(framework))
}

/// List concepts with pagination and filtering
#[utoipa::path(
    get,
    path = "/api/ontology/concepts",
    tag = "ontology",
    params(
        ConceptListQuery
    ),
    responses(
        (status = 200, description = "Paginated list of concepts", body = PaginatedResponse<Concept>)
    )
)]
pub async fn list_concepts(
    State(state): State<AppState>,
    Query(query): Query<ConceptListQuery>,
) -> Result<Json<PaginatedResponse<Concept>>, StatusCode> {
    let offset = (query.page - 1) * query.limit;

    // Build WHERE clause
    let (where_clause, framework_param, type_param) =
        match (&query.framework_id, &query.concept_type) {
            (Some(fw), Some(ct)) => (
                "WHERE framework_id = ? AND concept_type = ?",
                Some(fw.clone()),
                Some(ct.clone()),
            ),
            (Some(fw), None) => ("WHERE framework_id = ?", Some(fw.clone()), None),
            (None, Some(ct)) => ("WHERE concept_type = ?", None, Some(ct.clone())),
            (None, None) => ("", None, None),
        };

    // Count total
    let count_query = format!("SELECT COUNT(*) as count FROM concepts {}", where_clause);
    let total: i64 = match (&framework_param, &type_param) {
        (Some(fw), Some(ct)) => sqlx::query_scalar(&count_query)
            .bind(fw)
            .bind(ct)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (Some(fw), None) => sqlx::query_scalar(&count_query)
            .bind(fw)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (None, Some(ct)) => sqlx::query_scalar(&count_query)
            .bind(ct)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (None, None) => sqlx::query_scalar(&count_query)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    // Fetch concepts
    let concepts_query = format!(
        "SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at FROM concepts {} ORDER BY framework_id, sort_order, name_en LIMIT ? OFFSET ?",
        where_clause
    );

    let concepts: Vec<Concept> = match (&framework_param, &type_param) {
        (Some(fw), Some(ct)) => sqlx::query_as(&concepts_query)
            .bind(fw)
            .bind(ct)
            .bind(query.limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (Some(fw), None) => sqlx::query_as(&concepts_query)
            .bind(fw)
            .bind(query.limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (None, Some(ct)) => sqlx::query_as(&concepts_query)
            .bind(ct)
            .bind(query.limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        (None, None) => sqlx::query_as(&concepts_query)
            .bind(query.limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    Ok(Json(PaginatedResponse::new(
        concepts,
        total,
        query.page,
        query.limit,
    )))
}

/// Get concept by ID
#[utoipa::path(
    get,
    path = "/api/ontology/concepts/{id}",
    tag = "ontology",
    params(
        ("id" = String, Path, description = "Concept ID")
    ),
    responses(
        (status = 200, description = "Concept details", body = Concept),
        (status = 404, description = "Concept not found")
    )
)]
pub async fn get_concept(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Concept>, StatusCode> {
    let concept = sqlx::query_as::<_, Concept>(
        r#"SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at FROM concepts WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(concept))
}

/// Get concept with its relationships
#[utoipa::path(
    get,
    path = "/api/ontology/concepts/{id}/relationships",
    tag = "ontology",
    params(
        ("id" = String, Path, description = "Concept ID")
    ),
    responses(
        (status = 200, description = "Concept with relationships", body = ConceptWithRelationships),
        (status = 404, description = "Concept not found")
    )
)]
pub async fn get_concept_relationships(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ConceptWithRelationships>, StatusCode> {
    // Get the concept
    let concept = sqlx::query_as::<_, Concept>(
        r#"SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at FROM concepts WHERE id = ?"#
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Get outgoing relationships
    let outgoing: Vec<RelatedConcept> = sqlx::query_as(
        r#"
        SELECT
            r.id as relationship_id,
            r.relationship_type,
            c.id as concept_id,
            c.framework_id as concept_framework_id,
            c.name_en as concept_name_en,
            c.name_nb as concept_name_nb,
            'outgoing' as direction
        FROM relationships r
        JOIN concepts c ON c.id = r.target_concept_id
        WHERE r.source_concept_id = ?
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get incoming relationships
    let incoming: Vec<RelatedConcept> = sqlx::query_as(
        r#"
        SELECT
            r.id as relationship_id,
            r.relationship_type,
            c.id as concept_id,
            c.framework_id as concept_framework_id,
            c.name_en as concept_name_en,
            c.name_nb as concept_name_nb,
            'incoming' as direction
        FROM relationships r
        JOIN concepts c ON c.id = r.source_concept_id
        WHERE r.target_concept_id = ?
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Combine relationships
    let mut related_concepts = outgoing;
    related_concepts.extend(incoming);

    // Query guidance data concurrently (4 queries in parallel)
    let guidance_query = sqlx::query(
        "SELECT source_pdf, source_page, about_en, about_nb FROM concept_guidance WHERE concept_id = ?",
    )
    .bind(&id)
    .fetch_optional(&state.db);

    let actions_query = sqlx::query(
        "SELECT action_text_en, action_text_nb, sort_order FROM concept_actions WHERE concept_id = ? ORDER BY sort_order",
    )
    .bind(&id)
    .fetch_all(&state.db);

    let questions_query = sqlx::query(
        "SELECT question_text_en, question_text_nb, sort_order FROM concept_transparency_questions WHERE concept_id = ? ORDER BY sort_order",
    )
    .bind(&id)
    .fetch_all(&state.db);

    let references_query = sqlx::query(
        "SELECT reference_type, title, authors, year, venue, url, sort_order FROM concept_references WHERE concept_id = ? ORDER BY sort_order",
    )
    .bind(&id)
    .fetch_all(&state.db);

    let (guidance_row, actions_rows, questions_rows, references_rows) = tokio::try_join!(
        guidance_query,
        actions_query,
        questions_query,
        references_query,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Assemble guidance if present
    let guidance = guidance_row.map(|row| {
        let suggested_actions = actions_rows
            .iter()
            .map(|r| ActionResponse {
                sort_order: r.get("sort_order"),
                text_en: r.get("action_text_en"),
                text_nb: r.get("action_text_nb"),
            })
            .collect();
        let transparency_questions = questions_rows
            .iter()
            .map(|r| QuestionResponse {
                sort_order: r.get("sort_order"),
                text_en: r.get("question_text_en"),
                text_nb: r.get("question_text_nb"),
            })
            .collect();
        let references = references_rows
            .iter()
            .map(|r| ReferenceResponse {
                reference_type: r.get("reference_type"),
                title: r.get("title"),
                authors: r.get("authors"),
                year: r.get("year"),
                venue: r.get("venue"),
                url: r.get("url"),
            })
            .collect();
        ConceptGuidanceResponse {
            source_pdf: row.get("source_pdf"),
            source_page: row.get("source_page"),
            about_en: row.get("about_en"),
            about_nb: row.get("about_nb"),
            suggested_actions,
            transparency_questions,
            references,
        }
    });

    Ok(Json(ConceptWithRelationships {
        concept,
        related_concepts,
        guidance,
    }))
}

/// Search concepts by full-text search
#[utoipa::path(
    get,
    path = "/api/ontology/concepts/search",
    tag = "ontology",
    params(
        SearchQuery
    ),
    responses(
        (status = 200, description = "Search results", body = PaginatedResponse<Concept>)
    )
)]
pub async fn search_concepts(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<PaginatedResponse<Concept>>, StatusCode> {
    let offset = (query.page - 1) * query.limit;
    let search_term = format!("%{}%", query.q);

    // Count total
    let total: i64 = if let Some(ref fw) = query.framework_id {
        sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM concepts
               WHERE framework_id = ?
               AND (name_en LIKE ? OR name_nb LIKE ? OR definition_en LIKE ? OR definition_nb LIKE ?)"#
        )
        .bind(fw)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM concepts
               WHERE name_en LIKE ? OR name_nb LIKE ? OR definition_en LIKE ? OR definition_nb LIKE ?"#
        )
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Fetch concepts
    let concepts: Vec<Concept> = if let Some(ref fw) = query.framework_id {
        sqlx::query_as(
            r#"SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at
               FROM concepts
               WHERE framework_id = ?
               AND (name_en LIKE ? OR name_nb LIKE ? OR definition_en LIKE ? OR definition_nb LIKE ?)
               ORDER BY name_en
               LIMIT ? OFFSET ?"#
        )
        .bind(fw)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(query.limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as(
            r#"SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at
               FROM concepts
               WHERE name_en LIKE ? OR name_nb LIKE ? OR definition_en LIKE ? OR definition_nb LIKE ?
               ORDER BY name_en
               LIMIT ? OFFSET ?"#
        )
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(&search_term)
        .bind(query.limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(PaginatedResponse::new(
        concepts,
        total,
        query.page,
        query.limit,
    )))
}

/// List all relationships
#[utoipa::path(
    get,
    path = "/api/ontology/relationships",
    tag = "ontology",
    responses(
        (status = 200, description = "List of relationships", body = Vec<Relationship>)
    )
)]
pub async fn list_relationships(
    State(state): State<AppState>,
) -> Result<Json<Vec<Relationship>>, StatusCode> {
    let relationships = sqlx::query_as::<_, Relationship>(
        r#"SELECT id, source_concept_id, target_concept_id, relationship_type, description, created_at FROM relationships"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(relationships))
}

/// List all topic tags for cross-cutting theme filtering
#[utoipa::path(
    get,
    path = "/api/ontology/topics",
    tag = "ontology",
    responses(
        (status = 200, description = "List of topic tags", body = Vec<Topic>)
    )
)]
pub async fn list_topics() -> Result<Json<Vec<Topic>>, StatusCode> {
    let file_path = std::path::Path::new("../ontology-data/topic-tags.json");
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let topic_tags: TopicTagsFile =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(topic_tags.topics))
}

/// Get verification proof for a framework
#[utoipa::path(
    get,
    path = "/api/ontology/frameworks/{id}/proof",
    tag = "ontology",
    params(
        ("id" = String, Path, description = "Framework ID")
    ),
    responses(
        (status = 200, description = "Verification proof data", body = ProofResponse),
        (status = 404, description = "Framework not found")
    )
)]
pub async fn get_framework_proof(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProofResponse>, StatusCode> {
    // Validate framework exists and get verification metadata
    let row = sqlx::query_as::<_, Framework>(
        r#"SELECT id, name, version, description, source_url, verification_status, verification_date, verification_source, verification_notes, created_at, updated_at FROM frameworks WHERE id = ?"#
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Derive proof file path server-side from framework ID (never from client input)
    let proof_path = format!("../docs/sources/{}-proof.md", row.id);
    let proof_content = tokio::fs::read_to_string(&proof_path).await.ok();

    Ok(Json(ProofResponse {
        framework_id: row.id,
        verification_status: row.verification_status,
        verification_date: row.verification_date,
        verification_source: row.verification_source,
        verification_notes: row.verification_notes,
        proof_content,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/frameworks", get(list_frameworks))
        .route("/frameworks/:id", get(get_framework))
        .route("/frameworks/:id/proof", get(get_framework_proof))
        .route("/concepts", get(list_concepts))
        .route("/concepts/search", get(search_concepts))
        .route("/concepts/:id", get(get_concept))
        .route(
            "/concepts/:id/relationships",
            get(get_concept_relationships),
        )
        .route("/relationships", get(list_relationships))
        .route("/topics", get(list_topics))
}
