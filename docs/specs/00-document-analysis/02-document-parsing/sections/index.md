<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-deps-and-types
section-02-pdf-parser
section-03-docx-parser
section-04-text-parser-dispatch
section-05-tokenizer
section-06-wiring
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-deps-and-types | - | 02, 03, 04 | Yes |
| section-02-pdf-parser | 01 | 04 | Yes |
| section-03-docx-parser | 01 | 04 | Yes |
| section-04-text-parser-dispatch | 01, 02, 03 | 06 | No |
| section-05-tokenizer | - | 06 | Yes |
| section-06-wiring | 04, 05 | - | No |

## Execution Order

1. section-01-deps-and-types, section-05-tokenizer (parallel, no deps)
2. section-02-pdf-parser, section-03-docx-parser (parallel after 01)
3. section-04-text-parser-dispatch (after 02+03)
4. section-06-wiring (after all)

## Section Summaries

### section-01-deps-and-types
Add pdf-extract, zip, quick-xml to Cargo.toml. Create parser.rs with ParsingError, ParsedDocument, DocumentSection types.

### section-02-pdf-parser
DocumentParser::parse_pdf() using pdf-extract. Page-by-page extraction with sections.

### section-03-docx-parser
DocumentParser::parse_docx() using zip + quick-xml. Heading detection for sections.

### section-04-text-parser-dispatch
DocumentParser::parse_text() and parse() dispatch with file size check and tracing.

### section-05-tokenizer
tokenizer.rs with sentence_split, extract_keywords, generate_ngrams, term_frequency.

### section-06-wiring
Update analysis/mod.rs to export parser and tokenizer modules.
