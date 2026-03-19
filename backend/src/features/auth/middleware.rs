use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use super::models::AuthUser;
use super::permissions::{Action, Feature, HasPermission};

/// Check if the authenticated user has the required permission.
/// Returns Ok(()) if allowed, or an appropriate error response.
///
/// Usage in handlers:
/// ```rust,ignore
/// async fn delete_assessment(user: AuthUser, ...) -> Result<impl IntoResponse, AppError> {
///     check_permission(&user, Feature::Compliance, Action::Delete)?;
///     // ... handler logic
/// }
/// ```
pub fn check_permission(
    user: &AuthUser,
    feature: Feature,
    action: Action,
) -> Result<(), Response> {
    if user.has_permission(feature, action) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "forbidden",
                "message": "Insufficient permissions"
            })),
        )
            .into_response())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_user(role: &str) -> AuthUser {
        AuthUser {
            id: "1".to_string(),
            email: "test@test.com".to_string(),
            name: "Test".to_string(),
            role: role.to_string(),
            session_id: "s1".to_string(),
        }
    }

    #[test]
    fn check_permission_allows_admin() {
        let user = make_user("admin");
        assert!(check_permission(&user, Feature::Compliance, Action::Delete).is_ok());
    }

    #[test]
    fn check_permission_denies_viewer_write() {
        let user = make_user("viewer");
        assert!(check_permission(&user, Feature::Compliance, Action::Create).is_err());
    }

    #[test]
    fn check_permission_allows_viewer_read() {
        let user = make_user("viewer");
        assert!(check_permission(&user, Feature::Compliance, Action::Read).is_ok());
    }

    #[test]
    fn check_permission_denies_specialist_delete() {
        let user = make_user("specialist");
        assert!(check_permission(&user, Feature::Compliance, Action::Delete).is_err());
    }

    #[test]
    fn check_permission_allows_risk_manager_delete() {
        let user = make_user("risk_manager");
        assert!(check_permission(&user, Feature::Compliance, Action::Delete).is_ok());
    }
}
