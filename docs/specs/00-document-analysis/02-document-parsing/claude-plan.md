# Implementation Plan: 02-document-parsing

## Overview

This plan adds a text extraction pipeline to the existing `analysis` feature module in the risk-security-ctrl Rust/Axum backend. It creates two new files (`parser.rs` and `tokenizer.rs`) and updates module wiring and Cargo dependencies.

The pipeline extracts plain text from PDF files (via `pdf-extract`), DOCX files (via manual ZIP + quick-xml parsing), or raw text input. It produces a `ParsedDocument` struct containing the full text, detected sections, word count, and token count estimate. The tokenizer provides utilities for sentence splitting, keyword extraction, n-gram generation, and term frequency computation — all needed by the downstream matching engine (split 03).

This split does NOT add routes or HTTP handlers — those come in split 04. It only creates the library code that the route handlers will call.

## Why This Matters

The Document Analysis Engine needs to turn user-uploaded files into queryable text before the matching engine can compare it against ontology frameworks. Without this pipeline, there's no way to get from "user uploads a PDF" to "system identifies relevant security controls."

The parser is consumed by:
- **Split 03 (matching-engine)** — uses `ParsedDocument.full_text` and tokenizer output for FTS5 queries and TF-IDF scoring
- **Split 04 (backend-api-export)** — calls `DocumentParser::parse()` in the upload handler, stores `extracted_text` in the `analyses` table

---

## Section 1: Cargo Dependencies

### File: `backend/Cargo.toml`

Add three new dependencies under `[dependencies]`:

| Crate | Version | Purpose |
|-------|---------|---------|
| `pdf-extract` | `0.10` | Extract text from PDF files, page-by-page |
| `zip` | `2` | Read DOCX files (which are ZIP archives) |
| `quick-xml` | `0.37` | Parse XML inside DOCX to extract text nodes |

Group them under a `# Document parsing` comment, following the existing section-comment convention in Cargo.toml.

---

## Section 2: Parser Types and Error Handling

### File: `backend/src/features/analysis/parser.rs`

#### ParsingError enum

Using `thiserror::Error`. Variants:

- `UnsupportedFormat(String)` — file extension is not `.pdf` or `.docx`. Display: `"Unsupported format: {0}"`
- `CorruptFile(String)` — parser library returned an error. Display: `"Could not parse file: {0}"`
- `EmptyDocument` — parsing succeeded but no text was extracted. Display: `"No text content found in document"`
- `FileTooLarge { size: usize, max: usize }` — exceeds 20MB. Display: `"File too large: {size} bytes (max: {max})"`
- `IoError(std::io::Error)` — with `#[from]` for automatic conversion. Display: `"IO error: {0}"`

#### ParsedDocument struct

Fields:
- `full_text: String` — all extracted text concatenated
- `sections: Vec<DocumentSection>` — detected sections (pages for PDF, headings for DOCX, paragraphs for text)
- `word_count: usize` — split on whitespace, count
- `token_count_estimate: usize` — `(word_count as f64 * 1.33) as usize`

#### DocumentSection struct

Fields:
- `heading: Option<String>` — section heading if detected (DOCX headings, or "Page N" for PDF)
- `text: String` — section text content
- `page_number: Option<usize>` — page number (PDF only)

Both structs derive `Debug, Clone, Serialize` (Serialize for potential API exposure later).

---

## Section 3: PDF Parser

### File: `backend/src/features/analysis/parser.rs` (continued)

#### DocumentParser struct

`pub struct DocumentParser;` — a unit struct with associated functions. No state needed.

#### DocumentParser::parse_pdf

Takes `file_path: &Path`, returns `Result<ParsedDocument, ParsingError>`.

**Note:** `extract_text_by_pages()` must be verified to exist in `pdf-extract` 0.10. If not available, fall back to `extract_text()` and produce a single section.

Implementation approach:
1. Call `pdf_extract::extract_text_by_pages(file_path)` to get `Vec<String>` (one string per page)
2. Map errors to `ParsingError::CorruptFile`
3. Create one `DocumentSection` per page with `heading: Some("Page N")` and `page_number: Some(n)`
4. Filter out empty pages
5. Concatenate all page text with double newlines for `full_text`
6. If no pages have text, return `ParsingError::EmptyDocument`
7. Compute `word_count` and `token_count_estimate`

---

## Section 4: DOCX Parser

### File: `backend/src/features/analysis/parser.rs` (continued)

#### DocumentParser::parse_docx

Takes `file_path: &Path`, returns `Result<ParsedDocument, ParsingError>`.

