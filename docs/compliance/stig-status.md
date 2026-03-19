# STIG Compliance Status

**Last updated**: 2026-03-19
**Controls tracked**: 53 (8 passed, 30 failed, 11 N/A, 4 manual)

## auth
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222425 | Enforce approved authorizations | I | FAIL | 2026-03-19 | Auth is placeholder; no RBAC middleware |
| V-222426 | Enforce DAC policies | I | FAIL | 2026-03-19 | No object-level access control |
| V-222432 | 3 invalid logon attempts in 15 min | I | FAIL | 2026-03-19 | No rate limiting or lockout |
| V-222520 | Reauthentication for sensitive ops | II | FAIL | 2026-03-19 | No reauthentication flow |
| V-222524 | Accept PIV credentials | II | N/A | 2026-03-19 | PIV out of scope; future phase |
| V-222530 | Replay-resistant auth (privileged) | I | FAIL | 2026-03-19 | Static session tokens |
| V-222531 | Replay-resistant auth (non-privileged) | I | FAIL | 2026-03-19 | Static session tokens |
| V-222536 | 15-char minimum password | I | FAIL | 2026-03-19 | Spec sets 8-char (NIST 800-63B) |
| V-222538 | Password complexity | I | FAIL | 2026-03-19 | No complexity rules per NIST 800-63B |
| V-222542 | Cryptographic password storage | I | FAIL | 2026-03-19 | Schema ready; Argon2id not implemented |
| V-222543 | Cryptographic password transmission | I | FAIL | 2026-03-19 | HTTP only; no TLS |
| V-222544 | 24-hour minimum password lifetime | II | FAIL | 2026-03-19 | No password lifetime tracking |
| V-222545 | 60-day maximum password lifetime | II | FAIL | 2026-03-19 | No password expiry |
| V-222546 | Password reuse (5 generations) | II | FAIL | 2026-03-19 | No password history |
| V-222547 | Temp password with immediate change | II | FAIL | 2026-03-19 | No temp password flow |
| V-222552 | PKI identity mapping | II | N/A | 2026-03-19 | No PKI auth in scope |

## session-management
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222577 | Not expose session IDs | I | FAIL | 2026-03-19 | Token in response body per spec |
| V-222578 | Destroy session on logoff | I | FAIL | 2026-03-19 | Logout not implemented |
| V-222579 | Unique session ID per session | II | FAIL | 2026-03-19 | Designed but not implemented |
| V-222581 | No URL-embedded session IDs | I | PASS | 2026-03-19 | Cookie + header only by design |
| V-222582 | No session ID reuse/recycling | I | PASS | 2026-03-19 | Single-session enforcement by design |
| V-222583 | FIPS 140-2 crypto modules | II | MANUAL | 2026-03-19 | Rust crates not FIPS-certified; verify requirement |

## input-validation
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222606 | Validate all input | I | FAIL | 2026-03-19 | String-formatted SQL in get_findings() |
| V-222609 | Input handling vulnerabilities | I | PASS | 2026-03-19 | Upload validates extension, magic bytes, size |
| V-222612 | Overflow attacks | I | PASS | 2026-03-19 | Rust memory safety; bounded params |
| V-222605 | Canonical representation | II | PASS | 2026-03-19 | UUID-based IDs; no path traversal |

## injection
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222602 | XSS protection | I | PASS | 2026-03-19 | React auto-escaping; no unsafe innerHTML |
| V-222603 | CSRF protection | I | MANUAL | 2026-03-19 | Stateless API (N/A now); revisit with cookie auth |
| V-222604 | Command injection | I | PASS | 2026-03-19 | No shell execution in codebase |
| V-222607 | SQL Injection | I | FAIL | 2026-03-19 | CRITICAL: string-formatted SQL in get_findings() |
| V-222608 | XML attacks | I | PASS | 2026-03-19 | quick_xml no DTD processing |

## error-handling
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222585 | Fail to secure state | I | PASS | 2026-03-19 | Generic error responses; fail-closed |
| V-222610 | Safe error messages | II | PASS | 2026-03-19 | Generic messages to users; tracing server-side |
| V-222611 | Error messages to admins only | II | MANUAL | 2026-03-19 | Pending auth implementation |
| V-222586 | Preserve failure information | II | PASS | 2026-03-19 | Structured tracing logging |

## information-disclosure
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222601 | No sensitive data in hidden fields | I | MANUAL | 2026-03-19 | Verify prompt config exposure |
| V-222596 | Protect transmitted data | II | FAIL | 2026-03-19 | HTTP only; no HTTPS |
| V-222598 | Confidentiality pre-transmission | II | FAIL | 2026-03-19 | Plaintext SQLite storage |
| V-222599 | Confidentiality during reception | II | FAIL | 2026-03-19 | Upload errors may leak paths |
| V-222588 | Approved crypto for data at rest | I | FAIL | 2026-03-19 | No encryption at rest |
| V-222597 | Crypto for data in transit | I | FAIL | 2026-03-19 | No TLS; CORS allows Any |

## cryptography
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222570 | FIPS modules for signing | I | N/A | 2026-03-19 | No signing implemented |
| V-222571 | FIPS modules for hashing | I | N/A | 2026-03-19 | No hashing beyond planned Argon2id |
| V-222555 | Federal crypto module auth | I | N/A | 2026-03-19 | No FIPS scope |
| V-222573 | SAML FIPS random numbers | II | N/A | 2026-03-19 | No SAML |
| V-222553 | PKI revocation cache | II | N/A | 2026-03-19 | No PKI |

## audit-logging
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222474 | Sufficient audit record info | II | PASS | 2026-03-19 | Schema includes all required fields |
| V-222475 | Outcome in audit records | II | FAIL | 2026-03-19 | No success/failure field |
| V-222483 | Warn at 75% audit storage | II | FAIL | 2026-03-19 | No storage monitoring |
| V-222485 | Alert on audit failure | I | FAIL | 2026-03-19 | Best-effort logging; no alerting |
| V-222487 | Central audit review | II | FAIL | 2026-03-19 | No audit review endpoints |
| V-222489 | Audit reduction/reporting | II | FAIL | 2026-03-19 | No report generation |

## configuration
| V-ID | Title | CAT | Status | Last Checked | Evidence/Notes |
|------|-------|-----|--------|--------------|----------------|
| V-222642 | No embedded auth data | I | FAIL | 2026-03-19 | .env committed to repository |
| V-222643 | Mark sensitive output | II | FAIL | 2026-03-19 | No classification markings on exports |
| V-222645 | Hash files for deployment | II | N/A | 2026-03-19 | No deployment pipeline |
| V-222646 | Designate security tester | II | N/A | 2026-03-19 | Organizational/procedural |
| V-222647 | Test startup/shutdown security | II | N/A | 2026-03-19 | No ops automation |
| V-222653 | Follow coding standards | II | FAIL | 2026-03-19 | Debug formatting in error logs |
| V-222615 | Verify security function operation | II | FAIL | 2026-03-19 | No security integration tests |
