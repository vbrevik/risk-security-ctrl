# Prompt Contract: Section 04 — Auth Service

## GOAL
Implement auth service layer with user CRUD, session management (with SHA-256 token hashing), and audit logging.

## CONSTRAINTS
- All functions take &SqlitePool, return Result<T, AppError>
- Session tokens hashed with SHA-256 before DB storage
- Single-session enforcement (DELETE old sessions before INSERT)
- create_session wrapped in transaction
- create_user catches UNIQUE violation → generic validation error
- Login validation errors from Section 06 must map to InvalidCredentials (noted from Section 02 review)

## FAILURE CONDITIONS
- SHALL NOT store raw session tokens in the database
- SHALL NOT reveal whether an email exists on duplicate registration
- SHALL NOT skip single-session enforcement in create_session
