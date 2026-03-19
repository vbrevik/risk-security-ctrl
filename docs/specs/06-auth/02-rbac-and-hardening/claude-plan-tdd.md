# TDD Plan: RBAC & Security Hardening

## Testing Infrastructure

- **Framework:** Rust built-in + `#[tokio::test]` for async
- **Integration tests:** `backend/tests/` with `create_test_app()` extended for auth
- **Rate limiting tests:** Use burst requests in quick succession; may need timing tolerance

---

## Section 1: Dependencies and Configuration — Tests

```rust
// Test: Config parses BEHIND_PROXY from env var (default false)
// Test: Config parses ENABLE_HTTPS from env var (default false)
// Test: tower-http set-header feature compiles
// Test: App compiles without swagger feature (--no-default-features)
// Test: App compiles with swagger feature (default)
// Test: Swagger UI route returns 200 when feature enabled
// Test: Swagger UI route returns 404 when feature disabled
```

## Section 2: Permission Matrix — Tests

```rust
// Test: admin has all permissions on all features
// Test: risk_manager can read/create/update/delete on compliance
// Test: risk_manager cannot manage users
// Test: specialist can create compliance assessments
// Test: specialist cannot delete compliance assessments
// Test: specialist cannot write ontology
// Test: viewer can read all features
// Test: viewer cannot create/update/delete anything
// Test: viewer cannot export reports
// Test: unknown role string defaults to viewer permissions
// Test: HasPermission trait works on AuthUser struct
```

## Section 3: RBAC Middleware — Tests

```rust
// Test: GET request to protected route passes without RBAC (read is universal)
// Test: POST to ontology returns 403 for viewer
// Test: POST to ontology returns 200 for admin
// Test: POST to ontology returns 200 for risk_manager
// Test: DELETE on compliance returns 403 for specialist
// Test: DELETE on compliance returns 200 for risk_manager
// Test: User management endpoint returns 403 for risk_manager
// Test: User management endpoint returns 200 for admin
// Test: Unauthenticated request returns 401 (not 403)
```

## Section 4: Rate Limiting — Tests

```rust
// Test: Login endpoint allows burst_size requests
// Test: Login endpoint returns 429 after exceeding burst
// Test: 429 response includes x-ratelimit-limit header
// Test: 429 response includes retry-after header
// Test: API endpoints have higher burst tolerance than auth
// Test: Rate limiter uses PeerIpKeyExtractor by default
// Test: Rate limiter uses SmartIpKeyExtractor when BEHIND_PROXY=true
// Test: into_make_service_with_connect_info is used (IP extraction works)
```

## Section 5: Security Headers — Tests

```rust
// Test: Response includes X-Frame-Options: DENY
// Test: Response includes X-Content-Type-Options: nosniff
// Test: Response includes Referrer-Policy header
// Test: Response includes Content-Security-Policy header
// Test: Response includes Permissions-Policy header
// Test: HSTS header present when ENABLE_HTTPS=true
// Test: HSTS header absent when ENABLE_HTTPS=false
// Test: Authorization and Cookie headers marked as sensitive (not in logs)
```

## Section 6: CSRF Protection — Tests

```rust
// Test: GET request passes without X-Requested-With header
// Test: OPTIONS request passes without X-Requested-With header
// Test: POST without X-Requested-With returns 403
// Test: PUT without X-Requested-With returns 403
// Test: DELETE without X-Requested-With returns 403
// Test: POST with X-Requested-With: XMLHttpRequest passes
// Test: POST with wrong X-Requested-With value returns 403
```

## Section 7: Session Cleanup — Tests

```rust
// Test: Expired session is deleted when encountered during validation
// Test: cleanup_expired_sessions deletes all expired sessions
// Test: cleanup_expired_sessions does not delete valid sessions
// Test: cleanup_expired_sessions returns count of deleted sessions
```

## Section 8: Wiring — Tests

```rust
// Test: Middleware executes in correct order (headers visible on rate-limited response)
// Test: Full flow: register -> login -> access protected route -> rate limit kicks in
// Test: CORS + credentials + CSRF all work together on a POST request
// Test: OpenAPI spec reflects security requirements on protected endpoints
```
