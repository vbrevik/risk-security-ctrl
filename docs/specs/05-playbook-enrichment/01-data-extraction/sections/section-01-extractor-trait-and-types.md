Now I have all the context needed. Let me generate the section content.

# Section 01: Extractor Trait and Types

## Overview

This section sets up the foundational types and trait for the PDF extraction framework. It creates the `extraction` feature module, defines the `PdfExtractor` trait that all extractor plugins implement, all core data types (`ExtractionConfig`, `ExtractionResult`, `ExtractedSection`, `Subsection`, enums), the `ExtractionError` type, and shared utility function signatures (`read_pdf_pages`, `resolve_concept_id`). It also registers the new feature module.

This section has **no dependencies** on other sections and **blocks** sections 02, 03, 04, and 05.

---

## Tests (Write First)

All tests live in `backend/src/features/extraction/extractor.rs` as a `#[cfg(test)]` module.

### Tests for `ExtractionError`

```rust
#[cfg(test)]
mod tests {
    use super::*;

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
        // Verify Display trait works for all variants
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
}
```

### Tests for `resolve_concept_id`

These tests load the real ontology file at `ontology-data/nist-ai-rmf.json` (relative to workspace root). They verify that concept code strings map correctly to ontology concept IDs.

```rust
#[test]
fn resolve_concept_id_govern_1_1() {
    // Load ontology, build map, look up "GOVERN 1.1" -> expect "nist-ai-gv-1-1"
}

#[test]
fn resolve_concept_id_measure_2_3() {
    // Look up "MEASURE 2.3" -> expect the correct concept_id from ontology
}

#[test]
fn resolve_concept_id_unknown_returns_none() {
    // Look up "GOVERN 99.99" -> None
}

#[test]
fn resolve_concept_id_subcategory_no_dot_returns_none() {
    // Look up "GOVERN 1" (no dot) -> None (these are subcategory-level, not action-level)
}

#[test]
fn resolve_concept_id_all_75_actions() {
    // Load ontology, filter concepts where concept_type == "action",
    // verify each has a "code" field, and resolve_concept_id returns Some for all of them
}
```

### Tests for `read_pdf_pages`

```rust
#[test]
fn read_pdf_pages_nonexistent_file_returns_error() {
    // Calling read_pdf_pages on a non-existent path returns ExtractionError::FileNotFound
}

#[test]
fn read_pdf_pages_returns_zero_based_sequential_indices() {
    // If we can construct or use a test PDF, verify page indices are 0-based and sequential
    // This may be deferred to integration tests (section-06)
}
```

---

## Implementation Details

### New Files to Create

#### 1. `backend/src/features/extraction/mod.rs`

The feature module root. Re-exports all public items:

```rust
pub mod extractor;
pub mod page_offset;
pub mod playbook;
pub mod validation;
pub mod cli;
```

Note: `page_offset`, `playbook`, `validation`, and `cli` modules will be empty stubs initially (just enough to compile). They are implemented in later sections.

#### 2. `backend/src/features/extraction/extractor.rs`

This is the core file for this section. It contains:

**The `PdfExtractor` trait:**

```rust
use std::path::Path;

pub trait PdfExtractor: Send + Sync {
    /// Human-readable name for this extractor (e.g., "NIST AI RMF Playbook")
    fn name(&self) -> &str;

    /// The framework_id this extractor targets (e.g., "nist-ai-rmf")
    fn framework_id(&self) -> &str;

    /// Extract structured sections from the PDF at the given path.
    fn extract(&self, pdf_path: &Path, config: &ExtractionConfig) -> Result<ExtractionResult, ExtractionError>;

    /// Validate extracted data against the ontology concepts.
    fn validate(&self, result: &ExtractionResult, ontology_path: &Path) -> ValidationReport;
}
```

The trait is intentionally synchronous -- PDF extraction is CPU-bound with no async I/O.

**The `ExtractionError` enum** using `thiserror::Error` derive with five variants:

- `FileNotFound(String)`
- `InvalidPdf(String)`
- `NoSectionsFound`
- `PageOffsetError(String)`
- `IoError(#[from] std::io::Error)`

**Core data types** (all with `#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]` where appropriate):

