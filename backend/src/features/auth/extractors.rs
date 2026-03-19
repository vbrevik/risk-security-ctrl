use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::error::AppError;
use crate::AppState;

use super::models::AuthUser;
use super::service;

pub const SESSION_COOKIE_NAME: &str = "session_id";

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Try Bearer header first
        let token = if let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        {
            bearer.token().to_string()
        } else {
            // Fall back to encrypted session cookie
            let jar: PrivateCookieJar<Key> =
                PrivateCookieJar::from_request_parts(parts, state)
                    .await
                    .map_err(|_| AppError::Unauthorized)?;
            match jar.get(SESSION_COOKIE_NAME) {
                Some(cookie) => cookie.value().to_string(),
                None => return Err(AppError::Unauthorized),
            }
        };

        // Validate session
        let session = service::validate_session(&state.db, &token)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Load user
        let user = service::find_user_by_id(&state.db, &session.user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Check if user is active
        if user.is_active == 0 {
            return Err(AppError::Unauthorized);
        }

        Ok(AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            session_id: session.id,
        })
    }
}
