# Compliance Tracking Backend Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a complete compliance assessment API with CRUD operations for assessments, checklist items, evidence attachments, scoring calculation, and audit trail.

**Architecture:** Feature-based module at `backend/src/features/compliance/` following existing ontology patterns. Uses SQLx with compile-time query checking, Axum handlers with utoipa OpenAPI docs. Items auto-generated from ontology concepts on assessment creation. All changes logged to audit_log table.

**Tech Stack:** Rust, Axum 0.7, SQLx 0.8 (SQLite), utoipa 5, uuid, chrono, serde

---

## Task 1: Create Compliance Models

**Files:**
- Create: `backend/src/features/compliance/models.rs`
- Modify: `backend/src/features/compliance/mod.rs`

**Step 1: Create models file with all DTOs**

```rust
// backend/src/features/compliance/models.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

// ============================================================================
// ASSESSMENT MODELS
// ============================================================================

/// Assessment status enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentStatus {
    Draft,
    InProgress,
    Completed,
    Archived,
}

impl From<String> for AssessmentStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "draft" => Self::Draft,
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "archived" => Self::Archived,
            _ => Self::Draft,
        }
    }
}

impl From<AssessmentStatus> for String {
    fn from(s: AssessmentStatus) -> Self {
        match s {
            AssessmentStatus::Draft => "draft".to_string(),
            AssessmentStatus::InProgress => "in_progress".to_string(),
            AssessmentStatus::Completed => "completed".to_string(),
            AssessmentStatus::Archived => "archived".to_string(),
        }
    }
}

/// Assessment database row
#[derive(Debug, FromRow)]
pub struct AssessmentRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework_id: String,
    pub status: String,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Assessment response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Assessment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework_id: String,
    pub status: AssessmentStatus,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AssessmentRow> for Assessment {
    fn from(row: AssessmentRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            framework_id: row.framework_id,
            status: AssessmentStatus::from(row.status),
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Create assessment request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAssessmentRequest {
    pub name: String,
    pub description: Option<String>,
    pub framework_id: String,
}

/// Update assessment request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAssessmentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<AssessmentStatus>,
}

// ============================================================================
// COMPLIANCE ITEM MODELS
// ============================================================================

/// Compliance item status enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    NotStarted,
    InProgress,
    Implemented,
    NotApplicable,
}

impl From<String> for ComplianceStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "not_started" => Self::NotStarted,
            "in_progress" => Self::InProgress,
            "implemented" => Self::Implemented,
            "not_applicable" => Self::NotApplicable,
            _ => Self::NotStarted,
        }
    }
}

impl From<ComplianceStatus> for String {
    fn from(s: ComplianceStatus) -> Self {
        match s {
            ComplianceStatus::NotStarted => "not_started".to_string(),
            ComplianceStatus::InProgress => "in_progress".to_string(),
            ComplianceStatus::Implemented => "implemented".to_string(),
            ComplianceStatus::NotApplicable => "not_applicable".to_string(),
        }
    }
}

/// Compliance item database row
#[derive(Debug, FromRow)]
pub struct ComplianceItemRow {
    pub id: String,
    pub assessment_id: String,
    pub concept_id: String,
    pub status: String,
    pub notes: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: String,
}

/// Compliance item response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceItem {
    pub id: String,
    pub assessment_id: String,
    pub concept_id: String,
    pub status: ComplianceStatus,
    pub notes: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: String,
}

impl From<ComplianceItemRow> for ComplianceItem {
    fn from(row: ComplianceItemRow) -> Self {
        Self {
            id: row.id,
            assessment_id: row.assessment_id,
            concept_id: row.concept_id,
            status: ComplianceStatus::from(row.status),
            notes: row.notes,
            updated_by: row.updated_by,
            updated_at: row.updated_at,
        }
    }
}

/// Compliance item with concept details
#[derive(Debug, Serialize, ToSchema)]
pub struct ComplianceItemWithConcept {
    #[serde(flatten)]
    pub item: ComplianceItem,
    pub concept_name_en: String,
    pub concept_name_nb: Option<String>,
    pub concept_code: Option<String>,
    pub concept_type: String,
    pub parent_id: Option<String>,
}

/// Update compliance item request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateComplianceItemRequest {
    pub status: Option<ComplianceStatus>,
    pub notes: Option<String>,
}

/// Add note request
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddNoteRequest {
    pub note: String,
}

// ============================================================================
// EVIDENCE MODELS
// ============================================================================

/// Evidence type enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    File,
    Url,
}

impl From<String> for EvidenceType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "file" => Self::File,
            "url" => Self::Url,
            _ => Self::Url,
        }
    }
}

impl From<EvidenceType> for String {
    fn from(e: EvidenceType) -> Self {
        match e {
            EvidenceType::File => "file".to_string(),
            EvidenceType::Url => "url".to_string(),
        }
    }
}

/// Evidence database row
#[derive(Debug, FromRow)]
pub struct EvidenceRow {
    pub id: String,
    pub compliance_item_id: String,
    pub evidence_type: String,
    pub name: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub uploaded_by: Option<String>,
    pub created_at: String,
}

/// Evidence response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Evidence {
    pub id: String,
    pub compliance_item_id: String,
    pub evidence_type: EvidenceType,
    pub name: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub uploaded_by: Option<String>,
    pub created_at: String,
}

impl From<EvidenceRow> for Evidence {
    fn from(row: EvidenceRow) -> Self {
        Self {
            id: row.id,
            compliance_item_id: row.compliance_item_id,
            evidence_type: EvidenceType::from(row.evidence_type),
            name: row.name,
            url: row.url,
            file_path: row.file_path,
            mime_type: row.mime_type,
            file_size: row.file_size,
            uploaded_by: row.uploaded_by,
            created_at: row.created_at,
        }
    }
}

/// Create evidence request (URL type)
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvidenceRequest {
    pub name: String,
    pub url: Option<String>,
    /// Base64 encoded file content (for file upload)
    pub file_content: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
}

// ============================================================================
// SCORING MODELS
// ============================================================================

/// Compliance score for a section
#[derive(Debug, Serialize, ToSchema)]
pub struct SectionScore {
    pub concept_id: String,
    pub concept_name: String,
    pub total_items: i64,
    pub implemented: i64,
    pub in_progress: i64,
    pub not_started: i64,
    pub not_applicable: i64,
    pub percentage: f64,
}

/// Overall compliance score
#[derive(Debug, Serialize, ToSchema)]
pub struct ComplianceScore {
    pub assessment_id: String,
    pub overall_percentage: f64,
    pub total_items: i64,
    pub implemented: i64,
    pub in_progress: i64,
    pub not_started: i64,
    pub not_applicable: i64,
    pub sections: Vec<SectionScore>,
}

// ============================================================================
// AUDIT MODELS
// ============================================================================

/// Audit log entry
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: String,
}

// ============================================================================
// QUERY PARAMETERS
// ============================================================================

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 50 }

/// Query parameters for listing assessments
#[derive(Debug, Deserialize, IntoParams)]
pub struct AssessmentListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub status: Option<String>,
    pub framework_id: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, limit: i64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as i64;
        Self { data, total, page, limit, total_pages }
    }
}
```