- `ExtractionConfig` -- has `page_offset_override: Option<i32>` and `output_format: OutputFormat`
- `OutputFormat` enum -- `Json`, `Markdown`, `Raw`
- `ExtractionResult` -- contains `framework_id`, `source_pdf`, `extracted_at` (chrono DateTime<Utc>), `sections` (Vec<ExtractedSection>), `page_offset_detected: i32`, `page_offset_source: PageOffsetSource`
- `PageOffsetSource` enum -- `Auto`, `Manual`, `Default`
- `ExtractedSection` -- contains `concept_code: String`, `concept_id: Option<String>`, `physical_page: usize`, `logical_page: usize`, `raw_text: String`, `subsections: Vec<Subsection>`
- `Subsection` -- contains `kind: SubsectionKind`, `text: String`
- `SubsectionKind` enum -- `About`, `SuggestedActions`, `TransparencyQuestions`, `Resources`, `References`
- `ValidationReport` -- contains `total_expected: usize`, `total_extracted: usize`, `missing_concepts: Vec<String>`, `unmatched_sections: Vec<String>`, `warnings: Vec<String>`

**Shared utility functions:**

`read_pdf_pages(path: &Path) -> Result<Vec<(usize, String)>, ExtractionError>` -- thin wrapper over `pdf_extract` that returns `(page_index, text)` pairs. Returns `ExtractionError::FileNotFound` if the file does not exist, or `ExtractionError::InvalidPdf` if the PDF cannot be parsed.

`resolve_concept_id(code: &str, ontology_path: &Path) -> Option<String>` -- loads the ontology JSON file, builds a `HashMap<String, String>` mapping the `code` field to the `id` field for each concept, then looks up the given code. Important: this does NOT algorithmically convert function names to prefixes. The ontology JSON is the source of truth. NIST AI RMF uses abbreviated prefixes in IDs (`gv-`, `mp-`, `ms-`, `mg-`), not the full function names (GOVERN, MAP, MEASURE, MANAGE).

The ontology JSON is located at `ontology-data/nist-ai-rmf.json` relative to the workspace root. Its structure has a `concepts` array where each concept has `id`, `code`, `concept_type`, and other fields. Only concepts with `concept_type == "action"` have codes like "GOVERN 1.1" with a decimal component.

#### 3. Stub modules for later sections

Create minimal placeholder files so the project compiles:

- `backend/src/features/extraction/page_offset.rs` -- empty or a stub `detect_page_offset` signature
- `backend/src/features/extraction/validation.rs` -- empty or a stub
- `backend/src/features/extraction/cli.rs` -- empty or a stub
- `backend/src/features/extraction/playbook/mod.rs` -- empty or a stub `PlaybookExtractor` struct

### File to Modify

#### `backend/src/features/mod.rs`

Add the extraction module registration:

```rust
pub mod extraction;
```

This line is added alongside the existing `pub mod analysis;`, `pub mod auth;`, etc.

---

## Dependencies

- `thiserror` (already in `Cargo.toml`) -- for `ExtractionError` derive
- `serde` / `serde_json` (already in `Cargo.toml`) -- for Serialize/Deserialize on types and for loading ontology JSON
- `chrono` (already in `Cargo.toml`) -- for `DateTime<Utc>` in `ExtractionResult`
- `pdf-extract` (already in `Cargo.toml` as version 0.10) -- used by `read_pdf_pages`

No new crate dependencies are needed for this section. The `clap` dependency is added in section-05.

---

## Key Design Decisions

1. **Synchronous trait:** The `PdfExtractor` trait methods are synchronous because PDF extraction is CPU-bound. No async machinery needed.

2. **Ontology as source of truth for concept IDs:** The `resolve_concept_id` function loads and parses the JSON rather than using algorithmic prefix conversion. This avoids bugs from the non-obvious abbreviated prefixes (`gv-` not `govern-`, `mp-` not `map-`, etc.).

3. **Feature-based module location:** Follows the existing pattern at `backend/src/features/{ontology,compliance,reports,analysis}/`. The new module lives at `backend/src/features/extraction/`.

4. **ValidationReport as a plain struct:** Defined here alongside the other types so that the `PdfExtractor::validate` method signature compiles. The actual validation logic is implemented in section-04.