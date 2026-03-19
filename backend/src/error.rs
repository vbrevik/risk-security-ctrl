use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

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
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
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
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
        });

        (status, body).into_response()
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
