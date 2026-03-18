# Section 06: Module Wiring

## Overview

This section wires the two new modules created by previous sections -- `parser.rs` (sections 01-04) and `tokenizer.rs` (section 05) -- into the existing `analysis` feature module. This is pure module registration; no new types, logic, or routes are added.

## Dependencies

- **section-04-text-parser-dispatch**: The file `backend/src/features/analysis/parser.rs` must exist with all parser types and the `DocumentParser` struct.
- **section-05-tokenizer**: The file `backend/src/features/analysis/tokenizer.rs` must exist with `sentence_split`, `extract_keywords`, `generate_ngrams`, and `term_frequency` functions.

## Tests

There are no dedicated unit tests for this section. Validation is performed by running:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend
cargo check
cargo test
```

Both commands must pass without errors. `cargo check` confirms that the module declarations resolve correctly and all symbols are visible. `cargo test` confirms that the tests written in sections 02-05 (which live inside `parser.rs` and `tokenizer.rs` via `#[cfg(test)] mod tests`) compile and run through the module tree.

## Implementation

### File to modify: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs`

The current contents of this file are:

```rust
pub mod engine;
pub mod models;
```

Add two `pub mod` declarations for the new modules:

```rust
pub mod engine;
pub mod models;
pub mod parser;
pub mod tokenizer;
```

That is the only change required. Both modules are declared `pub` so that downstream consumers (the matching engine in split 03 and the upload route handler in split 04) can access them via `crate::features::analysis::parser::DocumentParser` and `crate::features::analysis::tokenizer::*`.

### No other files need changes

This split deliberately does not add any HTTP routes or handlers. The parser and tokenizer are library code only. Route wiring happens in split 04 (backend-api-export), which will call `DocumentParser::parse()` and `DocumentParser::parse_text()` from within an Axum handler wrapped in `tokio::task::spawn_blocking`.

## Verification Checklist

1. `cargo check` succeeds -- confirms module declarations are correct and all imports resolve.
2. `cargo test` succeeds -- confirms all tests from sections 02-05 execute through the module tree.
3. `cargo clippy` produces no new warnings in the analysis module.
4. The symbols `crate::features::analysis::parser::DocumentParser`, `crate::features::analysis::parser::ParsedDocument`, `crate::features::analysis::parser::ParsingError`, `crate::features::analysis::tokenizer::sentence_split`, `crate::features::analysis::tokenizer::extract_keywords`, `crate::features::analysis::tokenizer::generate_ngrams`, and `crate::features::analysis::tokenizer::term_frequency` are all publicly accessible from other modules in the crate.