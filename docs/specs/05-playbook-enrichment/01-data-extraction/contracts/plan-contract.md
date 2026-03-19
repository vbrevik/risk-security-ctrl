# Prompt Contract: claude-plan.md

## GOAL
Deliver a self-contained prose blueprint for building a reusable PDF extraction framework in Rust with a Playbook-specific plugin, CLI integration, and automated validation — enabling Claude-assisted structured data extraction from NIST reference PDFs.

## CONTEXT
This plan drives all downstream section files and implementation via deep-implement. An engineer or LLM with no prior context must understand what to build, why, and how just from reading this document. The extraction framework will be reused for 3 additional NIST PDFs after the Playbook.

## CONSTRAINTS
- Plans are prose documents with minimal code (type definitions, signatures, directory structure only)
- Zero full function implementations — that is deep-implement's job
- Must follow existing codebase patterns (Axum, feature-based modules, pdf-extract crate)
- CLI subcommand of existing server binary, not a new binary
- Plugin architecture: common `PdfExtractor` trait with per-PDF-type implementations
- Rust extracts raw text + page numbers; Claude structures into final JSON in conversation
- Auto-detect page offset with manual override
- Built-in validation against ontology concept IDs

## FORMAT
Single file `claude-plan.md` with sections mapping to implementable units:
1. Extractor trait and shared utilities
2. Playbook-specific plugin implementation
3. CLI subcommand integration
4. Page offset detection
5. Validation logic
6. Output schema and companion file
7. Testing strategy

## FAILURE CONDITIONS
- SHALL NOT contain full function bodies
- SHALL NOT assume reader has prior context
- SHALL NOT omit testing strategy
- SHALL NOT add features beyond the spec
- SHALL NOT introduce Python dependencies (extraction is Rust-based)
- SHALL NOT create a separate binary target

## STIG Constraints (auto-detected: input-validation, error-handling)

Minimal applicability — this is a local CLI tool, not a web service. Two controls apply:

- **V-222605 (CAT II)**: Canonicalize file paths before access. The CLI accepts a PDF path argument; validate it exists and resolve symlinks before opening. Do not allow path traversal outside expected directories.
- **V-222585 (CAT I)**: Fail to secure state on errors. If PDF parsing fails (corrupt file, missing pages, unexpected structure), exit with a clear error message and non-zero exit code. Do not produce partial/invalid output files.
