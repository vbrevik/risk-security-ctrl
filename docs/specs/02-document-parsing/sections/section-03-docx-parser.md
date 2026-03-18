# Section 03: DOCX Parser

## Overview

This section implements `DocumentParser::parse_docx()` in `backend/src/features/analysis/parser.rs`. It extracts body text from DOCX files by treating them as ZIP archives and parsing the inner `word/document.xml` with `quick-xml`. Headings are detected via `<w:pStyle>` elements to produce structured `DocumentSection` entries.

**Dependencies:** Section 01 (deps-and-types) must be completed first. The `parser.rs` file must already exist with `ParsingError`, `ParsedDocument`, `DocumentSection`, and the `DocumentParser` unit struct defined. The `zip` and `quick-xml` crates must be in `Cargo.toml`.

**Blocks:** Section 04 (text-parser-dispatch) depends on this section.

## Background

A DOCX file is a ZIP archive containing XML files. The main body text lives in `word/document.xml`. The XML structure relevant to text extraction is:

- `<w:p>` -- paragraph element
- `<w:pPr>` -- paragraph properties (child of `w:p`)
- `<w:pStyle w:val="Heading1"/>` -- paragraph style indicating a heading (child of `w:pPr`)
- `<w:r>` -- run element (child of `w:p`)
- `<w:t>` -- text element (child of `w:r`), contains the actual character data

Multiple `<w:t>` elements within the same paragraph are fragments that should be concatenated **without** spaces between them (they are parts of the same word or phrase). A newline should be appended at each `</w:p>` boundary.

**Limitations (documented, intentional for MVP):**
- Headers, footers, tables, and footnotes are NOT extracted -- only body text from `word/document.xml`.
- No image/embedded object handling.

## Types Reference (from Section 01)

These types are assumed to exist in `backend/src/features/analysis/parser.rs`:

```rust
#[derive(Debug, Clone, Serialize)]
pub struct DocumentSection {
    pub heading: Option<String>,
    pub text: String,
    pub page_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParsedDocument {
    pub full_text: String,
    pub sections: Vec<DocumentSection>,
    pub word_count: usize,
    pub token_count_estimate: usize,
}

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Could not parse file: {0}")]
    CorruptFile(String),
    #[error("No text content found in document")]
    EmptyDocument,
    // ... other variants
}

pub struct DocumentParser;
```

## Tests

All tests go in `backend/src/features/analysis/parser.rs` inside a `#[cfg(test)] mod tests` block (shared with Section 02 tests). Each test creates a minimal DOCX in memory as a ZIP archive with a `word/document.xml` entry.

### Test helper: create_test_docx

A helper function that builds a minimal DOCX ZIP archive from a `document.xml` string and writes it to a temp file. This avoids needing fixture files on disk.

```rust
/// Creates a minimal DOCX file (ZIP archive with word/document.xml) at the given path.
/// Returns the path for convenience.
fn create_test_docx(dir: &tempfile::TempDir, xml_content: &str) -> std::path::PathBuf {
    // Build a ZIP in memory containing word/document.xml with the given XML content
    // Write to dir.path().join("test.docx")
    // Return the path
}
```

Use `tempfile::TempDir` (add `tempfile` as a dev-dependency if not already present -- or note that section 02 should have added it for PDF tests).

### Test: parse_docx extracts text from a valid DOCX

```rust
#[test]
fn test_parse_docx_extracts_text() {
    // Create a DOCX with a simple document.xml containing two paragraphs:
    //   <w:p><w:r><w:t>Hello world</w:t></w:r></w:p>
    //   <w:p><w:r><w:t>Second paragraph</w:t></w:r></w:p>
    // Call DocumentParser::parse_docx(&path)
    // Assert full_text contains "Hello world" and "Second paragraph"
    // Assert word_count >= 4
}
```

### Test: parse_docx detects headings as section boundaries

```rust
#[test]
fn test_parse_docx_detects_headings() {
    // Create a DOCX with heading style paragraphs:
    //   <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Introduction</w:t></w:r></w:p>
    //   <w:p><w:r><w:t>Body text here.</w:t></w:r></w:p>
    //   <w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>Details</w:t></w:r></w:p>
    //   <w:p><w:r><w:t>More details.</w:t></w:r></w:p>
    // Call DocumentParser::parse_docx(&path)
    // Assert sections.len() == 2
    // Assert sections[0].heading == Some("Introduction")
    // Assert sections[0].text contains "Body text here"
    // Assert sections[1].heading == Some("Details")
    // Assert sections[1].text contains "More details"
}
```

### Test: parse_docx with corrupt ZIP returns CorruptFile

```rust
#[test]
fn test_parse_docx_corrupt_zip() {
    // Write random/invalid bytes to a .docx file
    // Call DocumentParser::parse_docx(&path)
    // Assert result is Err(ParsingError::CorruptFile(_))
}
```

