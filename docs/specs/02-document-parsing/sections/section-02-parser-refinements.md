Now I have all the context needed. Let me generate the section content.

# Section 2: Parser Refinements

## Overview

This section covers targeted improvements to the existing `backend/src/features/analysis/parser.rs` module and the addition of a `From<ParsingError> for AppError` conversion in `backend/src/error.rs`. There are four changes:

1. Change the `EmptyDocument` variant from a unit variant to `EmptyDocument(String)` so error messages can describe why the document is empty (e.g., scanned PDF detection).
2. Implement `From<ParsingError> for AppError` to allow `?` propagation from parsing code into Axum route handlers.
3. Add a public `parse_async` function that wraps the blocking `DocumentParser::parse` call in `tokio::task::spawn_blocking`.
4. Add scanned PDF detection heuristic inside `parse_pdf`.

**Dependencies**: None. This section can be implemented in parallel with Section 01 (Upload Handler). Section 03 (Route Completion) depends on this section being complete.

**Files to modify**:
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/parser.rs`
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/error.rs`

---

## Tests First

Add these tests to the existing `#[cfg(test)] mod tests` block in `parser.rs`. The first three test the new `EmptyDocument(String)` variant, the `From` impl, and `parse_async`. The fourth tests scanned PDF detection.

```rust
// Add to the existing #[cfg(test)] mod tests in parser.rs

#[test]
fn empty_document_error_has_message() {
    // EmptyDocument now carries a descriptive String
    let err = ParsingError::EmptyDocument("scanned PDF detected".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("scanned PDF detected"));
}

#[test]
fn scanned_pdf_detected_as_empty() {
    // Create a file that produces near-empty text but is large (>10KB),
    // simulating a scanned/image-based PDF.
    // This test verifies the heuristic: text.trim().len() < 50 && file_size > 10_000
    // triggers EmptyDocument with a scanned-PDF message.
    //
    // Implementation note: Constructing a real PDF that pdf_extract returns
    // near-empty text for is fragile. Instead, test the detection logic
    // directly by extracting it into a helper function (see implementation
    // details below) or by creating a minimal PDF with embedded image only.
    //
    // Stub: assert the EmptyDocument variant contains "scanned" or "image-based"
}

#[test]
fn parsing_error_converts_to_app_error_bad_request() {
    // UnsupportedFormat, EmptyDocument, FileTooLarge -> AppError::BadRequest
    use crate::error::AppError;

    let err: AppError = ParsingError::UnsupportedFormat("xlsx".into()).into();
    assert!(matches!(err, AppError::BadRequest(_)));

    let err: AppError = ParsingError::EmptyDocument("empty".into()).into();
    assert!(matches!(err, AppError::BadRequest(_)));

    let err: AppError = ParsingError::FileTooLarge { size: 25_000_000, max: 20_000_000 }.into();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[test]
fn parsing_error_converts_to_app_error_internal() {
    // CorruptFile, IoError -> AppError::Internal
    use crate::error::AppError;

    let err: AppError = ParsingError::CorruptFile("bad data".into()).into();
    assert!(matches!(err, AppError::Internal(_)));

    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
    let err: AppError = ParsingError::IoError(io_err).into();
    assert!(matches!(err, AppError::Internal(_)));
}

#[tokio::test]
async fn parse_async_returns_same_result() {
    // parse_async should produce the same result as synchronous parse.
    // Use a temp .docx (via the existing create_test_docx helper)
    // and verify the async wrapper returns an equivalent ParsedDocument.
    let dir = tempfile::tempdir().unwrap();
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:body>
            <w:p><w:r><w:t>Async test content</w:t></w:r></w:p>
        </w:body></w:document>"#;
    let path = create_test_docx(&dir, xml);

    let sync_result = DocumentParser::parse(&path).unwrap();
    let async_result = parse_async(path).await.unwrap();

    assert_eq!(sync_result.full_text, async_result.full_text);
    assert_eq!(sync_result.word_count, async_result.word_count);
}
```

---

## Implementation Details

### Change 1: `EmptyDocument` Variant with Message

In `parser.rs`, change the `ParsingError` enum:

**Before:**
```rust
#[error("No text content found in document")]
EmptyDocument,
```

**After:**
```rust
#[error("No text content found in document: {0}")]
EmptyDocument(String),
```

Then update every existing occurrence of `ParsingError::EmptyDocument` (there are currently three: in `parse_text` twice and in `parse_pdf` and `parse_docx` once each) to pass a default message:

