# RBAC & Hardening — Research

## 1. Axum RBAC Middleware Patterns

### Recommended: `from_fn_with_state` per route group

Group routes by permission level, apply middleware per group via `.route_layer()`:

```rust
let admin_routes = Router::new()
    .route("/users", get(list_users))
    .route_layer(middleware::from_fn_with_state(state.clone(), require_admin));
```

Key: use `.route_layer()` not `.layer()` — applies only to defined routes, not 404 fallbacks.

### Permission string convention

Use `"resource:action"` strings (e.g., `"analysis:create"`, `"compliance:delete"`) for a clean permission matrix mapping.

### Middleware order

CORS → Rate Limit → Auth (session) → RBAC (permission) → Handler

**Sources:** [Axum middleware docs](https://docs.rs/axum/latest/axum/middleware/index.html), [rustzen-admin](https://dev.to/idiabin/rustzen-admin-part-2-complete-declarative-permission-system-architecture-for-axum-backends-3kh1)

---

## 2. tower_governor Rate Limiting

### Per-route application

Apply different GovernorLayer instances to different router groups:
- Strict for auth: `per_second(4)`, `burst_size(5)` — ~5 attempts then blocked
- Moderate for API: `per_second(1)`, `burst_size(30)`

**Critical:** Must use `.into_make_service_with_connect_info::<SocketAddr>()` for IP extraction.

### IP extractors

- `PeerIpKeyExtractor` — direct connections
- `SmartIpKeyExtractor` — behind reverse proxy (reads X-Forwarded-For)

### Response headers (`.use_headers()`)

- `x-ratelimit-limit`, `x-ratelimit-remaining`, `x-ratelimit-after`, `retry-after`

**Sources:** [tower-governor](https://github.com/benwis/tower-governor), [Shuttle blog](https://www.shuttle.dev/blog/2024/02/22/api-rate-limiting-rust)

---

## 3. OWASP API Security Top 10 — Relevance

| # | Risk | Priority | Action |
|---|------|----------|--------|
| API1 | BOLA | HIGH | Service-layer ownership checks on data queries |
| API2 | Broken Auth | HIGH | Already addressed in split 01 |
| API3 | Property-Level Authz | MEDIUM | Separate response DTOs, no raw DB models |
| API4 | Resource Consumption | HIGH | Rate limiting + upload limits + pagination caps |
| API5 | Function-Level Authz | HIGH | RBAC middleware — this split's core purpose |
| API8 | Security Misconfiguration | MEDIUM | Security headers, disable Swagger in prod, generic errors |

### Concrete gaps to address in this split

1. **BOLA protection** — data queries need ownership guards (`WHERE owner_id = ? OR role = 'admin'`)
2. **Security headers** — HSTS, X-Frame-Options, X-Content-Type-Options, CSP, Permissions-Policy
3. **Error sanitization** — return generic errors to clients, log details server-side
4. **Pagination caps** — enforce max `limit` parameter (e.g., 100)
5. **Swagger access** — gate behind auth or feature flag in production

---

## 4. Existing Codebase Patterns (from prior audit)

- **tower-http** already in Cargo.toml with `cors`, `compression-gzip`, `trace` features
- **Existing routes:** ontology, compliance, analysis, reports, auth — all need RBAC
- **Error handling:** `AppError` in `src/error.rs` with consistent JSON format
- **AuthUser extractor** from split 01 provides `id`, `email`, `name`, `role`, `session_id`
- **Test infrastructure:** `tests/common/mod.rs` with `create_test_app()`

## 5. Testing

- **Framework:** Rust built-in `#[cfg(test)]` + integration tests in `backend/tests/`
- **Existing pattern:** Use `tower::ServiceExt` to test routes
- **Rate limiting tests:** Need careful timing or mock clock to avoid flaky tests
