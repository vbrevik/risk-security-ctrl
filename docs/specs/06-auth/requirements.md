# Authentication & Authorization System

## Overview

Implement secure user authentication and authorization for the risk-security-ctrl application. The system manages sensitive governmental IT security compliance data, so security is paramount.

## Goals

- Users can register, log in, and log out
- Sessions are secure and properly managed
- Role-based access control (RBAC) enforces permissions per the existing role model (admin, risk_manager, specialist, viewer)
- All auth actions are audited via the existing audit_log table

## Existing Infrastructure

The database schema already exists with:
- `users` table (id, email, password_hash, name, role, is_active, last_login_at)
- `sessions` table (id, user_id, token, expires_at, ip_address, user_agent)
- `audit_log` table (user_id, action including 'login'/'logout', entity tracking)
- Relevant indexes on email, role, session token, session expiry

The backend has placeholder auth routes (GET /api/auth/me only) and error variants for Unauthorized/Forbidden.

The frontend API client already intercepts 401 responses and redirects to /login.

## Requirements

### Backend Authentication

1. **User Registration** - POST /api/auth/register
   - Accept email, password, name
   - Validate email format and uniqueness
   - Enforce password strength (min length, complexity)
   - Hash passwords securely (argon2 preferred for new systems)
   - Default role: viewer
   - Admin-only: ability to set role during creation

2. **User Login** - POST /api/auth/login
   - Accept email + password
   - Verify credentials against stored hash
   - Create session with secure random token
   - Store session in database with expiry, IP, user agent
   - Return session token
   - Update last_login_at
   - Log to audit_log

3. **User Logout** - POST /api/auth/logout
   - Invalidate current session
   - Log to audit_log

4. **Current User** - GET /api/auth/me
   - Return authenticated user profile
   - Requires valid session

5. **Session Management**
   - Sessions expire after configurable duration (default 24h)
   - Expired sessions cleaned up periodically or on access
   - One user can have multiple active sessions

### Backend Authorization

6. **Auth Middleware/Extractor**
   - Axum extractor that validates session token from Authorization header or cookie
   - Populates request with authenticated user context
   - Returns 401 for missing/invalid/expired tokens

7. **Role-Based Access Control**
   - Middleware or guard for checking user roles
   - Protect write endpoints (admin, risk_manager)
   - Read-only access for viewer role
   - Endpoint-level granularity

### Frontend Authentication

8. **Login Page** - /login route
   - Email + password form
   - Error display for invalid credentials
   - Redirect to dashboard on success

9. **Registration Page** - /register route
   - Email, name, password, confirm password form
   - Client-side validation
   - Redirect to login on success

10. **Auth State Management**
    - React context providing current user and auth status
    - useAuth hook for components
    - Token storage (httpOnly cookie preferred, or secure localStorage)
    - Auto-redirect to login when session expires

11. **Protected Routes**
    - Route guard redirecting unauthenticated users to /login
    - Role-based UI elements (show/hide based on permissions)

### Security Requirements

12. **Password Security**
    - Argon2id hashing with appropriate parameters
    - No password stored in plain text anywhere (logs, responses, etc.)
    - Password min 8 characters

13. **Session Security**
    - Cryptographically random session tokens (min 256 bits)
    - HttpOnly, Secure, SameSite cookie flags (if using cookies)
    - Session fixation prevention

14. **Rate Limiting**
    - Rate limit login attempts per IP and per account
    - Prevent brute force and credential stuffing

15. **Input Validation**
    - Server-side validation of all inputs
    - SQL injection prevention (already handled by SQLx parameterized queries)
    - XSS prevention in responses

## Constraints

- Stack: Rust/Axum backend, React/TanStack frontend (no changes to core stack)
- Database: SQLite3 via SQLx (schema already exists)
- Must integrate with existing feature-based module structure
- Must work with existing OpenAPI/utoipa documentation setup
- Internationalization: use existing i18next setup for auth UI strings

## Non-Goals (for this phase)

- OAuth/SSO integration (future phase)
- Two-factor authentication (future phase)
- Password reset via email (no email service configured)
- Account lockout after failed attempts (consider for future)
