# Prompt Contract: Section 05 — Auth Extractor

## GOAL
Implement FromRequestParts<AppState> for AuthUser that extracts credentials from Bearer header or encrypted cookie.

## CONSTRAINTS
- Bearer header takes precedence over cookie
- Missing credentials → 401 (not 500)
- Inactive user (is_active=0) → 401
- Cookie name must be "session_id" (matches Section 06 login handler)

## FAILURE CONDITIONS
- SHALL NOT return 500 when no auth credentials provided (must be 401)
- SHALL NOT allow inactive users to authenticate
- SHALL NOT skip session validation against the database
