# Implementation Plan: Playbook Data Extraction

## 1. Background and Goals

The NIST AI RMF Playbook PDF (`docs/reference-pdfs/AI_RMF_Playbook.pdf`) contains structured guidance for 75 action-level concepts across four functions (GOVERN, MAP, MEASURE, MANAGE). Each concept section follows a consistent layout: About, Suggested Actions, Transparency & Documentation, AI Transparency Resources, and References.

This plan builds a **reusable PDF extraction framework** in Rust that:
1. Extracts raw structured text from NIST reference PDFs via a CLI subcommand
2. Outputs section-level text with page numbers for Claude-assisted structuring
3. Validates extracted data against the existing ontology (`ontology-data/nist-ai-rmf.json`)
4. Produces a companion JSON file (`ontology-data/nist-ai-rmf-guidance.json`)

The framework uses a **plugin architecture** so future PDFs (NIST.AI.100-1, NIST.AI.600-1, NIST.SP.800-37r2) can be added as new extractor modules implementing a common trait.

### Two-Phase Workflow

The extraction is semi-automated:
- **Phase 1 (Rust CLI):** Parse the PDF, detect section boundaries, output raw text per concept with page numbers
- **Phase 2 (Claude in conversation):** Read the raw output and structure it into the final JSON schema with field-level parsing (splitting bullets, parsing citations, etc.)

This division plays to each tool's strengths: Rust for reliable PDF byte-level parsing, Claude for understanding natural language section semantics.

---

## 2. Extractor Trait and Shared Utilities

### Module Location

```
backend/src/features/extraction/
  mod.rs            # Feature module, re-exports
  extractor.rs      # PdfExtractor trait, core types, ExtractionError
  page_offset.rs    # Page offset detection utilities
  validation.rs     # Schema and completeness validation
  cli.rs            # CLI subcommand handler
  playbook/
    mod.rs          # PlaybookExtractor implementation
```

This follows the existing feature-based structure (`backend/src/features/{ontology,compliance,reports,analysis}/`).

### The PdfExtractor Trait

A common trait that all PDF-type plugins implement:

```rust
pub trait PdfExtractor: Send + Sync {
    /// Human-readable name for this extractor (e.g., "NIST AI RMF Playbook")
    fn name(&self) -> &str;

    /// The framework_id this extractor targets (e.g., "nist-ai-rmf")
    fn framework_id(&self) -> &str;

    /// Extract structured sections from the PDF at the given path.
    /// Returns a list of extracted sections with their raw text and metadata.
    fn extract(&self, pdf_path: &Path, config: &ExtractionConfig) -> Result<ExtractionResult, ExtractionError>;

    /// Validate extracted data against the ontology concepts.
    fn validate(&self, result: &ExtractionResult, ontology_path: &Path) -> ValidationReport;
}
```

Note: The trait methods are intentionally synchronous. PDF extraction is CPU-bound with no async I/O. When invoked from the CLI subcommand, this runs inside the `#[tokio::main]` runtime but does not need async machinery.

### Error Type

```rust
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("PDF file not found or unreadable: {0}")]
    FileNotFound(String),
    #[error("Invalid PDF: {0}")]
    InvalidPdf(String),
    #[error("No sections detected in PDF")]
    NoSectionsFound,
    #[error("Page offset detection failed: {0}")]
    PageOffsetError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Core Types

```rust
pub struct ExtractionConfig {
    pub page_offset_override: Option<i32>,
    pub output_format: OutputFormat,  // Json, Markdown, Raw
}

pub struct ExtractionResult {
    pub framework_id: String,
    pub source_pdf: String,
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    pub sections: Vec<ExtractedSection>,
    pub page_offset_detected: i32,
    pub page_offset_source: PageOffsetSource,  // Auto, Manual, Default
}

pub struct ExtractedSection {
    pub concept_code: String,       // e.g., "GOVERN 1.1"
    pub concept_id: Option<String>, // e.g., "nist-ai-gv-1-1" (resolved during validation)
    pub physical_page: usize,       // 0-based PDF page index
    pub logical_page: usize,        // Page number as shown in footer
    pub raw_text: String,           // Full section text
    pub subsections: Vec<Subsection>,
}

