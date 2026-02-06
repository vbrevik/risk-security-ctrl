use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::AppState;

use super::models::{
    AddNoteRequest, Assessment, AssessmentListQuery, AssessmentRow, AssessmentStatus,
    AuditLogEntry, ComplianceItemListQuery, ComplianceItemWithConcept, ComplianceScore,
    ComplianceStatus, CreateAssessmentRequest, CreateEvidenceRequest, Evidence, EvidenceRow,
    PaginatedResponse, SectionScore, UpdateAssessmentRequest, UpdateComplianceItemRequest,
};

// ============================================================================
// Assessment CRUD Handlers
// ============================================================================

/// Create a new assessment
///
/// Creates a new compliance assessment for a framework.
/// Automatically generates compliance items from all concepts in the framework.
#[utoipa::path(
    post,
    path = "/api/compliance/assessments",
    tag = "compliance",
    request_body = CreateAssessmentRequest,
    responses(
        (status = 201, description = "Assessment created successfully", body = Assessment),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Framework not found")
    )
)]
pub async fn create_assessment(
    State(state): State<AppState>,
    Json(req): Json<CreateAssessmentRequest>,
) -> AppResult<(StatusCode, Json<Assessment>)> {
    // Validate framework exists
    let framework_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM frameworks WHERE id = ?")
            .bind(&req.framework_id)
            .fetch_optional(&state.db)
            .await?;

    if framework_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Framework with id '{}' not found",
            req.framework_id
        )));
    }

    // Generate assessment ID and timestamps
    let assessment_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let status_str: String = AssessmentStatus::Draft.into();

    // Insert assessment
    sqlx::query(
        r#"INSERT INTO assessments (id, name, description, framework_id, status, created_by, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&assessment_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.framework_id)
    .bind(&status_str)
    .bind(&req.owner_id)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Auto-generate compliance items from framework concepts
    let concepts: Vec<(String,)> = sqlx::query_as("SELECT id FROM concepts WHERE framework_id = ?")
        .bind(&req.framework_id)
        .fetch_all(&state.db)
        .await?;

    for (concept_id,) in &concepts {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"INSERT INTO compliance_items (id, assessment_id, concept_id, status, updated_at)
               VALUES (?, ?, ?, 'not_assessed', ?)"#,
        )
        .bind(&item_id)
        .bind(&assessment_id)
        .bind(concept_id)
        .bind(&now)
        .execute(&state.db)
        .await?;
    }

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let new_value = serde_json::json!({
        "id": assessment_id,
        "name": req.name,
        "description": req.description,
        "framework_id": req.framework_id,
        "status": status_str,
        "owner_id": req.owner_id,
        "compliance_items_created": concepts.len()
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
           VALUES (?, ?, 'create', 'assessment', ?, ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&req.owner_id)
    .bind(&assessment_id)
    .bind(new_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Fetch and return the created assessment
    let row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments WHERE id = ?"#,
    )
    .bind(&assessment_id)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(Assessment::from(row))))
}

/// List assessments with pagination and filtering
///
/// Returns a paginated list of assessments. Supports filtering by framework_id,
/// status, and owner_id.
#[utoipa::path(
    get,
    path = "/api/compliance/assessments",
    tag = "compliance",
    params(AssessmentListQuery),
    responses(
        (status = 200, description = "Paginated list of assessments", body = PaginatedResponse<Assessment>)
    )
)]
pub async fn list_assessments(
    State(state): State<AppState>,
    Query(query): Query<AssessmentListQuery>,
) -> AppResult<Json<PaginatedResponse<Assessment>>> {
    let offset = (query.page - 1) * query.limit;

    // Build WHERE clauses dynamically
    let mut conditions: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(ref fw) = query.framework_id {
        conditions.push("framework_id = ?".to_string());
        params.push(fw.clone());
    }

    if let Some(ref status) = query.status {
        conditions.push("status = ?".to_string());
        let status_str: String = status.clone().into();
        params.push(status_str);
    }

    if let Some(ref owner) = query.owner_id {
        conditions.push("created_by = ?".to_string());
        params.push(owner.clone());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count total
    let count_sql = format!("SELECT COUNT(*) FROM assessments {}", where_clause);
    let total: i64 = match params.len() {
        0 => sqlx::query_scalar(&count_sql).fetch_one(&state.db).await?,
        1 => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .fetch_one(&state.db)
                .await?
        }
        2 => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .bind(&params[1])
                .fetch_one(&state.db)
                .await?
        }
        _ => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(&params[2])
                .fetch_one(&state.db)
                .await?
        }
    };

    // Fetch assessments
    let select_sql = format!(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments {}
           ORDER BY created_at DESC
           LIMIT ? OFFSET ?"#,
        where_clause
    );

    let rows: Vec<AssessmentRow> = match params.len() {
        0 => {
            sqlx::query_as(&select_sql)
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
        1 => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
        2 => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
        _ => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(&params[2])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
    };

    let assessments: Vec<Assessment> = rows.into_iter().map(Assessment::from).collect();

    Ok(Json(PaginatedResponse::new(
        assessments,
        total,
        query.page,
        query.limit,
    )))
}

