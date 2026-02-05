use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

// ============================================================================
// Assessment Models
// ============================================================================

/// Assessment status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentStatus {
    Draft,
    InProgress,
    UnderReview,
    Completed,
    Archived,
}

impl From<String> for AssessmentStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "draft" => AssessmentStatus::Draft,
            "in_progress" => AssessmentStatus::InProgress,
            "under_review" => AssessmentStatus::UnderReview,
            "completed" => AssessmentStatus::Completed,
            "archived" => AssessmentStatus::Archived,
            _ => AssessmentStatus::Draft,
        }
    }
}

impl From<AssessmentStatus> for String {
    fn from(status: AssessmentStatus) -> Self {
        match status {
            AssessmentStatus::Draft => "draft".to_string(),
            AssessmentStatus::InProgress => "in_progress".to_string(),
            AssessmentStatus::UnderReview => "under_review".to_string(),
            AssessmentStatus::Completed => "completed".to_string(),
            AssessmentStatus::Archived => "archived".to_string(),
        }
    }
}

/// Database row for assessment
#[derive(Debug, FromRow)]
pub struct AssessmentRow {
    pub id: String,
    pub framework_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub owner_id: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Assessment response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Assessment {
    pub id: String,
    pub framework_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: AssessmentStatus,
    pub owner_id: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AssessmentRow> for Assessment {
    fn from(row: AssessmentRow) -> Self {
        Self {
            id: row.id,
            framework_id: row.framework_id,
            name: row.name,
            description: row.description,
            status: AssessmentStatus::from(row.status),
            owner_id: row.owner_id,
            due_date: row.due_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Request to create a new assessment
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAssessmentRequest {
    pub framework_id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Option<String>,
    pub due_date: Option<String>,
}

/// Request to update an assessment
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAssessmentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<AssessmentStatus>,
    pub owner_id: Option<String>,
    pub due_date: Option<String>,
}

// ============================================================================
// Compliance Item Models
// ============================================================================

/// Compliance status for an item
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    NotAssessed,
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    NotApplicable,
}

impl From<String> for ComplianceStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "not_assessed" => ComplianceStatus::NotAssessed,
            "compliant" => ComplianceStatus::Compliant,
            "partially_compliant" => ComplianceStatus::PartiallyCompliant,
            "non_compliant" => ComplianceStatus::NonCompliant,
            "not_applicable" => ComplianceStatus::NotApplicable,
            _ => ComplianceStatus::NotAssessed,
        }
    }
}

impl From<ComplianceStatus> for String {
    fn from(status: ComplianceStatus) -> Self {
        match status {
            ComplianceStatus::NotAssessed => "not_assessed".to_string(),
            ComplianceStatus::Compliant => "compliant".to_string(),
            ComplianceStatus::PartiallyCompliant => "partially_compliant".to_string(),
            ComplianceStatus::NonCompliant => "non_compliant".to_string(),
            ComplianceStatus::NotApplicable => "not_applicable".to_string(),
        }
    }
}

/// Database row for compliance item
#[derive(Debug, FromRow)]
pub struct ComplianceItemRow {
    pub id: String,
    pub assessment_id: String,
    pub concept_id: String,
    pub status: String,
    pub notes: Option<String>,
    pub assessed_by: Option<String>,
    pub assessed_at: Option<String>,
    pub created_at: String,
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
    pub assessed_by: Option<String>,
    pub assessed_at: Option<String>,
    pub created_at: String,
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
            assessed_by: row.assessed_by,
            assessed_at: row.assessed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Database row for compliance item with concept information
#[derive(Debug, FromRow)]
pub struct ComplianceItemWithConceptRow {
    pub id: String,
    pub assessment_id: String,
    pub concept_id: String,
    pub status: String,
    pub notes: Option<String>,
    pub assessed_by: Option<String>,
    pub assessed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    // Concept fields
    pub concept_code: Option<String>,
    pub concept_name_en: String,
    pub concept_name_nb: Option<String>,
    pub concept_type: String,
    pub concept_definition_en: Option<String>,
}

/// Compliance item with concept information response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceItemWithConcept {
    pub id: String,
    pub assessment_id: String,
    pub concept_id: String,
    pub status: ComplianceStatus,
    pub notes: Option<String>,
    pub assessed_by: Option<String>,
    pub assessed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    // Concept info
    pub concept_code: Option<String>,
    pub concept_name_en: String,
    pub concept_name_nb: Option<String>,
    pub concept_type: String,
    pub concept_definition_en: Option<String>,
}