Implementation approach:
1. Read file bytes with `std::fs::read()`
2. Open as ZIP archive with `zip::ZipArchive::new(Cursor::new(bytes))`
3. Extract `word/document.xml` from the archive
4. Parse XML with `quick_xml::Reader`
5. Walk the XML tree:
   - Track when inside `<w:t>` elements — concatenate text WITHOUT spaces within a paragraph (multiple `<w:t>` in a run are fragments of the same word)
   - On `</w:p>` (end of paragraph), append newline to current section
   - On `<w:pStyle>` with `w:val` containing "Heading" (e.g., "Heading1", "Heading2"), start a new `DocumentSection` with the upcoming text as heading
6. If no headings detected, return a single section with all text
7. If no text extracted, return `ParsingError::EmptyDocument`
8. Compute word count and token estimate

#### XML element names to look for:
- `w:t` — text content
- `w:p` — paragraph boundary
- `w:pPr` → `w:pStyle` with attribute `w:val` — paragraph style (heading detection)

---

## Section 5: Text Parser and Dispatch

### File: `backend/src/features/analysis/parser.rs` (continued)

#### DocumentParser::parse_text

Takes `text: &str`, returns `Result<ParsedDocument, ParsingError>`.

1. Trim whitespace
2. If empty, return `ParsingError::EmptyDocument`
3. Normalize: collapse multiple blank lines to double newlines, trim each line
4. Split on double newlines for sections (each becomes a `DocumentSection` with no heading)
5. Compute word count and token estimate

#### DocumentParser::parse

Takes `file_path: &Path`, returns `Result<ParsedDocument, ParsingError>`.

1. Check file size with `std::fs::metadata(file_path)?.len()` — if > 20MB, return `ParsingError::FileTooLarge`
2. Dispatch based on file extension (case-insensitive):
   - `.pdf` → `parse_pdf(file_path)`
   - `.docx` → `parse_docx(file_path)`
   - Other → `ParsingError::UnsupportedFormat`
3. Add `tracing::info!` for parse start/completion with timing

**Note:** `parse()` is file-only. For text input, split 04 calls `parse_text()` directly based on `InputType`. This is intentional — text input has no file path.

---

## Section 6: Tokenizer

### File: `backend/src/features/analysis/tokenizer.rs`

Utilities consumed by the matching engine (split 03). All functions are pure — no database or async needed.

#### sentence_split

Takes `text: &str`, returns `Vec<String>`.

Split on sentence boundaries: period/exclamation/question mark followed by whitespace and a capital letter. Also split on newlines. Filter out empty strings.

Simple regex or manual approach — no NLP library needed for MVP.

#### extract_keywords

Takes `text: &str`, returns `Vec<String>`.

1. Lowercase the text
2. Split on non-alphanumeric characters
3. Filter out stopwords (hardcoded English list: "the", "and", "or", "is", "in", "to", "of", "a", "an", "for", "with", "on", "at", "by", "from", "as", "it", "that", "this", "are", "was", "be", "has", "have", "had", "not", "but", "will", "can", "do", "if", "so", "no", "all", "they", "we", "you", "their", "its", "our", "your", "my", "which", "who", "what", "when", "where", "how", "each", "other", "than", "then", "also", "been", "would", "could", "should", "may", "must", "shall")
4. Filter out tokens shorter than 3 characters
5. Deduplicate while preserving order

#### generate_ngrams

Takes `words: &[String]`, `n: usize`, returns `Vec<String>`.

Generate n-grams from a word list. For n=2: `["multi", "factor", "auth"]` → `["multi factor", "factor auth"]`.

#### term_frequency

Takes `text: &str`, returns `HashMap<String, usize>`.

Count occurrences of each keyword (after lowercasing and splitting). Used for TF-IDF scoring in the matching engine.

---

## Section 7: Module Wiring

### Files to modify

1. **`backend/src/features/analysis/mod.rs`** — Add `pub mod parser;` and `pub mod tokenizer;`

No route changes — this split only adds library code.

---

## Decision Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| DOCX parsing | ZIP + quick-xml | docx-rs is a writer, docx crate unmaintained. Manual parsing is ~30 lines, fully controlled, maintained deps. |
| PDF sections | Page-by-page | `extract_text_by_pages()` gives sections for free. More useful for evidence extraction. |
| Stopwords | Hardcoded list | No NLP dependency for MVP. List covers common English words. |
| Sentence splitting | Simple heuristic | No regex crate needed — manual period+capital check. Good enough for evidence extraction. |
| No OCR | Scanned PDFs → EmptyDocument | OCR adds large C dependencies (tesseract). Phase 2+. |
| DocumentParser | Unit struct with associated fns | No state needed. Keeps API clean. |
| parse() vs parse_text() | Separate entry points | parse() is file-only, parse_text() for text input. Split 04 dispatches based on InputType. |
| DOCX limitations | Headers/footers/tables/footnotes not extracted | Documented. MVP focuses on body text. |
| spawn_blocking | Split 04 concern | Parser functions are sync. Route handlers must wrap in spawn_blocking. |
| Tracing | Add info/warn logging | File received, parse start/complete/fail with timing. |