/// Get a single assessment by ID
///
/// Returns the assessment details for the specified ID.
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID")
    ),
    responses(
        (status = 200, description = "Assessment details", body = Assessment),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Assessment>> {
    let row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments WHERE id = ?"#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Assessment with id '{}' not found", id)))?;

    Ok(Json(Assessment::from(row)))
}

/// Update an assessment
///
/// Updates the specified assessment. Only provided fields will be updated.
#[utoipa::path(
    put,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID")
    ),
    request_body = UpdateAssessmentRequest,
    responses(
        (status = 200, description = "Assessment updated successfully", body = Assessment),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn update_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAssessmentRequest>,
) -> AppResult<Json<Assessment>> {
    // Fetch existing assessment for audit log
    let existing = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments WHERE id = ?"#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Assessment with id '{}' not found", id)))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Build update query dynamically
    let mut updates: Vec<String> = vec!["updated_at = ?".to_string()];
    let mut has_updates = false;

    if req.name.is_some() {
        updates.push("name = ?".to_string());
        has_updates = true;
    }
    if req.description.is_some() {
        updates.push("description = ?".to_string());
        has_updates = true;
    }
    if req.status.is_some() {
        updates.push("status = ?".to_string());
        has_updates = true;
    }
    if req.owner_id.is_some() {
        updates.push("created_by = ?".to_string());
        has_updates = true;
    }

    if !has_updates {
        // Nothing to update, just return existing
        return Ok(Json(Assessment::from(existing)));
    }

    let update_sql = format!("UPDATE assessments SET {} WHERE id = ?", updates.join(", "));

    // Execute update with appropriate bindings
    let mut query = sqlx::query(&update_sql).bind(&now);

    if let Some(ref name) = req.name {
        query = query.bind(name);
    }
    if let Some(ref desc) = req.description {
        query = query.bind(desc);
    }
    if let Some(ref status) = req.status {
        let status_str: String = status.clone().into();
        query = query.bind(status_str);
    }
    if let Some(ref owner) = req.owner_id {
        query = query.bind(owner);
    }

    query.bind(&id).execute(&state.db).await?;

    // Fetch updated assessment
    let updated = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments WHERE id = ?"#,
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let old_value = serde_json::json!({
        "name": existing.name,
        "description": existing.description,
        "status": existing.status,
        "owner_id": existing.owner_id
    });
    let new_value = serde_json::json!({
        "name": updated.name,
        "description": updated.description,
        "status": updated.status,
        "owner_id": updated.owner_id
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, old_value, new_value, created_at)
           VALUES (?, ?, 'update', 'assessment', ?, ?, ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&req.owner_id)
    .bind(&id)
    .bind(old_value.to_string())
    .bind(new_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(Json(Assessment::from(updated)))
}

/// Delete an assessment
///
/// Deletes the specified assessment. All associated compliance items and evidence
/// will be cascade deleted.
#[utoipa::path(
    delete,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID")
    ),
    responses(
        (status = 204, description = "Assessment deleted successfully"),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn delete_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    // Fetch existing assessment for audit log
    let existing = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, framework_id, name, description, status, created_by as owner_id,
           NULL as due_date, created_at, updated_at
           FROM assessments WHERE id = ?"#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Assessment with id '{}' not found", id)))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Delete assessment (compliance_items cascade deleted via ON DELETE CASCADE)
    let result = sqlx::query("DELETE FROM assessments WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            id
        )));
    }

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let old_value = serde_json::json!({
        "id": existing.id,
        "name": existing.name,
        "description": existing.description,
        "framework_id": existing.framework_id,
        "status": existing.status,
        "owner_id": existing.owner_id
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, old_value, created_at)
           VALUES (?, ?, 'delete', 'assessment', ?, ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&existing.owner_id)
    .bind(&id)
    .bind(old_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Compliance Items Handlers
// ============================================================================

/// Database row for compliance item joined with concept
#[derive(Debug, sqlx::FromRow)]
struct ComplianceItemWithConceptRow {
    id: String,
    assessment_id: String,
    concept_id: String,
    status: String,
    notes: Option<String>,
    updated_by: Option<String>,
    updated_at: String,
    concept_name_en: String,
    concept_name_nb: Option<String>,
    concept_code: Option<String>,
    concept_type: String,
    #[allow(dead_code)]
    parent_id: Option<String>,
}

impl From<ComplianceItemWithConceptRow> for ComplianceItemWithConcept {
    fn from(row: ComplianceItemWithConceptRow) -> Self {
        Self {
            id: row.id,
            assessment_id: row.assessment_id,
            concept_id: row.concept_id,
            status: ComplianceStatus::from(row.status),
            notes: row.notes,
            assessed_by: row.updated_by,
            assessed_at: None,
            created_at: row.updated_at.clone(),
            updated_at: row.updated_at,
            concept_code: row.concept_code,
            concept_name_en: row.concept_name_en,
            concept_name_nb: row.concept_name_nb,
            concept_type: row.concept_type,
            concept_definition_en: None,
        }
    }
}

/// Get compliance items for an assessment
///
/// Returns a paginated list of compliance items for the specified assessment,
/// including concept details (name, code, type).
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/items",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID"),
        ComplianceItemListQuery
    ),
    responses(
        (status = 200, description = "Paginated list of compliance items", body = PaginatedResponse<ComplianceItemWithConcept>),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_compliance_items(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<ComplianceItemListQuery>,
) -> AppResult<Json<PaginatedResponse<ComplianceItemWithConcept>>> {
    // Verify assessment exists
    let assessment_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM assessments WHERE id = ?")
            .bind(&id)
            .fetch_optional(&state.db)
            .await?;

    if assessment_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            id
        )));
    }

    let offset = (query.page - 1) * query.limit;

    // Build WHERE clauses dynamically
    let mut conditions: Vec<String> = vec!["ci.assessment_id = ?".to_string()];
    let mut params: Vec<String> = vec![id.clone()];

    if let Some(ref status) = query.status {
        conditions.push("ci.status = ?".to_string());
        let status_str: String = status.clone().into();
        params.push(status_str);
    }

    if let Some(ref concept_type) = query.concept_type {
        conditions.push("c.concept_type = ?".to_string());
        params.push(concept_type.clone());
    }

    let where_clause = format!("WHERE {}", conditions.join(" AND "));

    // Count total
    let count_sql = format!(
        r#"SELECT COUNT(*)
           FROM compliance_items ci
           JOIN concepts c ON ci.concept_id = c.id
           {}"#,
        where_clause
    );

    let total: i64 = match params.len() {
        1 => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .fetch_one(&state.db)
                .await?
        }
        2 => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .bind(&params[1])
                .fetch_one(&state.db)
                .await?
        }
        _ => {
            sqlx::query_scalar(&count_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(&params[2])
                .fetch_one(&state.db)
                .await?
        }
    };

    // Fetch compliance items with concept details
    let select_sql = format!(
        r#"SELECT
               ci.id,
               ci.assessment_id,
               ci.concept_id,
               ci.status,
               ci.notes,
               ci.updated_by,
               ci.updated_at,
               c.name_en as concept_name_en,
               c.name_nb as concept_name_nb,
               c.code as concept_code,
               c.concept_type,
               c.parent_id
           FROM compliance_items ci
           JOIN concepts c ON ci.concept_id = c.id
           {}
           ORDER BY c.sort_order, c.code, c.name_en
           LIMIT ? OFFSET ?"#,
        where_clause
    );

    let rows: Vec<ComplianceItemWithConceptRow> = match params.len() {
        1 => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
        2 => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
        _ => {
            sqlx::query_as(&select_sql)
                .bind(&params[0])
                .bind(&params[1])
                .bind(&params[2])
                .bind(query.limit)
                .bind(offset)
                .fetch_all(&state.db)
                .await?
        }
    };

    let items: Vec<ComplianceItemWithConcept> = rows
        .into_iter()
        .map(ComplianceItemWithConcept::from)
        .collect();

    Ok(Json(PaginatedResponse::new(
        items,
        total,
        query.page,
        query.limit,
    )))
}