impl From<ComplianceItemWithConceptRow> for ComplianceItemWithConcept {
    fn from(row: ComplianceItemWithConceptRow) -> Self {
        Self {
            id: row.id,
            assessment_id: row.assessment_id,
            concept_id: row.concept_id,
            status: ComplianceStatus::from(row.status),
            notes: row.notes,
            assessed_by: row.assessed_by,
            assessed_at: row.assessed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            concept_code: row.concept_code,
            concept_name_en: row.concept_name_en,
            concept_name_nb: row.concept_name_nb,
            concept_type: row.concept_type,
            concept_definition_en: row.concept_definition_en,
        }
    }
}

/// Request to update a compliance item
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateComplianceItemRequest {
    pub status: Option<ComplianceStatus>,
    pub notes: Option<String>,
}

/// Request to add a note to a compliance item
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddNoteRequest {
    pub note: String,
}

// ============================================================================
// Evidence Models
// ============================================================================

/// Evidence type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    Document,
    Link,
    Screenshot,
    Note,
    Other,
}

impl From<String> for EvidenceType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "document" => EvidenceType::Document,
            "link" => EvidenceType::Link,
            "screenshot" => EvidenceType::Screenshot,
            "note" => EvidenceType::Note,
            "other" => EvidenceType::Other,
            _ => EvidenceType::Other,
        }
    }
}

impl From<EvidenceType> for String {
    fn from(evidence_type: EvidenceType) -> Self {
        match evidence_type {
            EvidenceType::Document => "document".to_string(),
            EvidenceType::Link => "link".to_string(),
            EvidenceType::Screenshot => "screenshot".to_string(),
            EvidenceType::Note => "note".to_string(),
            EvidenceType::Other => "other".to_string(),
        }
    }
}

