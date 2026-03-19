use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::extractor::ExtractionResult;

/// Report from validating extracted data against the ontology.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub total_expected: usize,
    pub total_extracted: usize,
    pub missing_concepts: Vec<String>,
    pub unmatched_sections: Vec<String>,
    pub warnings: Vec<String>,
}

/// Minimal representation of an ontology concept for validation.
#[derive(Deserialize)]
struct OntologyConcept {
    id: String,
    concept_type: String,
    code: Option<String>,
}

#[derive(Deserialize)]
struct OntologyFile {
    concepts: Vec<OntologyConcept>,
}

/// Load action-level concepts from the ontology JSON, returning a code-to-id map.
pub fn load_action_concepts(
    ontology_path: &Path,
) -> Result<HashMap<String, String>, String> {
    let data = std::fs::read_to_string(ontology_path)
        .map_err(|e| format!("Failed to read ontology: {e}"))?;
    let ontology: OntologyFile =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse ontology: {e}"))?;

    let map = ontology
        .concepts
        .into_iter()
        .filter(|c| c.concept_type == "action")
        .filter_map(|c| {
            let code = c.code?;
            Some((code, c.id))
        })
        .collect();

    Ok(map)
}

/// Validate extracted sections against the ontology.
pub fn validate(
    result: &ExtractionResult,
    ontology_path: &Path,
    total_pdf_pages: usize,
) -> ValidationReport {
    let action_map = match load_action_concepts(ontology_path) {
        Ok(map) => map,
        Err(e) => {
            return ValidationReport {
                total_expected: 0,
                total_extracted: result.sections.len(),
                missing_concepts: Vec::new(),
                unmatched_sections: Vec::new(),
                warnings: vec![format!("Failed to load ontology: {e}")],
            };
        }
    };

    let total_expected = action_map.len();
    let total_extracted = result.sections.len();

    // Concept coverage: find missing and unmatched
    let extracted_codes: std::collections::HashSet<&str> = result
        .sections
        .iter()
        .map(|s| s.concept_code.as_str())
        .collect();

    let missing_concepts: Vec<String> = action_map
        .iter()
        .filter(|(code, _)| !extracted_codes.contains(code.as_str()))
        .map(|(_, id)| id.clone())
        .collect();

    let unmatched_sections: Vec<String> = result
        .sections
        .iter()
        .filter(|s| !action_map.contains_key(&s.concept_code))
        .map(|s| s.concept_code.clone())
        .collect();

    // Schema conformance warnings
    let mut warnings = Vec::new();

    for section in &result.sections {
        if section.raw_text.is_empty() {
            warnings.push(format!(
                "Section {} has empty raw_text",
                section.concept_code
            ));
        }
        if section.physical_page >= total_pdf_pages {
            warnings.push(format!(
                "Section {} has physical_page {} but PDF only has {} pages",
                section.concept_code, section.physical_page, total_pdf_pages
            ));
        }
        if section.subsections.is_empty() {
            warnings.push(format!(
                "Section {} has no subsections",
                section.concept_code
            ));
        }
    }

    ValidationReport {
        total_expected,
        total_extracted,
        missing_concepts,
        unmatched_sections,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::extraction::extractor::{
        ExtractedSection, ExtractionResult, PageOffsetSource, Subsection, SubsectionKind,
    };
    use chrono::Utc;

    fn ontology_path() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("ontology-data/nist-ai-rmf.json")
    }

    fn make_section(code: &str) -> ExtractedSection {
        ExtractedSection {
            concept_code: code.to_string(),
            concept_id: None,
            physical_page: 5,
            logical_page: 1,
            raw_text: "Some valid content here that is long enough".to_string(),
            subsections: vec![Subsection {
                kind: SubsectionKind::About,
                text: "About text".to_string(),
            }],
        }
    }

    fn make_result(sections: Vec<ExtractedSection>) -> ExtractionResult {
        ExtractionResult {
            framework_id: "nist-ai-rmf".to_string(),
            source_pdf: "test.pdf".to_string(),
            extracted_at: Utc::now(),
            sections,
            page_offset_detected: 0,
            page_offset_source: PageOffsetSource::Default,
        }
    }

    // -- Concept coverage tests --

    #[test]
    fn coverage_all_75_present_reports_no_missing() {
        let path = ontology_path();
        let action_map = load_action_concepts(&path).unwrap();
        assert_eq!(action_map.len(), 75);

        let sections: Vec<ExtractedSection> =
            action_map.keys().map(|code| make_section(code)).collect();
        let result = make_result(sections);

        let report = validate(&result, &path, 100);
        assert_eq!(report.total_expected, 75);
        assert_eq!(report.total_extracted, 75);
        assert!(report.missing_concepts.is_empty());
        assert!(report.unmatched_sections.is_empty());
    }

    #[test]
    fn coverage_one_missing_reports_missing_concept_id() {
        let path = ontology_path();
        let action_map = load_action_concepts(&path).unwrap();

        // Skip one concept
        let sections: Vec<ExtractedSection> = action_map
            .keys()
            .skip(1)
            .map(|code| make_section(code))
            .collect();
        let result = make_result(sections);

        let report = validate(&result, &path, 100);
        assert_eq!(report.total_extracted, 74);
        assert_eq!(report.missing_concepts.len(), 1);
    }

    #[test]
    fn coverage_extra_section_reports_unmatched() {
        let path = ontology_path();
        let action_map = load_action_concepts(&path).unwrap();

        let mut sections: Vec<ExtractedSection> =
            action_map.keys().map(|code| make_section(code)).collect();
        sections.push(make_section("FAKE 99.99"));
        let result = make_result(sections);

        let report = validate(&result, &path, 100);
        assert_eq!(report.total_extracted, 76);
        assert_eq!(report.unmatched_sections.len(), 1);
        assert_eq!(report.unmatched_sections[0], "FAKE 99.99");
    }

    #[test]
    fn coverage_empty_extraction_reports_all_missing() {
        let path = ontology_path();
        let result = make_result(vec![]);

        let report = validate(&result, &path, 100);
        assert_eq!(report.total_extracted, 0);
        assert_eq!(report.missing_concepts.len(), 75);
    }

    // -- Schema conformance tests --

    #[test]
    fn conformance_valid_section_no_warnings() {
        let path = ontology_path();
        let sections = vec![make_section("GOVERN 1.1")];
        let result = make_result(sections);

        let report = validate(&result, &path, 100);
        // Filter to warnings about GOVERN 1.1 specifically
        let gov_warnings: Vec<_> = report
            .warnings
            .iter()
            .filter(|w| w.contains("GOVERN 1.1"))
            .collect();
        assert!(gov_warnings.is_empty());
    }

    #[test]
    fn conformance_empty_raw_text_warns() {
        let path = ontology_path();
        let mut section = make_section("GOVERN 1.1");
        section.raw_text = String::new();
        let result = make_result(vec![section]);

        let report = validate(&result, &path, 100);
        assert!(report.warnings.iter().any(|w| w.contains("empty raw_text")));
    }

    #[test]
    fn conformance_invalid_page_warns() {
        let path = ontology_path();
        let mut section = make_section("GOVERN 1.1");
        section.physical_page = 200;
        let result = make_result(vec![section]);

        let report = validate(&result, &path, 100);
        assert!(report
            .warnings
            .iter()
            .any(|w| w.contains("physical_page")));
    }

    #[test]
    fn conformance_no_subsections_warns() {
        let path = ontology_path();
        let mut section = make_section("GOVERN 1.1");
        section.subsections = vec![];
        let result = make_result(vec![section]);

        let report = validate(&result, &path, 100);
        assert!(report
            .warnings
            .iter()
            .any(|w| w.contains("no subsections")));
    }

    // -- ValidationReport tests --

    #[test]
    fn report_no_issues_all_empty() {
        let report = ValidationReport {
            total_expected: 75,
            total_extracted: 75,
            missing_concepts: vec![],
            unmatched_sections: vec![],
            warnings: vec![],
        };
        assert!(report.missing_concepts.is_empty());
        assert!(report.unmatched_sections.is_empty());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn report_counts_expected_and_extracted() {
        let path = ontology_path();
        let sections = vec![make_section("GOVERN 1.1"), make_section("MAP 1.1")];
        let result = make_result(sections);

        let report = validate(&result, &path, 100);
        assert_eq!(report.total_expected, 75);
        assert_eq!(report.total_extracted, 2);
    }
}