**Step 2: Update mod.rs to export models**

```rust
// backend/src/features/compliance/mod.rs
pub mod models;
pub mod routes;
```

**Step 3: Verify TypeScript compiles**

Run: `cd backend && cargo check`
Expected: Compiles with no errors

**Step 4: Commit**

```bash
git add backend/src/features/compliance/models.rs backend/src/features/compliance/mod.rs
git commit -m "feat(compliance): add models and DTOs for compliance tracking"
```

---

## Task 2: Create Assessment CRUD Handlers

**Files:**
- Modify: `backend/src/features/compliance/routes.rs`

**Step 1: Write failing test for create assessment**

Add to `backend/tests/api_tests.rs`:

```rust
#[tokio::test]
async fn test_create_assessment() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"Test Assessment","framework_id":"iso31000"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["id"].is_string());
    assert_eq!(json["name"], "Test Assessment");
    assert_eq!(json["framework_id"], "iso31000");
    assert_eq!(json["status"], "draft");
}
```

**Step 2: Run test to verify it fails**

Run: `cd backend && cargo test test_create_assessment`
Expected: FAIL - endpoint returns placeholder response

**Step 3: Implement assessment CRUD handlers**

Replace `backend/src/features/compliance/routes.rs`:

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{error::AppError, AppState};

use super::models::{
    Assessment, AssessmentListQuery, AssessmentRow, AssessmentStatus,
    CreateAssessmentRequest, PaginatedResponse, UpdateAssessmentRequest,
};

// ============================================================================
// ASSESSMENT HANDLERS
// ============================================================================

/// Create a new assessment
#[utoipa::path(
    post,
    path = "/api/compliance/assessments",
    tag = "compliance",
    request_body = CreateAssessmentRequest,
    responses(
        (status = 201, description = "Assessment created", body = Assessment),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Framework not found")
    )
)]
pub async fn create_assessment(
    State(state): State<AppState>,
    Json(req): Json<CreateAssessmentRequest>,
) -> Result<(StatusCode, Json<Assessment>), AppError> {
    // Validate framework exists
    let framework_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM frameworks WHERE id = ?"
    )
    .bind(&req.framework_id)
    .fetch_optional(&state.db)
    .await?;

    if framework_exists.is_none() {
        return Err(AppError::NotFound(format!("Framework '{}' not found", req.framework_id)));
    }

    let id = Uuid::new_v4().to_string();
    let status: String = AssessmentStatus::Draft.into();

    sqlx::query(
        r#"INSERT INTO assessments (id, name, description, framework_id, status)
           VALUES (?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.framework_id)
    .bind(&status)
    .execute(&state.db)
    .await?;

    // Generate compliance items from framework concepts
    let concepts: Vec<(String,)> = sqlx::query_as(
        "SELECT id FROM concepts WHERE framework_id = ?"
    )
    .bind(&req.framework_id)
    .fetch_all(&state.db)
    .await?;

    for (concept_id,) in concepts {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"INSERT INTO compliance_items (id, assessment_id, concept_id, status)
               VALUES (?, ?, ?, 'not_started')"#
        )
        .bind(&item_id)
        .bind(&id)
        .bind(&concept_id)
        .execute(&state.db)
        .await?;
    }

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id, new_value)
           VALUES (?, 'create', 'assessment', ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&id)
    .bind(serde_json::to_string(&req).unwrap_or_default())
    .execute(&state.db)
    .await?;

    // Fetch and return created assessment
    let row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, name, description, framework_id, status, created_by, created_at, updated_at
           FROM assessments WHERE id = ?"#
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(Assessment::from(row))))
}

