# Research: 02-document-parsing

## Codebase Findings

- **Axum multipart already enabled** in Cargo.toml (`features = ["macros", "multipart"]`)
- **Compliance feature has upload pattern** at `POST /api/compliance/.../evidence/upload` — creates dirs, saves with UUID naming, captures mime/size
- **No PDF/DOCX crates** yet in Cargo.toml
- **Analysis module exists** with `models.rs` and `engine.rs` but no `parser.rs` or `routes.rs`
- **No `uploads/` directory** — compliance creates `uploads/evidence/` on-demand
- **Error bridge missing** — `AnalysisError` doesn't convert to `AppError` yet
- **Default body limit** is 2MB — need to raise for 20MB file uploads

## PDF Extraction: pdf-extract v0.10

**API:**
- `extract_text(path)` → `Result<String, OutputError>`
- `extract_text_from_mem(bytes)` → `Result<String, OutputError>` (best for uploads)
- `extract_text_by_pages(path)` → `Result<Vec<String>, OutputError>` (page-by-page)

**Limitations:** No OCR (scanned PDFs return empty), encoding issues with unusual fonts, no structured output. Pure Rust, no C deps.

## DOCX Extraction: Manual ZIP + quick-xml (RECOMMENDED)

**Key finding:** `docx-rs` is primarily a WRITER. The `docx` crate (v1.1.2) is unmaintained since 2020.

**Recommended approach:** Parse DOCX as ZIP archive, extract `word/document.xml`, use `quick-xml` to walk `<w:t>` text nodes and `<w:p>` paragraph boundaries. ~30 lines, fully controlled, well-maintained dependencies.

**Dependencies:** `zip = "2"` + `quick-xml = "0.37"` (both actively maintained)

**Heading detection:** DOCX headings have `<w:pStyle w:val="Heading1"/>` in paragraph properties — can detect for section splitting.

## Axum Multipart Upload

- Default body limit 2MB — override with `DefaultBodyLimit::max(20 * 1024 * 1024)` on analysis routes
- `field.bytes().await` reads entire file to memory (fine for ≤20MB)
- Sanitize filenames with `Path::file_name()` to prevent path traversal
- `field.file_name()` / `field.content_type()` for metadata

## New Cargo Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `pdf-extract` | `0.10` | PDF text extraction |
| `zip` | `2` | DOCX ZIP archive reading |
| `quick-xml` | `0.37` | DOCX XML parsing |

## Testing

Existing test setup: `#[cfg(test)] mod tests` with `cargo test`. Dev deps include `tokio-test` and `tower`.
For parser tests: create small test PDF/DOCX fixtures in `backend/tests/fixtures/`.
