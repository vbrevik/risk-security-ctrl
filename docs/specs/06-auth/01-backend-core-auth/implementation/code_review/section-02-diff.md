diff --git a/backend/src/error.rs b/backend/src/error.rs
index f3a9156..0fdb945 100644
--- a/backend/src/error.rs
+++ b/backend/src/error.rs
@@ -4,6 +4,13 @@ use axum::{
     Json,
 };
 use serde::Serialize;
+use utoipa::ToSchema;
+
+#[derive(Debug, Serialize, Clone, ToSchema)]
+pub struct FieldError {
+    pub field: String,
+    pub message: String,
+}
 
 #[derive(Debug, thiserror::Error)]
 pub enum AppError {
@@ -24,12 +31,23 @@ pub enum AppError {
 
     #[error("Internal server error: {0}")]
     Internal(String),
+
+    #[error("Invalid credentials")]
+    InvalidCredentials,
+
+    #[error("Validation error")]
+    ValidationError(Vec<FieldError>),
+
+    #[error("Session expired")]
+    SessionExpired,
 }
 
 #[derive(Serialize)]
 pub struct ErrorResponse {
     pub error: String,
     pub message: String,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub fields: Option<Vec<FieldError>>,
 }
 
 impl IntoResponse for AppError {
@@ -63,17 +81,59 @@ impl IntoResponse for AppError {
                     "An internal error occurred".to_string(),
                 )
             }
+            AppError::InvalidCredentials => (
+                StatusCode::UNAUTHORIZED,
+                "invalid_credentials",
+                "Invalid credentials".to_string(),
+            ),
+            AppError::SessionExpired => (
+                StatusCode::UNAUTHORIZED,
+                "session_expired",
+                "Session expired".to_string(),
+            ),
+            AppError::ValidationError(_) => (
+                StatusCode::UNPROCESSABLE_ENTITY,
+                "validation_error",
+                "Validation failed".to_string(),
+            ),
+        };
+
+        let fields = match &self {
+            AppError::ValidationError(f) => Some(f.clone()),
+            _ => None,
         };
 
         let body = Json(ErrorResponse {
             error: error_type.to_string(),
             message,
+            fields,
         });
 
         (status, body).into_response()
     }
 }
 
+impl From<validator::ValidationErrors> for AppError {
+    fn from(err: validator::ValidationErrors) -> Self {
+        let fields = err
+            .field_errors()
+            .into_iter()
+            .map(|(field, errors)| {
+                let message = errors
+                    .first()
+                    .and_then(|e| e.message.as_ref())
+                    .map(|m| m.to_string())
+                    .unwrap_or_else(|| format!("Invalid value for {field}"));
+                FieldError {
+                    field: field.to_string(),
+                    message,
+                }
+            })
+            .collect();
+        AppError::ValidationError(fields)
+    }
+}
+
 pub type AppResult<T> = Result<T, AppError>;
 
 impl From<crate::features::analysis::parser::ParsingError> for AppError {
diff --git a/backend/src/features/auth/mod.rs b/backend/src/features/auth/mod.rs
index 6a664ab..a0e1883 100644
--- a/backend/src/features/auth/mod.rs
+++ b/backend/src/features/auth/mod.rs
@@ -1 +1,2 @@
+pub mod models;
 pub mod routes;
diff --git a/backend/src/features/auth/models.rs b/backend/src/features/auth/models.rs
new file mode 100644
index 0000000..0949b8d
--- /dev/null
+++ b/backend/src/features/auth/models.rs
@@ -0,0 +1,123 @@
+use serde::{Deserialize, Serialize};
+use utoipa::ToSchema;
+use validator::Validate;
+
+/// Registration request body.
+#[derive(Debug, Deserialize, Validate, ToSchema)]
+pub struct RegisterRequest {
+    #[validate(email)]
+    pub email: String,
+    #[validate(length(min = 1, max = 255))]
+    pub name: String,
+    #[validate(length(min = 8))]
+    pub password: String,
+}
+
+/// Login request body.
+#[derive(Debug, Deserialize, Validate, ToSchema)]
+pub struct LoginRequest {
+    #[validate(email)]
+    pub email: String,
+    #[validate(length(min = 1))]
+    pub password: String,
+}
+
+/// Auth response returned on successful register/login.
+#[derive(Debug, Serialize, ToSchema)]
+pub struct AuthResponse {
+    pub token: String,
+    pub user: UserProfile,
+}
+
+/// Public-facing user representation (no sensitive fields).
+#[derive(Debug, Serialize, ToSchema)]
+pub struct UserProfile {
+    pub id: String,
+    pub email: String,
+    pub name: String,
+    pub role: String,
+}
+
+/// Populated by the FromRequestParts extractor after session validation.
+/// Handlers that need an authenticated user declare `AuthUser` as a parameter.
+/// Intentionally does NOT derive Serialize to prevent accidental session_id leakage.
+#[derive(Debug, Clone)]
+pub struct AuthUser {
+    pub id: String,
+    pub email: String,
+    pub name: String,
+    pub role: String,
+    pub session_id: String,
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use validator::Validate;
+
+    #[test]
+    fn register_request_rejects_invalid_email() {
+        let req = RegisterRequest {
+            email: "notanemail".to_string(),
+            name: "Test User".to_string(),
+            password: "validpassword123".to_string(),
+        };
+        let result = req.validate();
+        assert!(result.is_err());
+        let errors = result.unwrap_err();
+        assert!(errors.field_errors().contains_key("email"));
+    }
+
+    #[test]
+    fn register_request_rejects_short_password() {
+        let req = RegisterRequest {
+            email: "user@example.com".to_string(),
+            name: "Test User".to_string(),
+            password: "short".to_string(),
+        };
+        let result = req.validate();
+        assert!(result.is_err());
+        let errors = result.unwrap_err();
+        assert!(errors.field_errors().contains_key("password"));
+    }
+
+    #[test]
+    fn login_request_rejects_empty_password() {
+        let req = LoginRequest {
+            email: "user@example.com".to_string(),
+            password: "".to_string(),
+        };
+        let result = req.validate();
+        assert!(result.is_err());
+        let errors = result.unwrap_err();
+        assert!(errors.field_errors().contains_key("password"));
+    }
+
+    #[test]
+    fn user_profile_serializes_correctly() {
+        let profile = UserProfile {
+            id: "user-123".to_string(),
+            email: "user@example.com".to_string(),
+            name: "Test User".to_string(),
+            role: "admin".to_string(),
+        };
+        let value = serde_json::to_value(&profile).unwrap();
+        assert_eq!(value["id"], "user-123");
+        assert_eq!(value["email"], "user@example.com");
+        assert_eq!(value["name"], "Test User");
+        assert_eq!(value["role"], "admin");
+        // Ensure no sensitive fields leak
+        assert!(value.get("password_hash").is_none());
+        assert!(value.get("session_id").is_none());
+    }
+
+    #[test]
+    fn register_request_accepts_valid_input() {
+        let req = RegisterRequest {
+            email: "user@example.com".to_string(),
+            name: "Test User".to_string(),
+            password: "validpassword123".to_string(),
+        };
+        assert!(req.validate().is_ok());
+    }
+}