/// List all assessments with pagination and filtering
#[utoipa::path(
    get,
    path = "/api/compliance/assessments",
    tag = "compliance",
    params(AssessmentListQuery),
    responses(
        (status = 200, description = "List of assessments", body = PaginatedResponse<Assessment>)
    )
)]
pub async fn list_assessments(
    State(state): State<AppState>,
    Query(query): Query<AssessmentListQuery>,
) -> Result<Json<PaginatedResponse<Assessment>>, AppError> {
    let offset = (query.page - 1) * query.limit;

    // Build WHERE clause
    let mut conditions = Vec::new();
    if query.status.is_some() {
        conditions.push("status = ?");
    }
    if query.framework_id.is_some() {
        conditions.push("framework_id = ?");
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count total
    let count_sql = format!("SELECT COUNT(*) FROM assessments {}", where_clause);
    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
    if let Some(ref s) = query.status {
        count_query = count_query.bind(s);
    }
    if let Some(ref f) = query.framework_id {
        count_query = count_query.bind(f);
    }
    let total = count_query.fetch_one(&state.db).await?;

    // Fetch assessments
    let select_sql = format!(
        r#"SELECT id, name, description, framework_id, status, created_by, created_at, updated_at
           FROM assessments {} ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        where_clause
    );
    let mut select_query = sqlx::query_as::<_, AssessmentRow>(&select_sql);
    if let Some(ref s) = query.status {
        select_query = select_query.bind(s);
    }
    if let Some(ref f) = query.framework_id {
        select_query = select_query.bind(f);
    }
    let rows = select_query
        .bind(query.limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

    let assessments: Vec<Assessment> = rows.into_iter().map(Assessment::from).collect();

    Ok(Json(PaginatedResponse::new(assessments, total, query.page, query.limit)))
}

/// Get assessment by ID
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    responses(
        (status = 200, description = "Assessment details", body = Assessment),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Assessment>, AppError> {
    let row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, name, description, framework_id, status, created_by, created_at, updated_at
           FROM assessments WHERE id = ?"#
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Assessment '{}' not found", id)))?;

    Ok(Json(Assessment::from(row)))
}

/// Update assessment
#[utoipa::path(
    put,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    request_body = UpdateAssessmentRequest,
    responses(
        (status = 200, description = "Assessment updated", body = Assessment),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn update_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAssessmentRequest>,
) -> Result<Json<Assessment>, AppError> {
    // Get old value for audit
    let old_row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, name, description, framework_id, status, created_by, created_at, updated_at
           FROM assessments WHERE id = ?"#
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Assessment '{}' not found", id)))?;

    // Build update query
    let mut updates = vec!["updated_at = datetime('now')"];
    if req.name.is_some() {
        updates.push("name = ?");
    }
    if req.description.is_some() {
        updates.push("description = ?");
    }
    if req.status.is_some() {
        updates.push("status = ?");
    }

    let update_sql = format!("UPDATE assessments SET {} WHERE id = ?", updates.join(", "));
    let mut update_query = sqlx::query(&update_sql);

    if let Some(ref name) = req.name {
        update_query = update_query.bind(name);
    }
    if let Some(ref desc) = req.description {
        update_query = update_query.bind(desc);
    }
    if let Some(ref status) = req.status {
        let s: String = status.clone().into();
        update_query = update_query.bind(s);
    }
    update_query = update_query.bind(&id);
    update_query.execute(&state.db).await?;

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    let old_assessment = Assessment::from(old_row);
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id, old_value, new_value)
           VALUES (?, 'update', 'assessment', ?, ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&id)
    .bind(serde_json::to_string(&old_assessment).unwrap_or_default())
    .bind(serde_json::to_string(&req).unwrap_or_default())
    .execute(&state.db)
    .await?;

    // Fetch and return updated assessment
    let row = sqlx::query_as::<_, AssessmentRow>(
        r#"SELECT id, name, description, framework_id, status, created_by, created_at, updated_at
           FROM assessments WHERE id = ?"#
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(Assessment::from(row)))
}

/// Delete assessment
#[utoipa::path(
    delete,
    path = "/api/compliance/assessments/{id}",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    responses(
        (status = 204, description = "Assessment deleted"),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn delete_assessment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    // Check exists
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM assessments WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Assessment '{}' not found", id)));
    }

    // Log audit before delete
    let audit_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id)
           VALUES (?, 'delete', 'assessment', ?)"#
    )
    .bind(&audit_id)
    .bind(&id)
    .execute(&state.db)
    .await?;

    // Delete (cascade deletes compliance_items and evidence)
    sqlx::query("DELETE FROM assessments WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", get(list_assessments).post(create_assessment))
        .route(
            "/assessments/:id",
            get(get_assessment).put(update_assessment).delete(delete_assessment),
        )
}
```

**Step 4: Run tests to verify they pass**

Run: `cd backend && cargo test test_create_assessment`
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/features/compliance/routes.rs backend/tests/api_tests.rs
git commit -m "feat(compliance): add assessment CRUD endpoints"
```

