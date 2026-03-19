# Combined Spec: Document Parsing Pipeline

## What We're Building

A text extraction pipeline for the Document Analysis Engine. Users upload PDF or DOCX files (or paste text), and the system extracts structured text with section boundaries for downstream framework matching.

## Components

### 1. DocumentParser
- `parse_pdf(path)` — Extract text from PDF using `pdf-extract` crate
- `parse_docx(path)` — Extract text from DOCX using `zip` + `quick-xml` (DOCX is ZIP containing XML)
- `parse_text(text)` — Normalize whitespace, basic cleanup for pasted text
- `parse(path)` — Detect file type by extension and dispatch
- Returns `ParsedDocument` with full text, detected sections (heading + text + page number), word count, token estimate

### 2. Text Tokenizer
- Sentence splitting via `unicode-segmentation`
- Keyword extraction: stopword removal (English + Norwegian via `stop-words`), stemming (via `rust-stemmers`)
- N-gram generation (1-gram, 2-gram, 3-gram) using `slice::windows()`
- Term frequency counting for downstream TF-IDF scoring
- Language: support both English and Norwegian from the start

### 3. File Upload Handler
- Axum `Multipart` extractor
- 20MB limit, .pdf/.docx extensions only
- Stream to temp file first (medium-sized documents expected, 5-20MB)
- Store in `backend/uploads/{analysis_id}/` with UUID filenames
- Configurable retention (keep by default)

### 4. Error Types
- `ParsingError` enum: UnsupportedFormat, CorruptFile, EmptyDocument, FileTooLarge, IoError
- Scanned/image PDFs: detect near-empty text extraction and return `EmptyDocument` with explanatory message
- Convert to `AppError` at handler boundary

## Key Decisions
- **PDF extraction**: `pdf-extract` crate (simple API, 875K downloads, pure Rust)
- **DOCX extraction**: Manual `zip` + `quick-xml` parsing (docx-rs is a writer, not reader)
- **Section detection**: Preserve headings as sections in both PDF and DOCX (best-effort for PDF, reliable for DOCX headings via `w:pStyle` XML attributes)
- **Scanned PDFs**: Detect and return specific EmptyDocument error
- **Norwegian NLP**: Support English + Norwegian stopwords and stemming from day one
- **File size**: Stream to temp file, then parse (handles 5-20MB documents)
- **File retention**: Keep uploaded files by default, cleanup mechanism deferred

## Dependencies (from 01-db-models)
- `InputType` enum (Text, Pdf, Docx)
- `analyses.extracted_text` column for storing result
- `analyses.file_path` column for original file reference

## Provides to downstream
- `DocumentParser` and `ParsedDocument` → used by 04-backend-api-export
- Tokenizer utilities → used by 03-matching-engine for TF-IDF scoring

## New Cargo Dependencies
```toml
pdf-extract = "0.10"
zip = "0.6"
quick-xml = "0.31"
unicode-segmentation = "1"
stop-words = "0.10"
rust-stemmers = "1"
```
