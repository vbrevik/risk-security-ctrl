# Specification: Playbook Data Extraction

## Overview

Build a reusable PDF extraction framework in Rust, exposed as a CLI subcommand of the existing server binary, that extracts raw structured text from NIST reference PDFs. The first implementation targets the NIST AI RMF Playbook PDF, extracting guidance data for all 75 action-level concepts into a companion JSON file (`ontology-data/nist-ai-rmf-guidance.json`).

The extraction follows a two-phase workflow:
1. **Rust CLI** extracts raw text per section with page numbers from the PDF
2. **Claude (in conversation)** structures the raw output into the final JSON schema with human review

## Architecture Decisions

### Plugin-Based Extractor Framework
Each PDF type gets a Rust module implementing a common `PdfExtractor` trait. This supports:
- The AI RMF Playbook (75 action sections with consistent About/Actions/Transparency/Resources/References layout)
- Future extractors for NIST.AI.100-1, NIST.AI.600-1, NIST.SP.800-37r2
- Shared utilities: PDF page reading, TOC parsing, page offset detection

### CLI Integration
The extractor is a subcommand of the existing server binary: `cargo run -- extract-pdf <path> [--type playbook] [--page-offset N]`

### Page Offset Handling
Auto-detect the offset between physical PDF pages and logical page numbers (from TOC or `/PageLabels`), with a manual `--page-offset` override flag.

### Automated Validation
Built-in validation that checks:
- All 75 action concept IDs from `nist-ai-rmf.json` have entries
- Every `concept_id` in output matches an existing concept
- Schema conformance (required fields present, correct types)
- Completeness count (75/75 for Playbook)

## Output Schema

File: `ontology-data/nist-ai-rmf-guidance.json`

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "AI_RMF_Playbook.pdf",
  "extracted_at": "2026-03-19T14:00:00Z",
  "guidance": [
    {
      "concept_id": "nist-ai-ms-1-1",
      "source_page": 98,
      "about_en": "...",
      "suggested_actions_en": ["..."],
      "transparency_questions_en": ["..."],
      "resources": [{ "title": "...", "url": null, "type": "transparency" }],
      "references": [{ "title": "...", "authors": "...", "year": 2019, "venue": "...", "url": null }]
    }
  ]
}
```

## Dependencies

- **Existing:** `pdf-extract = "0.10"` crate already in `backend/Cargo.toml`
- **Existing:** `ontology-data/nist-ai-rmf.json` provides concept IDs and codes
- **Input:** `docs/reference-pdfs/AI_RMF_Playbook.pdf` (already downloaded)
- **Output:** `ontology-data/nist-ai-rmf-guidance.json` (new companion file)

## Quality Criteria

- All 75 action concepts have entries
- `about_en` captures full About section text (not truncated)
- `suggested_actions_en` preserves exact PDF wording
- References parse author/year where citation format allows
- Page numbers are correct (accounting for physical vs logical offset)

## Constraints

- English only (`_en` fields); schema supports `_nb` for later
- Only the Playbook PDF in this phase; framework supports future PDFs
- Uses existing `pdf-extract` crate (Rust), not Python libraries
- CLI subcommand, not a new binary
