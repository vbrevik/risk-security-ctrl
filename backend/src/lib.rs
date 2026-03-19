use axum::{extract::FromRef, http, middleware, routing::get, Router};
use axum_extra::extract::cookie::Key;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
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
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

/// Initialize the cookie encryption key from env, file, or auto-generation.
pub fn init_cookie_key(config: &Config) -> Key {
    if let Some(ref hex_string) = config.cookie_key {
        let bytes = hex::decode(hex_string).expect("COOKIE_KEY is not valid hex");
        if bytes.len() < 32 {
            panic!(
                "COOKIE_KEY must be at least 32 bytes (64 hex chars), got {} bytes ({} hex chars)",
                bytes.len(),
                hex_string.len()
            );
        }
        return Key::derive_from(&bytes);
    }

    let key_path = std::path::Path::new(".cookie_key");
    if key_path.exists() {
        let contents = std::fs::read_to_string(key_path).expect("Failed to read .cookie_key file");
        let hex_string = contents.trim();
        let bytes = hex::decode(hex_string).expect(".cookie_key file contains invalid hex");
        if bytes.len() < 32 {
            panic!(
                ".cookie_key file must contain at least 32 bytes (64 hex chars), got {} bytes",
                bytes.len()
            );
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(key_path)
                .expect("Failed to read .cookie_key metadata")
                .permissions()
                .mode();
            if mode & 0o077 != 0 {
                tracing::warn!(
                    ".cookie_key file has overly permissive permissions ({:o}), recommend 0600",
                    mode
                );
            }
        }

        return Key::derive_from(&bytes);
    }

    // Auto-generate a new key
    let mut bytes = [0u8; 32];
    rand::Fill::fill(&mut bytes, &mut rand::rng());
    let hex_string = hex::encode(bytes);
    std::fs::write(key_path, &hex_string).expect("Failed to write .cookie_key file");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(key_path, perms).expect("Failed to set .cookie_key permissions");
    }

    tracing::warn!(
        "Generated new cookie key and saved to .cookie_key. Set COOKIE_KEY env var for production."
    );
    Key::derive_from(&bytes)
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

/// Create the application router
pub fn create_router(state: AppState) -> Router {
    let enable_https = state.config.enable_https;

    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .frontend_url
                .parse::<http::header::HeaderValue>()
                .expect("Invalid FRONTEND_URL for CORS origin"),
        )
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::PATCH,
            http::Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::AllowHeaders::mirror_request())
        .allow_credentials(true);

    Router::new()
        .nest("/api", api_routes())
        .layer(middleware::from_fn(features::auth::middleware::csrf_check))
        .layer(cors)
        .layer(middleware::from_fn(move |req, next| {
            features::auth::middleware::security_headers(req, next, enable_https)
        }))
        .layer(SetSensitiveHeadersLayer::new([
            http::header::AUTHORIZATION,
            http::header::COOKIE,
        ]))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_short_cookie_key() {
        let config = Config {
            database_url: String::new(),
            host: String::new(),
            port: 0,
            frontend_url: String::new(),
            cookie_key: Some("aabbccdd".to_string()), // 4 bytes, too short
            session_duration_hours: 8,
            behind_proxy: false,
            enable_https: false,
        };
        let result = std::panic::catch_unwind(|| init_cookie_key(&config));
        assert!(result.is_err(), "Should panic on short cookie key");
    }

    #[test]
    fn accepts_valid_cookie_key() {
        let hex_key = "ab".repeat(32); // 32 bytes = 64 hex chars
        let config = Config {
            database_url: String::new(),
            host: String::new(),
            port: 0,
            frontend_url: String::new(),
            cookie_key: Some(hex_key),
            session_duration_hours: 8,
            behind_proxy: false,
            enable_https: false,
        };
        // Should not panic
        let _key = init_cookie_key(&config);
    }
}
