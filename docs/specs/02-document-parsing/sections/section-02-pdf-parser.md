Now I have all the context needed. Here is the section content:

# Section 02: PDF Parser

## Overview

This section implements `DocumentParser::parse_pdf()` in `backend/src/features/analysis/parser.rs`. It uses the `pdf-extract` crate to extract text page-by-page from PDF files and returns a `ParsedDocument` with one `DocumentSection` per page.

## Dependencies

**Depends on:** section-01-deps-and-types (must be completed first)
- The `pdf-extract` crate must be in `Cargo.toml`
- The types `ParsingError`, `ParsedDocument`, `DocumentSection`, and `DocumentParser` must exist in `parser.rs`

**Blocks:** section-04-text-parser-dispatch (the `parse()` dispatch method calls `parse_pdf`)

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/parser.rs`

This file is created by section-01. This section adds the `parse_pdf` associated function to the `DocumentParser` unit struct.

## Background Context

`DocumentParser` is a unit struct (`pub struct DocumentParser;`) with no state. All parsing methods are associated functions. The `parse_pdf` function is one of three format-specific parsers (PDF, DOCX, text). The dispatch function `parse()` in section-04 will call it based on file extension.

The types from section-01 that this function uses:

- `ParsingError` -- an enum with variants `CorruptFile(String)`, `EmptyDocument`, and `IoError(std::io::Error)` (among others)
- `ParsedDocument` -- struct with `full_text: String`, `sections: Vec<DocumentSection>`, `word_count: usize`, `token_count_estimate: usize`
- `DocumentSection` -- struct with `heading: Option<String>`, `text: String`, `page_number: Option<usize>`

## Tests (Write First)

All tests go in a `#[cfg(test)] mod tests` block at the bottom of `parser.rs`. These tests require a small test fixture PDF file. The recommended approach is to create one programmatically or include a minimal PDF binary in the test setup.

### Test: parse_pdf with a valid PDF returns non-empty full_text

Create or use a small test PDF fixture. Call `DocumentParser::parse_pdf(&path)`. Assert that `result.is_ok()`, `full_text` is not empty, and `word_count > 0`.

```rust
#[test]
fn test_parse_pdf_valid_returns_text() {
    // Create a minimal test PDF fixture (or use a file in tests/fixtures/)
    // Call DocumentParser::parse_pdf(&path)
    // Assert result.is_ok()
    // Assert !result.full_text.is_empty()
    // Assert result.word_count > 0
    // Assert result.token_count_estimate > 0
}
```

### Test: parse_pdf returns sections with page numbers

Using the same fixture, assert that `sections` is non-empty, each section has `page_number: Some(n)` where n starts at 1, and each section has `heading` matching `Some("Page N")`.

```rust
#[test]
fn test_parse_pdf_sections_have_page_numbers() {
    // Call DocumentParser::parse_pdf on a valid PDF
    // Assert sections.len() >= 1
    // Assert sections[0].page_number == Some(1)
    // Assert sections[0].heading == Some("Page 1".to_string())
}
```

### Test: parse_pdf with corrupt file returns CorruptFile error

Write a file with random bytes and a `.pdf` extension. Call `parse_pdf`. Assert it returns `Err(ParsingError::CorruptFile(_))`.

```rust
#[test]
fn test_parse_pdf_corrupt_returns_error() {
    // Write garbage bytes to a temp file with .pdf extension
    // Call DocumentParser::parse_pdf(&path)
    // Assert matches!(result, Err(ParsingError::CorruptFile(_)))
}
```

### Test: parse_pdf with non-existent path returns IoError

Call `parse_pdf` with a path that does not exist. Assert it returns an `IoError` or `CorruptFile` (depending on how `pdf-extract` surfaces missing files -- the test should accept either).

```rust
#[test]
fn test_parse_pdf_missing_file_returns_error() {
    // Call DocumentParser::parse_pdf on a non-existent path
    // Assert result.is_err()
}
```

### Test fixture strategy

For tests that need a valid PDF, there are two approaches. Either is acceptable:

1. **Programmatic creation:** Use a minimal PDF byte sequence. A valid minimal PDF is roughly 200 bytes and can be hardcoded as a `&[u8]` constant, then written to a temp file.
2. **Fixture file:** Place a small PDF at `backend/tests/fixtures/sample.pdf` and reference it from tests.

Use `std::env::temp_dir()` or `tempfile` crate for temp files in tests. If using `tempfile`, add it to `[dev-dependencies]` in Cargo.toml (coordinate with section-01 or add it here).

## Implementation Details

### DocumentParser::parse_pdf

Signature:

```rust
impl DocumentParser {
    pub fn parse_pdf(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        // ...
    }
}
```

**Step-by-step logic:**

1. **Extract text by pages.** Call `pdf_extract::extract_text_by_pages(file_path)`. This returns `Result<Vec<String>, _>` with one string per page. Map any error to `ParsingError::CorruptFile(e.to_string())`.

2. **Verify the function exists.** The `pdf-extract` 0.10 crate may only expose `extract_text()` (returns a single `String`) rather than `extract_text_by_pages()`. If `extract_text_by_pages` is not available, fall back to `extract_text()` and produce a single `DocumentSection` with `heading: Some("Page 1")` and `page_number: Some(1)`. Check the actual API when implementing.

3. **Build sections.** For each page (1-indexed), create a `DocumentSection`:
   - `heading: Some(format!("Page {}", page_number))`
   - `text`: the page's text content, trimmed
   - `page_number: Some(page_number)`
   - Skip pages where trimmed text is empty

4. **Check for empty document.** If no sections remain after filtering, return `Err(ParsingError::EmptyDocument)`.

5. **Build full_text.** Concatenate all section texts with `"\n\n"` separator.

6. **Compute counts:**
   - `word_count`: `full_text.split_whitespace().count()`
   - `token_count_estimate`: `(word_count as f64 * 1.33) as usize`

7. **Return** the `ParsedDocument`.

### Error mapping

The `pdf-extract` crate returns its own error type. Map it with:

```rust
.map_err(|e| ParsingError::CorruptFile(e.to_string()))
```

This covers corrupt PDFs, encrypted PDFs, and files that are not actually PDFs.

### Important notes

- This function is **synchronous** (no async). The route handler in split 04 will wrap calls in `tokio::task::spawn_blocking`.
- No tracing/logging is added here -- that is handled by the `parse()` dispatch function in section-04.
- Scanned PDFs (image-only) will produce empty text and return `EmptyDocument`. OCR is out of scope.

## Checklist

1. Write all four test stubs in the `#[cfg(test)]` block of `parser.rs`
2. Create or prepare a test PDF fixture
3. Implement `DocumentParser::parse_pdf` following the step-by-step logic above
4. Handle the `extract_text` vs `extract_text_by_pages` API availability
5. Run `cargo test` -- all four PDF parser tests should pass
6. Run `cargo clippy` -- no warnings