use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::warn;
use utoipa::{IntoParams, ToSchema};

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    Text,
    Pdf,
    Docx,
}

impl From<String> for InputType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "text" => Self::Text,
            "pdf" => Self::Pdf,
            "docx" => Self::Docx,
            _ => {
                warn!("Unknown InputType '{}', defaulting to Text", s);
                Self::Text
            }
        }
    }
}

impl From<InputType> for String {
    fn from(t: InputType) -> Self {
        match t {
            InputType::Text => "text".to_string(),
            InputType::Pdf => "pdf".to_string(),
            InputType::Docx => "docx".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Deleted,
}

impl From<String> for AnalysisStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => Self::Pending,
            "processing" => Self::Processing,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "deleted" => Self::Deleted,
            _ => {
                warn!("Unknown AnalysisStatus '{}', defaulting to Pending", s);
                Self::Pending
            }
        }
    }
}

impl From<AnalysisStatus> for String {
    fn from(s: AnalysisStatus) -> Self {
        match s {
            AnalysisStatus::Pending => "pending".to_string(),
            AnalysisStatus::Processing => "processing".to_string(),
            AnalysisStatus::Completed => "completed".to_string(),
            AnalysisStatus::Failed => "failed".to_string(),
            AnalysisStatus::Deleted => "deleted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingType {
    Addressed,
    PartiallyAddressed,
    Gap,
    NotApplicable,
}

impl From<String> for FindingType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "addressed" => Self::Addressed,
            "partially_addressed" => Self::PartiallyAddressed,
            "gap" => Self::Gap,
            "not_applicable" => Self::NotApplicable,
            _ => {
                warn!("Unknown FindingType '{}', defaulting to Gap", s);
                Self::Gap
            }
        }
    }
}

impl From<FindingType> for String {
    fn from(t: FindingType) -> Self {
        match t {
            FindingType::Addressed => "addressed".to_string(),
            FindingType::PartiallyAddressed => "partially_addressed".to_string(),
            FindingType::Gap => "gap".to_string(),
            FindingType::NotApplicable => "not_applicable".to_string(),
        }
    }
}

// ============================================================================
// Database Row Structs
// ============================================================================

#[derive(Debug, FromRow)]
pub struct AnalysisRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub input_type: String,
    pub input_text: Option<String>,
    pub original_filename: Option<String>,
    pub file_path: Option<String>,
    pub extracted_text: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub prompt_template: Option<String>,
    pub matched_framework_ids: Option<String>,
    pub processing_time_ms: Option<i64>,
    pub token_count: Option<i64>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct AnalysisFindingRow {
    pub id: String,
    pub analysis_id: String,
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: String,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
    pub sort_order: i32,
    pub created_at: String,
}