/// Update a compliance item
///
/// Updates the status and/or notes of a compliance item.
#[utoipa::path(
    put,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    request_body = UpdateComplianceItemRequest,
    responses(
        (status = 200, description = "Compliance item updated successfully", body = ComplianceItemWithConcept),
        (status = 404, description = "Assessment or compliance item not found")
    )
)]
pub async fn update_compliance_item(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<UpdateComplianceItemRequest>,
) -> AppResult<Json<ComplianceItemWithConcept>> {
    // Verify assessment exists
    let assessment_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM assessments WHERE id = ?")
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    if assessment_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            assessment_id
        )));
    }

    // Fetch existing compliance item
    let existing: Option<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, status, notes FROM compliance_items WHERE id = ? AND assessment_id = ?",
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_optional(&state.db)
    .await?;

    let (_, old_status, old_notes) = existing.ok_or_else(|| {
        AppError::NotFound(format!(
            "Compliance item with id '{}' not found in assessment '{}'",
            item_id, assessment_id
        ))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Build update query dynamically
    let mut updates: Vec<String> = vec!["updated_at = ?".to_string()];
    let mut has_updates = false;

    if req.status.is_some() {
        updates.push("status = ?".to_string());
        has_updates = true;
    }
    if req.notes.is_some() {
        updates.push("notes = ?".to_string());
        has_updates = true;
    }

    if has_updates {
        let update_sql = format!(
            "UPDATE compliance_items SET {} WHERE id = ? AND assessment_id = ?",
            updates.join(", ")
        );

        let mut query = sqlx::query(&update_sql).bind(&now);

        if let Some(ref status) = req.status {
            let status_str: String = status.clone().into();
            query = query.bind(status_str);
        }
        if let Some(ref notes) = req.notes {
            query = query.bind(notes);
        }

        query
            .bind(&item_id)
            .bind(&assessment_id)
            .execute(&state.db)
            .await?;

        // Update assessment's updated_at timestamp
        sqlx::query("UPDATE assessments SET updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&assessment_id)
            .execute(&state.db)
            .await?;

        // Log to audit_log
        let audit_id = Uuid::new_v4().to_string();
        let old_value = serde_json::json!({
            "status": old_status,
            "notes": old_notes
        });
        let new_value = serde_json::json!({
            "status": req.status.clone().map(String::from).unwrap_or(old_status),
            "notes": req.notes.clone().or(old_notes)
        });
        sqlx::query(
            r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, old_value, new_value, created_at)
               VALUES (?, NULL, 'update', 'compliance_item', ?, ?, ?, ?)"#,
        )
        .bind(&audit_id)
        .bind(&item_id)
        .bind(old_value.to_string())
        .bind(new_value.to_string())
        .bind(&now)
        .execute(&state.db)
        .await?;
    }

    // Fetch updated compliance item with concept details
    let row = sqlx::query_as::<_, ComplianceItemWithConceptRow>(
        r#"SELECT
               ci.id,
               ci.assessment_id,
               ci.concept_id,
               ci.status,
               ci.notes,
               ci.updated_by,
               ci.updated_at,
               c.name_en as concept_name_en,
               c.name_nb as concept_name_nb,
               c.code as concept_code,
               c.concept_type,
               c.parent_id
           FROM compliance_items ci
           JOIN concepts c ON ci.concept_id = c.id
           WHERE ci.id = ? AND ci.assessment_id = ?"#,
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ComplianceItemWithConcept::from(row)))
}

