# Combined Spec: 02-document-parsing

## What We're Building

Text extraction pipeline for the Document Analysis Engine. Parses PDF and DOCX files into plain text with section detection, plus text tokenization utilities for downstream matching. Includes file upload handling.

## Key Changes from Original Spec

- **DOCX:** Use `zip` + `quick-xml` instead of `docx-rs` (which is a writer, not reader)
- **PDF:** Use `extract_text_by_pages()` for page-based sections instead of flat `extract_text()`
- **Dependencies:** `pdf-extract`, `zip`, `quick-xml` (not `docx-rs`)

## Deliverables

1. **`parser.rs`** — `DocumentParser` with `parse_pdf()`, `parse_docx()`, `parse_text()`, `parse()`
2. **`tokenizer.rs`** — Sentence splitting, keyword extraction, n-grams, term frequency
3. **File upload utilities** — Multipart handling, validation, directory creation
4. **Cargo deps** — `pdf-extract = "0.10"`, `zip = "2"`, `quick-xml = "0.37"`

## Architecture Decisions

- PDF: `pdf-extract::extract_text_by_pages()` → one `DocumentSection` per page
- DOCX: Manual ZIP extraction of `word/document.xml` → parse `<w:t>` text nodes, detect `<w:pStyle>` headings for sections
- Text input: Normalize whitespace, split on blank lines for sections
- Token estimate: `word_count * 1.33` (rough)
- No OCR — scanned PDFs return EmptyDocument error
- File upload: 20MB limit via `DefaultBodyLimit`, save to `backend/uploads/analyses/{id}/`
