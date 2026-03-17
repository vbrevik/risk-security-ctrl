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
