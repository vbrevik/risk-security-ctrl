# Backend Core Auth — Research

## 1. Axum Auth Extractor Patterns

### Dual-Source Extractor (Cookie + Bearer Header)

The established pattern for Axum 0.7+ is implementing `FromRequestParts<S>` on a custom struct that checks both cookie and Authorization header.

**Key design:**
- Use `FromRef<S>` bound to access `AppState` (DB pool, cookie key) from within the extractor
- Two structs: `AuthUser` (required, rejects 401) and optionally `MaybeAuthUser` (returns `None` for anonymous)
- Check Authorization Bearer header first, fall back to cookie
- Rejection type must implement `IntoResponse` — return proper 401/403

**Middleware vs Extractor:**
- Use extractors for per-handler auth checks
- Use `middleware::from_fn_with_state` for route-group protection

**Sources:**
- https://mattrighetti.com/2025/05/03/authentication-with-axum
- https://github.com/tokio-rs/axum/discussions/2281

---

## 2. Argon2id Password Hashing (RustCrypto)

### Crate: `argon2 = "0.5"`

Uses Argon2id v19 by default (recommended variant).

**Hashing:**
```rust
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};

let salt = SaltString::generate(&mut OsRng);
let phc_hash = Argon2::default().hash_password(password, &salt)?.to_string();
// Stores: "$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>"
```

**Verification:**
```rust
let parsed_hash = PasswordHash::new(&stored_phc_string)?;
Argon2::default().verify_password(candidate, &parsed_hash)?;
```

**Default params (OWASP-aligned):**
| Param | Default | Meaning |
|-------|---------|---------|
| m_cost | 19,456 KiB (~19 MiB) | Memory |
| t_cost | 2 | Iterations |
| p_cost | 1 | Parallelism |

PHC string format is self-describing — verification extracts salt, params, and algorithm from stored hash automatically. No separate salt storage needed.

**Recommendations:**
- Start with `Argon2::default()` — already OWASP minimum
- Always `SaltString::generate(&mut OsRng)` — never hardcode
- Store full PHC string
- Benchmark on target hardware; increase m_cost if < 500ms

**Sources:**
- https://docs.rs/argon2
- https://rustcrypto.org/key-derivation/hashing-password.html
- https://github.com/RustCrypto/password-hashes/tree/master/argon2

---

## 3. Axum Private Cookie Sessions

### Crate: `axum-extra` with `cookie-private` feature

Uses AES-GCM encryption — cookies are encrypted and authenticated. Values opaque to client.

**Key management:**
- `Key::derive_from(&[u8])` — min 32 bytes, derives signing+encryption keys via HKDF
- Store master key (32+ bytes, hex-encoded) in env var `COOKIE_KEY`
- Must implement `FromRef<AppState> for Key` so `PrivateCookieJar` can find the key

**Setting cookie:**
```rust
let cookie = Cookie::build(("session_id", session_token))
    .path("/")
    .http_only(true)
    .secure(!cfg!(debug_assertions))  // true in prod
    .same_site(SameSite::Lax)
    .max_age(Duration::hours(8))
    .build();
(jar.add(cookie), response)
```

**Reading cookie:**
```rust
let session_id = jar.get("session_id").map(|c| c.value().to_string());
```

**Removing cookie (logout):**
```rust
(jar.remove(Cookie::from("session_id")), redirect)
```

**Critical:** `PrivateCookieJar` methods consume `self` and return new jar. Must return it from handler as part of response tuple, or changes are silently lost.

**Cookie flags:**
| Flag | Value | Purpose |
|------|-------|---------|
| http_only(true) | Always | Prevents JS access (XSS) |
| secure(true) | Production | HTTPS only |
| same_site(Lax) | Recommended | CSRF protection |
| path("/") | Typical | Site-wide |
| max_age(8h) | Per spec | Session duration |

**Sources:**
- https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/struct.PrivateCookieJar.html
- https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/index.html

---

## 4. Existing Codebase Patterns (from earlier audit)

**Already known from prior codebase research:**

- DB schema exists: `users`, `sessions`, `audit_log` tables with indexes
- Feature module pattern: `backend/src/features/{name}/mod.rs`, `routes.rs`, `models.rs`
- Error handling: `AppError` enum in `src/error.rs` with Unauthorized/Forbidden variants
- OpenAPI: utoipa annotations on all route handlers
- Existing route pattern: Axum `Router::new().route(...)` with state via `.with_state()`
- AppState: `SqlitePool` + `topics: HashMap` in shared state
- Testing: integration tests in `backend/tests/`, test utilities in `tests/common/mod.rs`

**Auth placeholder:**
- `backend/src/features/auth/routes.rs` — single GET /api/auth/me returning placeholder JSON
- Frontend `lib/api.ts` — 401 interceptor already redirects to /login

**Dependencies already present:**
- axum 0.7, tokio 1, tower 0.5, tower-http 0.6
- sqlx 0.8 (SQLite), serde/serde_json, uuid, chrono
- utoipa 5, utoipa-swagger-ui 8

**Dependencies to add:**
- argon2 (RustCrypto, 0.5.x)
- axum-extra with cookie-private feature
- validator with derive feature
- rand (for CSPRNG session tokens)