// ============================================================================
// API Response Structs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Analysis {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub input_type: InputType,
    pub input_text: Option<String>,
    pub original_filename: Option<String>,
    pub file_path: Option<String>,
    pub extracted_text: Option<String>,
    pub status: AnalysisStatus,
    pub error_message: Option<String>,
    pub prompt_template: Option<String>,
    pub matched_framework_ids: Vec<String>,
    pub processing_time_ms: Option<i64>,
    pub token_count: Option<i64>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AnalysisRow> for Analysis {
    fn from(row: AnalysisRow) -> Self {
        let matched_framework_ids = row
            .matched_framework_ids
            .as_deref()
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
            .unwrap_or_default();

        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            input_type: InputType::from(row.input_type),
            input_text: row.input_text,
            original_filename: row.original_filename,
            file_path: row.file_path,
            extracted_text: row.extracted_text,
            status: AnalysisStatus::from(row.status),
            error_message: row.error_message,
            prompt_template: row.prompt_template,
            matched_framework_ids,
            processing_time_ms: row.processing_time_ms,
            token_count: row.token_count,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AnalysisFinding {
    pub id: String,
    pub analysis_id: String,
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
    pub sort_order: i32,
    pub created_at: String,
}

impl From<AnalysisFindingRow> for AnalysisFinding {
    fn from(row: AnalysisFindingRow) -> Self {
        Self {
            id: row.id,
            analysis_id: row.analysis_id,
            concept_id: row.concept_id,
            framework_id: row.framework_id,
            finding_type: FindingType::from(row.finding_type),
            confidence_score: row.confidence_score,
            evidence_text: row.evidence_text,
            recommendation: row.recommendation,
            priority: row.priority,
            sort_order: row.sort_order,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AnalysisFindingWithConcept {
    pub id: String,
    pub analysis_id: String,
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
    pub sort_order: i32,
    pub created_at: String,
    // Concept metadata (from JOIN)
    pub concept_code: Option<String>,
    pub concept_name_en: String,
    pub concept_name_nb: String,
    pub concept_definition_en: String,
    pub concept_definition_nb: Option<String>,
    pub source_reference: Option<String>,
}

// ============================================================================
// Request Structs
// ============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAnalysisRequest {
    pub name: String,
    pub description: Option<String>,
    pub input_text: String,
    pub prompt_template: Option<String>,
}

// ============================================================================
// Summary / Aggregation Structs
// ============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct FrameworkFindingSummary {
    pub framework_id: String,
    pub framework_name: String,
    pub total_findings: i64,
    pub addressed_count: i64,
    pub partially_addressed_count: i64,
    pub gap_count: i64,
    pub not_applicable_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AnalysisSummary {
    #[serde(flatten)]
    pub analysis: Analysis,
    pub total_findings: i64,
    pub gap_count: i64,
    pub addressed_count: i64,
    pub partially_addressed_count: i64,
    pub frameworks_matched: Vec<FrameworkFindingSummary>,
}

// ============================================================================
// Query Parameter Structs
// ============================================================================

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    50
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct AnalysisListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub status: Option<AnalysisStatus>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct FindingsListQuery {
    pub framework_id: Option<String>,
    pub finding_type: Option<FindingType>,
    pub priority: Option<i32>,
    pub sort_by: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // InputType tests
    #[test]
    fn input_type_from_text() {
        assert_eq!(InputType::from("text".to_string()), InputType::Text);
    }

    #[test]
    fn input_type_from_pdf() {
        assert_eq!(InputType::from("pdf".to_string()), InputType::Pdf);
    }

    #[test]
    fn input_type_from_docx() {
        assert_eq!(InputType::from("docx".to_string()), InputType::Docx);
    }

    #[test]
    fn input_type_from_unknown_defaults_to_text() {
        assert_eq!(InputType::from("unknown".to_string()), InputType::Text);
    }

    #[test]
    fn input_type_to_string_text() {
        assert_eq!(String::from(InputType::Text), "text");
    }

    #[test]
    fn input_type_to_string_pdf() {
        assert_eq!(String::from(InputType::Pdf), "pdf");
    }

    #[test]
    fn input_type_to_string_docx() {
        assert_eq!(String::from(InputType::Docx), "docx");
    }

    #[test]
    fn input_type_serde_roundtrip() {
        for variant in [InputType::Text, InputType::Pdf, InputType::Docx] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: InputType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    // AnalysisStatus tests
    #[test]
    fn analysis_status_roundtrip_all_variants() {
        let variants = vec![
            AnalysisStatus::Pending,
            AnalysisStatus::Processing,
            AnalysisStatus::Completed,
            AnalysisStatus::Failed,
            AnalysisStatus::Deleted,
        ];
        for v in variants {
            let s = String::from(v.clone());
            let back = AnalysisStatus::from(s);
            assert_eq!(v, back);
        }
    }

    #[test]
    fn analysis_status_from_unknown_defaults_to_pending() {
        assert_eq!(
            AnalysisStatus::from("garbage".to_string()),
            AnalysisStatus::Pending
        );
    }

    #[test]
    fn analysis_status_serde_roundtrip() {
        for variant in [
            AnalysisStatus::Pending,
            AnalysisStatus::Processing,
            AnalysisStatus::Completed,
            AnalysisStatus::Failed,
            AnalysisStatus::Deleted,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: AnalysisStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    // FindingType tests
    #[test]
    fn finding_type_roundtrip_all_variants() {
        let variants = vec![
            FindingType::Addressed,
            FindingType::PartiallyAddressed,
            FindingType::Gap,
            FindingType::NotApplicable,
        ];
        for v in variants {
            let s = String::from(v.clone());
            let back = FindingType::from(s);
            assert_eq!(v, back);
        }
    }

    #[test]
    fn finding_type_partially_addressed_underscore() {
        assert_eq!(
            FindingType::from("partially_addressed".to_string()),
            FindingType::PartiallyAddressed
        );
    }

    #[test]
    fn finding_type_from_unknown_defaults_to_gap() {
        assert_eq!(
            FindingType::from("nonsense".to_string()),
            FindingType::Gap
        );
    }

    #[test]
    fn finding_type_serde_roundtrip() {
        for variant in [
            FindingType::Addressed,
            FindingType::PartiallyAddressed,
            FindingType::Gap,
            FindingType::NotApplicable,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: FindingType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    // From<AnalysisRow> tests
    fn make_analysis_row() -> AnalysisRow {
        AnalysisRow {
            id: "test-id".to_string(),
            name: "Test Analysis".to_string(),
            description: Some("desc".to_string()),
            input_type: "pdf".to_string(),
            input_text: None,
            original_filename: Some("doc.pdf".to_string()),
            file_path: Some("/uploads/test.pdf".to_string()),
            extracted_text: Some("extracted".to_string()),
            status: "completed".to_string(),
            error_message: None,
            prompt_template: None,
            matched_framework_ids: Some(r#"["nist-csf","iso31000"]"#.to_string()),
            processing_time_ms: Some(1200),
            token_count: Some(5000),
            created_by: None,
            created_at: "2026-03-17 10:00:00".to_string(),
            updated_at: "2026-03-17 10:00:00".to_string(),
        }
    }

    #[test]
    fn test_analysis_from_row_all_fields() {
        let row = make_analysis_row();
        let analysis = Analysis::from(row);
        assert_eq!(analysis.input_type, InputType::Pdf);
        assert_eq!(analysis.status, AnalysisStatus::Completed);
        assert_eq!(analysis.matched_framework_ids, vec!["nist-csf", "iso31000"]);
    }

    #[test]
    fn test_analysis_matched_frameworks_none() {
        let mut row = make_analysis_row();
        row.matched_framework_ids = None;
        let analysis = Analysis::from(row);
        assert!(analysis.matched_framework_ids.is_empty());
    }

    #[test]
    fn test_analysis_matched_frameworks_malformed_json() {
        let mut row = make_analysis_row();
        row.matched_framework_ids = Some("not json".to_string());
        let analysis = Analysis::from(row);
        assert!(analysis.matched_framework_ids.is_empty());
    }

    #[test]
    fn test_analysis_matched_frameworks_empty_string() {
        let mut row = make_analysis_row();
        row.matched_framework_ids = Some("".to_string());
        let analysis = Analysis::from(row);
        assert!(analysis.matched_framework_ids.is_empty());
    }

    // From<AnalysisFindingRow> tests
    #[test]
    fn test_finding_from_row() {
        let row = AnalysisFindingRow {
            id: "f-1".to_string(),
            analysis_id: "a-1".to_string(),
            concept_id: "c-1".to_string(),
            framework_id: "fw-1".to_string(),
            finding_type: "gap".to_string(),
            confidence_score: 0.75,
            evidence_text: None,
            recommendation: Some("Fix this".to_string()),
            priority: 2,
            sort_order: 0,
            created_at: "2026-03-17 10:00:00".to_string(),
        };
        let finding = AnalysisFinding::from(row);
        assert_eq!(finding.finding_type, FindingType::Gap);
        assert!((finding.confidence_score - 0.75).abs() < f64::EPSILON);
    }

    // Query parameter defaults
    #[test]
    fn test_analysis_list_query_defaults() {
        let query: AnalysisListQuery = serde_json::from_str("{}").unwrap();
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 50);
        assert!(query.status.is_none());
    }

    #[test]
    fn test_findings_list_query_defaults() {
        let query: FindingsListQuery = serde_json::from_str("{}").unwrap();
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 50);
        assert!(query.framework_id.is_none());
    }
}
