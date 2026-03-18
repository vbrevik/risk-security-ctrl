use std::path::Path;

use serde::Serialize;
use tracing;

/// Maximum file size: 20MB
pub const MAX_FILE_SIZE: usize = 20 * 1024 * 1024;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Could not parse file: {0}")]
    CorruptFile(String),

    #[error("No text content found in document")]
    EmptyDocument,

    #[error("File too large: {size} bytes (max: {max})")]
    FileTooLarge { size: usize, max: usize },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// ============================================================================
// Document Types
// ============================================================================

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

impl ParsedDocument {
    pub fn new(full_text: String, sections: Vec<DocumentSection>) -> Self {
        let word_count = full_text.split_whitespace().count();
        let token_count_estimate = (word_count as f64 * 1.33) as usize;
        Self {
            full_text,
            sections,
            word_count,
            token_count_estimate,
        }
    }
}

// ============================================================================
// Document Parser
// ============================================================================

pub struct DocumentParser;

impl DocumentParser {
    /// Parse a file based on its extension (.pdf, .docx)
    pub fn parse(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        // Check file size before reading
        let metadata = std::fs::metadata(file_path)?;
        let size = metadata.len() as usize;
        if size > MAX_FILE_SIZE {
            return Err(ParsingError::FileTooLarge {
                size,
                max: MAX_FILE_SIZE,
            });
        }

        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        tracing::info!(
            "Parsing file: {}, size: {} bytes",
            file_path.display(),
            size
        );

        let start = std::time::Instant::now();

        let result = match ext.as_deref() {
            Some("pdf") => Self::parse_pdf(file_path),
            Some("docx") => Self::parse_docx(file_path),
            Some(other) => Err(ParsingError::UnsupportedFormat(other.to_string())),
            None => Err(ParsingError::UnsupportedFormat("no extension".to_string())),
        };

        match &result {
            Ok(doc) => {
                tracing::info!(
                    "Parse complete: {} words, {} sections, {:.2?} elapsed",
                    doc.word_count,
                    doc.sections.len(),
                    start.elapsed()
                );
            }
            Err(e) => {
                tracing::warn!("Parse failed: {}", e);
            }
        }

        result
    }

