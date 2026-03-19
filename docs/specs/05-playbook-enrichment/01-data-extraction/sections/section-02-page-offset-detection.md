# Section 02: Page Offset Detection

## Overview

This section implements `page_offset.rs` in `backend/src/features/extraction/`, providing the `detect_page_offset()` function and related types. The Playbook PDF has a mismatch between physical page indices (0-based, as returned by `pdf-extract`) and logical page numbers (printed in the PDF footer / referenced in the TOC). This module auto-detects that offset so downstream code can map between the two coordinate systems.

**Dependencies:** Section 01 (Extractor Trait and Types) must be complete first. This section uses `ExtractionError::PageOffsetError` and the `PageOffsetSource` enum defined there.

**Blocks:** Section 03 (Playbook Extractor) depends on this module.

---

## File to Create

**`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/extraction/page_offset.rs`**

---

## Types

The `PageOffsetSource` enum is defined in section 01 (`extractor.rs`). For reference, it has three variants:

```rust
pub enum PageOffsetSource {
    Auto,
    Manual,
    Default,
}
```

The `detect_page_offset` function returns a tuple of `(i32, PageOffsetSource)` representing the offset value and how it was determined.

---

## Tests (Write First)

All tests live inside a `#[cfg(test)] mod tests` block at the bottom of `page_offset.rs`. They use hardcoded string slices to simulate PDF page text, with no actual PDF files needed.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Simulates a TOC page containing "GOVERN 1.1 ..... 4" on physical page 0,
    /// and the actual GOVERN 1.1 header text appearing on physical page 8.
    /// Expected: offset = 4 (logical page 4 maps to physical page 8, so offset = 8 - 4 = 4).
    #[test]
    fn test_detects_offset_from_toc_and_content_page() { }

    /// When no TOC pattern is found in the first 10 pages, falls back to offset 0
    /// and returns PageOffsetSource::Default.
    #[test]
    fn test_returns_zero_offset_when_no_toc_pattern_found() { }

    /// TOC lines may use different separators between the title and page number:
    /// dots ("GOVERN 1.1 ..... 4"), spaces ("GOVERN 1.1    4"), dashes ("GOVERN 1.1 --- 4").
    /// All should be parsed correctly.
    #[test]
    fn test_handles_toc_with_various_separators() { }

    /// When a manual override value is provided, it takes precedence over any
    /// auto-detected offset, and PageOffsetSource::Manual is returned.
    #[test]
    fn test_manual_override_takes_precedence() { }

    /// Verify that the returned PageOffsetSource matches the detection method:
    /// Auto when TOC-based detection succeeds, Manual when override is provided,
    /// Default when falling back.
    #[test]
    fn test_records_correct_page_offset_source() { }
}
```

Each test constructs a `Vec<(usize, String)>` representing pages (physical page index, page text) and calls `detect_page_offset` with or without an override.

---

## Implementation Details

### Function Signature

```rust
/// Detect the offset between physical PDF page indices and logical page numbers.
///
/// `pages` is a slice of (physical_page_index, page_text) pairs, expected to be
/// the first ~10 pages of the PDF. `manual_override` bypasses auto-detection.
///
/// Returns (offset_value, source). The offset is defined such that:
///   logical_page = physical_page - offset
pub fn detect_page_offset(
    pages: &[(usize, String)],
    manual_override: Option<i32>,
) -> (i32, PageOffsetSource)
```

### Detection Strategy (in priority order)

1. **Manual override.** If `manual_override` is `Some(n)`, return `(n, PageOffsetSource::Manual)` immediately. No scanning needed.

2. **Primary: TOC pattern scanning.** Scan the first 10 physical pages for lines matching a TOC entry pattern. The regex to use:

   ```
   (GOVERN|MAP|MEASURE|MANAGE)\s+(\d+\.\d+)\s*[.\-\s]+\s*(\d+)
   ```

   This captures the concept code (e.g., "GOVERN 1.1") and the logical page number (e.g., "4"). After finding a TOC entry, scan all pages for the actual concept header (e.g., a page whose text starts with or prominently contains "GOVERN 1.1" as a section header, not inside a TOC line). The offset is `physical_page_of_content - logical_page_from_toc`.

   Use the **first** TOC entry that can be cross-referenced with actual content as the authoritative offset. Log a warning if multiple TOC entries produce inconsistent offsets.

3. **Secondary: Footer page number scanning.** If no TOC entries are found, scan pages for a standalone number at the end of the page text (a common footer pattern). Use regex:

   ```
   \n\s*(\d+)\s*$
   ```

   Compare the extracted number to the physical page index. If the difference is consistent across 2+ pages, use it as the offset.

4. **Fallback.** If neither method works, return `(0, PageOffsetSource::Default)` and log a warning to `tracing::warn!`.

### Key Implementation Notes

- The function is **synchronous** (no async). PDF text has already been extracted; this operates on in-memory strings.
- Use the `regex` crate (already available in the project) for pattern matching.
- Use `tracing::warn!` for fallback/inconsistency warnings. Do not panic or return an error for detection failure -- the offset simply defaults to 0.
- The function scans only the slice of pages provided to it. The caller (the extractor) is responsible for passing the appropriate range (typically the first 10 pages). This keeps the function testable with small fixtures.
- When searching for the actual content page of a concept, distinguish TOC references from real section headers. A TOC line contains the concept code followed by dots/dashes and a page number on the same line. A real section header has the concept code prominently (often as the only text on a line or at the start of a substantial text block). Check that the candidate line does **not** match the TOC pattern itself.

### Module Registration

The `page_offset` module must be declared in `backend/src/features/extraction/mod.rs`:

```rust
pub mod page_offset;
```

This is expected to already be stubbed or planned as part of section 01. If not present, add the `pub mod` line.

---

## Summary of Deliverables

| Artifact | Path |
|----------|------|
| Page offset module | `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/extraction/page_offset.rs` |
| Module declaration | Add `pub mod page_offset;` in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/extraction/mod.rs` |

The module exports a single public function `detect_page_offset` and relies on `PageOffsetSource` from `extractor.rs` (section 01). No other public API is needed.