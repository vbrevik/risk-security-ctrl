diff --git a/backend/src/features/extraction/cli.rs b/backend/src/features/extraction/cli.rs
new file mode 100644
index 0000000..c0994c2
--- /dev/null
+++ b/backend/src/features/extraction/cli.rs
@@ -0,0 +1 @@
+// CLI integration — implemented in section-05
diff --git a/backend/src/features/extraction/extractor.rs b/backend/src/features/extraction/extractor.rs
new file mode 100644
index 0000000..11447f1
--- /dev/null
+++ b/backend/src/features/extraction/extractor.rs
@@ -0,0 +1,270 @@
+use std::collections::HashMap;
+use std::path::Path;
+
+use chrono::{DateTime, Utc};
+use serde::{Deserialize, Serialize};
+
+use crate::features::extraction::validation::ValidationReport;
+
+// ── Error Type ──────────────────────────────────────────────
+
+#[derive(Debug, thiserror::Error)]
+pub enum ExtractionError {
+    #[error("File not found: {0}")]
+    FileNotFound(String),
+
+    #[error("Invalid PDF: {0}")]
+    InvalidPdf(String),
+
+    #[error("No sections found in PDF")]
+    NoSectionsFound,
+
+    #[error("Page offset error: {0}")]
+    PageOffsetError(String),
+
+    #[error("I/O error: {0}")]
+    IoError(#[from] std::io::Error),
+}
+
+// ── Core Data Types ─────────────────────────────────────────
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub struct ExtractionConfig {
+    pub page_offset_override: Option<i32>,
+    pub output_format: OutputFormat,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub enum OutputFormat {
+    Json,
+    Markdown,
+    Raw,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub struct ExtractionResult {
+    pub framework_id: String,
+    pub source_pdf: String,
+    pub extracted_at: DateTime<Utc>,
+    pub sections: Vec<ExtractedSection>,
+    pub page_offset_detected: i32,
+    pub page_offset_source: PageOffsetSource,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub enum PageOffsetSource {
+    Auto,
+    Manual,
+    Default,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub struct ExtractedSection {
+    pub concept_code: String,
+    pub concept_id: Option<String>,
+    pub physical_page: usize,
+    pub logical_page: usize,
+    pub raw_text: String,
+    pub subsections: Vec<Subsection>,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub struct Subsection {
+    pub kind: SubsectionKind,
+    pub text: String,
+}
+
+#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
+pub enum SubsectionKind {
+    About,
+    SuggestedActions,
+    TransparencyQuestions,
+    Resources,
+    References,
+}
+
+// ── Extractor Trait ─────────────────────────────────────────
+
+pub trait PdfExtractor: Send + Sync {
+    /// Human-readable name for this extractor (e.g., "NIST AI RMF Playbook")
+    fn name(&self) -> &str;
+
+    /// The framework_id this extractor targets (e.g., "nist-ai-rmf")
+    fn framework_id(&self) -> &str;
+
+    /// Extract structured sections from the PDF at the given path.
+    fn extract(
+        &self,
+        pdf_path: &Path,
+        config: &ExtractionConfig,
+    ) -> Result<ExtractionResult, ExtractionError>;
+
+    /// Validate extracted data against the ontology concepts.
+    fn validate(&self, result: &ExtractionResult, ontology_path: &Path) -> ValidationReport;
+}
+
+// ── Utility Functions ───────────────────────────────────────
+
+/// Read all pages from a PDF file, returning (page_index, text) pairs.
+/// Page indices are 0-based.
+pub fn read_pdf_pages(path: &Path) -> Result<Vec<(usize, String)>, ExtractionError> {
+    if !path.exists() {
+        return Err(ExtractionError::FileNotFound(
+            path.display().to_string(),
+        ));
+    }
+
+    let bytes = std::fs::read(path)?;
+
+    let text = pdf_extract::extract_text_from_mem(&bytes)
+        .map_err(|e| ExtractionError::InvalidPdf(e.to_string()))?;
+
+    // pdf_extract returns all text concatenated with form-feed (\x0C) between pages
+    let pages: Vec<(usize, String)> = text
+        .split('\x0C')
+        .enumerate()
+        .map(|(i, page_text)| (i, page_text.to_string()))
+        .collect();
+
+    Ok(pages)
+}
+
+/// Resolve a concept code (e.g., "GOVERN 1.1") to its ontology concept ID
+/// (e.g., "nist-ai-gv-1-1") by loading and searching the ontology JSON file.
+///
+/// Returns None if the code is not found in the ontology.
+pub fn resolve_concept_id(code: &str, ontology_path: &Path) -> Option<String> {
+    let data = std::fs::read_to_string(ontology_path).ok()?;
+    let json: serde_json::Value = serde_json::from_str(&data).ok()?;
+
+    let concepts = json.get("concepts")?.as_array()?;
+
+    let code_map: HashMap<String, String> = concepts
+        .iter()
+        .filter_map(|c| {
+            let id = c.get("id")?.as_str()?.to_string();
+            let concept_code = c.get("code")?.as_str()?.to_string();
+            Some((concept_code, id))
+        })
+        .collect();
+
+    code_map.get(code).cloned()
+}
+
+// ── Tests ───────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    // -- ExtractionError tests --
+
+    #[test]
+    fn extraction_error_file_not_found_displays_message() {
+        let err = ExtractionError::FileNotFound("/tmp/missing.pdf".to_string());
+        assert!(err.to_string().contains("/tmp/missing.pdf"));
+    }
+
+    #[test]
+    fn extraction_error_io_converts_from_std_io() {
+        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
+        let err: ExtractionError = io_err.into();
+        assert!(matches!(err, ExtractionError::IoError(_)));
+    }
+
+    #[test]
+    fn extraction_error_each_variant_displays_human_readable() {
+        let variants: Vec<ExtractionError> = vec![
+            ExtractionError::FileNotFound("path".into()),
+            ExtractionError::InvalidPdf("bad".into()),
+            ExtractionError::NoSectionsFound,
+            ExtractionError::PageOffsetError("fail".into()),
+            ExtractionError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "io")),
+        ];
+        for v in variants {
+            assert!(!v.to_string().is_empty());
+        }
+    }
+
+    // -- resolve_concept_id tests --
+
+    fn ontology_path() -> std::path::PathBuf {
+        // Navigate from backend/ to workspace root
+        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
+            .parent()
+            .unwrap()
+            .join("ontology-data/nist-ai-rmf.json")
+    }
+
+    #[test]
+    fn resolve_concept_id_govern_1_1() {
+        let path = ontology_path();
+        let result = resolve_concept_id("GOVERN 1.1", &path);
+        assert_eq!(result, Some("nist-ai-gv-1-1".to_string()));
+    }
+
+    #[test]
+    fn resolve_concept_id_measure_2_3() {
+        let path = ontology_path();
+        let result = resolve_concept_id("MEASURE 2.3", &path);
+        assert_eq!(result, Some("nist-ai-ms-2-3".to_string()));
+    }
+
+    #[test]
+    fn resolve_concept_id_unknown_returns_none() {
+        let path = ontology_path();
+        let result = resolve_concept_id("GOVERN 99.99", &path);
+        assert!(result.is_none());
+    }
+
+    #[test]
+    fn resolve_concept_id_subcategory_resolves_to_subcategory_id() {
+        let path = ontology_path();
+        // "GOVERN 1" is a subcategory — it has a code and resolves
+        let result = resolve_concept_id("GOVERN 1", &path);
+        assert_eq!(result, Some("nist-ai-gv-1".to_string()));
+    }
+
+    #[test]
+    fn resolve_concept_id_nonexistent_code_returns_none() {
+        let path = ontology_path();
+        let result = resolve_concept_id("NONEXISTENT 99", &path);
+        assert!(result.is_none());
+    }
+
+    #[test]
+    fn resolve_concept_id_all_75_actions() {
+        let path = ontology_path();
+        let data = std::fs::read_to_string(&path).unwrap();
+        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
+        let concepts = json["concepts"].as_array().unwrap();
+
+        let actions: Vec<&serde_json::Value> = concepts
+            .iter()
+            .filter(|c| c.get("concept_type").and_then(|v| v.as_str()) == Some("action"))
+            .collect();
+
+        assert_eq!(actions.len(), 75, "Expected 75 action concepts");
+
+        for action in &actions {
+            let code = action["code"].as_str().expect("Action must have code field");
+            let resolved = resolve_concept_id(code, &path);
+            assert!(
+                resolved.is_some(),
+                "Failed to resolve concept ID for code: {code}"
+            );
+        }
+    }
+
+    // -- read_pdf_pages tests --
+
+    #[test]
+    fn read_pdf_pages_nonexistent_file_returns_error() {
+        let result = read_pdf_pages(Path::new("/tmp/nonexistent_test_file.pdf"));
+        assert!(result.is_err());
+        assert!(matches!(
+            result.unwrap_err(),
+            ExtractionError::FileNotFound(_)
+        ));
+    }
+}
diff --git a/backend/src/features/extraction/mod.rs b/backend/src/features/extraction/mod.rs
new file mode 100644
index 0000000..a466878
--- /dev/null
+++ b/backend/src/features/extraction/mod.rs
@@ -0,0 +1,5 @@
+pub mod cli;
+pub mod extractor;
+pub mod page_offset;
+pub mod playbook;
+pub mod validation;
diff --git a/backend/src/features/extraction/page_offset.rs b/backend/src/features/extraction/page_offset.rs
new file mode 100644
index 0000000..6be1f02
--- /dev/null
+++ b/backend/src/features/extraction/page_offset.rs
@@ -0,0 +1 @@
+// Page offset detection — implemented in section-02
diff --git a/backend/src/features/extraction/playbook/mod.rs b/backend/src/features/extraction/playbook/mod.rs
new file mode 100644
index 0000000..eac71aa
--- /dev/null
+++ b/backend/src/features/extraction/playbook/mod.rs
@@ -0,0 +1 @@
+// Playbook extractor — implemented in section-03
diff --git a/backend/src/features/extraction/validation.rs b/backend/src/features/extraction/validation.rs
new file mode 100644
index 0000000..60808e5
--- /dev/null
+++ b/backend/src/features/extraction/validation.rs
@@ -0,0 +1,11 @@
+use serde::{Deserialize, Serialize};
+
+/// Report from validating extracted data against the ontology.
+#[derive(Debug, Clone, Serialize, Deserialize)]
+pub struct ValidationReport {
+    pub total_expected: usize,
+    pub total_extracted: usize,
+    pub missing_concepts: Vec<String>,
+    pub unmatched_sections: Vec<String>,
+    pub warnings: Vec<String>,
+}
diff --git a/backend/src/features/mod.rs b/backend/src/features/mod.rs
index 9cfc85a..4381f22 100644
--- a/backend/src/features/mod.rs
+++ b/backend/src/features/mod.rs
@@ -1,5 +1,6 @@
 pub mod analysis;
 pub mod auth;
 pub mod compliance;
+pub mod extraction;
 pub mod ontology;
 pub mod reports;
diff --git a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/contracts/section-01-contract.md b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/contracts/section-01-contract.md
new file mode 100644
index 0000000..3682ca9
--- /dev/null
+++ b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/contracts/section-01-contract.md
@@ -0,0 +1,33 @@
+# Section 01 Prompt Contract: Extractor Trait and Types
+
+## GOAL
+Create the foundational extraction module with PdfExtractor trait, core data types, error types, and utility functions (read_pdf_pages, resolve_concept_id).
+
+## CONTEXT
+First section of playbook enrichment data extraction. Establishes types that all subsequent sections depend on. Must follow existing feature-module pattern.
+
+## CONSTRAINTS
+- Follow feature-based module pattern at `backend/src/features/extraction/`
+- Use thiserror for error types, serde for serialization
+- PdfExtractor trait must be synchronous (CPU-bound work)
+- resolve_concept_id must use ontology JSON as source of truth, not algorithmic prefix conversion
+- All types need Debug, Clone, Serialize, Deserialize where appropriate
+- Stub modules for page_offset, playbook, validation, cli must compile
+
+## FORMAT
+### Files to create:
+- `backend/src/features/extraction/mod.rs`
+- `backend/src/features/extraction/extractor.rs` (trait + types + tests)
+- `backend/src/features/extraction/page_offset.rs` (stub)
+- `backend/src/features/extraction/playbook/mod.rs` (stub)
+- `backend/src/features/extraction/validation.rs` (stub)
+- `backend/src/features/extraction/cli.rs` (stub)
+
+### Files to modify:
+- `backend/src/features/mod.rs` (add `pub mod extraction;`)
+
+## FAILURE CONDITIONS
+- SHALL NOT skip tests for ExtractionError, resolve_concept_id, or read_pdf_pages
+- SHALL NOT use algorithmic prefix conversion for concept IDs
+- SHALL NOT introduce async in the PdfExtractor trait
+- SHALL NOT add new crate dependencies (all already in Cargo.toml)
diff --git a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
new file mode 100644
index 0000000..b418e4d
--- /dev/null
+++ b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
@@ -0,0 +1,27 @@
+{
+  "plugin_root": "/Users/vidarbrevik/.claude/plugins/cache/piercelamb-plugins/deep-implement/0.2.0",
+  "sections_dir": "/Users/vidarbrevik/projects/risk-security-ctrl/docs/specs/05-playbook-enrichment/01-data-extraction/sections",
+  "target_dir": "/Users/vidarbrevik/projects/risk-security-ctrl",
+  "state_dir": "/Users/vidarbrevik/projects/risk-security-ctrl/docs/specs/05-playbook-enrichment/01-data-extraction/implementation",
+  "git_root": "/Users/vidarbrevik/projects/risk-security-ctrl",
+  "commit_style": "conventional",
+  "test_command": "uv run pytest",
+  "sections": [
+    "section-01-extractor-trait-and-types",
+    "section-02-page-offset-detection",
+    "section-03-playbook-extractor",
+    "section-04-validation-logic",
+    "section-05-cli-integration",
+    "section-06-integration-tests"
+  ],
+  "sections_state": {},
+  "pre_commit": {
+    "present": false,
+    "type": "none",
+    "config_file": null,
+    "native_hook": null,
+    "may_modify_files": false,
+    "detected_formatters": []
+  },
+  "created_at": "2026-03-19T23:14:11.172803+00:00"
+}
\ No newline at end of file
