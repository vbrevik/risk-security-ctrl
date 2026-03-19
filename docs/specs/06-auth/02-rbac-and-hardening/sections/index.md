<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-deps-and-config
section-02-permission-matrix
section-03-rbac-middleware
section-04-rate-limiting
section-05-security-headers
section-06-csrf-protection
section-07-session-cleanup
section-08-wiring
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-deps-and-config | - | all | Yes |
| section-02-permission-matrix | 01 | 03 | Yes |
| section-03-rbac-middleware | 01, 02 | 08 | No |
| section-04-rate-limiting | 01 | 08 | Yes (parallel with 02-06) |
| section-05-security-headers | 01 | 08 | Yes (parallel with 02-06) |
| section-06-csrf-protection | 01 | 08 | Yes (parallel with 02-05) |
| section-07-session-cleanup | 01 | 08 | Yes (parallel with 02-06) |
| section-08-wiring | 03, 04, 05, 06, 07 | - | No |

## Execution Order

1. section-01-deps-and-config (foundation)
2. section-02-permission-matrix, section-04-rate-limiting, section-05-security-headers, section-06-csrf-protection, section-07-session-cleanup (parallel after 01)
3. section-03-rbac-middleware (after 02)
4. section-08-wiring (final, after all)

## Section Summaries

### section-01-deps-and-config
Add governor/tower_governor deps, update tower-http features, add BEHIND_PROXY and ENABLE_HTTPS config, gate swagger behind feature flag.

### section-02-permission-matrix
Define Feature, Action, Role enums. Implement has_permission() static lookup. HasPermission trait on AuthUser.

### section-03-rbac-middleware
require_permission() middleware using from_fn pattern. Apply route_layer() to existing ontology, compliance, analysis, report routes.

### section-04-rate-limiting
GovernorConfig for auth (strict) and API (moderate). IP key extractor configurable. Response headers. Update serve() call.

### section-05-security-headers
tower-http SetResponseHeaderLayer for X-Frame-Options, CSP, etc. Conditional HSTS. Sensitive headers masking.

### section-06-csrf-protection
Custom middleware checking X-Requested-With on POST/PUT/DELETE. Pass through GET/HEAD/OPTIONS.

### section-07-session-cleanup
Bulk cleanup on startup. Lazy cleanup already in split 01 extractor.

### section-08-wiring
Assemble middleware stack in correct order. Integration tests for full flow.
