use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar, SameSite};
use validator::Validate;

use crate::error::AppError;
use crate::AppState;

use super::extractors::SESSION_COOKIE_NAME;
use super::models::{AuthResponse, AuthUser, LoginRequest, RegisterRequest, UserProfile};
use super::password;
use super::service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/me", get(me_handler))
}

/// Extract client IP from headers, falling back to "unknown".
fn extract_ip(headers: &HeaderMap) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(val) = forwarded.to_str() {
            if let Some(ip) = val.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(val) = real_ip.to_str() {
            return val.trim().to_string();
        }
    }
    "unknown".to_string()
}

/// Extract User-Agent from headers.
fn extract_user_agent(headers: &HeaderMap) -> String {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created", body = UserProfile),
        (status = 422, description = "Validation error"),
    ),
    tag = "auth"
)]
async fn register_handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;
    let hash = password::hash_password(&body.password)?;
    let profile = service::create_user(&state.db, &body.email, &hash, &body.name).await?;
    Ok((StatusCode::CREATED, Json(profile)))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "auth"
)]
async fn login_handler(
    State(state): State<AppState>,
    jar: PrivateCookieJar<Key>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Find user — return InvalidCredentials for both unknown email and wrong password
    let user = service::find_user_by_email(&state.db, &body.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Verify password
    if !password::verify_password(&body.password, &user.password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    // Check if user is active
    if user.is_active == 0 {
        return Err(AppError::Forbidden);
    }

    let ip = extract_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    // Create session (enforces single-session)
    let (session, raw_token) =
        service::create_session(&state.db, &user.id, &ip, &user_agent).await?;

    // Audit log
    service::log_audit(&state.db, &user.id, "login", "session", &session.id, &ip).await?;

    // Update last_login_at
    service::update_last_login(&state.db, &user.id).await?;

    // Build cookie
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, raw_token.clone());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(!cfg!(debug_assertions));
    cookie.set_same_site(SameSite::Lax);
    cookie.set_max_age(time::Duration::hours(
        state.config.session_duration_hours as i64,
    ));

    let profile = UserProfile {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
    };

    Ok((
        jar.add(cookie),
        Json(AuthResponse {
            token: raw_token,
            user: profile,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 204, description = "Logged out"),
        (status = 401, description = "Not authenticated"),
    ),
    security(("bearer" = [])),
    tag = "auth"
)]
async fn logout_handler(
    State(state): State<AppState>,
    jar: PrivateCookieJar<Key>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    service::delete_session(&state.db, &auth_user.session_id).await?;
    service::log_audit(
        &state.db,
        &auth_user.id,
        "logout",
        "session",
        &auth_user.session_id,
        "",
    )
    .await?;

    let jar = jar.remove(Cookie::from(SESSION_COOKIE_NAME));
    Ok((jar, StatusCode::NO_CONTENT))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = UserProfile),
        (status = 401, description = "Not authenticated"),
    ),
    security(("bearer" = [])),
    tag = "auth"
)]
async fn me_handler(auth_user: AuthUser) -> Json<UserProfile> {
    Json(UserProfile {
        id: auth_user.id,
        email: auth_user.email,
        name: auth_user.name,
        role: auth_user.role,
    })
}
