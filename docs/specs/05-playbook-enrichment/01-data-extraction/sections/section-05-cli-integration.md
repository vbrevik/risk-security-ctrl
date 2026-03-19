Good -- the extraction module does not exist yet; it would be created by earlier sections (01, 03, 04). Now I have everything needed.

# Section 05: CLI Integration

## Overview

This section adds the `extract-pdf` CLI subcommand to the existing server binary. It introduces `clap` as a new dependency, refactors `main.rs` to dispatch between `Serve` (default, backward-compatible) and `ExtractPdf` subcommands, implements input validation, output formatting, and error handling in a new `cli.rs` module, and wires together the extractor pipeline from earlier sections.

**Dependencies on prior sections:**
- Section 01 provides `PdfExtractor` trait, `ExtractionConfig`, `ExtractionResult`, `OutputFormat`, `ExtractionError`, and the `read_pdf_pages` utility in `backend/src/features/extraction/extractor.rs`
- Section 03 provides `PlaybookExtractor` in `backend/src/features/extraction/playbook/mod.rs`
- Section 04 provides `ValidationReport` and validation logic in `backend/src/features/extraction/validation.rs`

---

## Files to Create or Modify

| File | Action |
|------|--------|
| `backend/Cargo.toml` | Add `clap` dependency with `derive` feature |
| `backend/src/features/extraction/cli.rs` | **Create** -- CLI argument structs and `run_extract` handler |
| `backend/src/main.rs` | Refactor to add clap subcommand dispatch |
| `backend/src/features/mod.rs` | Register `extraction` module (may already be done by section 01) |

---

## Tests (Write First)

All CLI-related tests live in `backend/src/features/extraction/cli.rs` inside a `#[cfg(test)] mod tests` block, plus an integration test in `backend/tests/extraction_tests.rs` (section 06 expands these).

### Unit Tests for Argument Parsing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Test: parse "extract-pdf /path/to/file.pdf" with defaults
    /// Expects: pdf_path = "/path/to/file.pdf", extractor_type = None,
    ///          page_offset = None, output = None, format = Json, validate = None
    #[test]
    fn parse_extract_pdf_defaults() { todo!() }

    /// Test: parse with --type playbook --page-offset 4 --output out.json
    /// Expects: extractor_type = Some("playbook"), page_offset = Some(4), output = Some("out.json")
    #[test]
    fn parse_extract_pdf_all_options() { todo!() }

    /// Test: parse with --validate /path/to/ontology.json
    /// Expects: validate = Some(PathBuf from "/path/to/ontology.json")
    #[test]
    fn parse_extract_pdf_with_validate() { todo!() }

    /// Test: no subcommand defaults to serve behavior
    /// When Cli is parsed with no args, the command field should be None
    #[test]
    fn no_subcommand_defaults_to_serve() { todo!() }
}
```

### Unit Tests for Input Validation

```rust
#[cfg(test)]
mod tests {
    // (continued in same module)

    /// Test: rejects non-existent file path
    /// validate_pdf_path("nonexistent.pdf") returns Err(ExtractionError::FileNotFound(_))
    #[test]
    fn rejects_nonexistent_path() { todo!() }

    /// Test: rejects directory path
    /// validate_pdf_path("/tmp") returns Err(ExtractionError::InvalidPdf(_))
    #[test]
    fn rejects_directory_path() { todo!() }

    /// Test: rejects non-.pdf extension
    /// Create a temp file with .txt extension, call validate_pdf_path, expect Err
    #[test]
    fn rejects_non_pdf_extension() { todo!() }

    /// Test: rejects file without %PDF magic bytes
    /// Create a temp .pdf file containing "not a pdf", expect Err(ExtractionError::InvalidPdf(_))
    #[test]
    fn rejects_missing_magic_bytes() { todo!() }

    /// Test: accepts valid PDF file path after canonicalization
    /// Create a temp .pdf file with "%PDF-1.4" as first bytes, expect Ok(canonical_path)
    #[test]
    fn accepts_valid_pdf_path() { todo!() }
}
```

### Unit Tests for Error Handling

```rust
#[cfg(test)]
mod tests {
    // (continued in same module)

