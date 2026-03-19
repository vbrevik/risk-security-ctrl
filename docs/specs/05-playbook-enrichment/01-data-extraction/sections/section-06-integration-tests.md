No other sections exist yet. I have enough context to generate the section content.

# Section 06: Integration Tests

## Overview

This section adds end-to-end integration tests in `backend/tests/extraction_tests.rs`. These tests exercise the full extraction pipeline -- from `PlaybookExtractor` through validation and serialization -- using string fixtures and the real `nist-ai-rmf.json` ontology file. It also includes tests for the CLI `extract-pdf` subcommand (help output, invalid arguments, exit codes).

**Dependencies:** This section depends on sections 03 (PlaybookExtractor), 04 (validation logic), and 05 (CLI integration) being implemented. All extraction types and the `PdfExtractor` trait from section 01 must also be in place.

## File to Create

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/extraction_tests.rs`

## Background

Integration tests live in `backend/tests/` following the existing project convention (see `api_tests.rs`, `compliance_tests.rs`, `analysis_tests.rs`). Unlike those tests, the extraction tests do **not** need a database or Axum app setup -- they operate on the extraction library code and CLI binary directly.

The tests import from the `ontology_backend` library crate (defined in `backend/src/lib.rs` as `name = "ontology_backend"`). The extraction feature module is at `ontology_backend::features::extraction`.

Key types to import:
- `ontology_backend::features::extraction::extractor::{PdfExtractor, ExtractionConfig, ExtractionResult, OutputFormat}`
- `ontology_backend::features::extraction::playbook::PlaybookExtractor`
- `ontology_backend::features::extraction::validation::ValidationReport`

The real ontology file is at `../ontology-data/nist-ai-rmf.json` (relative to the `backend/` directory where `cargo test` runs). It contains exactly 75 concepts with `concept_type == "action"`, each having a `code` field (e.g., `"GOVERN 1.1"`) and an `id` field (e.g., `"nist-ai-gv-1-1"`).

## Tests

Write tests FIRST, then ensure the library code supports them. All tests go in `backend/tests/extraction_tests.rs`.

### Test 1: Full pipeline with string fixture

```rust
/// Test: PlaybookExtractor::extract with string fixture text returns correct section count.
///
/// Build a string fixture containing 3 mock concept sections (e.g., GOVERN 1.1,
/// GOVERN 1.2, MAP 1.1) with realistic subsection structure (About, Suggested Actions,
/// References). Feed this through PlaybookExtractor and verify:
/// - result.sections.len() == 3
/// - Each section has the expected concept_code
/// - Subsections are split correctly (at least About and SuggestedActions present)
/// - physical_page values are recorded
```

The string fixture should mimic PDF-extracted text with realistic formatting: concept headers on separate lines, subsection headers at line starts, bullet points with unicode bullet characters, and multi-line paragraph text. Example structure:

```
GOVERN 1.1

About
Legal and regulatory requirements involving AI are identified...

Suggested Actions
• Establish approaches for detecting...
• Identify testing procedures...

References
Sara R. Jordan (2019). Designing Artificial...
```

Since `PlaybookExtractor::extract` takes a `&Path` to a PDF file, and creating real PDF fixtures is complex, the test should exercise the text-processing methods exposed on `PlaybookExtractor` directly. The extractor should expose a method like `extract_from_text(pages: &[(usize, String)], config: &ExtractionConfig) -> Result<ExtractionResult, ExtractionError>` that the integration test can call with in-memory page text. This is the same internal method that `extract()` calls after reading the PDF, but it bypasses the PDF I/O.

### Test 2: Concept ID resolution against real ontology

```rust
/// Test: PlaybookExtractor::validate with real nist-ai-rmf.json resolves all concept IDs.
///
/// Load the real ontology JSON at `../ontology-data/nist-ai-rmf.json`.
/// Build an ExtractionResult with all 75 action concept codes.
/// Call validate() and verify:
/// - report.missing_concepts is empty
/// - report.unmatched_sections is empty
/// - report.total_expected == 75
/// - report.total_extracted == 75
///
/// This confirms the concept code-to-ID mapping works for the full ontology.
```

To build the 75 concept codes, the test should load the ontology JSON, extract all concepts where `concept_type == "action"`, read their `code` fields, and construct minimal `ExtractedSection` entries with those codes. This ensures the test stays in sync with the ontology data rather than hardcoding all 75 codes.

### Test 3: Full pipeline serialization

```rust
/// Test: full pipeline: extract -> validate -> serialize produces valid JSON.
///
/// Use the same string fixture from Test 1.
/// Run extract_from_text(), then validate(), then serialize the ExtractionResult
/// to JSON via serde_json::to_string_pretty().
/// Verify:
/// - The JSON parses back into a serde_json::Value
/// - Top-level keys include "framework_id", "source_pdf", "extracted_at", "sections"
/// - "extracted_at" is a valid ISO 8601 string
/// - "page_offset" contains "value" and "source" keys
/// - "sections" is an array with the expected length
```

### Test 4: CLI help output

```rust
/// Test: CLI subcommand "extract-pdf" with --help shows usage.
///
/// Invoke the binary with `extract-pdf --help` using std::process::Command.
/// Verify:
/// - Exit code is 0
/// - stdout contains "extract-pdf" and "PDF_PATH"
/// - stdout contains option names like "--format", "--validate", "--page-offset"
```

The binary name is `ontology-backend` (defined in `Cargo.toml` as `[[bin]] name = "ontology-backend"`). Use `env!("CARGO_BIN_EXE_ontology-backend")` or `assert_cmd`-style invocation to locate the compiled binary.

### Test 5: CLI with invalid path exits non-zero

```rust
/// Test: CLI subcommand with invalid path exits with non-zero code.
///
/// Invoke the binary with `extract-pdf /nonexistent/path.pdf`.
/// Verify:
/// - Exit code is non-zero
/// - stderr contains an error message about file not found
/// - stdout is empty (no partial output)
```

## Implementation Notes

### String fixture construction

Define a helper function or constant at the top of the test file that builds realistic page text. Each "page" is a `(usize, String)` tuple where the `usize` is the 0-based physical page index. Spread 3 concept sections across 4-5 pages to test multi-page concatenation.

```rust
/// Build test fixture: 3 concept sections across 5 pages.
/// Returns Vec<(page_index, page_text)> mimicking pdf-extract output.
fn build_test_pages() -> Vec<(usize, String)> {
    // Page 0-1: front matter / TOC (no concept headers)
    // Page 2: GOVERN 1.1 section (About + Suggested Actions)
    // Page 3: GOVERN 1.1 continued (References) + GOVERN 1.2 start
    // Page 4: GOVERN 1.2 continued + MAP 1.1 complete section
    // ...
}
```

### CLI test binary location

For CLI tests, use `std::process::Command::new(env!("CARGO_BIN_EXE_ontology-backend"))`. This macro resolves to the path of the compiled binary during `cargo test`. The binary must be compiled before integration tests run, which `cargo test` handles automatically.

### Ontology file path

The integration tests run with the working directory set to `backend/`. The ontology file path is `../ontology-data/nist-ai-rmf.json`. Use `std::path::Path::new("../ontology-data/nist-ai-rmf.json")` and assert it exists at the start of any test that needs it, with a clear panic message if missing.

### No database or async needed

Unlike the existing integration tests in `api_tests.rs` that use `#[tokio::test]` and require a database, the extraction tests are synchronous. Use `#[test]` for library-level tests (extract, validate, serialize). The CLI tests also use `#[test]` since `std::process::Command` is synchronous.