pub struct Subsection {
    pub kind: SubsectionKind,  // About, SuggestedActions, TransparencyQuestions, Resources, References
    pub text: String,
}
```

### Shared Utilities

The `pdf-extract` crate (already in `Cargo.toml` as version 0.10) provides `extract_text_by_pages()` which returns text per physical page. Shared utilities build on this:

- **`read_pdf_pages(path) -> Vec<(usize, String)>`**: Thin wrapper over `pdf_extract` that returns (page_index, text) pairs
- **`detect_page_offset(pages) -> i32`**: Scans early pages for TOC patterns or page number footers to compute the offset between physical and logical pages
- **`resolve_concept_id(code, ontology) -> Option<String>`**: Maps a concept code like "GOVERN 1.1" to its concept_id by loading the ontology JSON and building a `HashMap<String, String>` from the `code` field to the `id` field. **Do not** algorithmically convert function names to prefixes (e.g., "GOVERN" → "gv-") — the ontology JSON is the source of truth. Note: NIST AI RMF uses abbreviated prefixes (`gv-`, `mp-`, `ms-`, `mg-`) in concept IDs, not the full function names.

---

## 3. Page Offset Detection

The Playbook's TOC uses logical page numbers (e.g., "MEASURE 1.1 ..... 93") but these differ from physical PDF page indices by ~4-5 pages due to cover and TOC pages.

### Detection Strategy

1. **Primary:** Scan the first 10 physical pages for a pattern like `"GOVERN 1.1"` followed by a number. If found, and GOVERN 1.1's text appears on a different physical page, the difference is the offset.
2. **Secondary:** Scan page footer text for explicit page numbers (e.g., a standalone number at the bottom of a page) and compare to physical page index. Note: the `pdf-extract` crate does **not** expose PDF `/PageLabels` metadata or bookmarks, so this approach uses text content only.
3. **Fallback:** Default offset of 0 with a warning if auto-detection fails.
4. **Override:** `--page-offset N` CLI flag always takes precedence.

The offset is stored in `ExtractionResult::page_offset_detected` and `page_offset_source` for traceability.

---

## 4. Playbook Extractor Plugin

The `PlaybookExtractor` implements `PdfExtractor` with knowledge of the Playbook's consistent section structure.

### Section Detection

Each of the 75 action concepts in the Playbook starts with a header like "GOVERN 1.1" or "MEASURE 2.3". The extractor:

1. Reads all PDF pages as text
2. Scans for concept header patterns using a regex that accounts for PDF extraction artifacts:
   - Primary pattern: `(GOVERN|MAP|MEASURE|MANAGE)\s+\d+\.\d+` (action-level, with dot)
   - Must distinguish from subcategory headers like "GOVERN 1" (no dot) — only match patterns with the decimal component
   - Handle possible spaced-out characters from heading fonts (e.g., "G O V E R N  1 . 1") by normalizing whitespace before matching
   - Use multiline scanning across the full page text rather than line-by-line matching (PDF extraction may not preserve line boundaries reliably)
3. Records the physical page index where each concept header appears
4. Extracts all text from that header to the next concept header (or end of document) as the section's raw text

### Subsection Splitting

Within each section, the Playbook follows a predictable layout. The extractor identifies subsection boundaries using **anchored pattern matching** — subsection headers must appear at the start of a line (after optional whitespace) and be followed by a newline or the section body. This prevents false matches within body text.

Subsection header patterns (case-sensitive, line-start anchored):

- Line starting with the concept code (e.g., "GOVERN 1.1") or containing "About" as a standalone heading → `SubsectionKind::About`
- `"Suggested Actions"` at line start → `SubsectionKind::SuggestedActions`
- `"Transparency"` at line start, or `"Organizations can document"` → `SubsectionKind::TransparencyQuestions`
- `"AI Transparency Resources"` at line start → `SubsectionKind::Resources`
- `"References"` at line start (but not within a citation like "See References...") → `SubsectionKind::References`

If a subsection heading is not found, that subsection is omitted (some concepts may not have all five subsections).

### Known Limitations of pdf-extract

The `pdf-extract` crate has limitations that affect text quality:

- **Text ordering:** Extracts text in PDF content stream order, which may not match visual reading order for complex layouts (columns, sidebars, text boxes). The Playbook has a simple single-column layout, so this is low risk, but edge cases may occur.
- **Ligatures and encoding:** Some PDF fonts use ligature glyphs (e.g., "fi" as a single character) or non-standard encodings that may produce garbled text. If specific sections have extraction artifacts, they can be manually corrected in Phase 2.
- **No metadata access:** The crate does not expose PDF bookmarks, `/PageLabels`, or table of contents structure — only raw text per page.

If text quality is poor for specific pages, the `--verbose` flag outputs per-page extraction results so the user can identify and manually address problematic sections.

### Handling Multi-Page Concepts

Some concepts span 2-3 PDF pages. The extractor concatenates text across page boundaries, tracking the starting page for `physical_page`. Page breaks within a section are joined with a space, and hyphenated line-break words are rejoined.

---

## 5. CLI Subcommand Integration

The extraction tool is added as a subcommand of the existing server binary, following clap's subcommand pattern.

### Command Structure

```
cargo run -- extract-pdf <PDF_PATH> [OPTIONS]