- In `parse_text`: `ParsingError::EmptyDocument("document contains no text".into())`
- In `parse_pdf` (the existing empty-sections check): `ParsingError::EmptyDocument("PDF contains no extractable text".into())`
- In `parse_docx`: `ParsingError::EmptyDocument("DOCX contains no text content".into())`

Also update the existing test `parsing_error_display_messages` which currently matches on the old unit variant. Change:
```rust
format!("{}", ParsingError::EmptyDocument)
```
to:
```rust
format!("{}", ParsingError::EmptyDocument("test message".into()))
```
and update the expected string accordingly to `"No text content found in document: test message"`.

Similarly, update the two tests `parse_text_empty_string` and `parse_text_whitespace_only` that pattern-match on `Err(ParsingError::EmptyDocument)` to use `Err(ParsingError::EmptyDocument(_))` instead.

Update `parse_docx_empty_document` test the same way.

### Change 2: `From<ParsingError> for AppError`

In `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/error.rs`, add the following `impl` block. This requires importing `ParsingError` from the analysis feature module.

```rust
impl From<crate::features::analysis::parser::ParsingError> for AppError {
    fn from(err: crate::features::analysis::parser::ParsingError) -> Self {
        use crate::features::analysis::parser::ParsingError;
        match err {
            ParsingError::UnsupportedFormat(msg) => AppError::BadRequest(msg),
            ParsingError::EmptyDocument(msg) => AppError::BadRequest(msg),
            ParsingError::FileTooLarge { size, max } => {
                AppError::BadRequest(format!("File too large: {} bytes (max: {})", size, max))
            }
            ParsingError::CorruptFile(msg) => AppError::Internal(msg),
            ParsingError::IoError(e) => AppError::Internal(e.to_string()),
        }
    }
}
```

The mapping rationale:
- `UnsupportedFormat`, `EmptyDocument`, `FileTooLarge` are client errors (the user submitted a bad file) and map to `AppError::BadRequest`.
- `CorruptFile` and `IoError` are server-side failures (file corruption, disk I/O) and map to `AppError::Internal`.

### Change 3: `parse_async` Wrapper

Add a public async function at module level in `parser.rs`:

```rust
/// Async wrapper around DocumentParser::parse that runs the blocking
/// file I/O and parsing on a dedicated thread via spawn_blocking.
pub async fn parse_async(file_path: std::path::PathBuf) -> Result<ParsedDocument, ParsingError> {
    tokio::task::spawn_blocking(move || DocumentParser::parse(&file_path))
        .await
        .map_err(|e| ParsingError::CorruptFile(format!("task join error: {}", e)))?
}
```

The `JoinError` from `spawn_blocking` is mapped to `CorruptFile` since it represents an unexpected runtime failure (task panic or cancellation). This function is what route handlers in Section 03 will call instead of `DocumentParser::parse` directly.

### Change 4: Scanned PDF Detection

In the `parse_pdf` method, after extracting text from the PDF but before the existing empty-sections check, add a heuristic to detect scanned/image-based PDFs. The logic is:

- Compute total extracted text length: `full_extracted_text.trim().len()`
- Get file size from the path metadata (already available since `parse` checks it before dispatching)
- If `text_len < 50 && file_size > 10_000`, this is almost certainly a scanned PDF with no embedded text layer

Modify `parse_pdf` to accept file size as a parameter or read metadata internally. Since `parse_pdf` already receives a `&Path`, reading metadata is straightforward:

```rust
// Inside parse_pdf, after extracting pages and building sections but before
// the existing empty-sections check:

let total_text_len: usize = pages.iter().map(|p| p.trim().len()).sum();
let file_size = std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);

if total_text_len < 50 && file_size > 10_000 {
    return Err(ParsingError::EmptyDocument(
        "This appears to be a scanned/image-based PDF. Text-based PDFs are required.".into(),
    ));
}
```

This check should come **before** the existing `if sections.is_empty()` check, so the user gets the more specific scanned-PDF message rather than a generic "no text" message. The existing empty-sections check remains as a fallback for genuinely empty (non-scanned) PDFs.

---

## Summary of All File Changes

| File | Change |
|------|--------|
| `backend/src/features/analysis/parser.rs` | `EmptyDocument` variant gets `String` payload; add `parse_async` function; add scanned PDF heuristic in `parse_pdf`; update 4 existing tests to match new variant shape |
| `backend/src/error.rs` | Add `impl From<ParsingError> for AppError` with appropriate mapping |