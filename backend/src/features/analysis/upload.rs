use std::path::{Path, PathBuf};

use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use super::models::InputType;
use super::parser::ParsingError;

/// Maximum file size: 20MB
pub const MAX_FILE_SIZE: u64 = 20 * 1024 * 1024;

/// Allowed file extensions
pub const ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx"];

/// Default upload directory
pub const UPLOAD_DIR: &str = "uploads";

/// Validate file upload: check size, extension, and magic bytes.
/// Returns the detected InputType on success.
pub fn validate_upload(filename: &str, size: u64, header: &[u8]) -> Result<InputType, ParsingError> {
    // Check size
    if size > MAX_FILE_SIZE {
        return Err(ParsingError::FileTooLarge {
            size: size as usize,
            max: MAX_FILE_SIZE as usize,
        });
    }

    // Extract and validate extension
    let ext = filename
        .rsplit('.')
        .next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(ParsingError::UnsupportedFormat(format!(
            "unsupported file extension: .{}",
            ext
        )));
    }

    // Map extension to InputType
    let input_type = match ext.as_str() {
        "pdf" => InputType::Pdf,
        "docx" => InputType::Docx,
        _ => unreachable!(), // already validated above
    };

    // Validate magic bytes
    match input_type {
        InputType::Pdf => {
            if !header.starts_with(b"%PDF") {
                return Err(ParsingError::CorruptFile(
                    "file header does not match expected PDF format".into(),
                ));
            }
        }
        InputType::Docx => {
            if !header.starts_with(b"PK") {
                return Err(ParsingError::CorruptFile(
                    "file header does not match expected DOCX/ZIP format".into(),
                ));
            }
        }
        _ => {}
    }

    Ok(input_type)
}

/// Save uploaded file bytes to disk under a UUID-namespaced directory.
/// Delegates to `save_upload_to` with the default UPLOAD_DIR.
pub fn save_upload(
    analysis_id: &str,
    filename: &str,
    data: &[u8],
) -> Result<PathBuf, ParsingError> {
    save_upload_to(Path::new(UPLOAD_DIR), analysis_id, filename, data)
}

/// Save uploaded file bytes to a specified base directory.
/// Validates analysis_id is UUID to prevent path traversal.
pub fn save_upload_to(
    base_dir: &Path,
    analysis_id: &str,
    filename: &str,
    data: &[u8],
) -> Result<PathBuf, ParsingError> {
    // Validate analysis_id is a valid UUID
    Uuid::parse_str(analysis_id).map_err(|_| {
        ParsingError::CorruptFile("invalid analysis ID format".into())
    })?;

    // Extract extension
    let ext = filename
        .rsplit('.')
        .next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    // Generate unique filename
    let stored_name = if ext.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        format!("{}.{}", Uuid::new_v4(), ext)
    };

    // Create directory and write file
    let dir = base_dir.join(analysis_id);
    std::fs::create_dir_all(&dir)?;

    let file_path = dir.join(&stored_name);
    std::fs::write(&file_path, data)?;

    Ok(file_path)
}

/// Stream a multipart field to disk chunk by chunk.
/// For large files that shouldn't be buffered entirely in memory.
pub async fn stream_upload_to_file(
    analysis_id: &str,
    filename: &str,
    mut field: axum::extract::multipart::Field<'_>,
) -> Result<PathBuf, ParsingError> {
    // Validate analysis_id
    Uuid::parse_str(analysis_id).map_err(|_| {
        ParsingError::CorruptFile("invalid analysis ID format".into())
    })?;

    let ext = filename
        .rsplit('.')
        .next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let stored_name = if ext.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        format!("{}.{}", Uuid::new_v4(), ext)
    };

    let dir = PathBuf::from(UPLOAD_DIR).join(analysis_id);
    tokio::fs::create_dir_all(&dir).await?;

    let file_path = dir.join(&stored_name);
    let mut file = tokio::fs::File::create(&file_path).await?;

    while let Some(chunk) = field.chunk().await.map_err(|e| {
        ParsingError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })? {
        file.write_all(&chunk).await?;
    }

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_file_too_large() {
        let result = validate_upload("report.pdf", MAX_FILE_SIZE + 1, b"%PDF-1.4");
        assert!(matches!(result, Err(ParsingError::FileTooLarge { .. })));
    }

    #[test]
    fn reject_unknown_extension() {
        let result = validate_upload("report.exe", 100, b"\x00\x00\x00\x00");
        assert!(matches!(result, Err(ParsingError::UnsupportedFormat(_))));
    }

    #[test]
    fn accept_pdf_extension() {
        let result = validate_upload("report.pdf", 1000, b"%PDF-1.4");
        assert!(matches!(result, Ok(InputType::Pdf)));
    }

    #[test]
    fn accept_docx_case_insensitive() {
        let result = validate_upload("report.DOCX", 1000, b"PK\x03\x04");
        assert!(matches!(result, Ok(InputType::Docx)));
    }

    #[test]
    fn reject_path_traversal_analysis_id() {
        let result = save_upload_to(
            Path::new("/tmp"),
            "../../../etc",
            "test.pdf",
            b"data",
        );
        assert!(result.is_err());
    }

    #[test]
    fn validate_pdf_magic_bytes() {
        let result = validate_upload("doc.pdf", 500, b"%PDF-1.7 some content");
        assert!(matches!(result, Ok(InputType::Pdf)));
    }

    #[test]
    fn validate_zip_magic_bytes() {
        let result = validate_upload("doc.docx", 500, b"PK\x03\x04 zip content");
        assert!(matches!(result, Ok(InputType::Docx)));
    }

    #[test]
    fn reject_mismatched_magic_bytes() {
        // PDF extension but ZIP magic bytes
        let result = validate_upload("doc.pdf", 500, b"PK\x03\x04");
        assert!(matches!(result, Err(ParsingError::CorruptFile(_))));
    }

    #[test]
    fn save_upload_creates_directory_and_file() {
        let dir = tempfile::tempdir().unwrap();
        let analysis_id = Uuid::new_v4().to_string();
        let data = b"test PDF content";

        let result = save_upload_to(dir.path(), &analysis_id, "test.pdf", data);
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.exists());
        assert_eq!(path.extension().unwrap(), "pdf");
        assert!(path.parent().unwrap().ends_with(&analysis_id));
        assert_eq!(std::fs::read(&path).unwrap(), data);
    }
}