Options:
  --type <TYPE>           Extractor type [default: auto-detect]
                          Values: playbook, ai-100-1, ai-600-1, sp-800-37
  --page-offset <N>       Manual page offset override
  --output <PATH>         Output file path [default: stdout]
  --format <FMT>          Output format [default: json]
                          Values: json, markdown, raw
  --validate <ONTOLOGY>   Path to ontology JSON for validation
  --verbose               Show detailed extraction progress
```

### Auto-Detection

If `--type` is not specified, the CLI examines the PDF filename and early page content to guess the extractor type. For example, if the text contains "AI Risk Management Framework Playbook", it selects `PlaybookExtractor`.

### Integration with Existing Binary

**Important:** The current `main.rs` has no argument parsing — it directly boots the Axum server. This plan requires adding `clap` as a new dependency with the `derive` feature.

The `main.rs` is refactored to use a clap `Command` enum:

```rust
enum Command {
    Serve { /* existing server args */ },
    ExtractPdf { /* extraction args */ },
}
```

**Backward compatibility:** When no subcommand is provided, the binary must default to `Serve` behavior so that existing `cargo run` invocations continue to work. Use clap's `#[command(default)]` or match on `None` to fall through to the serve path.

### Input Validation (STIG V-222605)

The CLI canonicalizes the PDF path argument before opening:
- Resolve symlinks with `std::fs::canonicalize()`
- Verify the file exists and is a regular file (not a directory or device)
- Verify the file has a `.pdf` extension
- Verify the file starts with the `%PDF` magic bytes (reuse existing `validate_upload` logic from `analysis/upload.rs`)

No directory restriction is enforced — the CLI is a developer tool that should accept any readable PDF path. Canonicalization + extension + magic bytes validation satisfies the path traversal control.

### Error Handling (STIG V-222585)

On any error (file not found, corrupt PDF, extraction failure), the CLI:
- Prints a clear error message to stderr
- Exits with a non-zero exit code
- Does NOT produce partial output files (write to a temp file first, rename on success)

---

## 6. Validation Logic

### Concept ID Cross-Reference

After extraction, validation loads the ontology JSON and checks:

1. **Coverage:** Every action concept in the ontology (`concept_type == "action"`) has a matching extracted section
2. **ID resolution:** Each extracted `concept_code` maps to a valid `concept_id` in the ontology
3. **Completeness count:** For the Playbook, exactly 75 sections should be extracted

### Schema Conformance

Each extracted section is checked for:
- Non-empty `raw_text`
- Valid `physical_page` (within PDF page count)
- At least one subsection (the About section should always be present)

### ValidationReport

```rust
pub struct ValidationReport {
    pub total_expected: usize,
    pub total_extracted: usize,
    pub missing_concepts: Vec<String>,      // concept_ids with no extracted section
    pub unmatched_sections: Vec<String>,     // extracted codes that don't match any concept
    pub warnings: Vec<String>,              // non-fatal issues (e.g., empty subsections)
}
```

The report is printed to stdout (or included in JSON output) so the human reviewer and Claude can see what needs attention.

---

## 7. Output Schema

### Raw CLI Output (Phase 1)

The CLI outputs structured JSON with raw text per section:

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "AI_RMF_Playbook.pdf",
  "extracted_at": "2026-03-19T14:00:00Z",
  "page_offset": { "value": 4, "source": "auto" },
  "validation": {
    "expected": 75,
    "extracted": 75,
    "missing": [],
    "unmatched": []
  },
  "sections": [
    {
      "concept_code": "GOVERN 1.1",
      "concept_id": "nist-ai-gv-1-1",
      "physical_page": 8,
      "logical_page": 4,
      "subsections": {
        "about": "Legal and regulatory requirements...",
        "suggested_actions": "• Establish approaches for detecting...\n• Identify testing procedures...",
        "transparency_questions": "• How will the appropriate performance metrics...",
        "resources": "GAO-21-519SP: AI Accountability Framework...",
        "references": "Sara R. Jordan (2019). Designing Artificial..."
      }
    }
  ]
}
```

### Final Companion File (Phase 2 — Claude-structured)

After Claude processes the raw output, the final file at `ontology-data/nist-ai-rmf-guidance.json` follows the schema from the spec:

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "AI_RMF_Playbook.pdf",
  "guidance": [
    {
      "concept_id": "nist-ai-ms-1-1",
      "source_page": 98,
      "about_en": "The development and utility of trustworthy AI systems...",
      "suggested_actions_en": [
        "Establish approaches for detecting, tracking and measuring known risks...",
        "Identify testing procedures and metrics..."
      ],
      "transparency_questions_en": [
        "How will the appropriate performance metrics..."
      ],
      "resources": [
        { "title": "GAO-21-519SP: AI Accountability Framework...", "url": null, "type": "transparency" }
      ],
      "references": [
        { "title": "Designing Artificial Intelligence Review Boards...", "authors": "Sara R. Jordan", "year": 2019, "venue": "2019 IEEE International Symposium on Technology and Society (ISTAS)", "url": null }
      ]
    }
  ]
}
```

