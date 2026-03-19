use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Validation error")]
    ValidationError(Vec<FieldError>),

    #[error("Session expired")]
    SessionExpired,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldError>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "A database error occurred".to_string(),
                )
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication required".to_string(),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "Access denied".to_string(),
            ),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "Invalid credentials".to_string(),
            ),
            AppError::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                "session_expired",
                "Session expired".to_string(),
            ),
            AppError::ValidationError(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                "Validation failed".to_string(),
            ),
        };

        let fields = match &self {
            AppError::ValidationError(f) => Some(f.clone()),
            _ => None,
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
            fields,
        });

        (status, body).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        let fields = err
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                let message = errors
                    .first()
                    .and_then(|e| e.message.as_ref())
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid value for {field}"));
                FieldError {
                    field: field.to_string(),
                    message,
                }
            })
            .collect();
        AppError::ValidationError(fields)
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl From<crate::features::analysis::parser::ParsingError> for AppError {
    fn from(err: crate::features::analysis::parser::ParsingError) -> Self {
        use crate::features::analysis::parser::ParsingError;
        match err {
            ParsingError::UnsupportedFormat(msg) => AppError::BadRequest(msg),
            ParsingError::EmptyDocument(msg) => AppError::BadRequest(msg),
            ParsingError::FileTooLarge { size, max } => {
                AppError::BadRequest(format!("File too large: {} bytes (max: {})", size, max))
            }
            ParsingError::CorruptFile(msg) => AppError::Internal(msg),
            ParsingError::IoError(e) => AppError::Internal(e.to_string()),
        }
    }
}