---

## Task 3: Add Compliance Items Handlers

**Files:**
- Modify: `backend/src/features/compliance/routes.rs`

**Step 1: Write failing test for get compliance items**

Add to `backend/tests/api_tests.rs`:

```rust
#[tokio::test]
async fn test_get_compliance_items() {
    let app = create_test_app().await;

    // First create an assessment
    let create_response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"Items Test","framework_id":"iso31000"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let assessment: Value = serde_json::from_slice(&body).unwrap();
    let assessment_id = assessment["id"].as_str().unwrap();

    // Get compliance items
    let app = create_test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/compliance/assessments/{}/items", assessment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_array());
    assert!(json["total"].as_i64().unwrap() > 0);
}
```

**Step 2: Run test to verify it fails**

Run: `cd backend && cargo test test_get_compliance_items`
Expected: FAIL - endpoint not found

**Step 3: Add compliance items handlers to routes.rs**

Add these handlers and update the router:

```rust
// Add to imports at top of routes.rs
use super::models::{
    // ... existing imports ...
    ComplianceItem, ComplianceItemRow, ComplianceItemWithConcept,
    UpdateComplianceItemRequest, AddNoteRequest,
};

// ============================================================================
// COMPLIANCE ITEMS HANDLERS
// ============================================================================

/// Compliance item with concept row
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
    parent_id: Option<String>,
}

/// Get compliance items for an assessment
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/items",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    responses(
        (status = 200, description = "List of compliance items", body = PaginatedResponse<ComplianceItemWithConcept>),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_compliance_items(
    State(state): State<AppState>,
    Path(assessment_id): Path<String>,
) -> Result<Json<PaginatedResponse<ComplianceItemWithConcept>>, AppError> {
    // Verify assessment exists
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM assessments WHERE id = ?")
        .bind(&assessment_id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Assessment '{}' not found", assessment_id)));
    }

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM compliance_items WHERE assessment_id = ?"
    )
    .bind(&assessment_id)
    .fetch_one(&state.db)
    .await?;

    let rows: Vec<ComplianceItemWithConceptRow> = sqlx::query_as(
        r#"SELECT
            ci.id, ci.assessment_id, ci.concept_id, ci.status, ci.notes, ci.updated_by, ci.updated_at,
            c.name_en as concept_name_en, c.name_nb as concept_name_nb, c.code as concept_code,
            c.concept_type, c.parent_id
           FROM compliance_items ci
           JOIN concepts c ON c.id = ci.concept_id
           WHERE ci.assessment_id = ?
           ORDER BY c.sort_order, c.name_en"#
    )
    .bind(&assessment_id)
    .fetch_all(&state.db)
    .await?;

    let items: Vec<ComplianceItemWithConcept> = rows
        .into_iter()
        .map(|row| ComplianceItemWithConcept {
            item: ComplianceItem {
                id: row.id,
                assessment_id: row.assessment_id,
                concept_id: row.concept_id,
                status: super::models::ComplianceStatus::from(row.status),
                notes: row.notes,
                updated_by: row.updated_by,
                updated_at: row.updated_at,
            },
            concept_name_en: row.concept_name_en,
            concept_name_nb: row.concept_name_nb,
            concept_code: row.concept_code,
            concept_type: row.concept_type,
            parent_id: row.parent_id,
        })
        .collect();

    Ok(Json(PaginatedResponse::new(items, total, 1, total)))
}

/// Update a compliance item status
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
        (status = 200, description = "Item updated", body = ComplianceItem),
        (status = 404, description = "Item not found")
    )
)]
pub async fn update_compliance_item(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<UpdateComplianceItemRequest>,
) -> Result<Json<ComplianceItem>, AppError> {
    // Get old value for audit
    let old_row = sqlx::query_as::<_, ComplianceItemRow>(
        r#"SELECT id, assessment_id, concept_id, status, notes, updated_by, updated_at
           FROM compliance_items WHERE id = ? AND assessment_id = ?"#
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Compliance item '{}' not found", item_id)))?;

    // Build update
    let mut updates = vec!["updated_at = datetime('now')"];
    if req.status.is_some() {
        updates.push("status = ?");
    }
    if req.notes.is_some() {
        updates.push("notes = ?");
    }

    let update_sql = format!(
        "UPDATE compliance_items SET {} WHERE id = ?",
        updates.join(", ")
    );
    let mut update_query = sqlx::query(&update_sql);

    if let Some(ref status) = req.status {
        let s: String = status.clone().into();
        update_query = update_query.bind(s);
    }
    if let Some(ref notes) = req.notes {
        update_query = update_query.bind(notes);
    }
    update_query = update_query.bind(&item_id);
    update_query.execute(&state.db).await?;

    // Update assessment's updated_at
    sqlx::query("UPDATE assessments SET updated_at = datetime('now') WHERE id = ?")
        .bind(&assessment_id)
        .execute(&state.db)
        .await?;

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    let old_item = ComplianceItem::from(old_row);
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id, old_value, new_value)
           VALUES (?, 'update', 'compliance_item', ?, ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&item_id)
    .bind(serde_json::to_string(&old_item).unwrap_or_default())
    .bind(serde_json::to_string(&req).unwrap_or_default())
    .execute(&state.db)
    .await?;

    // Fetch updated item
    let row = sqlx::query_as::<_, ComplianceItemRow>(
        r#"SELECT id, assessment_id, concept_id, status, notes, updated_by, updated_at
           FROM compliance_items WHERE id = ?"#
    )
    .bind(&item_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ComplianceItem::from(row)))
}

/// Add a note to a compliance item
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
        (status = 200, description = "Note added", body = ComplianceItem),
        (status = 404, description = "Item not found")
    )
)]
pub async fn add_item_note(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<AddNoteRequest>,
) -> Result<Json<ComplianceItem>, AppError> {
    // Verify item exists
    let existing: Option<ComplianceItemRow> = sqlx::query_as(
        r#"SELECT id, assessment_id, concept_id, status, notes, updated_by, updated_at
           FROM compliance_items WHERE id = ? AND assessment_id = ?"#
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_optional(&state.db)
    .await?;

    let old_row = existing
        .ok_or_else(|| AppError::NotFound(format!("Compliance item '{}' not found", item_id)))?;

    // Append note with timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let new_note = if let Some(ref existing_notes) = old_row.notes {
        format!("{}\n\n[{}]\n{}", existing_notes, timestamp, req.note)
    } else {
        format!("[{}]\n{}", timestamp, req.note)
    };

    sqlx::query(
        "UPDATE compliance_items SET notes = ?, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(&new_note)
    .bind(&item_id)
    .execute(&state.db)
    .await?;

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id, new_value)
           VALUES (?, 'add_note', 'compliance_item', ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&item_id)
    .bind(&req.note)
    .execute(&state.db)
    .await?;

    // Fetch updated item
    let row = sqlx::query_as::<_, ComplianceItemRow>(
        r#"SELECT id, assessment_id, concept_id, status, notes, updated_by, updated_at
           FROM compliance_items WHERE id = ?"#
    )
    .bind(&item_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ComplianceItem::from(row)))
}

// Update router at bottom of file
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", get(list_assessments).post(create_assessment))
        .route(
            "/assessments/:id",
            get(get_assessment).put(update_assessment).delete(delete_assessment),
        )
        .route("/assessments/:id/items", get(get_compliance_items))
        .route(
            "/assessments/:assessment_id/items/:item_id",
            put(update_compliance_item),
        )
        .route(
            "/assessments/:assessment_id/items/:item_id/notes",
            post(add_item_note),
        )
}
```

