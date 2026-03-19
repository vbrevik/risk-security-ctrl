I have all the context I need. Let me now generate the section content.

# Section 1: Upload Handler

## Overview

This section creates a new module `backend/src/features/analysis/upload.rs` that provides file upload validation and storage utilities. The route handler (section 03) will call these functions to process multipart uploads. This module has no dependencies on other sections and can be implemented in parallel with section 02 (parser refinements).

## Background

The analysis feature already has a parser (`parser.rs`) that can handle PDF and DOCX files, a `ParsingError` enum for error reporting, and an `InputType` enum in `models.rs` with variants `Text`, `Pdf`, and `Docx`. The upload module provides the bridge between receiving raw file bytes from an HTTP request and handing a validated, persisted file path to the parser.

Key existing types this module uses:

- `super::parser::ParsingError` -- error type with variants `UnsupportedFormat(String)`, `FileTooLarge { size, max }`, `IoError`, `CorruptFile(String)`, `EmptyDocument`
- `super::models::InputType` -- enum with `Text`, `Pdf`, `Docx` variants
- `axum::extract::multipart::Field` -- for streaming upload support
- `uuid::Uuid` -- already in `Cargo.toml` with `v4` feature

## Files to Create or Modify

| File | Action |
|------|--------|
| `backend/src/features/analysis/upload.rs` | **Create** -- new module |
| `backend/src/features/analysis/mod.rs` | **Modify** -- add `pub mod upload;` line |

## Tests First

Place all tests inside `upload.rs` in a `#[cfg(test)] mod tests` block. These tests should be written before the implementation functions. Each test stub below describes the exact behavior to verify.

```rust
// backend/src/features/analysis/upload.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_file_too_large() {
        // Call validate_upload with size = MAX_FILE_SIZE + 1
        // Assert result is Err(ParsingError::FileTooLarge { .. })
    }

    #[test]
    fn reject_unknown_extension() {
        // Call validate_upload with filename "report.exe", size 100, header b"\x00\x00\x00\x00"
        // Assert result is Err(ParsingError::UnsupportedFormat(_))
    }

    #[test]
    fn accept_pdf_extension() {
        // Call validate_upload with filename "report.pdf", size 1000, header b"%PDF-1.4"
        // Assert result is Ok(InputType::Pdf)
    }

    #[test]
    fn accept_docx_case_insensitive() {
        // Call validate_upload with filename "report.DOCX", size 1000, header b"PK\x03\x04"
        // Assert result is Ok(InputType::Docx)
    }

    #[test]
    fn reject_path_traversal_analysis_id() {
        // Call save_upload with analysis_id "../../../etc", any filename, any data
        // Assert result is Err (should reject non-UUID analysis_id)
    }

    #[test]
    fn validate_pdf_magic_bytes() {
        // Call validate_upload with filename "doc.pdf", valid size, header starting with b"%PDF"
        // Assert Ok(InputType::Pdf)
    }

    #[test]
    fn validate_zip_magic_bytes() {
        // Call validate_upload with filename "doc.docx", valid size, header starting with b"PK" (0x50, 0x4B)
        // Assert Ok(InputType::Docx)
    }

    #[test]
    fn reject_mismatched_magic_bytes() {
        // Call validate_upload with filename "doc.pdf", valid size, header starting with b"PK\x03\x04"
        // This is a ZIP header but filename says PDF -- should return Err(ParsingError::CorruptFile(_))
    }

    #[test]
    fn save_upload_creates_directory_and_file() {
        // Use tempfile to override UPLOAD_DIR or pass a temp directory
        // Call save_upload with a valid UUID analysis_id, filename "test.pdf", some bytes
        // Assert the returned PathBuf exists, has .pdf extension, and parent directory contains the analysis_id
        // Assert file contents match the input bytes
    }
}
```

## Implementation Details

### Constants

Define at the top of `upload.rs`:

- `MAX_FILE_SIZE: u64 = 20 * 1024 * 1024` (20 MB). Note: `parser.rs` already defines `MAX_FILE_SIZE` as `usize`. The upload module uses `u64` because multipart content-length values are `u64`. Both enforce the same 20 MB limit at different layers.
- `ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx"]`
- `UPLOAD_DIR: &str = "uploads"`

### Function: `validate_upload`

```rust
/// Validate file upload: check size, extension, and magic bytes.
/// Returns the detected InputType on success.
pub fn validate_upload(filename: &str, size: u64, header: &[u8]) -> Result<InputType, ParsingError>
```

