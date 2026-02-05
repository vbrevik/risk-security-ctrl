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

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
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
}