The transformation from Phase 1 to Phase 2 involves:
- Splitting bullet text into arrays
- Parsing citation strings into structured fields (author, year, venue)
- The `concept_id` is already resolved in Phase 1 (carried through from validation)
- Using `logical_page` as `source_page`
- Adding `_en` suffixes to field names (Phase 1 uses bare keys; Phase 2 adds language suffixes for i18n readiness)

### Integration with Existing System

The guidance companion file (`nist-ai-rmf-guidance.json`) is **not** auto-imported by the existing ontology importer in `main.rs`. The importer expects the framework concept schema and would not understand the guidance schema. The guidance file is consumed by a future API endpoint (spec `02-schema-import`) that loads it into the database as enrichment data linked to existing concepts.

---

## 8. Testing Strategy

### Unit Tests

Located in `backend/src/features/extraction/` as `#[cfg(test)]` modules:

1. **Page offset detection:** Test with known page text patterns. Verify auto-detection returns correct offset. Verify manual override takes precedence.

2. **Section header detection:** Test regex matching against various PDF-extracted text samples (clean text, text with extra whitespace, text with line breaks in the middle of a header).

3. **Subsection splitting:** Given a raw section text block, verify correct identification of About, Suggested Actions, Transparency, Resources, and References subsections.

4. **Concept ID resolution:** Test mapping from concept codes ("GOVERN 1.1", "MEASURE 2.3") to concept IDs in the ontology JSON. Test with codes that exist and codes that don't.

5. **Validation logic:** Test coverage checking (missing concepts, extra sections), schema conformance (empty fields, invalid page numbers).

### Integration Tests

Located in `backend/tests/`:

1. **End-to-end extraction:** Run the PlaybookExtractor against a small test PDF (a fixture with 2-3 mock sections) and verify the output structure is correct.

2. **CLI invocation:** Test the `extract-pdf` subcommand with valid and invalid arguments. Verify exit codes and error messages.

3. **Validation against real ontology:** Load `ontology-data/nist-ai-rmf.json` and verify concept ID resolution works for all 75 action concepts.

### Test Fixtures

**Primary approach: string fixtures.** Test the text-processing logic (section detection, subsection splitting, concept ID resolution, validation) using raw text string constants that mimic PDF-extracted output. This is fast, deterministic, and avoids binary fixtures in the repo.

**Supplementary: small PDF fixture.** Optionally create a minimal test PDF using the `genpdf` crate (already in `Cargo.toml`) for end-to-end integration tests. This is only needed to verify the `pdf-extract` → text pipeline, not the parsing logic itself.

---

## 9. File Changes Summary

### New Files
- `backend/src/features/extraction/mod.rs` — Feature module
- `backend/src/features/extraction/extractor.rs` — `PdfExtractor` trait, core types, `ExtractionError`
- `backend/src/features/extraction/page_offset.rs` — Offset detection utilities
- `backend/src/features/extraction/validation.rs` — Validation logic
- `backend/src/features/extraction/cli.rs` — CLI subcommand handler
- `backend/src/features/extraction/playbook/mod.rs` — `PlaybookExtractor` implementation

### New Dependencies
- `clap` with `derive` feature — CLI argument parsing and subcommand dispatch

### Modified Files
- `backend/Cargo.toml` — Add `clap` dependency
- `backend/src/main.rs` — Add clap subcommand dispatch with `Serve` (default) and `ExtractPdf`
- `backend/src/features/mod.rs` — Register extraction feature module

### Test Files
- `backend/src/features/extraction/playbook/tests.rs` or inline `#[cfg(test)]` modules
- `backend/tests/extraction_tests.rs` — Integration tests

### Output Files (produced at runtime, not checked in)
- `ontology-data/nist-ai-rmf-guidance.json` — Final companion file (after Claude structuring)
