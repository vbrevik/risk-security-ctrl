use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
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

/// Security headers middleware.
/// Sets security response headers on every response.
pub async fn security_headers(
    req: Request<Body>,
    next: Next,
    enable_https: bool,
) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'"),
    );
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("0"),
    );

    if enable_https {
        headers.insert(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        );
    }

    response
}

/// CSRF protection middleware.
/// Rejects POST, PUT, DELETE requests missing X-Requested-With: XMLHttpRequest header.
pub async fn csrf_check(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();

    // Safe methods pass through
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
        return next.run(req).await;
    }

    // For state-changing methods, require X-Requested-With header
    let has_csrf_header = req
        .headers()
        .get("x-requested-with")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("XMLHttpRequest"))
        .unwrap_or(false);

    if !has_csrf_header {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "forbidden",
                "message": "CSRF validation failed"
            })),
        )
            .into_response();
    }

    next.run(req).await
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