    /// Test: extraction error produces non-zero exit code
    /// run_extract with invalid path returns Err, caller maps to non-zero exit
    #[test]
    fn extraction_error_produces_nonzero_exit() { todo!() }

    /// Test: partial output file is not left behind on failure
    /// Specify --output /tmp/test_out.json, trigger extraction error,
    /// verify /tmp/test_out.json does not exist
    #[test]
    fn no_partial_output_on_failure() { todo!() }
}
```

---

## Implementation Details

### 1. Add `clap` Dependency

In `backend/Cargo.toml`, add under `[dependencies]`:

```toml
clap = { version = "4", features = ["derive"] }
```

### 2. CLI Argument Structs (`backend/src/features/extraction/cli.rs`)

Define clap-derived structs for argument parsing and a handler function:

```rust
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Top-level CLI definition. The server binary uses this to decide
/// whether to run the Axum server (default) or execute a subcommand.
#[derive(Parser, Debug)]
#[command(name = "ontology-backend", about = "Risk Management Framework Explorer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Extract structured data from a NIST reference PDF
    ExtractPdf(ExtractPdfArgs),
}

#[derive(clap::Args, Debug)]
pub struct ExtractPdfArgs {
    /// Path to the PDF file to extract
    pub pdf_path: PathBuf,

    /// Extractor type (auto-detected from PDF content if omitted)
    #[arg(long = "type", value_enum)]
    pub extractor_type: Option<ExtractorType>,

    /// Manual page offset override
    #[arg(long)]
    pub page_offset: Option<i32>,

    /// Output file path (stdout if omitted)
    #[arg(long, short)]
    pub output: Option<PathBuf>,

    /// Output format
    #[arg(long, value_enum, default_value = "json")]
    pub format: CliOutputFormat,

    /// Path to ontology JSON for validation
    #[arg(long)]
    pub validate: Option<PathBuf>,

