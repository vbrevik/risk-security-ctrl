use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

/// Framework response
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Framework {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub source_url: Option<String>,
    pub verification_status: Option<String>,
    pub verification_date: Option<String>,
    pub verification_source: Option<String>,
    pub verification_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Concept response
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Concept {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub concept_type: String,
    pub code: Option<String>,
    pub name_en: String,
    pub name_nb: Option<String>,
    pub definition_en: Option<String>,
    pub definition_nb: Option<String>,
    pub source_reference: Option<String>,
    pub sort_order: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

/// Relationship response
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Relationship {
    pub id: String,
    pub source_concept_id: String,
    pub target_concept_id: String,
    pub relationship_type: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

/// Concept with related concept information
#[derive(Debug, Serialize, ToSchema)]
pub struct ConceptWithRelationships {
    #[serde(flatten)]
    pub concept: Concept,
    pub related_concepts: Vec<RelatedConcept>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<ConceptGuidanceResponse>,
}

/// Related concept information
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct RelatedConcept {
    pub relationship_id: String,
    pub relationship_type: String,
    pub concept_id: String,
    pub concept_framework_id: String,
    pub concept_name_en: String,
    pub concept_name_nb: Option<String>,
    pub direction: String, // "outgoing" or "incoming"
}

/// Suggested action from playbook guidance
#[derive(Debug, Serialize, ToSchema)]
pub struct ActionResponse {
    pub sort_order: i64,
    pub text_en: String,
    pub text_nb: Option<String>,
}

/// Transparency question from playbook guidance
#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionResponse {
    pub sort_order: i64,
    pub text_en: String,
    pub text_nb: Option<String>,
}

/// Reference entry from playbook guidance
#[derive(Debug, Serialize, ToSchema)]
pub struct ReferenceResponse {
    #[serde(rename = "type")]
    pub reference_type: String,
    pub title: String,
    pub authors: Option<String>,
    pub year: Option<i64>,
    pub venue: Option<String>,
    pub url: Option<String>,
}

/// Top-level container for concept guidance data
#[derive(Debug, Serialize, ToSchema)]
pub struct ConceptGuidanceResponse {
    pub source_pdf: String,
    pub source_page: i64,
    pub about_en: Option<String>,
    pub about_nb: Option<String>,
    pub suggested_actions: Vec<ActionResponse>,
    pub transparency_questions: Vec<QuestionResponse>,
    pub references: Vec<ReferenceResponse>,
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
        Self {
            data,
            total,
            page,
            limit,
            total_pages,
        }
    }
}

/// Verification proof response for a framework
#[derive(Debug, Serialize, ToSchema)]
pub struct ProofResponse {
    pub framework_id: String,
    pub verification_status: Option<String>,
    pub verification_date: Option<String>,
    pub verification_source: Option<String>,
    pub verification_notes: Option<String>,
    pub proof_content: Option<String>,
}

/// Topic tag for cross-cutting theme filtering
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Topic {
    pub id: String,
    pub name_en: String,
    pub name_nb: String,
    pub description_en: String,
    pub description_nb: String,
    pub concept_ids: Vec<String>,
}

/// Topic tags file structure
#[derive(Debug, Deserialize)]
pub struct TopicTagsFile {
    pub topics: Vec<Topic>,
}

/// Query parameters for listing concepts
#[derive(Debug, Deserialize, IntoParams)]
pub struct ConceptListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub framework_id: Option<String>,
    pub concept_type: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    50
}

/// Search query parameters
#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub framework_id: Option<String>,
}

#[cfg(test)]
mod guidance_response_tests {
    use super::*;

