# Section 1: Dependencies and Types

## Goal

Add the crate dependencies needed for document parsing to `Cargo.toml`, and create the foundational types in a new `parser.rs` file within the existing `analysis` feature module. This section produces zero functionality on its own but defines the shared types that sections 02, 03, and 04 depend on.

## Background

The document parsing pipeline turns user-uploaded files (PDF, DOCX, or raw text) into a `ParsedDocument` struct that downstream systems consume. The matching engine (split 03) uses the extracted text for FTS5 queries and TF-IDF scoring. The backend API (split 04) calls the parser from upload handlers.

This section is a prerequisite for sections 02 (PDF parser), 03 (DOCX parser), and 04 (text parser and dispatch). It can be implemented in parallel with section 05 (tokenizer), which has no dependency on this section.

## Tests

Tests live in `backend/src/features/analysis/parser.rs` inside a `#[cfg(test)] mod tests` block. There are no tests for the Cargo dependency additions -- those are validated by `cargo check`.

### Test: ParsingError variants display meaningful messages

Construct each `ParsingError` variant and assert that its `Display` output matches the expected format strings:
- `UnsupportedFormat("xlsx".into())` displays `"Unsupported format: xlsx"`
- `CorruptFile("bad header".into())` displays `"Could not parse file: bad header"`
- `EmptyDocument` displays `"No text content found in document"`
- `FileTooLarge { size: 25_000_000, max: 20_000_000 }` displays `"File too large: 25000000 bytes (max: 20000000)"`

### Test: ParsingError::IoError converts from std::io::Error via `?`

Write a helper function returning `Result<(), ParsingError>` that uses the `?` operator on an `std::io::Error`. Verify the conversion works and the display includes the IO error message.

### Test: ParsedDocument computes word_count and token_count_estimate correctly

Given `full_text = "The quick brown fox jumps"` (5 words), verify `word_count` is 5 and `token_count_estimate` is `(5.0 * 1.33) as usize` which equals 6.

### Test: ParsedDocument with empty text has zero counts

Given `full_text = ""`, verify both `word_count` and `token_count_estimate` are 0.

## Implementation

### Step 1: Add Cargo dependencies

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml`

Add three new dependencies under a `# Document parsing` comment. Place this group after the existing `# Utilities` section (after the `chrono` line, before `[lib]`):

```toml
# Document parsing
pdf-extract = "0.10"
zip = "2"
quick-xml = "0.37"
```

The existing `Cargo.toml` uses a section-comment convention (e.g., `# Web framework`, `# Database`, `# Serialization`). Follow that pattern.

### Step 2: Create parser.rs with types

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/parser.rs`

This is a new file. It defines three items: `ParsingError`, `ParsedDocument`, and `DocumentSection`.

#### ParsingError enum

Derive `thiserror::Error` and `Debug`. Variants:

- `#[error("Unsupported format: {0}")]` `UnsupportedFormat(String)` -- file extension is not `.pdf` or `.docx`
- `#[error("Could not parse file: {0}")]` `CorruptFile(String)` -- parser library returned an error
- `#[error("No text content found in document")]` `EmptyDocument` -- parsing succeeded but no text was extracted
- `#[error("File too large: {size} bytes (max: {max})")]` `FileTooLarge { size: usize, max: usize }` -- exceeds 20 MB limit
- `#[error("IO error: {0}")]` `IoError(#[from] std::io::Error)` -- automatic conversion from `std::io::Error`

#### DocumentSection struct

Derive `Debug`, `Clone`, `Serialize`. Fields:

- `heading: Option<String>` -- section heading if detected (DOCX headings, or "Page N" for PDF)
- `text: String` -- section text content
- `page_number: Option<usize>` -- page number (PDF only)

#### ParsedDocument struct

Derive `Debug`, `Clone`, `Serialize`. Fields:

- `full_text: String` -- all extracted text concatenated
- `sections: Vec<DocumentSection>` -- detected sections
- `word_count: usize` -- split on whitespace, count non-empty tokens
- `token_count_estimate: usize` -- `(word_count as f64 * 1.33) as usize`

Provide a constructor or associated function (e.g., `fn new(full_text: String, sections: Vec<DocumentSection>) -> Self`) that computes `word_count` and `token_count_estimate` from `full_text` automatically. This avoids every call site computing them manually. The constructor should:

1. Compute `word_count` as `full_text.split_whitespace().count()`
2. Compute `token_count_estimate` as `(word_count as f64 * 1.33) as usize`
3. Return the struct with all fields populated

#### DocumentParser struct

Define `pub struct DocumentParser;` as a unit struct. It will hold associated functions added by later sections (parse_pdf, parse_docx, parse_text, parse). No state is needed.

### Step 3: Verify with cargo check

After creating the file, run `cargo check` from `/Users/vidarbrevik/projects/risk-security-ctrl/backend/` to confirm the new dependencies resolve and the types compile. Note that `parser.rs` will NOT be reachable from the crate root yet -- the `mod parser;` declaration is added in section 06 (wiring). To make the tests runnable immediately, you can temporarily add `pub mod parser;` to `backend/src/features/analysis/mod.rs`, but be aware section 06 will also add it. Alternatively, just ensure the file compiles via `cargo check` by including the mod declaration now (section 06 will be a no-op for this line).

The pragmatic choice: go ahead and add `pub mod parser;` to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs` now so tests can run. Section 06 handles the tokenizer module declaration and any final wiring.

## File Summary

| File | Action |
|------|--------|
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml` | Modify -- add 3 dependencies |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/parser.rs` | Create -- types, error enum, unit struct |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs` | Modify -- add `pub mod parser;` |

## Dependencies

- **None** -- this section has no dependencies on other sections.
- **Blocks:** sections 02, 03, and 04 all import types defined here.