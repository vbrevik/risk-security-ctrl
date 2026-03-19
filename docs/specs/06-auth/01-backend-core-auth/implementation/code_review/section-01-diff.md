diff --git a/backend/.gitignore b/backend/.gitignore
index f8cf2a0..82c24a8 100644
--- a/backend/.gitignore
+++ b/backend/.gitignore
@@ -16,6 +16,9 @@
 *.swp
 *.swo
 
+# Auth cookie key (auto-generated)
+.cookie_key
+
 # OS files
 .DS_Store
 Thumbs.db
diff --git a/backend/Cargo.toml b/backend/Cargo.toml
index 7bba508..77f9194 100644
--- a/backend/Cargo.toml
+++ b/backend/Cargo.toml
@@ -40,6 +40,15 @@ dotenvy = "0.15"
 uuid = { version = "1", features = ["v4", "serde"] }
 chrono = { version = "0.4", features = ["serde"] }
 
+# Auth
+argon2 = "0.5"
+axum-extra = { version = "0.10", features = ["cookie-private", "cookie-key-expansion"] }
+validator = { version = "0.20", features = ["derive"] }
+rand = "0.9"
+hex = "0.4"
+time = "0.3"
+sha2 = "0.10"
+
 # Document parsing
 pdf-extract = "0.10"
 zip = "2"
diff --git a/backend/src/config.rs b/backend/src/config.rs
index ea3e0a2..579c15a 100644
--- a/backend/src/config.rs
+++ b/backend/src/config.rs
@@ -6,6 +6,8 @@ pub struct Config {
     pub host: String,
     pub port: u16,
     pub frontend_url: String,
+    pub cookie_key: Option<String>,
+    pub session_duration_hours: u64,
 }
 
 impl Config {
@@ -20,6 +22,50 @@ impl Config {
                 .unwrap_or(3000),
             frontend_url: env::var("FRONTEND_URL")
                 .unwrap_or_else(|_| "http://localhost:5173".to_string()),
+            cookie_key: env::var("COOKIE_KEY").ok(),
+            session_duration_hours: env::var("SESSION_DURATION_HOURS")
+                .ok()
+                .and_then(|s| s.parse().ok())
+                .unwrap_or(8),
         }
     }
 }
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::sync::Mutex;
+
+    static ENV_LOCK: Mutex<()> = Mutex::new(());
+
+    #[test]
+    fn config_parses_cookie_key_from_env() {
+        let _lock = ENV_LOCK.lock().unwrap();
+        // 64-char hex string = 32 bytes
+        let hex_key = "a".repeat(64);
+        env::set_var("COOKIE_KEY", &hex_key);
+        let config = Config::from_env();
+        env::remove_var("COOKIE_KEY");
+
+        assert!(config.cookie_key.is_some());
+        let decoded = hex::decode(config.cookie_key.unwrap()).unwrap();
+        assert!(decoded.len() >= 32);
+    }
+
+    #[test]
+    fn config_defaults_session_duration_to_8() {
+        let _lock = ENV_LOCK.lock().unwrap();
+        env::remove_var("SESSION_DURATION_HOURS");
+        let config = Config::from_env();
+        assert_eq!(config.session_duration_hours, 8);
+    }
+
+    #[test]
+    fn config_parses_session_duration_from_env() {
+        let _lock = ENV_LOCK.lock().unwrap();
+        env::set_var("SESSION_DURATION_HOURS", "24");
+        let config = Config::from_env();
+        env::remove_var("SESSION_DURATION_HOURS");
+        assert_eq!(config.session_duration_hours, 24);
+    }
+}
diff --git a/backend/src/lib.rs b/backend/src/lib.rs
index 3e7cadc..444acfc 100644
--- a/backend/src/lib.rs
+++ b/backend/src/lib.rs
@@ -1,7 +1,8 @@
-use axum::{routing::get, Router};
+use axum::{extract::FromRef, http, routing::get, Router};
+use axum_extra::extract::cookie::Key;
 use tower_http::{
     compression::CompressionLayer,
-    cors::{Any, CorsLayer},
+    cors::CorsLayer,
     trace::TraceLayer,
 };
 
