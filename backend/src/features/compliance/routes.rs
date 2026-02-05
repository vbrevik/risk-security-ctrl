use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct PlaceholderResponse {
    pub message: String,
}

/// List assessments (placeholder)
pub async fn list_assessments() -> Json<PlaceholderResponse> {
    Json(PlaceholderResponse {
        message: "Compliance assessments endpoint - coming in Sprint 3".to_string(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/assessments", get(list_assessments))
}