### Test: parse_docx with empty document returns EmptyDocument

```rust
#[test]
fn test_parse_docx_empty_document() {
    // Create a DOCX with a document.xml that has paragraphs but no <w:t> elements:
    //   <w:document><w:body><w:p><w:r></w:r></w:p></w:body></w:document>
    // Call DocumentParser::parse_docx(&path)
    // Assert result is Err(ParsingError::EmptyDocument)
}
```

## Implementation

### File: `backend/src/features/analysis/parser.rs`

Add the `parse_docx` associated function to the existing `DocumentParser` impl block.

### Dev-dependency needed

If not already present (section 02 may have added it):

```toml
[dev-dependencies]
tempfile = "3"
```

### DocumentParser::parse_docx

Signature:

```rust
impl DocumentParser {
    pub fn parse_docx(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        // ...
    }
}
```

**Algorithm:**

1. Read the file bytes with `std::fs::read(file_path)?` (the `IoError` variant handles `From<std::io::Error>` automatically).

2. Open the bytes as a ZIP archive:
   ```rust
   let cursor = std::io::Cursor::new(bytes);
   let mut archive = zip::ZipArchive::new(cursor)
       .map_err(|e| ParsingError::CorruptFile(e.to_string()))?;
   ```

3. Extract `word/document.xml` from the archive:
   ```rust
   let mut xml_file = archive.by_name("word/document.xml")
       .map_err(|e| ParsingError::CorruptFile(e.to_string()))?;
   ```
   Read its content into a `String`.

4. Parse the XML with `quick_xml::Reader::from_str(&xml_content)`. Walk events in a loop:

   **State variables to maintain:**
   - `current_text: String` -- text accumulated for the current paragraph
   - `current_section_text: String` -- text accumulated for the current section
   - `current_heading: Option<String>` -- heading for the current section
   - `sections: Vec<DocumentSection>` -- completed sections
   - `in_text_element: bool` -- true when inside a `<w:t>` element
   - `next_para_is_heading: bool` -- set to true when `<w:pStyle>` with "Heading" value is detected
   - `in_paragraph_props: bool` -- true when inside `<w:pPr>` to scope heading detection

   **Event handling:**
   - `Event::Start` or `Event::Empty` for element with local name `pStyle`: check if the `w:val` attribute contains "Heading" (case-sensitive substring match). If so, set `next_para_is_heading = true`.
   - `Event::Start` for `t` (local name): set `in_text_element = true`.
   - `Event::End` for `t`: set `in_text_element = false`.
   - `Event::Text` when `in_text_element`: append the unescaped text to `current_text` (no space between fragments).
   - `Event::End` for `p` (paragraph end):
     - If `next_para_is_heading` is true: push any existing section (if `current_section_text` is non-empty), start a new section with `current_text` as the heading, reset `current_section_text`.
     - Otherwise: append `current_text` + newline to `current_section_text`.
     - Reset `current_text` and `next_para_is_heading`.
   - `Event::Start`/`Event::End` for `pPr`: track `in_paragraph_props` to scope style detection.

5. After the loop, push the final section if `current_section_text` is non-empty.

6. If no sections were created (no headings found), consolidate all text into a single section with `heading: None`.

7. If no text was extracted at all (all sections empty after trimming), return `Err(ParsingError::EmptyDocument)`.

8. Build `full_text` by joining all section texts with `"\n\n"`.

9. Compute `word_count` from `full_text.split_whitespace().count()` and `token_count_estimate` as `(word_count as f64 * 1.33) as usize`.

10. Return `Ok(ParsedDocument { full_text, sections, word_count, token_count_estimate })`.

### XML namespace handling note

DOCX XML uses the `w:` namespace prefix. With `quick-xml`, element local names are accessed via `e.local_name()`. The local name for `<w:t>` is `t`, for `<w:p>` is `p`, for `<w:pStyle>` is `pStyle`, for `<w:pPr>` is `pPr`. Attribute `w:val` has local name `val`. Use `e.local_name().as_ref()` to compare against byte slices like `b"t"`, `b"p"`, `b"pStyle"`, `b"pPr"`, `b"val"`.

### Minimal valid document.xml for reference

This is the structure the parser expects:

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:pPr><w:pStyle w:val="Heading1"/></w:pPr>
      <w:r><w:t>Section Title</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>Body paragraph text.</w:t></w:r>
    </w:p>
  </w:body>
</w:document>
```

## Files Modified

| File | Action |
|------|--------|
| `backend/src/features/analysis/parser.rs` | Add `parse_docx` method to `DocumentParser` impl block |
| `backend/Cargo.toml` | Add `tempfile = "3"` to `[dev-dependencies]` if not already present |

## Verification

After implementation, run:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend
cargo test parse_docx -- --nocapture
```

All four DOCX-related tests should pass. Also run `cargo clippy` to catch any warnings.