    /// Parse raw text input (normalize whitespace, split on blank lines)
    pub fn parse_text(text: &str) -> Result<ParsedDocument, ParsingError> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(ParsingError::EmptyDocument);
        }

        // Normalize: collapse 3+ newlines to 2, trim each line
        let normalized: String = trimmed
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join("\n");

        // Split on double newlines for sections
        let sections: Vec<DocumentSection> = normalized
            .split("\n\n")
            .filter(|s| !s.trim().is_empty())
            .map(|s| DocumentSection {
                heading: None,
                text: s.trim().to_string(),
                page_number: None,
            })
            .collect();

        if sections.is_empty() {
            return Err(ParsingError::EmptyDocument);
        }

        let full_text = sections
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ParsedDocument::new(full_text, sections))
    }

    /// Extract text from a PDF file, page by page
    pub fn parse_pdf(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        let pages = pdf_extract::extract_text_by_pages(file_path)
            .map_err(|e| ParsingError::CorruptFile(e.to_string()))?;

        let sections: Vec<DocumentSection> = pages
            .iter()
            .enumerate()
            .filter(|(_, text)| !text.trim().is_empty())
            .map(|(i, text)| DocumentSection {
                heading: Some(format!("Page {}", i + 1)),
                text: text.trim().to_string(),
                page_number: Some(i + 1),
            })
            .collect();

        if sections.is_empty() {
            return Err(ParsingError::EmptyDocument);
        }

        let full_text = sections
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ParsedDocument::new(full_text, sections))
    }

    /// Extract text from a DOCX file (ZIP archive with XML)
    pub fn parse_docx(file_path: &Path) -> Result<ParsedDocument, ParsingError> {
        let bytes = std::fs::read(file_path)?;
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|e| ParsingError::CorruptFile(e.to_string()))?;

        let mut xml_content = String::new();
        {
            let mut xml_file = archive
                .by_name("word/document.xml")
                .map_err(|e| ParsingError::CorruptFile(e.to_string()))?;
            std::io::Read::read_to_string(&mut xml_file, &mut xml_content)?;
        }

        // Parse XML to extract text
        let mut reader = quick_xml::Reader::from_str(&xml_content);
        let mut sections: Vec<DocumentSection> = Vec::new();
        let mut current_text = String::new();
        let mut current_section_text = String::new();
        let mut current_heading: Option<String> = None;
        let mut in_text_element = false;
        let mut next_para_is_heading = false;

        loop {
            match reader.read_event() {
                Ok(quick_xml::events::Event::Start(ref e))
                | Ok(quick_xml::events::Event::Empty(ref e)) => {
                    let local = e.local_name();
                    match local.as_ref() {
                        b"t" => in_text_element = true,
                        b"pStyle" => {
                            // Check if this is a heading style
                            for attr in e.attributes().flatten() {
                                if attr.key.local_name().as_ref() == b"val" {
                                    let val = String::from_utf8_lossy(&attr.value);
                                    if val.contains("Heading") {
                                        next_para_is_heading = true;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::End(ref e)) => {
                    let local = e.local_name();
                    match local.as_ref() {
                        b"t" => in_text_element = false,
                        b"p" => {
                            // End of paragraph
                            if next_para_is_heading && !current_text.trim().is_empty() {
                                // Push previous section if it has content
                                if !current_section_text.trim().is_empty() {
                                    sections.push(DocumentSection {
                                        heading: current_heading.take(),
                                        text: current_section_text.trim().to_string(),
                                        page_number: None,
                                    });
                                }
                                current_heading = Some(current_text.trim().to_string());
                                current_section_text.clear();
                            } else if !current_text.trim().is_empty() {
                                current_section_text.push_str(current_text.trim());
                                current_section_text.push('\n');
                            }
                            current_text.clear();
                            next_para_is_heading = false;
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) if in_text_element => {
                    if let Ok(text) = e.unescape() {
                        current_text.push_str(&text);
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => return Err(ParsingError::CorruptFile(e.to_string())),
                _ => {}
            }
        }

        // Push final section
        if !current_section_text.trim().is_empty() {
            sections.push(DocumentSection {
                heading: current_heading.take(),
                text: current_section_text.trim().to_string(),
                page_number: None,
            });
        }

        if sections.is_empty() {
            return Err(ParsingError::EmptyDocument);
        }

        let full_text = sections
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ParsedDocument::new(full_text, sections))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // --- ParsingError tests ---

    #[test]
    fn parsing_error_display_messages() {
        assert_eq!(
            format!("{}", ParsingError::UnsupportedFormat("xlsx".into())),
            "Unsupported format: xlsx"
        );
        assert_eq!(
            format!("{}", ParsingError::CorruptFile("bad header".into())),
            "Could not parse file: bad header"
        );
        assert_eq!(
            format!("{}", ParsingError::EmptyDocument),
            "No text content found in document"
        );
        assert_eq!(
            format!(
                "{}",
                ParsingError::FileTooLarge {
                    size: 25_000_000,
                    max: 20_000_000
                }
            ),
            "File too large: 25000000 bytes (max: 20000000)"
        );
    }

    #[test]
    fn parsing_error_io_conversion() {
        fn try_io() -> Result<(), ParsingError> {
            let _f = std::fs::File::open("/nonexistent/path/that/does/not/exist")?;
            Ok(())
        }
        assert!(matches!(try_io(), Err(ParsingError::IoError(_))));
    }

    // --- ParsedDocument tests ---

    #[test]
    fn parsed_document_word_count() {
        let doc = ParsedDocument::new("The quick brown fox jumps".to_string(), vec![]);
        assert_eq!(doc.word_count, 5);
        assert_eq!(doc.token_count_estimate, (5.0 * 1.33) as usize);
    }

    #[test]
    fn parsed_document_empty_text() {
        let doc = ParsedDocument::new("".to_string(), vec![]);
        assert_eq!(doc.word_count, 0);
        assert_eq!(doc.token_count_estimate, 0);
    }

    // --- parse_text tests ---

    #[test]
    fn parse_text_splits_on_blank_lines() {
        let input = "First paragraph here.\n\nSecond paragraph here.\n\nThird paragraph.";
        let result = DocumentParser::parse_text(input).unwrap();
        assert_eq!(result.sections.len(), 3);
        assert!(result.sections[0].text.contains("First paragraph"));
        assert!(result.sections[1].text.contains("Second paragraph"));
        assert!(result.sections[2].text.contains("Third paragraph"));
        assert!(result.word_count > 0);
    }

    #[test]
    fn parse_text_empty_string() {
        assert!(matches!(
            DocumentParser::parse_text(""),
            Err(ParsingError::EmptyDocument)
        ));
    }

    #[test]
    fn parse_text_whitespace_only() {
        assert!(matches!(
            DocumentParser::parse_text("   \n\n  \t  "),
            Err(ParsingError::EmptyDocument)
        ));
    }

    // --- parse dispatch tests ---

    #[test]
    fn parse_unsupported_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("some_file.txt");
        std::fs::write(&path, "hello").unwrap();
        assert!(matches!(
            DocumentParser::parse(&path),
            Err(ParsingError::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn parse_case_insensitive_extension() {
        let path = Path::new("/tmp/nonexistent_test_file.PDF");
        let result = DocumentParser::parse(path);
        // Should attempt PDF parsing, not return UnsupportedFormat
        assert!(!matches!(result, Err(ParsingError::UnsupportedFormat(_))));
    }

    #[test]
    fn parse_file_too_large() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("huge.pdf");
        let mut f = std::fs::File::create(&path).unwrap();
        let chunk = vec![0u8; 1024 * 1024];
        for _ in 0..21 {
            f.write_all(&chunk).unwrap();
        }
        drop(f);

        assert!(matches!(
            DocumentParser::parse(&path),
            Err(ParsingError::FileTooLarge { .. })
        ));
    }

    // --- DOCX tests ---

    fn create_test_docx(dir: &tempfile::TempDir, xml_content: &str) -> std::path::PathBuf {
        use std::io::Write as _;
        let path = dir.path().join("test.docx");
        let file = std::fs::File::create(&path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("word/document.xml", options).unwrap();
        zip.write_all(xml_content.as_bytes()).unwrap();
        zip.finish().unwrap();
        path
    }

    #[test]
    fn parse_docx_extracts_text() {
        let dir = tempfile::tempdir().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p><w:r><w:t>Hello world</w:t></w:r></w:p>
                <w:p><w:r><w:t>Second paragraph</w:t></w:r></w:p>
            </w:body></w:document>"#;
        let path = create_test_docx(&dir, xml);
        let result = DocumentParser::parse_docx(&path).unwrap();
        assert!(result.full_text.contains("Hello world"));
        assert!(result.full_text.contains("Second paragraph"));
        assert!(result.word_count >= 4);
    }

    #[test]
    fn parse_docx_detects_headings() {
        let dir = tempfile::tempdir().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Introduction</w:t></w:r></w:p>
                <w:p><w:r><w:t>Body text here.</w:t></w:r></w:p>
                <w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>Details</w:t></w:r></w:p>
                <w:p><w:r><w:t>More details.</w:t></w:r></w:p>
            </w:body></w:document>"#;
        let path = create_test_docx(&dir, xml);
        let result = DocumentParser::parse_docx(&path).unwrap();
        assert_eq!(result.sections.len(), 2);
        assert_eq!(result.sections[0].heading, Some("Introduction".to_string()));
        assert!(result.sections[0].text.contains("Body text here"));
        assert_eq!(result.sections[1].heading, Some("Details".to_string()));
        assert!(result.sections[1].text.contains("More details"));
    }

    #[test]
    fn parse_docx_corrupt_zip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("corrupt.docx");
        std::fs::write(&path, b"this is not a zip file").unwrap();
        assert!(matches!(
            DocumentParser::parse_docx(&path),
            Err(ParsingError::CorruptFile(_))
        ));
    }

    #[test]
    fn parse_docx_empty_document() {
        let dir = tempfile::tempdir().unwrap();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body><w:p><w:r></w:r></w:p></w:body></w:document>"#;
        let path = create_test_docx(&dir, xml);
        assert!(matches!(
            DocumentParser::parse_docx(&path),
            Err(ParsingError::EmptyDocument)
        ));
    }
}