Logic:
1. If `size > MAX_FILE_SIZE`, return `ParsingError::FileTooLarge { size: size as usize, max: MAX_FILE_SIZE as usize }`.
2. Extract the file extension by splitting `filename` on `.` and taking the last segment. Convert to lowercase. If no extension or extension is not in `ALLOWED_EXTENSIONS`, return `ParsingError::UnsupportedFormat`.
3. Map the extension to an `InputType`: `"pdf"` to `InputType::Pdf`, `"docx"` to `InputType::Docx`.
4. Validate magic bytes against the detected type:
   - For `InputType::Pdf`: check that `header` starts with `b"%PDF"` (bytes `0x25, 0x50, 0x44, 0x46`).
   - For `InputType::Docx`: check that `header` starts with `b"PK"` (bytes `0x50, 0x4B`) -- DOCX files are ZIP archives.
   - If the header does not match, return `ParsingError::CorruptFile("file header does not match expected format".into())`.
5. Return `Ok(input_type)`.

### Function: `save_upload`

```rust
/// Save uploaded file bytes to disk under a UUID-namespaced directory.
/// Returns the path to the saved file.
pub fn save_upload(analysis_id: &str, filename: &str, data: &[u8]) -> Result<PathBuf, ParsingError>
```

Logic:
1. Validate `analysis_id` is a valid UUID by calling `uuid::Uuid::parse_str(analysis_id)`. If it fails, return `ParsingError::CorruptFile("invalid analysis ID format".into())`. This prevents path traversal attacks since UUIDs contain only hex digits and hyphens.
2. Extract the file extension from `filename` (same as in `validate_upload`). Default to empty string if none.
3. Generate a new UUID for the stored filename: `format!("{}.{}", Uuid::new_v4(), ext)`.
4. Construct the directory path: `PathBuf::from(UPLOAD_DIR).join(analysis_id)`.
5. Create the directory with `std::fs::create_dir_all`.
6. Write `data` to the file path using `std::fs::write`.
7. Return the full file path.

For testability, consider making the base directory configurable (either via a parameter or by extracting the path construction into a helper). The `save_upload_creates_directory_and_file` test can either use a temp directory by modifying the function signature to accept a base path, or by setting up `UPLOAD_DIR` to point to a temp directory.

A practical approach: add an internal helper `save_upload_to(base_dir: &Path, analysis_id: &str, filename: &str, data: &[u8])` that the public `save_upload` delegates to with `Path::new(UPLOAD_DIR)`. Tests call the `_to` variant directly.

### Function: `stream_upload_to_file`

```rust
/// Stream a multipart field to disk chunk by chunk. For large files.
/// Returns the path to the saved file.
pub async fn stream_upload_to_file(
    analysis_id: &str,
    filename: &str,
    mut field: axum::extract::multipart::Field<'_>,
) -> Result<PathBuf, ParsingError>
```

Logic:
1. Validate `analysis_id` as UUID (same as `save_upload`).
2. Extract extension, generate UUID filename, create directory -- same pattern as `save_upload`.
3. Open a file with `tokio::fs::File::create`.
4. Loop over `field.chunk().await` -- each call yields an `Option<Bytes>`:
   - `Some(chunk)` -- write chunk to file with `tokio::io::AsyncWriteExt::write_all`.
   - `None` -- break, upload complete.
5. Return the file path.

This function is harder to unit test because it requires constructing an `axum::extract::multipart::Field`. It will be exercised primarily by integration tests in section 04. A note in the test suite explaining this is sufficient.

### Module Registration

Add to `backend/src/features/analysis/mod.rs`:

```rust
pub mod upload;
```

This line should be added alongside the existing module declarations (`engine`, `matcher`, `models`, `parser`, `routes`, `tokenizer`).

## Dependencies on Other Sections

- **Section 03 (Route Completion)** depends on this module -- the route handler will call `validate_upload`, `save_upload`, and/or `stream_upload_to_file`.
- **Section 02 (Parser Refinements)** is independent and can be implemented in parallel.
- This section has **no dependencies** on other sections.

## Crate Dependencies

No new crate dependencies are needed. All required crates are already in `backend/Cargo.toml`:
- `uuid` (v1, with `v4` feature) for UUID generation and validation
- `axum` (v0.7, with `multipart` feature) for `Field` type
- `tokio` (v1, with `full` feature) for async file I/O
- `tempfile` (v3, dev-dependency) for tests