# Section 01 Code Review

## Failure Condition Audit

| Failure Condition | Status |
|---|---|
| SHALL NOT leave CORS with `allow_origin(Any)` | PASS |
| SHALL NOT allow cookie keys shorter than 32 bytes without panicking | PASS |
| SHALL NOT break existing test compilation | PASS |
| SHALL NOT skip unit tests for Config parsing and key validation | PASS |

## Critical Issues

### 1. `allow_headers(Any)` + `allow_credentials(true)` — Credentialed Requests Will Fail
**Confidence: 95%**

CORS spec forbids wildcard `*` in `Access-Control-Allow-Headers` with credentialed requests. Browsers reject the preflight. Fix: use `AllowHeaders::mirror_request()`.

### 2. `allow_methods(Any)` + `allow_credentials(true)` — Same Wildcard Conflict
**Confidence: 85%**

Same restriction applies to methods. Fix: use explicit method list.

## Important Issues

### 3. File Permission Check Is Advisory, Not Enforced
**Confidence: 85%**

Existing `.cookie_key` file with bad permissions (e.g., 0644) logs warning but continues. Should be a hard error for a session encryption key.

## Summary

Structural work is correct. Two CORS issues will break cookie-based auth in browsers. One security hardening gap on file permissions.