/// Database row for evidence
#[derive(Debug, FromRow)]
pub struct EvidenceRow {
    pub id: String,
    pub compliance_item_id: String,
    pub evidence_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub uploaded_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Evidence response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Evidence {
    pub id: String,
    pub compliance_item_id: String,
    pub evidence_type: EvidenceType,
    pub title: String,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub uploaded_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<EvidenceRow> for Evidence {
    fn from(row: EvidenceRow) -> Self {
        Self {
            id: row.id,
            compliance_item_id: row.compliance_item_id,
            evidence_type: EvidenceType::from(row.evidence_type),
            title: row.title,
            description: row.description,
            file_path: row.file_path,
            url: row.url,
            uploaded_by: row.uploaded_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Request to create evidence
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvidenceRequest {
    pub evidence_type: EvidenceType,
    pub title: String,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub url: Option<String>,
}

// ============================================================================
// Scoring Models
// ============================================================================

/// Score for a section/category of compliance items
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SectionScore {
    pub section_id: String,
    pub section_name: String,
    pub total_items: i64,
    pub compliant: i64,
    pub partially_compliant: i64,
    pub non_compliant: i64,
    pub not_assessed: i64,
    pub not_applicable: i64,
    pub compliance_percentage: f64,
}

/// Overall compliance score for an assessment
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceScore {
    pub assessment_id: String,
    pub total_items: i64,
    pub compliant: i64,
    pub partially_compliant: i64,
    pub non_compliant: i64,
    pub not_assessed: i64,
    pub not_applicable: i64,
    pub overall_compliance_percentage: f64,
    pub sections: Vec<SectionScore>,
}

// ============================================================================
// Audit Models
// ============================================================================

/// Audit log entry for tracking changes
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: Option<String>,
}

// ============================================================================
// Query Parameters
// ============================================================================

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    50
}

/// Query parameters for listing assessments
#[derive(Debug, Deserialize, IntoParams)]
pub struct AssessmentListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub framework_id: Option<String>,
    pub status: Option<AssessmentStatus>,
    pub owner_id: Option<String>,
}

/// Query parameters for listing compliance items
#[derive(Debug, Deserialize, IntoParams)]
pub struct ComplianceItemListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub status: Option<ComplianceStatus>,
    pub concept_type: Option<String>,
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
        let total_pages = if limit > 0 {
            (total as f64 / limit as f64).ceil() as i64
        } else {
            0
        };
        Self {
            data,
            total,
            page,
            limit,
            total_pages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessment_status_conversion() {
        assert_eq!(
            AssessmentStatus::from("draft".to_string()),
            AssessmentStatus::Draft
        );
        assert_eq!(
            AssessmentStatus::from("in_progress".to_string()),
            AssessmentStatus::InProgress
        );
        assert_eq!(
            AssessmentStatus::from("under_review".to_string()),
            AssessmentStatus::UnderReview
        );
        assert_eq!(
            AssessmentStatus::from("completed".to_string()),
            AssessmentStatus::Completed
        );
        assert_eq!(
            AssessmentStatus::from("archived".to_string()),
            AssessmentStatus::Archived
        );
        assert_eq!(
            AssessmentStatus::from("unknown".to_string()),
            AssessmentStatus::Draft
        );

        assert_eq!(String::from(AssessmentStatus::Draft), "draft");
        assert_eq!(String::from(AssessmentStatus::InProgress), "in_progress");
    }

    #[test]
    fn test_compliance_status_conversion() {
        assert_eq!(
            ComplianceStatus::from("not_assessed".to_string()),
            ComplianceStatus::NotAssessed
        );
        assert_eq!(
            ComplianceStatus::from("compliant".to_string()),
            ComplianceStatus::Compliant
        );
        assert_eq!(
            ComplianceStatus::from("partially_compliant".to_string()),
            ComplianceStatus::PartiallyCompliant
        );
        assert_eq!(
            ComplianceStatus::from("non_compliant".to_string()),
            ComplianceStatus::NonCompliant
        );
        assert_eq!(
            ComplianceStatus::from("not_applicable".to_string()),
            ComplianceStatus::NotApplicable
        );

        assert_eq!(String::from(ComplianceStatus::Compliant), "compliant");
    }

    #[test]
    fn test_evidence_type_conversion() {
        assert_eq!(
            EvidenceType::from("document".to_string()),
            EvidenceType::Document
        );
        assert_eq!(EvidenceType::from("link".to_string()), EvidenceType::Link);
        assert_eq!(
            EvidenceType::from("screenshot".to_string()),
            EvidenceType::Screenshot
        );
        assert_eq!(EvidenceType::from("note".to_string()), EvidenceType::Note);
        assert_eq!(EvidenceType::from("other".to_string()), EvidenceType::Other);
        assert_eq!(
            EvidenceType::from("unknown".to_string()),
            EvidenceType::Other
        );

        assert_eq!(String::from(EvidenceType::Document), "document");
    }

    #[test]
    fn test_paginated_response() {
        let data = vec![1, 2, 3, 4, 5];
        let response = PaginatedResponse::new(data, 100, 1, 10);

        assert_eq!(response.total, 100);
        assert_eq!(response.page, 1);
        assert_eq!(response.limit, 10);
        assert_eq!(response.total_pages, 10);
        assert_eq!(response.data.len(), 5);
    }

    #[test]
    fn test_paginated_response_zero_limit() {
        let data: Vec<i32> = vec![];
        let response = PaginatedResponse::new(data, 0, 1, 0);

        assert_eq!(response.total_pages, 0);
    }
}
