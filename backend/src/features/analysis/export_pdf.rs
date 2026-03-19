use crate::features::analysis::charts;
use crate::features::analysis::models::{Analysis, AnalysisFindingWithConcept, FindingType};
use genpdf::Element as _;

#[derive(Debug)]
pub enum ExportError {
    FontLoading(String),
    ChartRendering(String),
    PdfGeneration(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FontLoading(msg) => write!(f, "Font loading error: {msg}"),
            Self::ChartRendering(msg) => write!(f, "Chart rendering error: {msg}"),
            Self::PdfGeneration(msg) => write!(f, "PDF generation error: {msg}"),
        }
    }
}

impl std::error::Error for ExportError {}

/// Generate a PDF report for a completed analysis.
///
/// Returns the raw PDF bytes on success.
pub fn generate_pdf(
    analysis: &Analysis,
    findings: &[AnalysisFindingWithConcept],
    frameworks: &[(String, String)],
) -> Result<Vec<u8>, ExportError> {
    // 1. Load fonts (we only have Regular, Bold, Italic - reuse Bold for BoldItalic)
    let font_dir = std::path::Path::new("./fonts");
    let load = |suffix: &str| -> Result<genpdf::fonts::FontData, ExportError> {
        let path = font_dir.join(format!("LiberationSans-{suffix}.ttf"));
        genpdf::fonts::FontData::new(
            std::fs::read(&path).map_err(|e| {
                ExportError::FontLoading(format!("Failed to read {}: {e}", path.display()))
            })?,
            None,
        )
        .map_err(|e| ExportError::FontLoading(e.to_string()))
    };
    let regular = load("Regular")?;
    let bold = load("Bold")?;
    let italic = load("Italic")?;
    let bold_italic = load("Bold")?; // fallback: reuse Bold
    let font_family = genpdf::fonts::FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };

    // 2. Document setup
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(&analysis.name);
    doc.set_minimal_conformance();
    doc.set_paper_size(genpdf::PaperSize::A4);
    doc.set_font_size(10);

    let decorator = genpdf::SimplePageDecorator::new();
    doc.set_page_decorator(decorator);

    // 3. Title page
    push_title_page(&mut doc, analysis);

    // 4. Executive summary
    push_executive_summary(&mut doc, analysis, findings);

    // 5. Coverage heatmap
    push_coverage_heatmap(&mut doc, findings, frameworks);

    // 6. Per-framework sections
    for (fw_id, fw_name) in frameworks {
        push_framework_section(&mut doc, fw_id, fw_name, findings);
    }

    // 7. Priority breakdown
    push_priority_breakdown(&mut doc, findings);

    // 8. Appendix
    push_appendix(&mut doc, analysis);

    // 9. Render to bytes
    let mut buf = Vec::new();
    doc.render(&mut buf)
        .map_err(|e| ExportError::PdfGeneration(e.to_string()))?;

    Ok(buf)
}

fn push_title_page(doc: &mut genpdf::Document, analysis: &Analysis) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    doc.push(Paragraph::new(&analysis.name).styled(style::Style::new().bold().with_font_size(24)));
    doc.push(Break::new(1));

    if let Some(ref desc) = analysis.description {
        doc.push(Paragraph::new(desc).styled(style::Style::new().with_font_size(12)));
        doc.push(Break::new(0.5));
    }

    let input_type_str: String = analysis.input_type.clone().into();
    let status_str: String = analysis.status.clone().into();

    let meta_lines = [
        format!("Input type: {}", input_type_str),
        format!("Status: {}", status_str),
        format!("Created: {}", analysis.created_at),
        format!(
            "Processing time: {}ms",
            analysis.processing_time_ms.unwrap_or(0)
        ),
        format!("Token count: {}", analysis.token_count.unwrap_or(0)),
    ];

    for line in &meta_lines {
        doc.push(Paragraph::new(line).styled(style::Style::new().with_font_size(9)));
    }

    doc.push(Break::new(1.5));
}

