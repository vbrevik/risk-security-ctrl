# Section 4: Text Parser and Dispatch

## Overview

This section implements two functions on the `DocumentParser` unit struct in `backend/src/features/analysis/parser.rs`:

1. **`parse_text`** -- handles raw text input by splitting into sections on blank lines
2. **`parse`** -- the main dispatch entry point that checks file size, determines format from extension, and delegates to `parse_pdf`, `parse_docx`, or returns an error

These functions complete the parser module. After this section, all three input formats (PDF, DOCX, plain text) are handled.

## Dependencies

- **Section 01 (deps-and-types):** The `ParsingError` enum (including `FileTooLarge`, `UnsupportedFormat`, `EmptyDocument`), `ParsedDocument`, and `DocumentSection` structs must exist in `parser.rs`.
- **Section 02 (pdf-parser):** `DocumentParser::parse_pdf` must be implemented.
- **Section 03 (docx-parser):** `DocumentParser::parse_docx` must be implemented.

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/parser.rs`

This file already exists from sections 01-03. This section adds two more associated functions to the existing `DocumentParser` struct.

## Tests First

All tests go in a `#[cfg(test)] mod tests` block at the bottom of `parser.rs` (or in the existing test module if sections 02/03 already created one). These tests are independent of the PDF/DOCX tests from earlier sections.

### Test: parse_text with multi-paragraph text returns sections split on blank lines

```rust
#[test]
fn test_parse_text_splits_on_blank_lines() {
    let input = "First paragraph here.\n\nSecond paragraph here.\n\nThird paragraph.";
    let result = DocumentParser::parse_text(input).unwrap();
    assert_eq!(result.sections.len(), 3);
    assert!(result.sections[0].text.contains("First paragraph"));
    assert!(result.sections[1].text.contains("Second paragraph"));
    assert!(result.sections[2].text.contains("Third paragraph"));
    assert!(result.sections.iter().all(|s| s.heading.is_none()));
    assert!(result.sections.iter().all(|s| s.page_number.is_none()));
    assert!(result.word_count > 0);
}
```

### Test: parse_text with empty string returns EmptyDocument

```rust
#[test]
fn test_parse_text_empty_string() {
    let result = DocumentParser::parse_text("");
    assert!(matches!(result, Err(ParsingError::EmptyDocument)));
}
```

### Test: parse_text with whitespace-only returns EmptyDocument

```rust
#[test]
fn test_parse_text_whitespace_only() {
    let result = DocumentParser::parse_text("   \n\n  \t  ");
    assert!(matches!(result, Err(ParsingError::EmptyDocument)));
}
```

### Test: parse dispatches .pdf to parse_pdf

This test requires a real PDF file on disk. Use `tempfile` or create a minimal fixture. The key assertion is that `.pdf` extension triggers `parse_pdf` logic. If no test fixture PDF is available, this test can verify the path is attempted by checking for an `IoError` on a non-existent `.pdf` path (confirming dispatch happened rather than returning `UnsupportedFormat`).

```rust
#[test]
fn test_parse_dispatches_pdf() {
    let path = Path::new("/tmp/nonexistent_test_file.pdf");
    let result = DocumentParser::parse(path);
    // Should NOT be UnsupportedFormat -- it should attempt PDF parsing
    assert!(!matches!(result, Err(ParsingError::UnsupportedFormat(_))));
}
```

### Test: parse dispatches .docx to parse_docx

Same approach -- a non-existent `.docx` file should produce an IO or corrupt-file error, not `UnsupportedFormat`.

```rust
#[test]
fn test_parse_dispatches_docx() {
    let path = Path::new("/tmp/nonexistent_test_file.docx");
    let result = DocumentParser::parse(path);
    assert!(!matches!(result, Err(ParsingError::UnsupportedFormat(_))));
}
```

### Test: parse with .txt returns UnsupportedFormat

```rust
#[test]
fn test_parse_unsupported_format() {
    let path = Path::new("/tmp/some_file.txt");
    let result = DocumentParser::parse(path);
    assert!(matches!(result, Err(ParsingError::UnsupportedFormat(_))));
}
```

### Test: parse with .PDF (uppercase) works (case-insensitive)

