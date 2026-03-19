# RBAC & Hardening — Interview Transcript

## Q1: Data ownership model (BOLA protection)?
**Answer:** All data visible, write restricted by role. Any authenticated user can read analyses/assessments. Create/update/delete follows the permission matrix. No per-user data ownership scoping.

## Q2: Swagger UI access in production?
**Answer:** Disable via feature flag. Compile Swagger UI out of production builds entirely.

## Q3: Rate limiter IP extraction?
**Answer:** Configurable via env var. Default to PeerIpKeyExtractor (direct). Switch to SmartIpKeyExtractor via `BEHIND_PROXY=true`.

## Q4: TLS / HTTPS?
**Answer:** Configurable. Default HTTP for dev, HTTPS for prod. Security headers (HSTS, Secure cookie flag) adapt based on config.

## Q5: CSRF custom header scope?
**Answer:** All state-changing requests (POST/PUT/DELETE). Frontend must send `X-Requested-With` header on all mutations. Global protection via middleware.
