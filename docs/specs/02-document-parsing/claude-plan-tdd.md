# TDD Plan: 02-document-parsing

## Section 1: Cargo Dependencies
No tests. Validated by `cargo check`.

## Section 2: Parser Types and Error Handling
- Test: ParsingError variants display meaningful messages
- Test: ParsingError::IoError converts from std::io::Error via `?`
- Test: ParsedDocument computes word_count and token_count_estimate correctly
- Test: ParsedDocument with empty text has zero counts

## Section 3: PDF Parser
- Test: parse_pdf with a valid PDF returns non-empty full_text
- Test: parse_pdf returns sections with page numbers
- Test: parse_pdf with empty/corrupt file returns CorruptFile error
- Test: parse_pdf with non-existent path returns IoError
Note: Requires a small test fixture PDF. Create programmatically or include a minimal file.

## Section 4: DOCX Parser
- Test: parse_docx extracts text from a valid DOCX
- Test: parse_docx detects headings as section boundaries
- Test: parse_docx with corrupt ZIP returns CorruptFile
- Test: parse_docx with empty document returns EmptyDocument
Note: Can create minimal DOCX as ZIP with document.xml in tests.

## Section 5: Text Parser and Dispatch
- Test: parse_text with multi-paragraph text returns sections split on blank lines
- Test: parse_text with empty string returns EmptyDocument
- Test: parse_text with whitespace-only returns EmptyDocument
- Test: parse() dispatches .pdf to parse_pdf
- Test: parse() dispatches .docx to parse_docx
- Test: parse() with .txt returns UnsupportedFormat
- Test: parse() with .PDF (uppercase) works (case-insensitive)
- Test: parse() checks file size and returns FileTooLarge for oversized files

## Section 6: Tokenizer
- Test: sentence_split breaks on period+capital
- Test: sentence_split handles newlines
- Test: extract_keywords removes stopwords
- Test: extract_keywords filters short tokens (<3 chars)
- Test: extract_keywords deduplicates
- Test: generate_ngrams produces correct bigrams
- Test: generate_ngrams with n=3 produces trigrams
- Test: term_frequency counts correctly
- Test: term_frequency is case-insensitive

## Section 7: Module Wiring
No tests. Validated by `cargo check` and `cargo test`.
