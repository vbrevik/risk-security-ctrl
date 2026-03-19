use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Registration request body.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 8))]
    pub password: String,
}

/// Login request body.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

/// Auth response returned on successful register/login.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

/// Public-facing user representation (no sensitive fields).
#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

/// Populated by the FromRequestParts extractor after session validation.
/// Handlers that need an authenticated user declare `AuthUser` as a parameter.
/// Intentionally does NOT derive Serialize to prevent accidental session_id leakage.
/// Debug is manually implemented to redact session_id from log output.
#[derive(Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub session_id: String,
}

impl std::fmt::Debug for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthUser")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("name", &self.name)
            .field("role", &self.role)
            .field("session_id", &"[redacted]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn register_request_rejects_invalid_email() {
        let req = RegisterRequest {
            email: "notanemail".to_string(),
            name: "Test User".to_string(),
            password: "validpassword123".to_string(),
        };
        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn register_request_rejects_short_password() {
        let req = RegisterRequest {
            email: "user@example.com".to_string(),
            name: "Test User".to_string(),
            password: "short".to_string(),
        };
        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn login_request_rejects_empty_password() {
        let req = LoginRequest {
            email: "user@example.com".to_string(),
            password: "".to_string(),
        };
        let result = req.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn user_profile_serializes_correctly() {
        let profile = UserProfile {
            id: "user-123".to_string(),
            email: "user@example.com".to_string(),
            name: "Test User".to_string(),
            role: "admin".to_string(),
        };
        let value = serde_json::to_value(&profile).unwrap();
        assert_eq!(value["id"], "user-123");
        assert_eq!(value["email"], "user@example.com");
        assert_eq!(value["name"], "Test User");
        assert_eq!(value["role"], "admin");
        // Ensure no sensitive fields leak
        assert!(value.get("password_hash").is_none());
        assert!(value.get("session_id").is_none());
    }

    #[test]
    fn register_request_accepts_valid_input() {
        let req = RegisterRequest {
            email: "user@example.com".to_string(),
            name: "Test User".to_string(),
            password: "validpassword123".to_string(),
        };
        assert!(req.validate().is_ok());
    }
}