/// Add a note to a compliance item
///
/// Appends a timestamped note to the compliance item's existing notes.
#[utoipa::path(
    post,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}/notes",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    request_body = AddNoteRequest,
    responses(
        (status = 200, description = "Note added successfully", body = ComplianceItemWithConcept),
        (status = 404, description = "Assessment or compliance item not found")
    )
)]
pub async fn add_item_note(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<AddNoteRequest>,
) -> AppResult<Json<ComplianceItemWithConcept>> {
    // Verify assessment exists
    let assessment_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM assessments WHERE id = ?")
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    if assessment_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            assessment_id
        )));
    }

    // Fetch existing compliance item
    let existing: Option<(String, Option<String>)> =
        sqlx::query_as("SELECT id, notes FROM compliance_items WHERE id = ? AND assessment_id = ?")
            .bind(&item_id)
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    let (_, old_notes) = existing.ok_or_else(|| {
        AppError::NotFound(format!(
            "Compliance item with id '{}' not found in assessment '{}'",
            item_id, assessment_id
        ))
    })?;

    let now = chrono::Utc::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let timestamp = now.format("%Y-%m-%d %H:%M UTC").to_string();

    // Format the new note with timestamp
    let formatted_note = format!("[{}]\n{}", timestamp, req.note);

    // Append to existing notes or create new
    let new_notes = match old_notes {
        Some(ref existing_notes) if !existing_notes.is_empty() => {
            format!("{}\n\n{}", existing_notes, formatted_note)
        }
        _ => formatted_note,
    };

    // Update the compliance item
    sqlx::query(
        "UPDATE compliance_items SET notes = ?, updated_at = ? WHERE id = ? AND assessment_id = ?",
    )
    .bind(&new_notes)
    .bind(&now_str)
    .bind(&item_id)
    .bind(&assessment_id)
    .execute(&state.db)
    .await?;

    // Update assessment's updated_at timestamp
    sqlx::query("UPDATE assessments SET updated_at = ? WHERE id = ?")
        .bind(&now_str)
        .bind(&assessment_id)
        .execute(&state.db)
        .await?;

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let old_value = serde_json::json!({
        "notes": old_notes
    });
    let new_value = serde_json::json!({
        "notes": new_notes,
        "added_note": req.note
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, old_value, new_value, created_at)
           VALUES (?, NULL, 'add_note', 'compliance_item', ?, ?, ?, ?)"#,
    )
    .bind(&audit_id)
    .bind(&item_id)
    .bind(old_value.to_string())
    .bind(new_value.to_string())
    .bind(&now_str)
    .execute(&state.db)
    .await?;

    // Fetch updated compliance item with concept details
    let row = sqlx::query_as::<_, ComplianceItemWithConceptRow>(
        r#"SELECT
               ci.id,
               ci.assessment_id,
               ci.concept_id,
               ci.status,
               ci.notes,
               ci.updated_by,
               ci.updated_at,
               c.name_en as concept_name_en,
               c.name_nb as concept_name_nb,
               c.code as concept_code,
               c.concept_type,
               c.parent_id
           FROM compliance_items ci
           JOIN concepts c ON ci.concept_id = c.id
           WHERE ci.id = ? AND ci.assessment_id = ?"#,
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ComplianceItemWithConcept::from(row)))
}

