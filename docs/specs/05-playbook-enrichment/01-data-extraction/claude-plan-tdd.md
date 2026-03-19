# TDD Plan: Playbook Data Extraction

Mirrors the structure of `claude-plan.md`. For each section, defines tests to write BEFORE implementing.

**Testing framework:** Rust built-in `#[test]` and `#[tokio::test]`, following existing patterns in `backend/tests/`.

---

## 2. Extractor Trait and Shared Utilities

### Tests for `read_pdf_pages`
```rust
// Test: returns empty vec for empty/invalid PDF path
// Test: returns (page_index, text) pairs for a valid PDF
// Test: page_index is 0-based and sequential
```

### Tests for `resolve_concept_id`
```rust
// Test: maps "GOVERN 1.1" to correct concept_id from ontology JSON
// Test: maps "MEASURE 2.3" to correct concept_id
// Test: returns None for unknown code "GOVERN 99.99"
// Test: returns None for subcategory code "GOVERN 1" (no dot)
// Test: handles all 75 action codes from nist-ai-rmf.json
```

### Tests for `ExtractionError`
```rust
// Test: each variant displays a human-readable message (Display trait)
// Test: IoError converts from std::io::Error
```

---

## 3. Page Offset Detection

### Tests for `detect_page_offset`
```rust
// Test: detects offset when TOC pattern "GOVERN 1.1 ..... 4" appears on page 0 and GOVERN 1.1 text appears on page 8 → offset = 4
// Test: returns 0 with warning when no TOC pattern found
// Test: handles TOC with various separators (dots, spaces, dashes)
// Test: manual override takes precedence over auto-detected value
// Test: records correct PageOffsetSource (Auto, Manual, Default)
```

---

## 4. Playbook Extractor Plugin

### Tests for section header detection
```rust
// Test: detects "GOVERN 1.1" at start of text block
// Test: detects "MEASURE 2.3" with extra whitespace "MEASURE  2.3"
// Test: detects spaced-out header "G O V E R N  1 . 1" after normalization
// Test: does NOT match subcategory "GOVERN 1" (no decimal)
// Test: does NOT match partial match within body text "see GOVERN 1.1 for details" when not at section boundary
// Test: finds all 75 action headers in a realistic multi-page text sample
// Test: correctly orders detected sections by page number
```

### Tests for subsection splitting
```rust
// Test: splits text with all 5 subsections (About, Suggested Actions, Transparency, Resources, References)
// Test: splits text with only About and Suggested Actions (missing optional subsections)
// Test: "Suggested Actions" at line start triggers split, but "suggested actions" mid-sentence does not
// Test: "References" at line start triggers split, but "See References for..." does not
// Test: preserves full text content within each subsection
// Test: handles empty subsections (header present but no content before next header)
```

### Tests for multi-page concatenation
```rust
// Test: joins text across two pages with space
// Test: rejoins hyphenated words at page break ("exam-\nple" → "example")
// Test: records starting page as physical_page, not ending page
```

---

## 5. CLI Subcommand Integration

### Tests for CLI argument parsing
```rust
// Test: parse "extract-pdf /path/to/file.pdf" with defaults
// Test: parse with --type playbook --page-offset 4 --output out.json
// Test: parse with --validate /path/to/ontology.json
// Test: no subcommand defaults to serve behavior
```

### Tests for input validation
```rust
// Test: rejects non-existent file path
// Test: rejects directory path
// Test: rejects non-.pdf extension
// Test: rejects file without %PDF magic bytes
// Test: accepts valid PDF file path after canonicalization
```

### Tests for error handling
```rust
// Test: extraction error produces non-zero exit code
// Test: error message goes to stderr, not stdout
// Test: partial output file is not left behind on failure
```

---

## 6. Validation Logic

### Tests for concept coverage validation
```rust
// Test: 75/75 extracted sections → no missing concepts
// Test: 74/75 extracted sections → reports 1 missing concept with its ID
// Test: 76 extracted sections (one unmatched) → reports 1 unmatched section
// Test: 0 extracted sections → reports all 75 as missing
```

### Tests for schema conformance
```rust
// Test: section with non-empty raw_text and valid page → passes
// Test: section with empty raw_text → warning
// Test: section with physical_page > PDF page count → warning
// Test: section with no subsections → warning
```

### Tests for ValidationReport
```rust
// Test: report with no issues has empty missing/unmatched/warnings
// Test: report correctly counts total_expected and total_extracted
```

---

## 7. Output Schema

### Tests for JSON serialization
```rust
// Test: ExtractionResult serializes to expected JSON structure
// Test: extracted_at field serializes as ISO 8601 string
// Test: page_offset includes both value and source
// Test: validation section includes expected/extracted/missing/unmatched
// Test: subsections serialize with correct keys (about, suggested_actions, etc.)
```

---

## 8. Integration Tests

Located in `backend/tests/extraction_tests.rs`:

```rust
// Test: PlaybookExtractor::extract with string fixture text returns correct section count
// Test: PlaybookExtractor::validate with real nist-ai-rmf.json resolves all concept IDs
// Test: full pipeline: extract → validate → serialize produces valid JSON
// Test: CLI subcommand "extract-pdf" with --help shows usage
// Test: CLI subcommand with invalid path exits with non-zero code
```
