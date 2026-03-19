# TDD Plan: Document Parsing Pipeline

## Testing Infrastructure

Existing: `backend/tests/common/mod.rs` with `create_test_app()`. Unit tests use `#[cfg(test)] mod tests` within each module. Run with `cargo test`.

## Section 1: Upload Handler Tests

```rust
#[cfg(test)]
mod tests {
    #[test] fn reject_file_too_large() {}
    #[test] fn reject_unknown_extension() {}
    #[test] fn accept_pdf_extension() {}
    #[test] fn accept_docx_case_insensitive() {}
    #[test] fn reject_path_traversal_analysis_id() {}
    #[test] fn validate_pdf_magic_bytes() {}
    #[test] fn validate_zip_magic_bytes() {}
    #[test] fn reject_mismatched_magic_bytes() {}
    #[test] fn save_upload_creates_directory_and_file() {}
}
```

## Section 2: Parser Refinement Tests

```rust
#[cfg(test)]
mod tests {
    #[test] fn empty_document_error_has_message() {}
    #[test] fn scanned_pdf_detected_as_empty() {}
    #[test] fn parsing_error_converts_to_app_error_bad_request() {}
    #[test] fn parsing_error_converts_to_app_error_internal() {}
    #[tokio::test] async fn parse_async_returns_same_result() {}
}
```

## Section 3: Route Handler Tests

```rust
#[tokio::test] async fn upload_pdf_creates_analysis() {}
#[tokio::test] async fn upload_oversized_returns_400() {}
#[tokio::test] async fn upload_unsupported_format_returns_400() {}
```

## Section 4: Integration Tests

```rust
#[tokio::test] async fn full_pipeline_pdf_upload_to_findings() {}
```
