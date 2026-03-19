# Section 01 Code Review Interview

## User Decisions
- **resolve_concept_id performance**: User chose "Optimize now" — split into `build_concept_code_map()` + `resolve_concept_id(code, &map)` to avoid per-call I/O

## Auto-fixes Applied
- TOCTOU race in read_pdf_pages: Removed exists() check, map NotFound from std::fs::read directly
- PartialEq derives: Added to ExtractionConfig, OutputFormat, PageOffsetSource, ValidationReport
- Test deviation documented: Added comment to subcategory test explaining ontology data differs from plan

## Let Go
- ValidationReport location in validation.rs (better separation)
- Form-feed splitting (deferred to integration tests)