@@ -18,6 +19,76 @@ pub struct AppState {
     pub db: sqlx::SqlitePool,
     pub config: Config,
     pub topics: Vec<Topic>,
+    pub cookie_key: Key,
+}
+
+impl FromRef<AppState> for Key {
+    fn from_ref(state: &AppState) -> Self {
+        state.cookie_key.clone()
+    }
+}
+
+/// Initialize the cookie encryption key from env, file, or auto-generation.
+pub fn init_cookie_key(config: &Config) -> Key {
+    if let Some(ref hex_string) = config.cookie_key {
+        let bytes = hex::decode(hex_string).expect("COOKIE_KEY is not valid hex");
+        if bytes.len() < 32 {
+            panic!(
+                "COOKIE_KEY must be at least 32 bytes (64 hex chars), got {} bytes ({} hex chars)",
+                bytes.len(),
+                hex_string.len()
+            );
+        }
+        return Key::derive_from(&bytes);
+    }
+
+    let key_path = std::path::Path::new(".cookie_key");
+    if key_path.exists() {
+        let contents = std::fs::read_to_string(key_path).expect("Failed to read .cookie_key file");
+        let hex_string = contents.trim();
+        let bytes = hex::decode(hex_string).expect(".cookie_key file contains invalid hex");
+        if bytes.len() < 32 {
+            panic!(
+                ".cookie_key file must contain at least 32 bytes (64 hex chars), got {} bytes",
+                bytes.len()
+            );
+        }
+
+        #[cfg(unix)]
+        {
+            use std::os::unix::fs::PermissionsExt;
+            let mode = std::fs::metadata(key_path)
+                .expect("Failed to read .cookie_key metadata")
+                .permissions()
+                .mode();
+            if mode & 0o077 != 0 {
+                tracing::warn!(
+                    ".cookie_key file has overly permissive permissions ({:o}), recommend 0600",
+                    mode
+                );
+            }
+        }
+
+        return Key::derive_from(&bytes);
+    }
+
+    // Auto-generate a new key
+    let mut bytes = [0u8; 32];
+    rand::Fill::fill(&mut bytes, &mut rand::rng());
+    let hex_string = hex::encode(bytes);
+    std::fs::write(key_path, &hex_string).expect("Failed to write .cookie_key file");
+
+    #[cfg(unix)]
+    {
+        use std::os::unix::fs::PermissionsExt;
+        let perms = std::fs::Permissions::from_mode(0o600);
+        std::fs::set_permissions(key_path, perms).expect("Failed to set .cookie_key permissions");
+    }
+
+    tracing::warn!(
+        "Generated new cookie key and saved to .cookie_key. Set COOKIE_KEY env var for production."
+    );
+    Key::derive_from(&bytes)
 }
 
 /// Load topics from a JSON file at the given path.
@@ -67,9 +138,16 @@ pub fn load_topics(path: &std::path::Path) -> Vec<Topic> {
 /// Create the application router (for testing)
 pub fn create_router(state: AppState) -> Router {
     let cors = CorsLayer::new()
-        .allow_origin(Any)
-        .allow_methods(Any)
-        .allow_headers(Any);
+        .allow_origin(
+            state
+                .config
+                .frontend_url
+                .parse::<http::header::HeaderValue>()
+                .expect("Invalid FRONTEND_URL for CORS origin"),
+        )
+        .allow_methods(tower_http::cors::Any)
+        .allow_headers(tower_http::cors::Any)
+        .allow_credentials(true);
 
     Router::new()
         .nest("/api", api_routes())
@@ -88,3 +166,37 @@ fn api_routes() -> Router<AppState> {
         .nest("/auth", features::auth::routes::router())
         .nest("/analyses", features::analysis::routes::router())
 }
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn rejects_short_cookie_key() {
+        let config = Config {
+            database_url: String::new(),
+            host: String::new(),
+            port: 0,
+            frontend_url: String::new(),
+            cookie_key: Some("aabbccdd".to_string()), // 4 bytes, too short
+            session_duration_hours: 8,
+        };
+        let result = std::panic::catch_unwind(|| init_cookie_key(&config));
+        assert!(result.is_err(), "Should panic on short cookie key");
+    }
+
+    #[test]
+    fn accepts_valid_cookie_key() {
+        let hex_key = "ab".repeat(32); // 32 bytes = 64 hex chars
+        let config = Config {
+            database_url: String::new(),
+            host: String::new(),
+            port: 0,
+            frontend_url: String::new(),
+            cookie_key: Some(hex_key),
+            session_duration_hours: 8,
+        };
+        // Should not panic
+        let _key = init_cookie_key(&config);
+    }
+}
diff --git a/backend/src/main.rs b/backend/src/main.rs
index c0f6593..776cf54 100644
--- a/backend/src/main.rs
+++ b/backend/src/main.rs
@@ -148,11 +148,15 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let topics = ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
     tracing::info!("Loaded {} topics for analysis matching", topics.len());
 
+    // Initialize cookie key for session encryption
+    let cookie_key = ontology_backend::init_cookie_key(&config);
+
     // Create application state
     let state = AppState {
         db,
         config: config.clone(),
         topics,
+        cookie_key,
     };
 
     // Build router with Swagger UI
diff --git a/backend/tests/common/mod.rs b/backend/tests/common/mod.rs
index 08b7205..f5bf3ad 100644
--- a/backend/tests/common/mod.rs
+++ b/backend/tests/common/mod.rs
@@ -42,10 +42,12 @@ pub async fn create_test_app() -> Router {
 
     let topics = ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
 
+    let cookie_key = axum_extra::extract::cookie::Key::generate();
     let state = AppState {
         db: pool,
         config: config.clone(),
         topics,
+        cookie_key,
     };
 
     ontology_backend::create_router(state)
