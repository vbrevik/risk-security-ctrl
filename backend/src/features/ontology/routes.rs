use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::AppState;

use super::models::{
    Concept, ConceptListQuery, ConceptWithRelationships, Framework, PaginatedResponse,
    RelatedConcept, Relationship, SearchQuery,
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
pub async fn list_frameworks(State(state): State<AppState>) -> Result<Json<Vec<Framework>>, StatusCode> {
    let frameworks = sqlx::query_as::<_, Framework>(
        r#"SELECT id, name, version, description, source_url, created_at, updated_at FROM frameworks ORDER BY name"#
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
        r#"SELECT id, name, version, description, source_url, created_at, updated_at FROM frameworks WHERE id = ?"#
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
    let (where_clause, framework_param, type_param) = match (&query.framework_id, &query.concept_type) {
        (Some(fw), Some(ct)) => ("WHERE framework_id = ? AND concept_type = ?", Some(fw.clone()), Some(ct.clone())),
        (Some(fw), None) => ("WHERE framework_id = ?", Some(fw.clone()), None),
        (None, Some(ct)) => ("WHERE concept_type = ?", None, Some(ct.clone())),
        (None, None) => ("", None, None),
    };

    // Count total
    let count_query = format!("SELECT COUNT(*) as count FROM concepts {}", where_clause);
    let total: i64 = match (&framework_param, &type_param) {
        (Some(fw), Some(ct)) => {
            sqlx::query_scalar(&count_query)
                .bind(fw)
                .bind(ct)
                .fetch_one(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (Some(fw), None) => {
            sqlx::query_scalar(&count_query)
                .bind(fw)
                .fetch_one(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (None, Some(ct)) => {
            sqlx::query_scalar(&count_query)
                .bind(ct)
                .fetch_one(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (None, None) => {
            sqlx::query_scalar(&count_query)
                .fetch_one(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
    };

    // Fetch concepts
    let concepts_query = format!(
        "SELECT id, framework_id, parent_id, concept_type, code, name_en, name_nb, definition_en, definition_nb, source_reference, sort_order, created_at, updated_at FROM concepts {} ORDER BY framework_id, sort_order, name_en LIMIT ? OFFSET ?",
        where_clause
    );

    let concepts: Vec<Concept> = match (&framework_param, &type_param) {
        (Some(fw), Some(ct)) => {
            sqlx::query_as(&concepts_query)
                .bind(fw)
                .bind(ct)
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (Some(fw), None) => {
            sqlx::query_as(&concepts_query)
                .bind(fw)
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (None, Some(ct)) => {
            sqlx::query_as(&concepts_query)
                .bind(ct)
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        (None, None) => {
            sqlx::query_as(&concepts_query)
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
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

    Ok(Json(ConceptWithRelationships {
        concept,
        related_concepts,
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
pub async fn list_relationships(State(state): State<AppState>) -> Result<Json<Vec<Relationship>>, StatusCode> {
    let relationships = sqlx::query_as::<_, Relationship>(
        r#"SELECT id, source_concept_id, target_concept_id, relationship_type, description, created_at FROM relationships"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(relationships))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/frameworks", get(list_frameworks))
        .route("/frameworks/:id", get(get_framework))
        .route("/concepts", get(list_concepts))
        .route("/concepts/search", get(search_concepts))
        .route("/concepts/:id", get(get_concept))
        .route("/concepts/:id/relationships", get(get_concept_relationships))
        .route("/relationships", get(list_relationships))
}
