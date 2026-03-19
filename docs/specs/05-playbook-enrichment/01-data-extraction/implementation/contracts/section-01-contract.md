# Section 01 Prompt Contract: Extractor Trait and Types

## GOAL
Create the foundational extraction module with PdfExtractor trait, core data types, error types, and utility functions (read_pdf_pages, resolve_concept_id).

## CONTEXT
First section of playbook enrichment data extraction. Establishes types that all subsequent sections depend on. Must follow existing feature-module pattern.

## CONSTRAINTS
- Follow feature-based module pattern at `backend/src/features/extraction/`
- Use thiserror for error types, serde for serialization
- PdfExtractor trait must be synchronous (CPU-bound work)
- resolve_concept_id must use ontology JSON as source of truth, not algorithmic prefix conversion
- All types need Debug, Clone, Serialize, Deserialize where appropriate
- Stub modules for page_offset, playbook, validation, cli must compile

## FORMAT
### Files to create:
- `backend/src/features/extraction/mod.rs`
- `backend/src/features/extraction/extractor.rs` (trait + types + tests)
- `backend/src/features/extraction/page_offset.rs` (stub)
- `backend/src/features/extraction/playbook/mod.rs` (stub)
- `backend/src/features/extraction/validation.rs` (stub)
- `backend/src/features/extraction/cli.rs` (stub)

### Files to modify:
- `backend/src/features/mod.rs` (add `pub mod extraction;`)

## FAILURE CONDITIONS
- SHALL NOT skip tests for ExtractionError, resolve_concept_id, or read_pdf_pages
- SHALL NOT use algorithmic prefix conversion for concept IDs
- SHALL NOT introduce async in the PdfExtractor trait
- SHALL NOT add new crate dependencies (all already in Cargo.toml)
