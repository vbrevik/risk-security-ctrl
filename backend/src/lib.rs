use axum::{routing::get, Router};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub mod config;
pub mod error;
pub mod features;
pub mod import;

pub use config::Config;
use features::analysis::matcher::Topic;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
    pub topics: Vec<Topic>,
}

/// Load topics from a JSON file at the given path.
///
/// Returns an empty Vec (with a warning) if the file is missing or malformed.
pub fn load_topics(path: &std::path::Path) -> Vec<Topic> {
    if !path.exists() {
        tracing::warn!("topic-tags.json not found at {:?}, analysis matching will have no topics", path);
        return vec![];
    }
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to read topic-tags.json: {}", e);
            return vec![];
        }
    };
    let file: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Failed to parse topic-tags.json: {}", e);
            return vec![];
        }
    };
    let Some(arr) = file["topics"].as_array() else {
        tracing::warn!("topic-tags.json missing 'topics' array");
        return vec![];
    };
    let mut topics = Vec::with_capacity(arr.len());
    for (i, v) in arr.iter().enumerate() {
        match (v["id"].as_str(), v["name_en"].as_str(), v["concept_ids"].as_array()) {
            (Some(id), Some(name), Some(cids)) => {
                topics.push(Topic {
                    id: id.to_string(),
                    name_en: name.to_string(),
                    concept_ids: cids.iter().filter_map(|c| c.as_str().map(String::from)).collect(),
                });
            }
            _ => {
                tracing::warn!("Skipping malformed topic at index {}", i);
            }
        }
    }
    topics
}

/// Create the application router (for testing)
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api", api_routes())
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(features::ontology::routes::health_check))
        .nest("/ontology", features::ontology::routes::router())
        .nest("/compliance", features::compliance::routes::router())
        .nest("/reports", features::reports::routes::router())
        .nest("/auth", features::auth::routes::router())
        .nest("/analyses", features::analysis::routes::router())
}
