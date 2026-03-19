use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
    pub cookie_key: Option<String>,
    pub session_duration_hours: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:ontology.db".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            cookie_key: env::var("COOKIE_KEY").ok(),
            session_duration_hours: env::var("SESSION_DURATION_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn config_parses_cookie_key_from_env() {
        let _lock = ENV_LOCK.lock().unwrap();
        // 64-char hex string = 32 bytes
        let hex_key = "a".repeat(64);
        env::set_var("COOKIE_KEY", &hex_key);
        let config = Config::from_env();
        env::remove_var("COOKIE_KEY");

        assert!(config.cookie_key.is_some());
        let decoded = hex::decode(config.cookie_key.unwrap()).unwrap();
        assert!(decoded.len() >= 32);
    }

    #[test]
    fn config_defaults_session_duration_to_8() {
        let _lock = ENV_LOCK.lock().unwrap();
        env::remove_var("SESSION_DURATION_HOURS");
        let config = Config::from_env();
        assert_eq!(config.session_duration_hours, 8);
    }

    #[test]
    fn config_parses_session_duration_from_env() {
        let _lock = ENV_LOCK.lock().unwrap();
        env::set_var("SESSION_DURATION_HOURS", "24");
        let config = Config::from_env();
        env::remove_var("SESSION_DURATION_HOURS");
        assert_eq!(config.session_duration_hours, 24);
    }
}