fn push_executive_summary(
    doc: &mut genpdf::Document,
    analysis: &Analysis,
    findings: &[AnalysisFindingWithConcept],
) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    doc.push(
        Paragraph::new("Executive Summary").styled(style::Style::new().bold().with_font_size(16)),
    );
    doc.push(Break::new(0.5));

    let total = findings.len();
    let addressed = findings
        .iter()
        .filter(|f| f.finding_type == FindingType::Addressed)
        .count();
    let partial = findings
        .iter()
        .filter(|f| f.finding_type == FindingType::PartiallyAddressed)
        .count();
    let gaps = findings
        .iter()
        .filter(|f| f.finding_type == FindingType::Gap)
        .count();

    let fw_count = analysis.matched_framework_ids.len();

    let summary = format!(
        "This analysis matched {} framework(s) and produced {} total findings: \
         {} addressed, {} partially addressed, {} gaps.",
        fw_count, total, addressed, partial, gaps
    );
    doc.push(Paragraph::new(&summary));
    doc.push(Break::new(1));
}

fn push_coverage_heatmap(
    doc: &mut genpdf::Document,
    findings: &[AnalysisFindingWithConcept],
    frameworks: &[(String, String)],
) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    if frameworks.is_empty() {
        return;
    }

    doc.push(
        Paragraph::new("Coverage Overview").styled(style::Style::new().bold().with_font_size(16)),
    );
    doc.push(Break::new(0.5));

    // Compute coverage per framework
    let coverages: Vec<(String, f64)> = frameworks
        .iter()
        .map(|(fw_id, fw_name)| {
            let fw_findings: Vec<_> = findings.iter().filter(|f| &f.framework_id == fw_id).collect();
            let total = fw_findings.len() as f64;
            let addressed = fw_findings
                .iter()
                .filter(|f| f.finding_type == FindingType::Addressed)
                .count() as f64;
            let coverage = if total > 0.0 { addressed / total } else { 0.0 };
            (fw_name.clone(), coverage)
        })
        .collect();

    match charts::render_coverage_heatmap(&coverages) {
        Ok(png_bytes) => {
            if let Err(e) = embed_png(doc, &png_bytes) {
                tracing::warn!("Failed to embed coverage heatmap: {e}");
                push_chart_fallback(doc);
            }
        }
        Err(e) => {
            tracing::warn!("Failed to render coverage heatmap: {e}");
            push_chart_fallback(doc);
        }
    }

    doc.push(Break::new(1));
}

fn push_framework_section(
    doc: &mut genpdf::Document,
    fw_id: &str,
    fw_name: &str,
    findings: &[AnalysisFindingWithConcept],
) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    let fw_findings: Vec<_> = findings.iter().filter(|f| f.framework_id == fw_id).collect();
    if fw_findings.is_empty() {
        return;
    }

    doc.push(Paragraph::new(fw_name).styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));

    // Radar chart - compute finding type distribution as normalized values
    let types = ["Addressed", "Partial", "Gap", "N/A"];
    let total = fw_findings.len() as f64;
    let labels: Vec<String> = types.iter().map(|s| s.to_string()).collect();
    let values: Vec<f64> = vec![
        fw_findings
            .iter()
            .filter(|f| f.finding_type == FindingType::Addressed)
            .count() as f64
            / total,
        fw_findings
            .iter()
            .filter(|f| f.finding_type == FindingType::PartiallyAddressed)
            .count() as f64
            / total,
        fw_findings
            .iter()
            .filter(|f| f.finding_type == FindingType::Gap)
            .count() as f64
            / total,
        fw_findings
            .iter()
            .filter(|f| f.finding_type == FindingType::NotApplicable)
            .count() as f64
            / total,
    ];

    match charts::render_radar_chart(&labels, &values) {
        Ok(png_bytes) => {
            if let Err(e) = embed_png(doc, &png_bytes) {
                tracing::warn!("Failed to embed radar chart for {fw_name}: {e}");
                push_chart_fallback(doc);
            }
        }
        Err(e) => {
            tracing::warn!("Failed to render radar chart for {fw_name}: {e}");
            push_chart_fallback(doc);
        }
    }

    doc.push(Break::new(0.5));

    // Findings table as formatted paragraphs
    doc.push(
        Paragraph::new("Findings").styled(style::Style::new().bold().with_font_size(11)),
    );
    doc.push(Break::new(0.3));

    for finding in &fw_findings {
        let finding_type_str: String = finding.finding_type.clone().into();
        let code = finding.concept_code.as_deref().unwrap_or("-");
        let name = truncate(&finding.concept_name_en, 40);
        let confidence = format!("{:.0}%", finding.confidence_score * 100.0);
        let evidence = finding
            .evidence_text
            .as_deref()
            .map(|t| truncate(t, 100))
            .unwrap_or_default();

        let line = format!(
            "P{} | {} | {} | {} | {} | {}",
            finding.priority, code, name, finding_type_str, confidence, evidence
        );
        doc.push(Paragraph::new(&line).styled(style::Style::new().with_font_size(8)));
    }

    doc.push(Break::new(1));
}

