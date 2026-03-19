use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::features::extraction::validation::ValidationReport;

// ── Error Type ──────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid PDF: {0}")]
    InvalidPdf(String),

    #[error("No sections found in PDF")]
    NoSectionsFound,

    #[error("Page offset error: {0}")]
    PageOffsetError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

// ── Core Data Types ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractionConfig {
    pub page_offset_override: Option<i32>,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Markdown,
    Raw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub framework_id: String,
    pub source_pdf: String,
    pub extracted_at: DateTime<Utc>,
    pub sections: Vec<ExtractedSection>,
    pub page_offset_detected: i32,
    pub page_offset_source: PageOffsetSource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PageOffsetSource {
    Auto,
    Manual,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedSection {
    pub concept_code: String,
    pub concept_id: Option<String>,
    pub physical_page: usize,
    pub logical_page: usize,
    pub raw_text: String,
    pub subsections: Vec<Subsection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsection {
    pub kind: SubsectionKind,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubsectionKind {
    About,
    SuggestedActions,
    TransparencyQuestions,
    Resources,
    References,
}

// ── Extractor Trait ─────────────────────────────────────────

pub trait PdfExtractor: Send + Sync {
    /// Human-readable name for this extractor (e.g., "NIST AI RMF Playbook")
    fn name(&self) -> &str;

    /// The framework_id this extractor targets (e.g., "nist-ai-rmf")
    fn framework_id(&self) -> &str;

    /// Extract structured sections from the PDF at the given path.
    fn extract(
        &self,
        pdf_path: &Path,
        config: &ExtractionConfig,
    ) -> Result<ExtractionResult, ExtractionError>;

    /// Validate extracted data against the ontology concepts.
    fn validate(&self, result: &ExtractionResult, ontology_path: &Path) -> ValidationReport;
}

// ── Utility Functions ───────────────────────────────────────

/// Read all pages from a PDF file, returning (page_index, text) pairs.
/// Page indices are 0-based.
pub fn read_pdf_pages(path: &Path) -> Result<Vec<(usize, String)>, ExtractionError> {
    let bytes = std::fs::read(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ExtractionError::FileNotFound(path.display().to_string())
        } else {
            ExtractionError::IoError(e)
        }
    })?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| ExtractionError::InvalidPdf(e.to_string()))?;

    // pdf_extract returns all text concatenated with form-feed (\x0C) between pages
    let pages: Vec<(usize, String)> = text
        .split('\x0C')
        .enumerate()
        .map(|(i, page_text)| (i, page_text.to_string()))
        .collect();

    Ok(pages)
}

/// Build a code-to-ID lookup map from the ontology JSON file.
/// Call once and reuse the map for all resolve_concept_id lookups.
pub fn build_concept_code_map(
    ontology_path: &Path,
) -> Result<HashMap<String, String>, ExtractionError> {
    let data = std::fs::read_to_string(ontology_path)?;
    let json: serde_json::Value =
        serde_json::from_str(&data).map_err(|e| ExtractionError::InvalidPdf(e.to_string()))?;

    let concepts = json
        .get("concepts")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            ExtractionError::InvalidPdf("Missing 'concepts' array in ontology JSON".to_string())
        })?;

    let map = concepts
        .iter()
        .filter_map(|c| {
            let id = c.get("id")?.as_str()?.to_string();
            let concept_code = c.get("code")?.as_str()?.to_string();
            Some((concept_code, id))
        })
        .collect();

    Ok(map)
}

/// Resolve a concept code (e.g., "GOVERN 1.1") to its ontology concept ID
/// (e.g., "nist-ai-gv-1-1") using a pre-built lookup map.
///
/// Returns None if the code is not found.
pub fn resolve_concept_id(code: &str, code_map: &HashMap<String, String>) -> Option<String> {
    code_map.get(code).cloned()
}

// ── Tests ───────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -- ExtractionError tests --

    #[test]
    fn extraction_error_file_not_found_displays_message() {
        let err = ExtractionError::FileNotFound("/tmp/missing.pdf".to_string());
        assert!(err.to_string().contains("/tmp/missing.pdf"));
    }

    #[test]
    fn extraction_error_io_converts_from_std_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let err: ExtractionError = io_err.into();
        assert!(matches!(err, ExtractionError::IoError(_)));
    }

    #[test]
    fn extraction_error_each_variant_displays_human_readable() {
        let variants: Vec<ExtractionError> = vec![
            ExtractionError::FileNotFound("path".into()),
            ExtractionError::InvalidPdf("bad".into()),
            ExtractionError::NoSectionsFound,
            ExtractionError::PageOffsetError("fail".into()),
            ExtractionError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "io")),
        ];
        for v in variants {
            assert!(!v.to_string().is_empty());
        }
    }

    // -- resolve_concept_id tests --

    fn ontology_path() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("ontology-data/nist-ai-rmf.json")
    }

    fn code_map() -> HashMap<String, String> {
        build_concept_code_map(&ontology_path()).unwrap()
    }

    #[test]
    fn resolve_concept_id_govern_1_1() {
        let map = code_map();
        assert_eq!(resolve_concept_id("GOVERN 1.1", &map), Some("nist-ai-gv-1-1".to_string()));
    }

    #[test]
    fn resolve_concept_id_measure_2_3() {
        let map = code_map();
        assert_eq!(resolve_concept_id("MEASURE 2.3", &map), Some("nist-ai-ms-2-3".to_string()));
    }

    #[test]
    fn resolve_concept_id_unknown_returns_none() {
        let map = code_map();
        assert!(resolve_concept_id("GOVERN 99.99", &map).is_none());
    }

    #[test]
    fn resolve_concept_id_subcategory_resolves_to_subcategory_id() {
        let map = code_map();
        // Deviation from plan: "GOVERN 1" exists in ontology as a subcategory with a code field
        assert_eq!(resolve_concept_id("GOVERN 1", &map), Some("nist-ai-gv-1".to_string()));
    }

    #[test]
    fn resolve_concept_id_nonexistent_code_returns_none() {
        let map = code_map();
        assert!(resolve_concept_id("NONEXISTENT 99", &map).is_none());
    }

    #[test]
    fn resolve_concept_id_all_75_actions() {
        let path = ontology_path();
        let map = code_map();
        let data = std::fs::read_to_string(&path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let concepts = json["concepts"].as_array().unwrap();

        let actions: Vec<&serde_json::Value> = concepts
            .iter()
            .filter(|c| c.get("concept_type").and_then(|v| v.as_str()) == Some("action"))
            .collect();

        assert_eq!(actions.len(), 75, "Expected 75 action concepts");

        for action in &actions {
            let code = action["code"].as_str().expect("Action must have code field");
            let resolved = resolve_concept_id(code, &map);
            assert!(
                resolved.is_some(),
                "Failed to resolve concept ID for code: {code}"
            );
        }
    }

    // -- read_pdf_pages tests --

    #[test]
    fn read_pdf_pages_nonexistent_file_returns_error() {
        let result = read_pdf_pages(Path::new("/tmp/nonexistent_test_file.pdf"));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ExtractionError::FileNotFound(_)
        ));
    }
}
