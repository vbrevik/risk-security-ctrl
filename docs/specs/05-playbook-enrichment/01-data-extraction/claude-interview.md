# Interview Transcript: 01-data-extraction

## Q1: Extraction Approach
**Q:** The spec says "semi-automated: Claude reads PDF pages and generates JSON". Should this be a standalone Python script (using pymupdf4llm) that extracts everything in one run, or should Claude Code itself read the PDF pages interactively and produce the JSON through conversation?

**A:** Claude-assisted extraction — Claude reads the PDF page-by-page in conversation and produces the JSON with human review.

## Q2: Language Choice
**Q:** The existing backend uses Rust's pdf-extract crate for document analysis. Should this extraction reuse the Rust infrastructure, or is a separate Python tool acceptable since this is a one-time data preparation step?

**A:** Rust (extend backend) — Add extraction logic to the existing Rust backend for consistency.

## Q3: Reusability Across PDFs
**Q:** You mentioned planning similar enhancements for the other 3 PDFs (NIST.AI.100-1, NIST.AI.600-1, NIST.SP.800-37r2). Should the extraction tool be designed as a reusable framework that can handle different PDF structures, or is it fine to have separate extraction logic per document?

**A:** Reusable framework — Common extraction pipeline with per-document configuration/templates.

## Q4: Rust's Role in Extraction
**Q:** With Claude-assisted extraction using Rust: should the Rust backend expose a CLI command or API endpoint that extracts raw text per section from the PDF, and then Claude (in conversation) structures that into the final JSON? Or should the Rust code do the full extraction and structuring?

**A:** Rust extracts raw text, Claude structures — Rust CLI outputs raw section text + page numbers; Claude refines into structured JSON with proper field parsing.

## Q5: Plugin Architecture for PDF Types
**Q:** For the reusable framework across all 4 PDFs: the Playbook has a very consistent section structure, but the other PDFs likely have different layouts. What level of configurability should the framework support?

**A:** Plugin architecture — Each PDF type gets a Rust extractor module implementing a common trait.

## Q6: Automated Validation
**Q:** For quality validation of the extracted data: should there be automated validation (schema check, concept ID cross-reference, completeness count) built into the tool, or is manual review of the JSON output sufficient?

**A:** Automated validation built-in — The tool validates against nist-ai-rmf.json concept IDs, checks completeness (75/75), and schema conformance.

## Q7: Page Offset Handling
**Q:** The spec mentions page number offset (TOC page numbers vs physical PDF pages differ by ~4-5). Should the Rust extractor auto-detect this offset by scanning for the TOC, or should it be a configuration parameter per PDF type?

**A:** Both — auto with override. Try auto-detection first, allow manual override in config.

## Q8: CLI Location
**Q:** Where should the Rust CLI extraction command live?

**A:** Subcommand of server binary — `cargo run -- extract-pdf <path>` alongside the serve subcommand.
