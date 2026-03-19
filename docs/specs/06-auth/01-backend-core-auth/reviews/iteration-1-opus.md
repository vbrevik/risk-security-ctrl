# Opus Review

**Model:** claude-opus-4-6
**Generated:** 2026-03-19T19:50:00Z

---

## Overall Assessment

The plan is well-structured, covers the spec requirements thoroughly, and makes sound technology choices for the Rust/Axum stack. However, there are several concrete issues ranging from a critical security problem to implementation gaps.

## CRITICAL: CORS Configuration

The existing CORS config allows `Any` origin. Plan must include a step to restrict CORS to configured `frontend_url` and enable `allow_credentials(true)` for cookie-based auth to work.

## Security Issues

1. **Session token stored in plaintext** — should store SHA-256 hash, lookup by hashing presented token
2. **No rate limiting on login/register** — brute-force from insider threat; at minimum note as known gap
3. **Audit log missing IP address** — `log_audit` signature doesn't accept `ip_address` despite table column existing
4. **Password hash in debug logs** — ensure internal `User` struct never derives `Debug` in a leaky way
5. **`time` vs `chrono` crate** — project uses chrono; use `time` only for cookie-specific API

## Architectural Issues

6. **Logout handler token gap** — `AuthUser` extractor doesn't carry session_id; logout can't delete session. Fix: add `session_id: String` to `AuthUser`
7. **`Key` clone compatibility** — verify `axum_extra::cookie::Key` is Clone-friendly with AppState
8. **Single-session enforcement UX** — acknowledge trade-off in plan

## Completeness Gaps

9. **No session garbage collection** — expired sessions accumulate; add cleanup task
10. **No `LoginRequest` validation** — add basic non-empty checks to avoid unnecessary Argon2 on empty input
11. **Missing `updated_at` handling** — no trigger; service layer must update manually
12. **No test section in plan** — spec lists 7 test cases but plan has no testing guidance per section
13. **Open registration flagged** — unusual for government app; flag as follow-up

## Minor Issues

14. Set `.cookie_key` file permissions to 0600 on creation
15. Validate COOKIE_KEY minimum length (32 bytes) at startup
16. Clarify `InvalidCredentials` vs existing `Unauthorized` error variant relationship
17. Verify `axum-extra` version compatibility with `axum 0.7`

## Summary

**Must fix:** CORS restriction, `session_id` in AuthUser, `ip_address` in audit log
**Should fix:** Hash session tokens, login validation, session GC, test plan, file permissions
**Document as gaps:** Rate limiting, open registration, `updated_at`, multi-session UX
