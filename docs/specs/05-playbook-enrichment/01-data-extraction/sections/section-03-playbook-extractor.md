I now have all the context needed. Let me produce the section content.

# Section 03: Playbook Extractor

## Overview

This section implements `PlaybookExtractor`, the concrete implementation of the `PdfExtractor` trait (defined in section-01) that knows how to parse the NIST AI RMF Playbook PDF. It handles section header detection for all 75 action-level concepts, subsection splitting within each concept, and multi-page text concatenation.

**Dependencies:**
- **section-01-extractor-trait-and-types** must be complete: provides `PdfExtractor` trait, `ExtractionConfig`, `ExtractionResult`, `ExtractedSection`, `Subsection`, `SubsectionKind`, `ExtractionError`, and `read_pdf_pages()`
- **section-02-page-offset-detection** must be complete: provides `detect_page_offset()` used to compute logical page numbers

**Blocks:** section-05 (CLI integration) and section-06 (integration tests)

---

## File Locations

All code for this section lives in:

```
backend/src/features/extraction/playbook/
  mod.rs          # PlaybookExtractor struct and PdfExtractor impl
```

Tests are written as an inline `#[cfg(test)]` module within `mod.rs`.

---

## Tests (Write First)

All tests go in a `#[cfg(test)] mod tests` block inside `backend/src/features/extraction/playbook/mod.rs`. Use string constants as fixtures rather than actual PDF files. The tests exercise three areas: header detection, subsection splitting, and multi-page concatenation.

