# Prompt Contract: Section 02 — Auth Models and Types

## GOAL
Create auth request/response types with validation, AuthUser extractor struct, and extend AppError with auth-specific variants.

## CONSTRAINTS
- RegisterRequest/LoginRequest must use validator derives for email and password
- AuthUser must NOT derive Serialize (prevents accidental session_id leakage)
- InvalidCredentials must use deliberately vague message (no email enumeration)
- FieldError must live in error.rs, not models.rs
- ErrorResponse must add optional `fields` with skip_serializing_if

## FAILURE CONDITIONS
- SHALL NOT expose session_id in any serializable type
- SHALL NOT distinguish "email not found" from "wrong password" in error messages
- SHALL NOT break existing error handling (backward compatible ErrorResponse)
