# 02-document-parsing: Text Extraction Pipeline

## Summary

Extract plain text from PDF and DOCX files uploaded by users. Handle file upload mechanics (multipart, size limits, temp storage), document parsing, and text tokenization for downstream matching.

## Requirements Source

- Feature spec: `docs/specs/2026-03-17-document-analysis-engine-design.md` (Analysis Pipeline step 1-2)
- Interview: `docs/specs/deep_project_interview.md`

## What to Build

### Document Parser (`backend/src/features/analysis/parser.rs`)

A `DocumentParser` struct with methods:

```rust
impl DocumentParser {
    /// Parse a PDF file into plain text
    pub fn parse_pdf(file_path: &Path) -> Result<ParsedDocument, ParsingError>;

    /// Parse a DOCX file into plain text
    pub fn parse_docx(file_path: &Path) -> Result<ParsedDocument, ParsingError>;

    /// Pass-through for text input (normalize whitespace, basic cleanup)
    pub fn parse_text(text: &str) -> Result<ParsedDocument, ParsingError>;

    /// Detect file type and dispatch to appropriate parser
    pub fn parse(file_path: &Path) -> Result<ParsedDocument, ParsingError>;
}

pub struct ParsedDocument {
    pub full_text: String,
    pub sections: Vec<DocumentSection>,  // if structure is detectable
    pub word_count: usize,
    pub token_count_estimate: usize,     // rough estimate for cost tracking
}

pub struct DocumentSection {
    pub heading: Option<String>,
    pub text: String,
    pub page_number: Option<usize>,      // for PDF
}
```

### Text Tokenization (`backend/src/features/analysis/tokenizer.rs`)

Utilities for preparing text for the matching engine:

- **Sentence splitting** — Split text into sentences for evidence extraction
- **Keyword extraction** — Extract significant terms (remove stopwords, normalize)
- **N-gram generation** — 1-gram, 2-gram, 3-gram for matching compound terms (e.g., "multi-factor authentication")
- **Term frequency** — Count term occurrences for TF-IDF scoring

### File Upload Handling

- Accept multipart file upload via Axum's `Multipart` extractor
- Validate: max 20MB, allowed extensions (.pdf, .docx), non-empty
- Store in `backend/uploads/{analysis_id}/` directory
- Return file path for parser consumption
- Create uploads directory on startup if not exists

### Error Handling

Custom `ParsingError` enum:
- `UnsupportedFormat` — Not PDF or DOCX
- `CorruptFile` — Parser cannot read the file
- `EmptyDocument` — File parsed but no text content extracted
- `FileTooLarge` — Exceeds 20MB limit
- `IoError` — File system errors

## Key Decisions

- **Section detection is best-effort** — PDF structure detection is unreliable. Return flat text if sections can't be identified. DOCX headings are more reliable.
- **Token count is an estimate** — Use word_count / 0.75 as rough token estimate. Exact count not needed for MVP (no LLM billing yet).
- **No OCR** — Text-based PDFs only. Scanned/image PDFs will return `EmptyDocument` error. OCR is Phase 2+.
- **English only** — No special language detection for MVP.

## Dependencies

- **Needs from 01-db-models:** `InputType` enum
- **Provides to 04-backend-api-export:** `DocumentParser`, `ParsedDocument`, file upload utilities
- **Provides to 03-matching-engine:** `ParsedDocument` struct (text + token data)

## New Cargo Dependencies

- `pdf-extract` — PDF text extraction (pure Rust)
- `docx-rs` — DOCX reading/writing (also used by 04 for export)

## Existing Patterns to Follow

- Error types: `backend/src/features/` uses `thiserror` for error enums
- File handling: Axum `Multipart` extractor (axum already has multipart support built-in)
- The backend already has `multer` available through axum's multipart feature