### Section Header Detection Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // --- Header detection ---

    /// "GOVERN 1.1" at the start of a text block is detected as a section header.
    #[test]
    fn detects_govern_header_at_start() { todo!() }

    /// "MEASURE 2.3" with extra internal whitespace ("MEASURE  2.3") is still detected.
    #[test]
    fn detects_header_with_extra_whitespace() { todo!() }

    /// Spaced-out characters from heading fonts ("G O V E R N  1 . 1") are
    /// detected after whitespace normalization.
    #[test]
    fn detects_spaced_out_header() { todo!() }

    /// Subcategory headers like "GOVERN 1" (no decimal) are NOT matched.
    #[test]
    fn does_not_match_subcategory_without_decimal() { todo!() }

    /// Inline references like "see GOVERN 1.1 for details" in body text are
    /// not treated as new section boundaries. Only headers that begin a new
    /// concept block (e.g., appearing after the previous section's content
    /// or at the top of a new page) should match.
    #[test]
    fn does_not_match_inline_reference() { todo!() }

    /// Given text containing multiple concept headers, they are returned in
    /// page-number order.
    #[test]
    fn orders_detected_sections_by_page() { todo!() }

    // --- Subsection splitting ---

    /// A section with all 5 subsection headers splits into 5 Subsection values
    /// with correct SubsectionKind variants.
    #[test]
    fn splits_all_five_subsections() { todo!() }

    /// A section with only About and Suggested Actions (missing optional
    /// subsections) produces exactly 2 Subsection values.
    #[test]
    fn splits_partial_subsections() { todo!() }

    /// "Suggested Actions" at the start of a line triggers a split, but the
    /// same phrase mid-sentence does not.
    #[test]
    fn line_anchored_suggested_actions() { todo!() }

    /// "References" at line start triggers a split, but "See References for..."
    /// within body text does not.
    #[test]
    fn line_anchored_references() { todo!() }

    /// Full text content within each subsection is preserved (not truncated).
    #[test]
    fn preserves_subsection_content() { todo!() }

    /// A subsection header present with no content before the next header
    /// produces a Subsection with empty text.
    #[test]
    fn handles_empty_subsection_body() { todo!() }

    // --- Multi-page concatenation ---

    /// Text from two consecutive pages is joined with a space.
    #[test]
    fn joins_pages_with_space() { todo!() }

    /// Hyphenated words at a page break ("exam-\nple") are rejoined to "example".
    #[test]
    fn rejoins_hyphenated_words_at_page_break() { todo!() }

    /// The `physical_page` field records the starting page, not the ending page.
    #[test]
    fn records_starting_page() { todo!() }
}
```

### Test Fixture Strategy

Each test constructs a `&str` constant mimicking PDF-extracted text. For example, a header detection test might use:

```rust
const SAMPLE_TWO_SECTIONS: &str = "\
GOVERN 1.1\n\
Legal and regulatory requirements involving AI are understood...\n\
Suggested Actions\n\
• Establish approaches for detecting known risks.\n\
\n\
GOVERN 1.2\n\
Contingency processes are in place...\n";
```

The key is that these constants approximate the output of `pdf-extract` (raw text per page, with newlines and bullet characters) without requiring an actual PDF fixture.

---

## Implementation Details

### PlaybookExtractor Struct

```rust
pub struct PlaybookExtractor;
```

A unit struct with no fields. All configuration comes from the `ExtractionConfig` parameter passed to `extract()`.

### PdfExtractor Trait Implementation

The `PlaybookExtractor` implements the three required trait methods:

- **`name()`** returns `"NIST AI RMF Playbook"`
- **`framework_id()`** returns `"nist-ai-rmf"`
- **`extract()`** is the main entry point, described below
- **`validate()`** delegates to the shared validation module from section-04

### extract() Method Flow

1. **Read pages.** Call `read_pdf_pages(pdf_path)` (from section-01) to get `Vec<(usize, String)>` of (page_index, text) pairs.

2. **Detect page offset.** Call `detect_page_offset(&pages)` (from section-02) or use the override from `config.page_offset_override`.

3. **Normalize page text.** For each page, apply whitespace normalization to collapse spaced-out heading fonts (e.g., `"G O V E R N  1 . 1"` becomes `"GOVERN 1.1"`). Normalization is applied to a copy used for header scanning only; the raw text in subsections is preserved.

4. **Scan for concept headers.** Apply the section header regex against each normalized page. Collect all matches as `(concept_code, physical_page_index, char_offset_in_page)` tuples.

5. **Extract section text.** For each detected header, gather all text from that header's position to the next header's position (or end of document). This involves concatenating text across page boundaries when a concept spans multiple pages.

6. **Split subsections.** Within each section's raw text, identify subsection boundaries and produce `Vec<Subsection>`.

7. **Build result.** Construct `ExtractedSection` values and wrap in `ExtractionResult`.

### Section Header Regex

The primary regex pattern for detecting action-level concept headers:

```rust
r"(?m)^(GOVERN|MAP|MEASURE|MANAGE)\s+(\d+\.\d+)"
```

Key details:
- `(?m)` enables multiline mode so `^` matches the start of any line
- Only matches headers with a decimal component (`\d+\.\d+`) to exclude subcategory headers like "GOVERN 1"
- The `\s+` allows for extra whitespace between function name and number

Before applying this regex, the page text is passed through a **normalization function** that collapses single-character spacing:

```rust
/// Collapse spaced-out characters like "G O V E R N  1 . 1" into "GOVERN 1.1".
/// Only applied to lines that look like they contain spaced-out uppercase words.
fn normalize_spaced_headers(text: &str) -> String { /* ... */ }
```

The normalization function detects lines where most characters are followed by a space (a pattern caused by decorative/heading PDF fonts) and collapses them. It only transforms lines that match this heuristic to avoid corrupting body text.

### Distinguishing Headers from Inline References

A naive regex match will also capture inline references like "see GOVERN 1.1 for details." The extractor uses two heuristics to filter false positives:

1. **Line-start anchoring:** The regex uses `^` in multiline mode, so only matches at the beginning of a line qualify. Inline references mid-sentence will not match.
2. **Context check (optional refinement):** If a match occurs on a line that also contains significant body text before the code (e.g., "For more details see GOVERN 1.1 which describes..."), it is filtered out. In practice, the Playbook places concept codes as standalone heading lines, so the `^` anchor handles most cases.

### Subsection Splitting

Subsection boundaries are detected with line-start-anchored pattern matching within each section's raw text. The function signature:

```rust
fn split_subsections(raw_text: &str, concept_code: &str) -> Vec<Subsection> { /* ... */ }
```

The function scans the raw text for these patterns (in order of appearance in the Playbook):

| Pattern | SubsectionKind |
|---------|---------------|
| Line starts with the concept code (e.g., `"GOVERN 1.1"`) or a standalone `"About"` heading | `About` |
| `^Suggested Actions` (case-sensitive) | `SuggestedActions` |
| `^Transparency` or line containing `"Organizations can document"` | `TransparencyQuestions` |
| `^AI Transparency Resources` | `Resources` |
| `^References` (but NOT preceded by "See " on the same line) | `References` |

The text between two consecutive subsection boundaries becomes the `text` field of the earlier `Subsection`. If a heading is found but the next heading immediately follows, the `text` is empty.

### Multi-Page Text Concatenation

When a concept spans multiple PDF pages, the extractor concatenates the text with these rules:

1. Join consecutive page texts with a single space (not a newline, since page breaks within a paragraph should be seamless).
2. Rejoin hyphenated line-break words: if the last non-whitespace characters on the previous page are `word-` and the next page begins with a lowercase letter, join them as `wordrest` (removing the hyphen and newline).
3. The `physical_page` field on `ExtractedSection` records the **first** page where the concept header was found.

Implementation approach:

```rust
fn concatenate_pages(pages: &[(usize, String)], start_page: usize, end_page: usize) -> String {
    /// Joins text from pages[start_page..=end_page], handling hyphens at boundaries.
    todo!()
}
```

### Module Registration

The `backend/src/features/extraction/playbook/mod.rs` file must be declared as a submodule. Ensure `backend/src/features/extraction/mod.rs` (created in section-01) contains:

```rust
pub mod playbook;
```

---

## Edge Cases to Handle

1. **Empty pages:** Some PDF pages may extract as empty strings (blank pages, image-only pages). Skip these when scanning for headers but preserve page indexing.

2. **Duplicate header matches:** If the regex matches the same concept code on multiple lines within the same page (e.g., the code appears in both a heading and a subheading), take only the first occurrence per concept code.

3. **Last section in document:** The final concept has no "next header" to bound it. Use end-of-document as the boundary.

4. **PDF extraction artifacts:** The `pdf-extract` crate may produce garbled text for some pages (ligature issues, encoding problems). The extractor should not crash on malformed text; it should produce sections with whatever text was extracted and let Phase 2 (Claude structuring) handle corrections. Log warnings for sections with suspiciously short text (e.g., raw_text under 50 characters for a concept that should have substantial content).

---

## Dependencies Summary

| What | From | Used For |
|------|------|----------|
| `PdfExtractor` trait | section-01 (`extractor.rs`) | Trait implemented by `PlaybookExtractor` |
| `ExtractionConfig`, `ExtractionResult`, `ExtractedSection`, `Subsection`, `SubsectionKind` | section-01 (`extractor.rs`) | Core data types |
| `ExtractionError` | section-01 (`extractor.rs`) | Error type returned from `extract()` |
| `read_pdf_pages()` | section-01 (`extractor.rs`) | Reading raw text from PDF pages |
| `detect_page_offset()` | section-02 (`page_offset.rs`) | Computing physical-to-logical page offset |
| `ValidationReport` | section-04 (`validation.rs`) | Return type of `validate()` (can stub until section-04 is done) |