**Step 4: Run test to verify it passes**

Run: `cd backend && cargo test test_get_compliance_items`
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/features/compliance/routes.rs backend/tests/api_tests.rs
git commit -m "feat(compliance): add compliance items endpoints"
```

---

## Task 4: Add Evidence Handlers

**Files:**
- Modify: `backend/src/features/compliance/routes.rs`

**Step 1: Write failing test for evidence upload**

Add to `backend/tests/api_tests.rs`:

```rust
#[tokio::test]
async fn test_add_evidence_url() {
    let app = create_test_app().await;

    // Create assessment first
    let create_response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"Evidence Test","framework_id":"iso31000"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let assessment: Value = serde_json::from_slice(&body).unwrap();
    let assessment_id = assessment["id"].as_str().unwrap();

    // Get first compliance item
    let app = create_test_app().await;
    let items_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/compliance/assessments/{}/items", assessment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(items_response.into_body(), usize::MAX).await.unwrap();
    let items: Value = serde_json::from_slice(&body).unwrap();
    let item_id = items["data"][0]["id"].as_str().unwrap();

    // Add evidence
    let app = create_test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/compliance/assessments/{}/items/{}/evidence", assessment_id, item_id))
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"Test Policy","url":"https://example.com/policy.pdf"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["id"].is_string());
    assert_eq!(json["name"], "Test Policy");
    assert_eq!(json["evidence_type"], "url");
}
```

**Step 2: Run test to verify it fails**

Run: `cd backend && cargo test test_add_evidence_url`
Expected: FAIL - endpoint not found

**Step 3: Add evidence handlers to routes.rs**

Add these handlers and update router:

```rust
// Add to imports
use super::models::{
    // ... existing imports ...
    Evidence, EvidenceRow, EvidenceType, CreateEvidenceRequest,
};

// ============================================================================
// EVIDENCE HANDLERS
// ============================================================================

