Now I have everything needed. Here is the section content:

# Section 04: Validation Logic

## Overview

This section implements `backend/src/features/extraction/validation.rs` containing concept coverage checking, schema conformance validation, and `ValidationReport` generation. The validation module loads the ontology JSON (`ontology-data/nist-ai-rmf.json`), builds a lookup map from concept codes to concept IDs, and cross-references extracted sections against the known 75 action-level concepts.

## Dependencies

- **Section 01 (Extractor Trait and Types):** This section depends on the core types defined in section 01, specifically `ExtractedSection`, `ExtractionResult`, and `Subsection`. Those types must exist (at least as stubs) before validation logic can compile.

## File to Create

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/extraction/validation.rs`

This file must be registered as `pub mod validation;` in the extraction feature's `mod.rs` (created in section 01).

## Tests First

All tests go in a `#[cfg(test)] mod tests` block at the bottom of `validation.rs`. Write these before the implementation.

### Concept Coverage Validation Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test: 75/75 extracted sections with matching concept codes -> no missing concepts
    #[test]
    fn coverage_all_75_present_reports_no_missing() { todo!() }

    // Test: 74/75 extracted sections -> reports 1 missing concept with its ID
    #[test]
    fn coverage_one_missing_reports_missing_concept_id() { todo!() }

    // Test: 76 extracted sections where one code doesn't exist in ontology -> reports 1 unmatched
    #[test]
    fn coverage_extra_section_reports_unmatched() { todo!() }

    // Test: 0 extracted sections -> reports all 75 as missing
    #[test]
    fn coverage_empty_extraction_reports_all_missing() { todo!() }
}
```

### Schema Conformance Tests

```rust
    // Test: section with non-empty raw_text and valid physical_page -> passes (no warnings)
    #[test]
    fn conformance_valid_section_no_warnings() { todo!() }

    // Test: section with empty raw_text -> produces a warning string
    #[test]
    fn conformance_empty_raw_text_warns() { todo!() }

    // Test: section with physical_page exceeding total PDF page count -> produces a warning
    #[test]
    fn conformance_invalid_page_warns() { todo!() }

    // Test: section with no subsections -> produces a warning
    #[test]
    fn conformance_no_subsections_warns() { todo!() }
```

### ValidationReport Tests

```rust
    // Test: report with no issues has empty missing/unmatched/warnings vecs
    #[test]
    fn report_no_issues_all_empty() { todo!() }

    // Test: report correctly populates total_expected and total_extracted counts
    #[test]
    fn report_counts_expected_and_extracted() { todo!() }
```

## Implementation Details

### ValidationReport Struct

```rust
/// Summary of validation results after cross-referencing extraction output
/// against the ontology.
pub struct ValidationReport {
    pub total_expected: usize,
    pub total_extracted: usize,
    pub missing_concepts: Vec<String>,   // concept_ids present in ontology but not extracted
    pub unmatched_sections: Vec<String>,  // concept_codes extracted but not in ontology
    pub warnings: Vec<String>,           // non-fatal issues (empty text, bad page, etc.)
}
```

Derive `Debug`, `Clone`, and `serde::Serialize` on this struct so it can be included in JSON output.

### Loading the Ontology and Building the Lookup Map

The validation module needs a function that:

1. Reads the ontology JSON file from disk (path provided as `&Path` argument, typically `ontology-data/nist-ai-rmf.json`)
2. Parses the top-level JSON structure, which has `"concepts": [...]` containing all concepts
3. Filters to only concepts where `"concept_type" == "action"` (there are exactly 75 of these)
4. Builds a `HashMap<String, String>` mapping the `"code"` field (e.g., `"GOVERN 1.1"`) to the `"id"` field (e.g., `"nist-ai-gv-1-1"`)

The ontology JSON structure looks like:

```json
{
  "framework": { "id": "nist-ai-rmf", ... },
  "concepts": [
    {
      "id": "nist-ai-gv-1-1",
      "concept_type": "action",
      "code": "GOVERN 1.1",
      ...
    }
  ]
}
```

Key design point from the plan: **Do not** algorithmically convert function names to prefixes (e.g., "GOVERN" to "gv-"). The ontology JSON is the source of truth. NIST AI RMF uses abbreviated prefixes (`gv-`, `mp-`, `ms-`, `mg-`) in concept IDs, not the full function names. Always look up the `code` field in the ontology data rather than constructing IDs programmatically.

Use a minimal serde model for deserialization -- you only need `id`, `code`, and `concept_type` from each concept entry:

```rust
/// Minimal representation of an ontology concept for validation purposes.
#[derive(serde::Deserialize)]
struct OntologyConcept {
    id: String,
    concept_type: String,
    code: Option<String>,
}

