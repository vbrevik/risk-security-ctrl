<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-extractor-trait-and-types
section-02-page-offset-detection
section-03-playbook-extractor
section-04-validation-logic
section-05-cli-integration
section-06-integration-tests
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-extractor-trait-and-types | - | 02, 03, 04, 05 | Yes |
| section-02-page-offset-detection | 01 | 03 | Yes |
| section-03-playbook-extractor | 01, 02 | 05, 06 | No |
| section-04-validation-logic | 01 | 03, 05, 06 | Yes |
| section-05-cli-integration | 01, 03, 04 | 06 | No |
| section-06-integration-tests | 03, 04, 05 | - | No |

## Execution Order

1. section-01-extractor-trait-and-types (no dependencies)
2. section-02-page-offset-detection, section-04-validation-logic (parallel after 01)
3. section-03-playbook-extractor (after 01 AND 02)
4. section-05-cli-integration (after 01, 03, 04)
5. section-06-integration-tests (final, after all)

## Section Summaries

### section-01-extractor-trait-and-types
Feature module setup, `PdfExtractor` trait definition, core types (`ExtractionConfig`, `ExtractionResult`, `ExtractedSection`, `Subsection`, `ExtractionError`), shared utility signatures (`read_pdf_pages`, `resolve_concept_id`). Registers the extraction feature in `features/mod.rs`.

### section-02-page-offset-detection
`page_offset.rs` with `detect_page_offset()` — scans early PDF pages for TOC patterns and footer page numbers to compute physical-to-logical page offset. Supports auto-detection with manual override. Unit tests with text fixtures.

### section-03-playbook-extractor
`PlaybookExtractor` implementing `PdfExtractor` trait. Section header detection with whitespace-normalized regex, subsection splitting with line-anchored patterns, multi-page text concatenation. Handles all 75 action concepts. Unit tests for header detection, subsection splitting, and edge cases.

### section-04-validation-logic
`validation.rs` with concept coverage checking (cross-reference against ontology JSON), schema conformance validation, and `ValidationReport` generation. Loads `nist-ai-rmf.json` and builds `code → id` HashMap for concept ID resolution. Unit tests for coverage, conformance, and report generation.

### section-05-cli-integration
Add `clap` dependency to `Cargo.toml`. Refactor `main.rs` with `Command` enum (`Serve` default, `ExtractPdf`). CLI argument parsing, input validation (canonicalize path, check extension, magic bytes), output formatting (JSON/markdown/raw), temp-file-then-rename error safety. Tests for argument parsing and validation.

### section-06-integration-tests
End-to-end tests in `backend/tests/extraction_tests.rs`. Full pipeline test with string fixtures. Concept ID resolution against real `nist-ai-rmf.json` for all 75 actions. CLI subcommand invocation tests (help, invalid args, exit codes).
