//! Integration tests for the PDF extraction pipeline.

use std::path::Path;

use ontology_backend::features::extraction::extractor::{ExtractionConfig, OutputFormat};
use ontology_backend::features::extraction::playbook::PlaybookExtractor;
use ontology_backend::features::extraction::validation;

fn ontology_path() -> &'static Path {
    let p = Path::new("../ontology-data/nist-ai-rmf.json");
    assert!(p.exists(), "Ontology file not found at {}", p.display());
    p
}

fn build_test_pages() -> Vec<(usize, String)> {
    vec![
        (
            0,
            "Table of Contents\nGOVERN 1.1 ..... 4\nGOVERN 1.2 ..... 6\nMAP 1.1 ..... 8\n"
                .to_string(),
        ),
        (1, "Introduction to the NIST AI RMF Playbook".to_string()),
        (
            2,
            "GOVERN 1.1\n\n\
             About\n\
             Legal and regulatory requirements involving AI are understood,\n\
             managed, and documented. This includes applicable privacy,\n\
             civil rights, and consumer protection laws.\n\n\
             Suggested Actions\n\
             \u{2022} Establish approaches for detecting, tracking and measuring\n\
               known risks related to AI systems.\n\
             \u{2022} Identify testing procedures and metrics for assessing\n\
               AI system trustworthiness characteristics.\n\n\
             References\n\
             Sara R. Jordan (2019). Designing Artificial Intelligence Review Boards.\n"
                .to_string(),
        ),
        (
            3,
            "GOVERN 1.2\n\n\
             About\n\
             Contingency processes are in place for AI systems.\n\
             Mechanisms are established to manage system-"
                .to_string(),
        ),
        (
            4,
            "level failures and unexpected outcomes.\n\n\
             Suggested Actions\n\
             \u{2022} Document contingency procedures for AI system failures.\n\
             \u{2022} Establish rollback mechanisms for AI deployments.\n\n\
             MAP 1.1\n\n\
             About\n\
             Intended purpose, context of use, and assumptions are identified.\n\n\
             Suggested Actions\n\
             \u{2022} Define the specific use case and deployment context.\n\
             \u{2022} Identify key stakeholders and impacted communities.\n\n\
             References\n\
             NIST AI 100-1 (2023). AI Risk Management Framework.\n"
                .to_string(),
        ),
    ]
}

fn default_config() -> ExtractionConfig {
    ExtractionConfig {
        page_offset_override: Some(0),
        output_format: OutputFormat::Json,
        ontology_path: "../ontology-data/nist-ai-rmf.json".to_string(),
    }
}

#[test]
fn extract_from_fixture_returns_correct_sections() {
    let pages = build_test_pages();
    let extractor = PlaybookExtractor;
    let config = default_config();

    let result = extractor.extract_from_text(&pages, &config).unwrap();

    assert_eq!(
        result.sections.len(),
        3,
        "Expected 3 sections, got {}. Codes: {:?}",
        result.sections.len(),
        result.sections.iter().map(|s| &s.concept_code).collect::<Vec<_>>()
    );
    assert_eq!(result.sections[0].concept_code, "GOVERN 1.1");
    assert_eq!(result.sections[1].concept_code, "GOVERN 1.2");
    assert_eq!(result.sections[2].concept_code, "MAP 1.1");

    // Check subsections exist — at minimum About should be detected
    assert!(
        !result.sections[0].subsections.is_empty(),
        "GOVERN 1.1 should have subsections"
    );

    // Verify the raw text contains expected content
    assert!(
        result.sections[0].raw_text.contains("Legal and regulatory"),
        "GOVERN 1.1 raw_text should contain about text"
    );

    // Check concept IDs resolved
    assert_eq!(
        result.sections[0].concept_id,
        Some("nist-ai-gv-1-1".to_string())
    );

    // Check physical pages
    assert_eq!(result.sections[0].physical_page, 2);
}

#[test]
fn validate_resolves_all_75_concept_ids() {
    let ontology = ontology_path();
    let action_map =
        ontology_backend::features::extraction::validation::load_action_concepts(ontology)
            .unwrap();

    assert_eq!(action_map.len(), 75);

    // Build minimal extraction result with all 75 action codes
    let sections: Vec<ontology_backend::features::extraction::extractor::ExtractedSection> =
        action_map
            .keys()
            .map(|code| {
                ontology_backend::features::extraction::extractor::ExtractedSection {
                    concept_code: code.clone(),
                    concept_id: None,
                    physical_page: 0,
                    logical_page: 0,
                    raw_text: "Test content".to_string(),
                    subsections: vec![
                        ontology_backend::features::extraction::extractor::Subsection {
                            kind:
                                ontology_backend::features::extraction::extractor::SubsectionKind::About,
                            text: "About text".to_string(),
                        },
                    ],
                }
            })
            .collect();

    let result = ontology_backend::features::extraction::extractor::ExtractionResult {
        framework_id: "nist-ai-rmf".to_string(),
        source_pdf: "test.pdf".to_string(),
        extracted_at: chrono::Utc::now(),
        sections,
        page_offset_detected: 0,
        page_offset_source:
            ontology_backend::features::extraction::extractor::PageOffsetSource::Default,
    };

    let report = validation::validate(&result, ontology, 100);
    assert_eq!(report.total_expected, 75);
    assert_eq!(report.total_extracted, 75);
    assert!(
        report.missing_concepts.is_empty(),
        "Missing: {:?}",
        report.missing_concepts
    );
    assert!(
        report.unmatched_sections.is_empty(),
        "Unmatched: {:?}",
        report.unmatched_sections
    );
}

#[test]
fn full_pipeline_produces_valid_json() {
    let pages = build_test_pages();
    let extractor = PlaybookExtractor;
    let config = default_config();

    let result = extractor.extract_from_text(&pages, &config).unwrap();
    let json = serde_json::to_string_pretty(&result).unwrap();

    // Parse back and verify structure
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["framework_id"], "nist-ai-rmf");
    assert!(parsed["extracted_at"].as_str().is_some());
    assert!(parsed["sections"].as_array().is_some());
    assert_eq!(parsed["sections"].as_array().unwrap().len(), 3);
}

#[test]
fn cli_extract_pdf_help() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ontology-backend"))
        .args(["extract-pdf", "--help"])
        .output()
        .expect("Failed to execute binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PDF_PATH"));
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("--validate"));
    assert!(stdout.contains("--page-offset"));
}

#[test]
fn cli_invalid_path_exits_nonzero() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ontology-backend"))
        .args(["extract-pdf", "/nonexistent/path.pdf"])
        .output()
        .expect("Failed to execute binary");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "Expected error message on stderr"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty(), "No partial output expected on stdout");
}
