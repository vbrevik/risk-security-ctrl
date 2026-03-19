<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-upload-handler
section-02-parser-refinements
section-03-route-completion
section-04-integration-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-upload-handler | - | 03 | Yes |
| section-02-parser-refinements | - | 03 | Yes |
| section-03-route-completion | 01, 02 | 04 | No |
| section-04-integration-tests | 03 | - | No |

## Execution Order

1. section-01-upload-handler, section-02-parser-refinements (parallel — no dependencies)
2. section-03-route-completion (after 01 and 02)
3. section-04-integration-tests (after 03)

## Section Summaries

### section-01-upload-handler
New `upload.rs` module with file validation (size, extension, magic bytes), UUID-sanitized directory creation, streaming upload to disk, and unit tests. Constants for MAX_FILE_SIZE, ALLOWED_EXTENSIONS, UPLOAD_DIR.

### section-02-parser-refinements
Targeted changes to existing `parser.rs`: add message to EmptyDocument variant, implement `From<ParsingError> for AppError`, add `parse_async` wrapper with `spawn_blocking`, add scanned PDF detection. Tests for each change.

### section-03-route-completion
Complete `routes.rs` with POST upload endpoint using Axum Multipart. Add RequestBodyLimitLayer. Wire into main router in `lib.rs`. Add OpenAPI annotations.

### section-04-integration-tests
Full pipeline integration test: multipart upload → parse → match → verify analysis record in DB. Test fixtures (small PDF). Error case tests (oversized, wrong format).