### Test file skeleton

```rust
//! Integration tests for the PDF extraction pipeline.
//!
//! These tests exercise the full extraction flow using string fixtures
//! and the real ontology data file.

use std::path::Path;
use ontology_backend::features::extraction::extractor::{
    ExtractionConfig, ExtractionResult, OutputFormat, PdfExtractor,
};
use ontology_backend::features::extraction::playbook::PlaybookExtractor;

/// Helper: build realistic page text fixtures for 3 concept sections.
fn build_test_pages() -> Vec<(usize, String)> {
    // ... construct pages with GOVERN 1.1, GOVERN 1.2, MAP 1.1
    todo!()
}

/// Helper: path to the real ontology JSON.
fn ontology_path() -> &'static Path {
    let p = Path::new("../ontology-data/nist-ai-rmf.json");
    assert!(p.exists(), "Ontology file not found at {}", p.display());
    p
}

#[test]
fn extract_from_fixture_returns_correct_sections() {
    // Test 1: full pipeline with string fixture
    todo!()
}

#[test]
fn validate_resolves_all_75_concept_ids() {
    // Test 2: concept ID resolution against real ontology
    todo!()
}

#[test]
fn full_pipeline_produces_valid_json() {
    // Test 3: extract -> validate -> serialize
    todo!()
}

#[test]
fn cli_extract_pdf_help() {
    // Test 4: --help output
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ontology-backend"))
        .args(["extract-pdf", "--help"])
        .output()
        .expect("Failed to execute binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("extract-pdf") || stdout.contains("PDF_PATH"));
}

#[test]
fn cli_invalid_path_exits_nonzero() {
    // Test 5: invalid path exit code
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ontology-backend"))
        .args(["extract-pdf", "/nonexistent/path.pdf"])
        .output()
        .expect("Failed to execute binary");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.is_empty(), "Expected error message on stderr");
}
```

## Design Requirement: `extract_from_text` public method

For integration tests to work without a real PDF file, `PlaybookExtractor` (section 03) must expose its text-processing pipeline as a public method. This is a requirement from section 06 on section 03:

```rust
impl PlaybookExtractor {
    /// Extract sections from pre-parsed page text.
    /// This is the core logic called by `PdfExtractor::extract()` after reading the PDF.
    /// Exposed publicly for integration testing with string fixtures.
    pub fn extract_from_text(
        &self,
        pages: &[(usize, String)],
        config: &ExtractionConfig,
    ) -> Result<ExtractionResult, ExtractionError> {
        // ... section detection, subsection splitting, multi-page concatenation
    }
}
```

If section 03 does not expose this method, the integration tests will need to create a minimal PDF fixture using the `genpdf` crate (already in `Cargo.toml`). The string fixture approach is strongly preferred for speed and determinism.

## Checklist

1. Create `backend/tests/extraction_tests.rs`
2. Write the `build_test_pages()` fixture helper with 3 concept sections across multiple pages
3. Write the `ontology_path()` helper that locates and validates `../ontology-data/nist-ai-rmf.json`
4. Implement Test 1: `extract_from_fixture_returns_correct_sections`
5. Implement Test 2: `validate_resolves_all_75_concept_ids`
6. Implement Test 3: `full_pipeline_produces_valid_json`
7. Implement Test 4: `cli_extract_pdf_help`
8. Implement Test 5: `cli_invalid_path_exits_nonzero`
9. Run `cargo test --test extraction_tests` from `backend/` and verify all tests pass