/// Add evidence to a compliance item
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
        (status = 201, description = "Evidence added", body = Evidence),
        (status = 404, description = "Item not found"),
        (status = 400, description = "Invalid request")
    )
)]
pub async fn add_evidence(
    State(state): State<AppState>,
    Path((assessment_id, item_id)): Path<(String, String)>,
    Json(req): Json<CreateEvidenceRequest>,
) -> Result<(StatusCode, Json<Evidence>), AppError> {
    // Verify compliance item exists
    let exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM compliance_items WHERE id = ? AND assessment_id = ?"
    )
    .bind(&item_id)
    .bind(&assessment_id)
    .fetch_optional(&state.db)
    .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Compliance item '{}' not found", item_id)));
    }

    // Determine evidence type
    let (evidence_type, url, file_path, mime_type, file_size): (String, Option<String>, Option<String>, Option<String>, Option<i64>) =
        if let Some(ref url) = req.url {
            ("url".to_string(), Some(url.clone()), None, None, None)
        } else if let Some(ref _file_content) = req.file_content {
            // For file uploads, we'd normally save to disk/S3
            // For now, store metadata only (file storage is out of scope)
            let file_name = req.file_name.clone().unwrap_or_else(|| "uploaded_file".to_string());
            let mime = req.mime_type.clone();
            ("file".to_string(), None, Some(format!("/evidence/{}", file_name)), mime, None)
        } else {
            return Err(AppError::BadRequest("Either url or file_content must be provided".to_string()));
        };

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"INSERT INTO evidence (id, compliance_item_id, evidence_type, name, url, file_path, mime_type, file_size)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&item_id)
    .bind(&evidence_type)
    .bind(&req.name)
    .bind(&url)
    .bind(&file_path)
    .bind(&mime_type)
    .bind(file_size)
    .execute(&state.db)
    .await?;

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id, new_value)
           VALUES (?, 'create', 'evidence', ?, ?)"#
    )
    .bind(&audit_id)
    .bind(&id)
    .bind(&req.name)
    .execute(&state.db)
    .await?;

    // Fetch created evidence
    let row = sqlx::query_as::<_, EvidenceRow>(
        r#"SELECT id, compliance_item_id, evidence_type, name, url, file_path, mime_type, file_size, uploaded_by, created_at
           FROM evidence WHERE id = ?"#
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(Evidence::from(row))))
}

/// Get evidence for a compliance item
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{assessment_id}/items/{item_id}/evidence",
    tag = "compliance",
    params(
        ("assessment_id" = String, Path, description = "Assessment ID"),
        ("item_id" = String, Path, description = "Compliance item ID")
    ),
    responses(
        (status = 200, description = "List of evidence", body = Vec<Evidence>),
        (status = 404, description = "Item not found")
    )
)]
pub async fn get_evidence(
    State(state): State<AppState>,
    Path((_assessment_id, item_id)): Path<(String, String)>,
) -> Result<Json<Vec<Evidence>>, AppError> {
    let rows = sqlx::query_as::<_, EvidenceRow>(
        r#"SELECT id, compliance_item_id, evidence_type, name, url, file_path, mime_type, file_size, uploaded_by, created_at
           FROM evidence WHERE compliance_item_id = ? ORDER BY created_at DESC"#
    )
    .bind(&item_id)
    .fetch_all(&state.db)
    .await?;

    let evidence: Vec<Evidence> = rows.into_iter().map(Evidence::from).collect();
    Ok(Json(evidence))
}

/// Delete evidence
#[utoipa::path(
    delete,
    path = "/api/evidence/{id}",
    tag = "compliance",
    params(("id" = String, Path, description = "Evidence ID")),
    responses(
        (status = 204, description = "Evidence deleted"),
        (status = 404, description = "Evidence not found")
    )
)]
pub async fn delete_evidence(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    // Check exists
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM evidence WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Evidence '{}' not found", id)));
    }

    // Log audit
    let audit_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO audit_log (id, action, entity_type, entity_id)
           VALUES (?, 'delete', 'evidence', ?)"#
    )
    .bind(&audit_id)
    .bind(&id)
    .execute(&state.db)
    .await?;

    sqlx::query("DELETE FROM evidence WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// Update router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", get(list_assessments).post(create_assessment))
        .route(
            "/assessments/:id",
            get(get_assessment).put(update_assessment).delete(delete_assessment),
        )
        .route("/assessments/:id/items", get(get_compliance_items))
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
        .route("/evidence/:id", delete(delete_evidence))
}
```

**Step 4: Run test to verify it passes**

Run: `cd backend && cargo test test_add_evidence_url`
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/features/compliance/routes.rs backend/tests/api_tests.rs
git commit -m "feat(compliance): add evidence upload and management endpoints"
```

---

## Task 5: Add Compliance Scoring Endpoint

**Files:**
- Modify: `backend/src/features/compliance/routes.rs`

**Step 1: Write failing test for scoring**

Add to `backend/tests/api_tests.rs`:

```rust
#[tokio::test]
async fn test_get_compliance_score() {
    let app = create_test_app().await;

    // Create assessment
    let create_response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"Score Test","framework_id":"iso31000"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let assessment: Value = serde_json::from_slice(&body).unwrap();
    let assessment_id = assessment["id"].as_str().unwrap();

    // Get score
    let app = create_test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/compliance/assessments/{}/score", assessment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["assessment_id"], assessment_id);
    assert!(json["overall_percentage"].is_number());
    assert!(json["total_items"].as_i64().unwrap() >= 0);
}
```