// ============================================================================
// Evidence Handlers
// ============================================================================

/// Add evidence to a compliance item
///
/// Attaches evidence (URL or file metadata) to a compliance item.
#[utoipa::path(
    post,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}/evidence",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    request_body = CreateEvidenceRequest,
    responses(
        (status = 201, description = "Evidence added successfully", body = Evidence),
        (status = 400, description = "Invalid request - must provide either url or file_path"),
        (status = 404, description = "Assessment or compliance item not found")
    )
)]
pub async fn add_evidence(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<CreateEvidenceRequest>,
) -> AppResult<(StatusCode, Json<Evidence>)> {
    // Verify compliance item exists and belongs to assessment
    let item_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM compliance_items WHERE id = ? AND assessment_id = ?")
            .bind(&item_id)
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    if item_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Compliance item with id '{}' not found in assessment '{}'",
            item_id, assessment_id
        )));
    }

    // Validate that either url or file_path is provided
    if req.url.is_none() && req.file_path.is_none() {
        return Err(AppError::BadRequest(
            "Must provide either 'url' or 'file_path'".to_string(),
        ));
    }

    let evidence_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let evidence_type_str: String = req.evidence_type.clone().into();

    // Insert evidence
    sqlx::query(
        r#"INSERT INTO evidence (id, compliance_item_id, evidence_type, title, description, file_path, url, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&evidence_id)
    .bind(&item_id)
    .bind(&evidence_type_str)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.file_path)
    .bind(&req.url)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let new_value = serde_json::json!({
        "id": evidence_id,
        "compliance_item_id": item_id,
        "evidence_type": evidence_type_str,
        "title": req.title,
        "description": req.description,
        "file_path": req.file_path,
        "url": req.url
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
           VALUES (?, NULL, 'create', 'evidence', ?, ?, ?)"#,
    )
    .bind(&audit_id)
    .bind(&evidence_id)
    .bind(new_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Fetch and return the created evidence
    let row = sqlx::query_as::<_, EvidenceRow>(
        r#"SELECT id, compliance_item_id, evidence_type, title, description, file_path, url, uploaded_by, created_at, updated_at
           FROM evidence WHERE id = ?"#,
    )
    .bind(&evidence_id)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(Evidence::from(row))))
}

