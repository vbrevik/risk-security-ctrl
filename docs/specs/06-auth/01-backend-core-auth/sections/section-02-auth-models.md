Now I have all the context needed. Here is the section content:

# Section 2: Auth Models and Types

## Overview

This section creates the request/response types, the `AuthUser` struct, and the `AppError` extensions needed by the entire auth system. All types live in a new file `backend/src/features/auth/models.rs`. The existing `backend/src/error.rs` is extended with three new error variants.

## Dependencies

- **Section 01 (deps-and-appstate)** must be completed first so that `validator`, `utoipa`, and other crate dependencies are available in `Cargo.toml`.

## Files to Create/Modify

| Action | File Path |
|--------|-----------|
| Create | `backend/src/features/auth/models.rs` |
| Modify | `backend/src/features/auth/mod.rs` |
| Modify | `backend/src/error.rs` |

## Tests First

All tests below are unit tests that live inline in `backend/src/features/auth/models.rs` within a `#[cfg(test)] mod tests` block. They require `validator::Validate` and `serde_json`.

### Test: RegisterRequest validation rejects invalid email

```rust
/// Construct RegisterRequest with email "notanemail", valid name and password.
/// Call .validate(). Assert error contains the `email` field.
#[test]
fn register_request_rejects_invalid_email() { todo!() }
```

### Test: RegisterRequest validation rejects short password

```rust
/// Construct RegisterRequest with a valid email, valid name, password "short" (< 8 chars).
/// Call .validate(). Assert error contains the `password` field.
#[test]
fn register_request_rejects_short_password() { todo!() }
```

### Test: LoginRequest validation rejects empty password

```rust
/// Construct LoginRequest with a valid email and empty password "".
/// Call .validate(). Assert error contains the `password` field.
#[test]
fn login_request_rejects_empty_password() { todo!() }
```

### Test: UserProfile serializes correctly

```rust
/// Construct a UserProfile with known values. Serialize to JSON via serde_json::to_value.
/// Assert all four fields (id, email, name, role) are present with correct values.
/// Assert no `password_hash` or other sensitive fields leak into the output.
#[test]
fn user_profile_serializes_correctly() { todo!() }
```

## Implementation Details

### 1. Create `backend/src/features/auth/models.rs`

This file defines all auth-related request, response, and extractor types.

#### Request Types

Define two structs that derive `Deserialize`, `Validate`, and `ToSchema` (from utoipa):

**`RegisterRequest`**
- `email: String` -- annotated with `#[validate(email)]`
- `name: String` -- annotated with `#[validate(length(min = 1, max = 255))]`
- `password: String` -- annotated with `#[validate(length(min = 8))]`

**`LoginRequest`**
- `email: String` -- annotated with `#[validate(email)]`
- `password: String` -- annotated with `#[validate(length(min = 1))]`

Both types should also derive `Debug` for logging/test output.

#### Response Types

Define two structs that derive `Serialize`, `Debug`, and `ToSchema`:

**`AuthResponse`**
- `token: String` -- the raw session token returned to the client
- `user: UserProfile`

**`UserProfile`**
- `id: String`
- `email: String`
- `name: String`
- `role: String`

These intentionally exclude `password_hash` and any other sensitive database fields. `UserProfile` is the public-facing representation of a user.

#### AuthUser Struct (Extractor Output)

```rust
/// Populated by the FromRequestParts extractor after session validation.
/// Handlers that need an authenticated user declare `AuthUser` as a parameter.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub session_id: String,
}
```

`session_id` is carried through so the logout handler can delete the exact session and reference it in the audit log entry. This struct does NOT derive `Serialize` -- it is internal only and never sent over the wire. The `UserProfile` type is used for API responses instead.

### 2. Update `backend/src/features/auth/mod.rs`

Add the models module declaration:

```rust
pub mod models;
pub mod routes;
```

### 3. Extend `AppError` in `backend/src/error.rs`

Add three new variants to the `AppError` enum:

**`InvalidCredentials`** -- returned when login fails (wrong email or wrong password). Maps to HTTP 401 with a deliberately vague message `"Invalid credentials"`. This prevents attackers from distinguishing between "email not found" and "wrong password".

**`ValidationError(Vec<FieldError>)`** -- returned when request body fails validation. Maps to HTTP 422. `FieldError` is a small struct:

```rust
/// Represents a single field-level validation error for API responses.
#[derive(Debug, Serialize, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
```

The `FieldError` struct should be defined in `error.rs` alongside `ErrorResponse`, and derive `Serialize` and `ToSchema` so it can appear in OpenAPI docs.

**`SessionExpired`** -- returned when a session token is found but has expired. Maps to HTTP 401 with message `"Session expired"`.

#### IntoResponse Mappings

Extend the existing `match` block in `impl IntoResponse for AppError`:

- `InvalidCredentials` -> `(401, "invalid_credentials", "Invalid credentials")`
- `SessionExpired` -> `(401, "session_expired", "Session expired")`
- `ValidationError(fields)` -> HTTP 422. The response body should include the field errors. One approach is to serialize the `Vec<FieldError>` into the `message` field as JSON, or extend `ErrorResponse` with an optional `fields` array. The simpler approach: use the existing `ErrorResponse` shape with `error: "validation_error"` and `message: "Validation failed"`, and add an optional `fields: Option<Vec<FieldError>>` to `ErrorResponse`. This keeps backward compatibility since existing errors will have `fields: None` (and `#[serde(skip_serializing_if = "Option::is_none")]` keeps it clean).

#### Important: Registration Duplicate Email Handling

The register endpoint must NOT reveal whether an email is already registered. When a SQLite UNIQUE constraint violation occurs during user creation, it should be caught and converted to a `ValidationError` with a generic message like `"Registration failed"` rather than `"Email already exists"`. This prevents account enumeration attacks. The mapping from SQLx constraint error to `AppError::ValidationError` will happen in the service layer (Section 04), but the error variant must be defined here.

#### Conversion from validator Errors

Add a `From<validator::ValidationErrors>` implementation for `AppError` that converts the validator crate's error structure into `AppError::ValidationError(Vec<FieldError>)`. This allows handlers to use the `?` operator after calling `.validate()` on request bodies:

```rust
/// Convert validator::ValidationErrors into AppError::ValidationError
/// by iterating field errors and extracting field name + first message.
impl From<validator::ValidationErrors> for AppError { ... }
```

## Key Design Decisions

- **AuthUser vs UserProfile separation:** `AuthUser` is the internal extractor type carrying `session_id`; `UserProfile` is the API-facing type without session details. They have overlapping fields but serve different purposes.
- **No Serialize on AuthUser:** Prevents accidental serialization of `session_id` into an API response.
- **Vague error messages:** `InvalidCredentials` deliberately does not distinguish between unknown email and wrong password. This is a security requirement, not a simplification.
- **FieldError in error.rs, not models.rs:** Since `FieldError` is used by `AppError` which is application-wide, it belongs in the error module, not inside the auth feature.