    /// Show detailed extraction progress
    #[arg(long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExtractorType {
    Playbook,
    // Future: Ai1001, Ai6001, Sp80037
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CliOutputFormat {
    Json,
    Markdown,
    Raw,
}
```

### 3. Input Validation Function

A standalone function that validates and canonicalizes the PDF path before passing it to an extractor. This satisfies STIG V-222605 (path traversal control):

```rust
/// Validate that the given path points to a readable PDF file.
/// Returns the canonicalized path on success.
///
/// Checks performed:
/// 1. Canonicalize (resolve symlinks)
/// 2. Exists and is a regular file (not directory/device)
/// 3. Has .pdf extension
/// 4. First 4 bytes are "%PDF" magic bytes
pub fn validate_pdf_path(path: &Path) -> Result<PathBuf, ExtractionError> {
    // Implementation: use std::fs::canonicalize, metadata().is_file(),
    // extension check, and read first 4 bytes with std::fs::File
    todo!()
}
```

### 4. The `run_extract` Handler

The main handler function that orchestrates the extraction pipeline:

```rust
/// Execute the extract-pdf subcommand.
/// Called from main.rs when the ExtractPdf subcommand is matched.
///
/// Steps:
/// 1. Validate input path (validate_pdf_path)
/// 2. Select extractor (auto-detect or --type)
/// 3. Build ExtractionConfig from CLI args
/// 4. Call extractor.extract()
/// 5. Optionally run validation if --validate is provided
/// 6. Format output (JSON/markdown/raw)
/// 7. Write to --output file (temp-file-then-rename) or stdout
///
/// Error safety: if --output is specified, writes to a temp file in the
/// same directory first, then renames atomically on success. On any error,
/// the temp file is cleaned up and no partial output remains.
pub fn run_extract(args: ExtractPdfArgs) -> Result<(), ExtractionError> {
    todo!()
}
```

**Auto-detection logic:** When `--type` is omitted, read the first few pages of text from the PDF and check for signature strings. If the text contains "AI Risk Management Framework Playbook", select `PlaybookExtractor`. Otherwise, return an error asking the user to specify `--type` explicitly.

**Temp-file-then-rename pattern (STIG V-222585):** When `--output` is specified:
1. Create a temp file in the same directory as the output path (use `tempfile::NamedTempFile::new_in(parent_dir)`)
2. Write the formatted output to the temp file
3. Call `temp_file.persist(output_path)` to atomically rename
4. If any step fails, the temp file is automatically cleaned up by `NamedTempFile`'s `Drop` implementation

### 5. Refactor `main.rs`

The existing `main.rs` currently has no argument parsing and directly boots the Axum server. Refactor it to parse CLI arguments first:

```rust
// At the top of main.rs, add:
use ontology_backend::features::extraction::cli::{Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ExtractPdf(args)) => {
            // Initialize minimal tracing (no DB needed)
            // ...tracing setup...
            
            if let Err(e) = ontology_backend::features::extraction::cli::run_extract(args) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
            Ok(())
        }
        None => {
            // Original server startup code (everything currently in main)
            // Move the entire existing body here, unchanged
            // ...dotenvy, tracing, config, db, migrations, ontology import, router, serve...
            Ok(())
        }
    }
}
```

**Backward compatibility:** When no subcommand is provided (`cli.command` is `None`), the binary behaves exactly as before -- it starts the Axum server. Existing `cargo run` invocations with no arguments are unaffected. The `ExtractPdf` path initializes only tracing (not the database or server), keeping it lightweight.

**Important:** The `ExtractPdf` branch should NOT initialize the database connection pool, run migrations, or start the HTTP server. It only needs tracing for diagnostic output and then calls `run_extract` synchronously.

### 6. Register the Extraction Module

In `backend/src/features/mod.rs`, add:

```rust
pub mod extraction;
```

This line may already be present if section 01 was implemented first. Verify before adding a duplicate.

---

## Output Formatting

The `run_extract` function formats the `ExtractionResult` based on the `--format` flag:

- **`json`** (default): Serialize `ExtractionResult` with `serde_json::to_string_pretty`. If validation was run, include the `ValidationReport` as a top-level `"validation"` key in the JSON output.
- **`markdown`**: Render each section as a markdown heading with concept code, page number, and raw text. Useful for human review.
- **`raw`**: Output only the concatenated `raw_text` of each section separated by section dividers. Minimal formatting for piping into other tools.

---

## Key Design Decisions

1. **clap `derive` feature:** Chosen over the builder API for compile-time validation of argument definitions and less boilerplate. The `#[command(subcommand)]` pattern with `Option<Commands>` allows the `None` case to fall through to serve mode.

2. **Synchronous `run_extract`:** The extraction pipeline is CPU-bound (PDF text extraction, regex matching, JSON serialization). It does not need async. It runs inside the tokio runtime (because `main` is `#[tokio::main]`) but does not spawn tasks or use `.await`.

3. **`tempfile` crate for safe output:** Already in `dev-dependencies`; may need to be moved to `[dependencies]` if not already there, or use `std::fs` manual temp file creation. The `tempfile` crate's `NamedTempFile::persist()` provides atomic rename semantics.

4. **Error messages to stderr:** All error output from the `ExtractPdf` path goes to `eprintln!`, keeping stdout clean for the extraction output itself. This allows `cargo run -- extract-pdf file.pdf > output.json` to work correctly even when errors occur.

---

## Checklist

- [ ] Add `clap = { version = "4", features = ["derive"] }` to `backend/Cargo.toml`
- [ ] Optionally move `tempfile` from `[dev-dependencies]` to `[dependencies]` (or add it)
- [ ] Create `backend/src/features/extraction/cli.rs` with `Cli`, `Commands`, `ExtractPdfArgs` structs
- [ ] Implement `validate_pdf_path` with canonicalize, is_file, extension, and magic bytes checks
- [ ] Implement `run_extract` with extractor selection, pipeline orchestration, and output formatting
- [ ] Implement temp-file-then-rename for `--output` flag
- [ ] Refactor `backend/src/main.rs` to parse `Cli` and dispatch on `command`
- [ ] Ensure `pub mod extraction;` is in `backend/src/features/mod.rs`
- [ ] Write all unit tests for argument parsing, input validation, and error handling
- [ ] Verify `cargo run` with no args still starts the server (backward compatibility)
- [ ] Verify `cargo run -- extract-pdf --help` shows usage information