```rust
#[test]
fn test_parse_case_insensitive_extension() {
    let path = Path::new("/tmp/nonexistent_test_file.PDF");
    let result = DocumentParser::parse(path);
    // Should attempt PDF parsing, not return UnsupportedFormat
    assert!(!matches!(result, Err(ParsingError::UnsupportedFormat(_))));
}
```

### Test: parse checks file size and returns FileTooLarge for oversized files

This test requires creating a file larger than 20MB. Use `tempfile` to create a sparse or actual large file.

```rust
#[test]
fn test_parse_file_too_large() {
    // Create a temp file > 20MB
    use std::io::Write;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("huge.pdf");
    let mut f = std::fs::File::create(&path).unwrap();
    // Write 21MB of zeros
    let chunk = vec![0u8; 1024 * 1024];
    for _ in 0..21 {
        f.write_all(&chunk).unwrap();
    }
    drop(f);

    let result = DocumentParser::parse(&path);
    assert!(matches!(
        result,
        Err(ParsingError::FileTooLarge { size: _, max: _ })
    ));
}
```

## Implementation Details

### `DocumentParser::parse_text`

**Signature:**

```rust
impl DocumentParser {
    pub fn parse_text(text: &str) -> Result<ParsedDocument, ParsingError> {
        // ...
    }
}
```

**Logic:**

1. Trim the input. If the trimmed result is empty, return `Err(ParsingError::EmptyDocument)`.
2. Normalize the text: collapse runs of 3+ newlines down to exactly 2 newlines; trim each line of leading/trailing whitespace.
3. Split on double newlines (`"\n\n"`) to produce section texts.
4. Filter out any sections that are empty after trimming.
5. If no sections remain, return `Err(ParsingError::EmptyDocument)`.
6. Map each section text into a `DocumentSection` with `heading: None`, `page_number: None`, and `text` set to the trimmed section content.
7. Build `full_text` by joining all section texts with `"\n\n"`.
8. Compute `word_count` by splitting `full_text` on whitespace and counting.
9. Compute `token_count_estimate` as `(word_count as f64 * 1.33) as usize`.
10. Return `Ok(ParsedDocument { full_text, sections, word_count, token_count_estimate })`.

### `DocumentParser::parse`

**Signature:**

```rust
impl DocumentParser {
    pub fn parse(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        // ...
    }
}
```

**Constants:** Define `const MAX_FILE_SIZE: usize = 20 * 1024 * 1024;` (20 MB) at the module level or inside the function.

**Logic:**

1. Read file metadata with `std::fs::metadata(file_path)?` (the `?` converts `io::Error` to `ParsingError::IoError` via the `#[from]` derive on the enum).
2. Check `metadata.len() as usize` against `MAX_FILE_SIZE`. If it exceeds, return `Err(ParsingError::FileTooLarge { size: metadata.len() as usize, max: MAX_FILE_SIZE })`.
3. Extract the file extension: `file_path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase())`.
4. Add a `tracing::info!` log: `"Parsing file: {}, size: {} bytes"` with the file path and size.
5. Record the start time with `std::time::Instant::now()`.
6. Match on extension:
   - `Some("pdf")` => call `Self::parse_pdf(file_path)`
   - `Some("docx")` => call `Self::parse_docx(file_path)`
   - `Some(other)` => return `Err(ParsingError::UnsupportedFormat(other.to_string()))`
   - `None` => return `Err(ParsingError::UnsupportedFormat("no extension".to_string()))`
7. After successful parsing, log with `tracing::info!`: `"Parse complete: {} words, {} sections, {:.2?} elapsed"` with word count, section count, and elapsed time.
8. Return the result.

### Important Design Notes

- **`parse()` is file-only.** It requires a file on disk. For raw text input (e.g., pasted text via API), the route handler in split 04 calls `parse_text()` directly. This is intentional -- text input has no file path and no file size to check.
- **`parse()` is synchronous.** The route handlers in split 04 must wrap calls in `tokio::task::spawn_blocking` since file I/O and PDF/DOCX parsing are blocking operations. That is not this section's concern.
- **Tracing:** Use `tracing::info!` and `tracing::warn!` (the `tracing` crate is already a dependency of the project via Axum). No additional dependencies needed for logging.
- **The 20MB limit** is a hard ceiling. The `FileTooLarge` error includes both the actual size and the max, so error messages are informative.