fn push_priority_breakdown(doc: &mut genpdf::Document, findings: &[AnalysisFindingWithConcept]) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    if findings.is_empty() {
        return;
    }

    doc.push(
        Paragraph::new("Priority Breakdown")
            .styled(style::Style::new().bold().with_font_size(16)),
    );
    doc.push(Break::new(0.5));

    let mut priority_counts = std::collections::BTreeMap::new();
    for f in findings {
        *priority_counts.entry(f.priority).or_insert(0i64) += 1;
    }

    let priorities: Vec<(String, i64)> = priority_counts
        .into_iter()
        .map(|(p, count)| (format!("P{p}"), count))
        .collect();

    match charts::render_priority_chart(&priorities) {
        Ok(png_bytes) => {
            if let Err(e) = embed_png(doc, &png_bytes) {
                tracing::warn!("Failed to embed priority chart: {e}");
                push_chart_fallback(doc);
            }
        }
        Err(e) => {
            tracing::warn!("Failed to render priority chart: {e}");
            push_chart_fallback(doc);
        }
    }

    doc.push(Break::new(1));
}

fn push_appendix(doc: &mut genpdf::Document, analysis: &Analysis) {
    use genpdf::elements::{Break, Paragraph};
    use genpdf::style;

    if let Some(ref text) = analysis.extracted_text {
        if text.is_empty() {
            return;
        }

        doc.push(
            Paragraph::new("Appendix: Extracted Text")
                .styled(style::Style::new().bold().with_font_size(16)),
        );
        doc.push(Break::new(0.5));

        let truncated = truncate(text, 2000);
        doc.push(Paragraph::new(&truncated).styled(style::Style::new().with_font_size(8)));

        if text.chars().count() > 2000 {
            doc.push(
                Paragraph::new("[Text truncated at 2000 characters]")
                    .styled(style::Style::new().italic().with_font_size(8)),
            );
        }
    }
}

fn push_chart_fallback(doc: &mut genpdf::Document) {
    use genpdf::elements::Paragraph;
    use genpdf::style;
    doc.push(
        Paragraph::new("[Chart could not be rendered]")
            .styled(style::Style::new().italic().with_font_size(9)),
    );
}

/// Scale factor to fit 800px-wide charts within A4 page margins (~482pt usable width).
const CHART_SCALE: f64 = 0.5;

