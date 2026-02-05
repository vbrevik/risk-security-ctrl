use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct PlaceholderResponse {
    pub message: String,
}

/// List reports (placeholder)
pub async fn list_reports() -> Json<PlaceholderResponse> {
    Json(PlaceholderResponse {
        message: "Reports endpoint - coming in Sprint 5".to_string(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_reports))
}
