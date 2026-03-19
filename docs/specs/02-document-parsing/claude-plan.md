# Implementation Plan: Document Parsing Pipeline (Delta)

## Overview

This plan covers **incremental improvements** to the existing document parsing pipeline in `backend/src/features/analysis/`. The core implementation (parser, tokenizer, matcher) already exists from parallel development. This plan identifies gaps and refinements needed for production readiness.

## What Already Exists

| File | Lines | Tests | Status |
|------|-------|-------|--------|
| `parser.rs` | 490 | 14 | Full DocumentParser with parse_pdf, parse_docx, parse_text, ParsedDocument, DocumentSection, ParsingError |
| `tokenizer.rs` | 187 | 9 | sentence_split, extract_keywords, generate_ngrams, term_frequency |
| `matcher.rs` | 1273 | 27 | Full DeterministicMatcher implementing MatchingEngine trait |
| `routes.rs` | 189 | 0 | API route stubs (started) |
| `engine.rs` | 119 | 0 | MatchingEngine trait, AnalysisError, MatchingResult |
| `models.rs` | 563 | ~10 | InputType, AnalysisStatus, FindingType enums + all model structs |

**Cargo.toml already has**: `zip = "2"`, `quick-xml = "0.37"`, `pdf-extract`, `unicode-segmentation`, `stop-words`, `rust-stemmers`

## What Needs to Be Done

### Section 1: Upload Handler (`upload.rs`)

**Status: Does not exist yet.**

This is the only genuinely new module. It provides file upload utilities that the route handler (unit 04) will call.

**Create `backend/src/features/analysis/upload.rs`:**

Constants:
- `MAX_FILE_SIZE: u64 = 20 * 1024 * 1024`
- `ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx"]`
- `UPLOAD_DIR: &str = "uploads"`

Functions:

`validate_upload(filename: &str, size: u64) -> Result<InputType, ParsingError>`
- Check file size against MAX_FILE_SIZE, return FileTooLarge if exceeded
- Extract extension case-insensitively, map to InputType
- Return UnsupportedFormat for unknown extensions
- **Validate magic bytes**: Check first 4 bytes — `%PDF` for PDF, `PK` (0x504B) for DOCX/ZIP
- Accept a `&[u8]` header parameter for magic byte checking

`save_upload(analysis_id: &str, filename: &str, data: &[u8]) -> Result<PathBuf, ParsingError>`
- **Validate analysis_id is UUID format** before using in path (prevent path traversal)
- Create `{UPLOAD_DIR}/{analysis_id}/` directory
- Generate UUID filename preserving extension
- Write bytes, return path

`stream_upload_to_file(analysis_id: &str, filename: &str, field: axum::extract::multipart::Field) -> Result<PathBuf, ParsingError>`
- For streaming large files: chunk-by-chunk writing via `field.chunk().await`
- Return path to saved file

Update `mod.rs` to export the new module.

### Section 2: Parser Refinements

**Status: Exists, needs targeted improvements.**

Changes to `parser.rs`:

1. **Add EmptyDocument message variant**: Change `EmptyDocument` from unit variant to `EmptyDocument(String)` so scanned PDF detection can provide an actionable message like "This appears to be a scanned/image-based PDF. Text-based PDFs are required."

2. **Add From<ParsingError> for AppError**: Implement the conversion in `error.rs` (or in `parser.rs` behind a feature import):
   - `UnsupportedFormat`, `EmptyDocument`, `FileTooLarge` → `AppError::BadRequest`
   - `CorruptFile`, `IoError` → `AppError::Internal`

3. **Wrap sync I/O in spawn_blocking**: The `parse_pdf` and `parse_docx` functions use blocking `std::fs` operations. Add a public async wrapper:
   ```rust
   pub async fn parse_async(file_path: PathBuf) -> Result<ParsedDocument, ParsingError>
   ```
   that calls `tokio::task::spawn_blocking(move || DocumentParser::parse(&file_path))`.

4. **Scanned PDF detection**: After `pdf_extract::extract_text_from_mem`, if `text.trim().len() < 50 && file_size > 10_000`, return `ParsingError::EmptyDocument("scanned/image-based PDF detected".into())`.

### Section 3: Route Handler Completion

**Status: Stubs exist, need full implementation.**

Complete `routes.rs` with:

1. **POST `/api/analysis/upload`**: Accept multipart upload, validate, save file, parse text, create analysis record in DB, trigger matching engine, return analysis ID.

2. **Router-level size limit**: Add `RequestBodyLimitLayer::new(25 * 1024 * 1024)` (slightly above 20MB to account for multipart overhead) and `DefaultBodyLimit::disable()`.

3. **Wire into main router**: Add `.nest("/analysis", features::analysis::routes::router())` in `lib.rs`.

4. **OpenAPI annotations**: Add `#[utoipa::path]` attributes matching existing patterns.

### Section 4: Module Wiring and Integration Tests

1. **Update `mod.rs`**: Export `upload` module alongside existing `parser`, `tokenizer`, `matcher`, `models`, `engine`.

2. **Integration test**: Add test in `backend/tests/` that uploads a real PDF via multipart → verify extracted text is stored in `analyses.extracted_text`.

3. **Test fixture**: Create a small `backend/tests/fixtures/sample.pdf` (1-page text PDF) for integration testing. Do NOT commit a 20MB test file — generate large files at test time using `tempfile`.

## Build Order

1. **Upload handler** (new module, no dependencies on parser changes)
2. **Parser refinements** (EmptyDocument variant, From impl, async wrapper, scanned detection)
3. **Route completion** (depends on upload handler + parser being ready)
4. **Integration tests** (depends on routes being complete)

## Cargo Dependencies

No new dependencies needed — all required crates are already in `Cargo.toml` from parallel development.

## Testing Strategy

### New tests needed:

**upload.rs unit tests:**
- Reject file > 20MB → FileTooLarge
- Reject unknown extension → UnsupportedFormat
- Accept .pdf → InputType::Pdf
- Accept .DOCX (case insensitive) → InputType::Docx
- Reject analysis_id with path traversal chars
- Validate magic bytes: PDF header, ZIP header, mismatch

**parser.rs additions:**
- Test scanned PDF detection (near-empty text + large file)
- Test `parse_async` wrapper
- Test `From<ParsingError> for AppError` mapping

**Integration tests:**
- Upload PDF multipart → verify analysis created with extracted text
- Upload file > 20MB → verify 400 response
- Upload .exe file → verify 400 response