/// Get evidence for a compliance item
///
/// Returns all evidence attached to a compliance item, ordered by creation date (newest first).
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}/evidence",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    responses(
        (status = 200, description = "List of evidence for the compliance item", body = Vec<Evidence>),
        (status = 404, description = "Assessment or compliance item not found")
    )
)]
pub async fn get_evidence(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
) -> AppResult<Json<Vec<Evidence>>> {
    // Verify compliance item exists and belongs to assessment
    let item_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM compliance_items WHERE id = ? AND assessment_id = ?")
            .bind(&item_id)
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    if item_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Compliance item with id '{}' not found in assessment '{}'",
            item_id, assessment_id
        )));
    }

    // Fetch all evidence for this compliance item
    let rows: Vec<EvidenceRow> = sqlx::query_as(
        r#"SELECT id, compliance_item_id, evidence_type, title, description, file_path, url, uploaded_by, created_at, updated_at
           FROM evidence
           WHERE compliance_item_id = ?
           ORDER BY created_at DESC"#,
    )
    .bind(&item_id)
    .fetch_all(&state.db)
    .await?;

    let evidence: Vec<Evidence> = rows.into_iter().map(Evidence::from).collect();

    Ok(Json(evidence))
}

/// Delete evidence
///
/// Deletes the specified evidence item.
#[utoipa::path(
    delete,
    path = "/api/compliance/evidence/{id}",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Evidence ID")
    ),
    responses(
        (status = 204, description = "Evidence deleted successfully"),
        (status = 404, description = "Evidence not found")
    )
)]
pub async fn delete_evidence(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    // Fetch existing evidence for audit log
    let existing = sqlx::query_as::<_, EvidenceRow>(
        r#"SELECT id, compliance_item_id, evidence_type, title, description, file_path, url, uploaded_by, created_at, updated_at
           FROM evidence WHERE id = ?"#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Evidence with id '{}' not found", id)))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Delete evidence
    let result = sqlx::query("DELETE FROM evidence WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Evidence with id '{}' not found",
            id
        )));
    }

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let old_value = serde_json::json!({
        "id": existing.id,
        "compliance_item_id": existing.compliance_item_id,
        "evidence_type": existing.evidence_type,
        "title": existing.title,
        "description": existing.description,
        "file_path": existing.file_path,
        "url": existing.url
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, old_value, created_at)
           VALUES (?, NULL, 'delete', 'evidence', ?, ?, ?)"#,
    )
    .bind(&audit_id)
    .bind(&id)
    .bind(old_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Upload a file as evidence for a compliance item
///
/// Accepts multipart/form-data with a file field. Stores the file to the
/// uploads directory and creates an evidence record.
#[utoipa::path(
    post,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}/evidence/upload",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    responses(
        (status = 201, description = "File uploaded and evidence created", body = Evidence),
        (status = 400, description = "No file provided or invalid request"),
        (status = 404, description = "Assessment or compliance item not found")
    )
)]
pub async fn upload_evidence(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    mut multipart: Multipart,
) -> AppResult<(StatusCode, Json<Evidence>)> {
    // Verify compliance item exists and belongs to assessment
    let item_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM compliance_items WHERE id = ? AND assessment_id = ?")
            .bind(&item_id)
            .bind(&assessment_id)
            .fetch_optional(&state.db)
            .await?;

    if item_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Compliance item with id '{}' not found in assessment '{}'",
            item_id, assessment_id
        )));
    }

    // Create uploads directory if it doesn't exist
    let upload_dir = std::path::Path::new("uploads").join("evidence");
    tokio::fs::create_dir_all(&upload_dir).await.map_err(|e| {
        AppError::Internal(format!("Failed to create upload directory: {}", e))
    })?;

    let mut file_name: Option<String> = None;
    let mut file_path: Option<String> = None;
    let mut mime_type: Option<String> = None;
    let mut file_size: Option<i64> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                let original_name = field
                    .file_name()
                    .unwrap_or("unnamed")
                    .to_string();
                let content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();

                let data = field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read file data: {}", e))
                })?;

                // Generate unique filename to avoid collisions
                let extension = std::path::Path::new(&original_name)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("bin");
                let unique_name = format!("{}.{}", Uuid::new_v4(), extension);
                let dest_path = upload_dir.join(&unique_name);

                tokio::fs::write(&dest_path, &data).await.map_err(|e| {
                    AppError::Internal(format!("Failed to write file: {}", e))
                })?;

                file_size = Some(data.len() as i64);
                mime_type = Some(content_type);
                file_name = Some(original_name);
                file_path = Some(dest_path.to_string_lossy().to_string());
            }
            "title" => {
                title = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::BadRequest(format!("Invalid title: {}", e)))?,
                );
            }
            "description" => {
                description = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::BadRequest(format!("Invalid description: {}", e)))?,
                );
            }
            _ => {} // Ignore unknown fields
        }
    }

    let stored_path = file_path.ok_or_else(|| {
        AppError::BadRequest("No file field provided in multipart request".to_string())
    })?;
    let actual_title = title.unwrap_or_else(|| file_name.clone().unwrap_or_else(|| "Untitled".to_string()));

    let evidence_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Insert evidence record
    sqlx::query(
        r#"INSERT INTO evidence (id, compliance_item_id, evidence_type, title, description, file_path, mime_type, file_size, created_at, updated_at)
           VALUES (?, ?, 'document', ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&evidence_id)
    .bind(&item_id)
    .bind(&actual_title)
    .bind(&description)
    .bind(&stored_path)
    .bind(&mime_type)
    .bind(file_size)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Log to audit_log
    let audit_id = Uuid::new_v4().to_string();
    let new_value = serde_json::json!({
        "id": evidence_id,
        "compliance_item_id": item_id,
        "evidence_type": "document",
        "title": actual_title,
        "file_name": file_name,
        "mime_type": mime_type,
        "file_size": file_size
    });
    sqlx::query(
        r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
           VALUES (?, NULL, 'upload', 'evidence', ?, ?, ?)"#,
    )
    .bind(&audit_id)
    .bind(&evidence_id)
    .bind(new_value.to_string())
    .bind(&now)
    .execute(&state.db)
    .await?;

    // Fetch and return the created evidence
    let row = sqlx::query_as::<_, EvidenceRow>(
        r#"SELECT id, compliance_item_id, evidence_type, title, description, file_path, url, uploaded_by, created_at, updated_at
           FROM evidence WHERE id = ?"#,
    )
    .bind(&evidence_id)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(Evidence::from(row))))
}