**Step 2: Run test to verify it fails**

Run: `cd backend && cargo test test_get_compliance_score`
Expected: FAIL - endpoint not found

**Step 3: Add scoring handler to routes.rs**

Add handler and update router:

```rust
// Add to imports
use super::models::{
    // ... existing imports ...
    ComplianceScore, SectionScore,
};

// ============================================================================
// SCORING HANDLERS
// ============================================================================

/// Status counts row
#[derive(Debug, sqlx::FromRow)]
struct StatusCounts {
    total: i64,
    implemented: i64,
    in_progress: i64,
    not_started: i64,
    not_applicable: i64,
}

/// Section score row
#[derive(Debug, sqlx::FromRow)]
struct SectionScoreRow {
    concept_id: String,
    concept_name: String,
    total: i64,
    implemented: i64,
    in_progress: i64,
    not_started: i64,
    not_applicable: i64,
}

/// Get compliance score for an assessment
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/score",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    responses(
        (status = 200, description = "Compliance score", body = ComplianceScore),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_compliance_score(
    State(state): State<AppState>,
    Path(assessment_id): Path<String>,
) -> Result<Json<ComplianceScore>, AppError> {
    // Verify assessment exists
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM assessments WHERE id = ?")
        .bind(&assessment_id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Assessment '{}' not found", assessment_id)));
    }

    // Get overall counts
    let overall: StatusCounts = sqlx::query_as(
        r#"SELECT
            COUNT(*) as total,
            SUM(CASE WHEN status = 'implemented' THEN 1 ELSE 0 END) as implemented,
            SUM(CASE WHEN status = 'in_progress' THEN 1 ELSE 0 END) as in_progress,
            SUM(CASE WHEN status = 'not_started' THEN 1 ELSE 0 END) as not_started,
            SUM(CASE WHEN status = 'not_applicable' THEN 1 ELSE 0 END) as not_applicable
           FROM compliance_items WHERE assessment_id = ?"#
    )
    .bind(&assessment_id)
    .fetch_one(&state.db)
    .await?;

    // Calculate overall percentage (exclude not_applicable from denominator)
    let applicable_total = overall.total - overall.not_applicable;
    let overall_percentage = if applicable_total > 0 {
        (overall.implemented as f64 / applicable_total as f64) * 100.0
    } else {
        0.0
    };

    // Get scores by top-level concept (section)
    let section_rows: Vec<SectionScoreRow> = sqlx::query_as(
        r#"SELECT
            c.id as concept_id,
            c.name_en as concept_name,
            COUNT(*) as total,
            SUM(CASE WHEN ci.status = 'implemented' THEN 1 ELSE 0 END) as implemented,
            SUM(CASE WHEN ci.status = 'in_progress' THEN 1 ELSE 0 END) as in_progress,
            SUM(CASE WHEN ci.status = 'not_started' THEN 1 ELSE 0 END) as not_started,
            SUM(CASE WHEN ci.status = 'not_applicable' THEN 1 ELSE 0 END) as not_applicable
           FROM compliance_items ci
           JOIN concepts c ON c.id = ci.concept_id AND c.parent_id IS NULL
           WHERE ci.assessment_id = ?
           GROUP BY c.id, c.name_en
           ORDER BY c.sort_order, c.name_en"#
    )
    .bind(&assessment_id)
    .fetch_all(&state.db)
    .await?;

    let sections: Vec<SectionScore> = section_rows
        .into_iter()
        .map(|row| {
            let applicable = row.total - row.not_applicable;
            let percentage = if applicable > 0 {
                (row.implemented as f64 / applicable as f64) * 100.0
            } else {
                0.0
            };
            SectionScore {
                concept_id: row.concept_id,
                concept_name: row.concept_name,
                total_items: row.total,
                implemented: row.implemented,
                in_progress: row.in_progress,
                not_started: row.not_started,
                not_applicable: row.not_applicable,
                percentage,
            }
        })
        .collect();

    Ok(Json(ComplianceScore {
        assessment_id,
        overall_percentage,
        total_items: overall.total,
        implemented: overall.implemented,
        in_progress: overall.in_progress,
        not_started: overall.not_started,
        not_applicable: overall.not_applicable,
        sections,
    }))
}

// Update router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", get(list_assessments).post(create_assessment))
        .route(
            "/assessments/:id",
            get(get_assessment).put(update_assessment).delete(delete_assessment),
        )
        .route("/assessments/:id/items", get(get_compliance_items))
        .route("/assessments/:id/score", get(get_compliance_score))
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
        .route("/evidence/:id", delete(delete_evidence))
}
```

**Step 4: Run test to verify it passes**