#[derive(serde::Deserialize)]
struct OntologyFile {
    concepts: Vec<OntologyConcept>,
}
```

### `resolve_concept_id` Function

This is a shared utility (likely also re-exported from `extractor.rs` per section 01), but the lookup map is built here:

```rust
/// Given a concept code like "GOVERN 1.1" and a code-to-id map,
/// returns the matching concept_id if it exists.
pub fn resolve_concept_id(code: &str, code_map: &HashMap<String, String>) -> Option<String>;
```

The map is built once by `load_action_concepts(ontology_path: &Path) -> Result<HashMap<String, String>, ...>` and passed into both validation and the resolve function.

### Concept Coverage Checking

The core validation function signature:

```rust
/// Validate extracted sections against the ontology.
/// - `result`: the extraction output containing a list of `ExtractedSection`
/// - `ontology_path`: path to the ontology JSON file
/// - `total_pdf_pages`: total number of physical pages in the source PDF (for page range checks)
pub fn validate(result: &ExtractionResult, ontology_path: &Path, total_pdf_pages: usize) -> ValidationReport;
```

Logic:

1. Load the ontology and build the code-to-id map (75 entries for actions)
2. Set `total_expected` to the number of action concepts in the map
3. Set `total_extracted` to `result.sections.len()`
4. For each action concept in the ontology map that has no matching `concept_code` in the extracted sections, add its `concept_id` to `missing_concepts`
5. For each extracted section whose `concept_code` does not appear in the ontology map, add the code to `unmatched_sections`
6. Also resolve and populate `concept_id` on each extracted section that has a match (mutating the section or returning the resolved IDs for the caller to apply)

### Schema Conformance Checking

As part of the same `validate` call (or a separate `check_conformance` helper), iterate over all extracted sections and produce warnings for:

- `raw_text` is empty: `"Section {concept_code} has empty raw_text"`
- `physical_page >= total_pdf_pages`: `"Section {concept_code} has physical_page {n} but PDF only has {total} pages"`
- `subsections` is empty: `"Section {concept_code} has no subsections"`

All warnings are appended to `ValidationReport::warnings`.

### Integration with PdfExtractor Trait

The `PdfExtractor` trait (from section 01) has a `validate` method. For `PlaybookExtractor` (section 03), that method should delegate to this module's `validate` function. The validation module is designed to be generic enough for any extractor that targets the NIST AI RMF ontology, but the `total_expected` count (75) is specific to the Playbook. Future extractors targeting different PDFs for the same framework would reuse the same validation logic since it counts action concepts dynamically from the ontology JSON.

## Test Data Strategy

For unit tests in this module, construct `ExtractionResult` values in-memory with synthetic `ExtractedSection` entries. You do not need an actual PDF or the real ontology file for most tests.

For the ontology lookup tests, either:
- Create a small inline JSON string with 2-3 mock action concepts and deserialize it, or
- Load the real `ontology-data/nist-ai-rmf.json` using a path relative to the cargo workspace root (use `env!("CARGO_MANIFEST_DIR")` to locate it reliably, then navigate to `../ontology-data/nist-ai-rmf.json`)

The real ontology file contains exactly 75 action concepts. The integration test in section 06 will verify all 75 resolve correctly; unit tests here should focus on the logic (missing, unmatched, warnings) using small synthetic data.