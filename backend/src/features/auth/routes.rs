use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct PlaceholderResponse {
    pub message: String,
}

/// Current user info (placeholder)
pub async fn me() -> Json<PlaceholderResponse> {
    Json(PlaceholderResponse {
        message: "Auth endpoint - coming in Sprint 7".to_string(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(me))
}