// ============================================================================
// Scoring Handlers
// ============================================================================

/// Helper struct for overall status counts from database
#[derive(Debug, sqlx::FromRow)]
struct StatusCounts {
    total: i64,
    compliant: i64,
    partially_compliant: i64,
    non_compliant: i64,
    not_assessed: i64,
    not_applicable: i64,
}

/// Helper struct for section score row from database
#[derive(Debug, sqlx::FromRow)]
struct SectionScoreRow {
    section_id: String,
    section_name: String,
    total: i64,
    compliant: i64,
    partially_compliant: i64,
    non_compliant: i64,
    not_assessed: i64,
    not_applicable: i64,
}

/// Get compliance score for an assessment
///
/// Calculates overall compliance statistics and per-section scores for the assessment.
/// Returns counts of items by status and compliance percentages.
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/score",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID")
    ),
    responses(
        (status = 200, description = "Compliance score for the assessment", body = ComplianceScore),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_compliance_score(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ComplianceScore>> {
    // Verify assessment exists
    let assessment_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM assessments WHERE id = ?")
            .bind(&id)
            .fetch_optional(&state.db)
            .await?;

    if assessment_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            id
        )));
    }

    // Get overall status counts
    let overall_counts = sqlx::query_as::<_, StatusCounts>(
        r#"SELECT
            COUNT(*) as total,
            SUM(CASE WHEN status = 'compliant' THEN 1 ELSE 0 END) as compliant,
            SUM(CASE WHEN status = 'partially_compliant' THEN 1 ELSE 0 END) as partially_compliant,
            SUM(CASE WHEN status = 'non_compliant' THEN 1 ELSE 0 END) as non_compliant,
            SUM(CASE WHEN status = 'not_assessed' THEN 1 ELSE 0 END) as not_assessed,
            SUM(CASE WHEN status = 'not_applicable' THEN 1 ELSE 0 END) as not_applicable
        FROM compliance_items WHERE assessment_id = ?"#,
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    // Calculate overall compliance percentage
    // (compliant + 0.5 * partially_compliant) / (total - not_applicable) * 100
    let applicable_total = overall_counts.total - overall_counts.not_applicable;
    let overall_percentage = if applicable_total > 0 {
        let weighted_compliant =
            overall_counts.compliant as f64 + 0.5 * overall_counts.partially_compliant as f64;
        (weighted_compliant / applicable_total as f64) * 100.0
    } else {
        0.0
    };

    // Get section scores (grouped by top-level concepts where parent_id IS NULL)
    let section_rows = sqlx::query_as::<_, SectionScoreRow>(
        r#"SELECT
            c.id as section_id,
            c.name_en as section_name,
            COUNT(*) as total,
            SUM(CASE WHEN ci.status = 'compliant' THEN 1 ELSE 0 END) as compliant,
            SUM(CASE WHEN ci.status = 'partially_compliant' THEN 1 ELSE 0 END) as partially_compliant,
            SUM(CASE WHEN ci.status = 'non_compliant' THEN 1 ELSE 0 END) as non_compliant,
            SUM(CASE WHEN ci.status = 'not_assessed' THEN 1 ELSE 0 END) as not_assessed,
            SUM(CASE WHEN ci.status = 'not_applicable' THEN 1 ELSE 0 END) as not_applicable
        FROM compliance_items ci
        JOIN concepts c ON c.id = ci.concept_id AND c.parent_id IS NULL
        WHERE ci.assessment_id = ?
        GROUP BY c.id, c.name_en
        ORDER BY c.sort_order, c.name_en"#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await?;

    // Convert section rows to SectionScore with calculated percentages
    let sections: Vec<SectionScore> = section_rows
        .into_iter()
        .map(|row| {
            let section_applicable = row.total - row.not_applicable;
            let section_percentage = if section_applicable > 0 {
                let weighted = row.compliant as f64 + 0.5 * row.partially_compliant as f64;
                (weighted / section_applicable as f64) * 100.0
            } else {
                0.0
            };

            SectionScore {
                section_id: row.section_id,
                section_name: row.section_name,
                total_items: row.total,
                compliant: row.compliant,
                partially_compliant: row.partially_compliant,
                non_compliant: row.non_compliant,
                not_assessed: row.not_assessed,
                not_applicable: row.not_applicable,
                compliance_percentage: section_percentage,
            }
        })
        .collect();

    Ok(Json(ComplianceScore {
        assessment_id: id,
        total_items: overall_counts.total,
        compliant: overall_counts.compliant,
        partially_compliant: overall_counts.partially_compliant,
        non_compliant: overall_counts.non_compliant,
        not_assessed: overall_counts.not_assessed,
        not_applicable: overall_counts.not_applicable,
        overall_compliance_percentage: overall_percentage,
        sections,
    }))
}

// ============================================================================
// Audit Trail Handlers
// ============================================================================

/// Get audit history for an assessment
///
/// Returns all audit log entries related to this assessment, including:
/// - Direct assessment changes
/// - Compliance item changes within this assessment
/// - Evidence changes for compliance items in this assessment
///
/// Entries are ordered by creation date, newest first.
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/history",
    tag = "compliance",
    params(
        ("id" = String, Path, description = "Assessment ID")
    ),
    responses(
        (status = 200, description = "Audit history for the assessment", body = Vec<AuditLogEntry>),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_assessment_history(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Vec<AuditLogEntry>>> {
    // Verify assessment exists
    let assessment_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM assessments WHERE id = ?")
            .bind(&id)
            .fetch_optional(&state.db)
            .await?;

    if assessment_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Assessment with id '{}' not found",
            id
        )));
    }

    // Get all audit entries related to this assessment:
    // - Direct assessment entries
    // - Compliance item entries for items in this assessment
    // - Evidence entries for evidence attached to compliance items in this assessment
    let entries = sqlx::query_as::<_, AuditLogEntry>(
        r#"SELECT id, user_id, action, entity_type, entity_id, old_value, new_value, ip_address, created_at
           FROM audit_log
           WHERE (entity_type = 'assessment' AND entity_id = ?)
              OR (entity_type = 'compliance_item' AND entity_id IN (
                  SELECT id FROM compliance_items WHERE assessment_id = ?
              ))
              OR (entity_type = 'evidence' AND entity_id IN (
                  SELECT e.id FROM evidence e
                  JOIN compliance_items ci ON ci.id = e.compliance_item_id
                  WHERE ci.assessment_id = ?
              ))
           ORDER BY created_at DESC"#,
    )
    .bind(&id)
    .bind(&id)
    .bind(&id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(entries))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", post(create_assessment))
        .route("/assessments", get(list_assessments))
        .route("/assessments/:id", get(get_assessment))
        .route("/assessments/:id", put(update_assessment))
        .route("/assessments/:id", delete(delete_assessment))
        .route("/assessments/:id/items", get(get_compliance_items))
        .route("/assessments/:id/score", get(get_compliance_score))
        .route("/assessments/:id/history", get(get_assessment_history))
        .route(
            "/assessments/:assessment_id/items/:item_id",
            put(update_compliance_item),
        )
        .route(
            "/assessments/:assessment_id/items/:item_id/notes",
            post(add_item_note),
        )
        .route(
            "/assessments/:assessment_id/items/:item_id/evidence",
            get(get_evidence).post(add_evidence),
        )
        .route(
            "/assessments/:assessment_id/items/:item_id/evidence/upload",
            post(upload_evidence),
        )
        .route("/evidence/:id", delete(delete_evidence))
}
