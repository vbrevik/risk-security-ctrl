# Research: Document Parsing Pipeline

## Codebase Research

### Backend Feature Structure
Feature-based modular organization in `backend/src/features/`. The analysis module already exists:
- `analysis/models.rs` — `InputType` (Text, Pdf, Docx), `AnalysisStatus`, `FindingType` enums, `Analysis` struct, query parameter structs
- `analysis/engine.rs` — `MatchingEngine` trait with `analyze()` method returning `MatchingResult`
- `analysis/mod.rs` — Module exports
- Database schema in `003_analysis_schema.sql` with `analyses` and `analysis_findings` tables

### Error Handling
Uses `thiserror` with `AppError` enum (`Database`, `NotFound`, `BadRequest`, `Unauthorized`, `Internal`). Implements `IntoResponse` for JSON error responses. Type alias: `AppResult<T> = Result<T, AppError>`.

### File Upload Pattern (from compliance evidence)
Axum `Multipart` extractor with:
- `tokio::fs::create_dir_all` for upload directory
- UUID-based filenames
- `field.bytes().await` for buffered reading
- Existing pattern in `compliance/routes.rs`

### Testing
- Integration tests using `create_test_app()` helper in `tests/common/mod.rs`
- Uses `tower::ServiceExt::oneshot` for request/response testing
- Unit tests within modules using `#[cfg(test)] mod tests`
- Migrations auto-run in test setup

### Dependencies Already Available
- `axum` with `multipart` feature enabled
- `tokio` with `full` features
- `thiserror`, `uuid`, `chrono`, `async-trait`, `tracing`, `serde`/`serde_json`

---

## Web Research

### PDF Text Extraction
**Recommended: `pdf-extract` (v0.10.0, 875K downloads)**
- Simple API: `pdf_extract::extract_text_from_mem(&bytes)` or `extract_text("path")`
- Built on `lopdf`, pure Rust
- Limitations: no OCR, imperfect text ordering on multi-column layouts, limited encrypted PDF support
- Alternative: `pdf_oxide` (v0.3.17) claims 5x faster, 100% pass on 3,830 PDFs — worth watching
- For corrupt PDFs: returns `OutputError` variants, wrap in proper error handling

### DOCX Text Extraction
**Recommended: `zip` + `quick-xml` (manual extraction)**
- `docx-rs` is primarily a **writer**, not a reader — misleading name
- DOCX is a ZIP archive containing XML. Parse `word/document.xml` for text
- Extract `<w:t>` elements within `<w:p>` (paragraph) boundaries
- Both `zip` and `quick-xml` are battle-tested crates (millions of downloads)
- For tables: parse `w:tbl > w:tr > w:tc > w:t` elements
- For headers/footers: read `word/header1.xml`, `word/footer1.xml` from ZIP

### Axum Multipart Upload
- Two-layer size limits: `DefaultBodyLimit::disable()` + `RequestBodyLimitLayer::new(N)` from `tower-http`
- For files under 20MB: `field.bytes().await` (buffered) is fine
- Security: sanitize filenames (UUID), validate content type, use temp directories
- `axum_typed_multipart` (2.3M downloads) provides type-safe derive macros but adds dependency

### Text Tokenization
- **`unicode-segmentation`** (326M downloads) — sentence and word splitting via Unicode UAX #29
- **`stop-words`** (921K downloads) — stopword lists for English and Norwegian
- **`rust-stemmers`** (16M downloads) — Snowball stemming for English and Norwegian
- **N-grams**: Use `slice::windows()` — no crate needed
- **TF-IDF**: Implement directly (50-100 lines) — no mature standalone crate exists

### Cargo Dependencies to Add
```toml
pdf-extract = "0.10"        # PDF text extraction
zip = "0.6"                 # DOCX unpacking (DOCX = ZIP)
quick-xml = "0.31"          # DOCX XML parsing
unicode-segmentation = "1"  # Sentence/word splitting
stop-words = "0.10"         # Stopword filtering
rust-stemmers = "1"         # Stemming (English + Norwegian)
```