Run: `cd backend && cargo test test_get_compliance_score`
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/features/compliance/routes.rs backend/tests/api_tests.rs
git commit -m "feat(compliance): add compliance scoring endpoint"
```

---

## Task 6: Add Audit Trail Endpoint

**Files:**
- Modify: `backend/src/features/compliance/routes.rs`

**Step 1: Write failing test for audit history**

Add to `backend/tests/api_tests.rs`:

```rust
#[tokio::test]
async fn test_get_assessment_history() {
    let app = create_test_app().await;

    // Create and update an assessment
    let create_response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name":"History Test","framework_id":"iso31000"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap();
    let assessment: Value = serde_json::from_slice(&body).unwrap();
    let assessment_id = assessment["id"].as_str().unwrap();

    // Update it
    let app = create_test_app().await;
    app.oneshot(
        Request::builder()
            .method("PUT")
            .uri(format!("/api/compliance/assessments/{}", assessment_id))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"History Test Updated"}"#))
            .unwrap(),
    )
    .await
    .unwrap();

    // Get history
    let app = create_test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/compliance/assessments/{}/history", assessment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_array());
    assert!(json.as_array().unwrap().len() >= 2); // create + update
}
```

**Step 2: Run test to verify it fails**

Run: `cd backend && cargo test test_get_assessment_history`
Expected: FAIL - endpoint not found

**Step 3: Add audit history handler to routes.rs**

Add handler and update router:

```rust
// Add to imports
use super::models::AuditLogEntry;

// ============================================================================
// AUDIT HANDLERS
// ============================================================================

/// Get audit history for an assessment
#[utoipa::path(
    get,
    path = "/api/compliance/assessments/{id}/history",
    tag = "compliance",
    params(("id" = String, Path, description = "Assessment ID")),
    responses(
        (status = 200, description = "Audit history", body = Vec<AuditLogEntry>),
        (status = 404, description = "Assessment not found")
    )
)]
pub async fn get_assessment_history(
    State(state): State<AppState>,
    Path(assessment_id): Path<String>,
) -> Result<Json<Vec<AuditLogEntry>>, AppError> {
    // Verify assessment exists
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM assessments WHERE id = ?")
        .bind(&assessment_id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::NotFound(format!("Assessment '{}' not found", assessment_id)));
    }

    // Get all audit entries for this assessment and its items/evidence
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
           ORDER BY created_at DESC"#
    )
    .bind(&assessment_id)
    .bind(&assessment_id)
    .bind(&assessment_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(entries))
}

// Final router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assessments", get(list_assessments).post(create_assessment))
        .route(
            "/assessments/:id",
            get(get_assessment).put(update_assessment).delete(delete_assessment),
        )
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
        .route("/evidence/:id", delete(delete_evidence))
}
```

**Step 4: Run test to verify it passes**

Run: `cd backend && cargo test test_get_assessment_history`
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/features/compliance/routes.rs backend/tests/api_tests.rs
git commit -m "feat(compliance): add audit history endpoint"
```

---

## Task 7: Run All Tests and Verify

**Step 1: Run full test suite**

Run: `cd backend && cargo test`
Expected: All tests pass

**Step 2: Run clippy for linting**

Run: `cd backend && cargo clippy`
Expected: No errors (warnings acceptable)

**Step 3: Run formatting check**

Run: `cd backend && cargo fmt --check`
Expected: No formatting issues (run `cargo fmt` if there are)

**Step 4: Start server and test manually with curl**

Run: `cd backend && cargo run`

Test create assessment:
```bash
curl -X POST http://localhost:3000/api/compliance/assessments \
  -H "Content-Type: application/json" \
  -d '{"name":"Manual Test","framework_id":"iso31000"}'
```
Expected: 201 Created with assessment JSON

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(compliance): complete Sprint 3 Compliance Tracking Backend

- Assessment CRUD (T3.1)
- Compliance items with concept linking (T3.2)
- Evidence attachments (T3.3)
- Compliance scoring (T3.4)
- Audit trail (T3.5)

All endpoints documented in Swagger at /swagger-ui"
```

---

## Summary

| Task | Description | Endpoints |
|------|-------------|-----------|
| 1 | Create models | DTOs for assessment, items, evidence, scoring |
| 2 | Assessment CRUD | POST/GET/PUT/DELETE /assessments |
| 3 | Compliance items | GET /assessments/:id/items, PUT items, POST notes |
| 4 | Evidence | GET/POST evidence, DELETE /evidence/:id |
| 5 | Scoring | GET /assessments/:id/score |
| 6 | Audit trail | GET /assessments/:id/history |
| 7 | Test & verify | Full test suite, clippy, manual testing |

## API Endpoints Summary

```
POST   /api/compliance/assessments                           - Create assessment
GET    /api/compliance/assessments                           - List assessments
GET    /api/compliance/assessments/:id                       - Get assessment
PUT    /api/compliance/assessments/:id                       - Update assessment
DELETE /api/compliance/assessments/:id                       - Delete assessment
GET    /api/compliance/assessments/:id/items                 - Get compliance items
PUT    /api/compliance/assessments/:aid/items/:iid           - Update item status
POST   /api/compliance/assessments/:aid/items/:iid/notes     - Add note
GET    /api/compliance/assessments/:aid/items/:iid/evidence  - Get evidence
POST   /api/compliance/assessments/:aid/items/:iid/evidence  - Add evidence
DELETE /api/evidence/:id                                     - Delete evidence
GET    /api/compliance/assessments/:id/score                 - Get compliance score
GET    /api/compliance/assessments/:id/history               - Get audit history
```