    #[test]
    fn concept_guidance_response_serializes_all_fields() {
        let response = ConceptGuidanceResponse {
            source_pdf: "playbook.pdf".into(),
            source_page: 42,
            about_en: Some("About text".into()),
            about_nb: Some("Om tekst".into()),
            suggested_actions: vec![ActionResponse {
                sort_order: 1,
                text_en: "Do X".into(),
                text_nb: None,
            }],
            transparency_questions: vec![QuestionResponse {
                sort_order: 1,
                text_en: "Why?".into(),
                text_nb: None,
            }],
            references: vec![ReferenceResponse {
                reference_type: "academic".into(),
                title: "Paper".into(),
                authors: Some("Smith".into()),
                year: Some(2024),
                venue: Some("ICML".into()),
                url: None,
            }],
        };
        let json: serde_json::Value = serde_json::to_value(&response).unwrap();
        assert_eq!(json["source_pdf"], "playbook.pdf");
        assert_eq!(json["source_page"], 42);
        assert_eq!(json["about_en"], "About text");
        assert_eq!(json["about_nb"], "Om tekst");
        assert!(json["suggested_actions"].is_array());
        assert!(json["transparency_questions"].is_array());
        assert!(json["references"].is_array());
    }

    #[test]
    fn action_response_serializes_correctly() {
        let action = ActionResponse {
            sort_order: 1,
            text_en: "Do X".into(),
            text_nb: None,
        };
        let json: serde_json::Value = serde_json::to_value(&action).unwrap();
        assert_eq!(json["sort_order"], 1);
        assert_eq!(json["text_en"], "Do X");
        assert!(json["text_nb"].is_null());
    }

    #[test]
    fn question_response_serializes_correctly() {
        let question = QuestionResponse {
            sort_order: 2,
            text_en: "How?".into(),
            text_nb: Some("Hvordan?".into()),
        };
        let json: serde_json::Value = serde_json::to_value(&question).unwrap();
        assert_eq!(json["sort_order"], 2);
        assert_eq!(json["text_en"], "How?");
        assert_eq!(json["text_nb"], "Hvordan?");
    }

    #[test]
    fn reference_response_renames_type_field() {
        let reference = ReferenceResponse {
            reference_type: "academic".into(),
            title: "Paper".into(),
            authors: None,
            year: None,
            venue: None,
            url: None,
        };
        let json: serde_json::Value = serde_json::to_value(&reference).unwrap();
        assert_eq!(json["type"], "academic");
        assert!(json.get("reference_type").is_none());
    }

    #[test]
    fn concept_with_relationships_omits_guidance_when_none() {
        let cwr = ConceptWithRelationships {
            concept: Concept {
                id: "test-1".into(),
                framework_id: "fw-1".into(),
                parent_id: None,
                concept_type: "principle".into(),
                code: None,
                name_en: "Test".into(),
                name_nb: None,
                definition_en: None,
                definition_nb: None,
                source_reference: None,
                sort_order: None,
                created_at: "2024-01-01".into(),
                updated_at: "2024-01-01".into(),
            },
            related_concepts: vec![],
            guidance: None,
        };
        let json: serde_json::Value = serde_json::to_value(&cwr).unwrap();
        assert!(json.get("guidance").is_none());
    }

    #[test]
    fn concept_with_relationships_includes_guidance_when_some() {
        let cwr = ConceptWithRelationships {
            concept: Concept {
                id: "test-2".into(),
                framework_id: "fw-1".into(),
                parent_id: None,
                concept_type: "action".into(),
                code: Some("GV-1.1-001".into()),
                name_en: "Test Action".into(),
                name_nb: None,
                definition_en: None,
                definition_nb: None,
                source_reference: None,
                sort_order: None,
                created_at: "2024-01-01".into(),
                updated_at: "2024-01-01".into(),
            },
            related_concepts: vec![],
            guidance: Some(ConceptGuidanceResponse {
                source_pdf: "playbook.pdf".into(),
                source_page: 10,
                about_en: None,
                about_nb: None,
                suggested_actions: vec![],
                transparency_questions: vec![],
                references: vec![],
            }),
        };
        let json: serde_json::Value = serde_json::to_value(&cwr).unwrap();
        let guidance = json.get("guidance").expect("guidance field should be present");
        assert_eq!(guidance["source_pdf"], "playbook.pdf");
        assert_eq!(guidance["source_page"], 10);
        assert_eq!(guidance["suggested_actions"], serde_json::json!([]));
        assert_eq!(guidance["transparency_questions"], serde_json::json!([]));
        assert_eq!(guidance["references"], serde_json::json!([]));
    }
}