fn embed_png(doc: &mut genpdf::Document, png_bytes: &[u8]) -> Result<(), ExportError> {
    let cursor = std::io::Cursor::new(png_bytes);
    let element = genpdf::elements::Image::from_reader(cursor)
        .map_err(|e| ExportError::PdfGeneration(e.to_string()))?
        .with_scale(genpdf::Scale::new(CHART_SCALE, CHART_SCALE));
    doc.push(element);
    Ok(())
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::analysis::models::{
        AnalysisStatus, FindingType, InputType,
    };

    fn make_test_analysis() -> Analysis {
        Analysis {
            id: "test-analysis-id".to_string(),
            name: "Test Analysis Report".to_string(),
            description: Some("A test description".to_string()),
            input_type: InputType::Pdf,
            input_text: None,
            original_filename: Some("test.pdf".to_string()),
            file_path: None,
            extracted_text: Some("This is extracted test text for the appendix section.".to_string()),
            status: AnalysisStatus::Completed,
            error_message: None,
            prompt_template: None,
            matched_framework_ids: vec!["fw-1".to_string()],
            processing_time_ms: Some(1500),
            token_count: Some(3000),
            created_by: None,
            created_at: "2026-03-19 09:00:00".to_string(),
            updated_at: "2026-03-19 09:01:00".to_string(),
        }
    }

    fn make_test_findings() -> Vec<AnalysisFindingWithConcept> {
        vec![
            AnalysisFindingWithConcept {
                id: "f-1".to_string(),
                analysis_id: "test-analysis-id".to_string(),
                concept_id: "c-1".to_string(),
                framework_id: "fw-1".to_string(),
                finding_type: FindingType::Addressed,
                confidence_score: 0.9,
                evidence_text: Some("Evidence for addressed finding".to_string()),
                recommendation: Some("Continue current approach".to_string()),
                priority: 1,
                sort_order: 0,
                created_at: "2026-03-19 09:00:00".to_string(),
                concept_code: Some("ID.AM-1".to_string()),
                concept_name_en: "Asset Management".to_string(),
                concept_name_nb: "Eiendelsforvaltning".to_string(),
                concept_definition_en: "Systems and assets are identified".to_string(),
                concept_definition_nb: None,
                source_reference: None,
            },
            AnalysisFindingWithConcept {
                id: "f-2".to_string(),
                analysis_id: "test-analysis-id".to_string(),
                concept_id: "c-2".to_string(),
                framework_id: "fw-1".to_string(),
                finding_type: FindingType::Gap,
                confidence_score: 0.7,
                evidence_text: Some("No evidence of risk assessment".to_string()),
                recommendation: Some("Implement risk assessment process".to_string()),
                priority: 2,
                sort_order: 1,
                created_at: "2026-03-19 09:00:00".to_string(),
                concept_code: Some("ID.RA-1".to_string()),
                concept_name_en: "Risk Assessment".to_string(),
                concept_name_nb: "Risikovurdering".to_string(),
                concept_definition_en: "Asset vulnerabilities are identified".to_string(),
                concept_definition_nb: None,
                source_reference: None,
            },
        ]
    }

    #[test]
    fn test_generate_pdf_returns_bytes() {
        let analysis = make_test_analysis();
        let findings = make_test_findings();
        let frameworks = vec![("fw-1".into(), "Test Framework".into())];
        let result = generate_pdf(&analysis, &findings, &frameworks);
        assert!(result.is_ok(), "generate_pdf failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..4], b"%PDF");
    }

    #[test]
    fn test_generate_pdf_contains_analysis_name() {
        let mut analysis = make_test_analysis();
        analysis.name = "UniqueReportTitle2026".to_string();
        let findings = make_test_findings();
        let frameworks = vec![];
        let result = generate_pdf(&analysis, &findings, &frameworks).unwrap();
        let text = String::from_utf8_lossy(&result);
        assert!(text.contains("UniqueReportTitle2026"));
    }

    #[test]
    fn test_generate_pdf_empty_findings() {
        let analysis = make_test_analysis();
        let findings = vec![];
        let frameworks = vec![];
        let result = generate_pdf(&analysis, &findings, &frameworks);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(&bytes[0..4], b"%PDF");
    }

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_long_string() {
        let result = truncate("hello world", 5);
        assert_eq!(result, "hello...");
    }
}
