diff --git a/backend/src/features/analysis/export_docx.rs b/backend/src/features/analysis/export_docx.rs
new file mode 100644
index 0000000..0500f56
--- /dev/null
+++ b/backend/src/features/analysis/export_docx.rs
@@ -0,0 +1,537 @@
+use std::collections::BTreeMap;
+
+use docx_rs::*;
+
+use crate::features::analysis::charts;
+use crate::features::analysis::models::{Analysis, AnalysisFindingWithConcept, FindingType};
+
+/// Error type for DOCX export failures.
+#[derive(Debug)]
+pub enum DocxExportError {
+    ChartRendering(String),
+    Generation(String),
+    Io(std::io::Error),
+}
+
+impl std::fmt::Display for DocxExportError {
+    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
+        match self {
+            Self::ChartRendering(msg) => write!(f, "Chart rendering error: {msg}"),
+            Self::Generation(msg) => write!(f, "DOCX generation error: {msg}"),
+            Self::Io(e) => write!(f, "I/O error: {e}"),
+        }
+    }
+}
+
+impl std::error::Error for DocxExportError {}
+
+impl From<std::io::Error> for DocxExportError {
+    fn from(e: std::io::Error) -> Self {
+        Self::Io(e)
+    }
+}
+
+/// EMUs per pixel (English Metric Units).
+const EMU_PER_PX: u32 = 9525;
+
+/// Chart image dimensions in pixels (scaled down from 800x600 originals).
+const CHART_WIDTH_PX: u32 = 400;
+const CHART_HEIGHT_PX: u32 = 300;
+
+/// Generate a DOCX report for a completed analysis.
+///
+/// Returns the raw DOCX bytes (ZIP format) on success.
+pub fn generate_docx(
+    analysis: &Analysis,
+    findings: &[AnalysisFindingWithConcept],
+    frameworks: &[(String, String)],
+) -> Result<Vec<u8>, DocxExportError> {
+    // Define heading styles
+    let heading1_style = Style::new("Heading1", StyleType::Paragraph)
+        .name("Heading 1")
+        .bold()
+        .size(48); // 24pt (size is in half-points)
+
+    let heading2_style = Style::new("Heading2", StyleType::Paragraph)
+        .name("Heading 2")
+        .bold()
+        .size(32); // 16pt
+
+    let heading3_style = Style::new("Heading3", StyleType::Paragraph)
+        .name("Heading 3")
+        .bold()
+        .size(28); // 14pt
+
+    let mut docx = Docx::new()
+        .add_style(heading1_style)
+        .add_style(heading2_style)
+        .add_style(heading3_style);
+
+    // 1. Title page
+    docx = add_title_page(docx, analysis);
+
+    // 2. Executive summary
+    docx = add_executive_summary(docx, analysis, findings);
+
+    // 3. Coverage heatmap
+    docx = add_coverage_heatmap(docx, findings, frameworks);
+
+    // 4. Per-framework sections
+    for (fw_id, fw_name) in frameworks {
+        docx = add_framework_section(docx, fw_id, fw_name, findings);
+    }
+
+    // 5. Priority breakdown
+    docx = add_priority_breakdown(docx, findings);
+
+    // 6. Appendix
+    docx = add_appendix(docx, analysis);
+
+    // Render to bytes
+    let mut buf = Vec::new();
+    let cursor = std::io::Cursor::new(&mut buf);
+    docx.build()
+        .pack(cursor)
+        .map_err(|e| DocxExportError::Generation(format!("{e}")))?;
+    Ok(buf)
+}
+
+fn add_title_page(docx: Docx, analysis: &Analysis) -> Docx {
+    let input_type_str: String = analysis.input_type.clone().into();
+    let status_str: String = analysis.status.clone().into();
+
+    let meta_lines = [
+        format!("Input type: {input_type_str}"),
+        format!("Status: {status_str}"),
+        format!("Created: {}", analysis.created_at),
+        format!(
+            "Processing time: {}ms",
+            analysis.processing_time_ms.unwrap_or(0)
+        ),
+        format!("Token count: {}", analysis.token_count.unwrap_or(0)),
+    ];
+
+    let mut d = docx.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text(&analysis.name))
+            .style("Heading1"),
+    );
+
+    if let Some(ref desc) = analysis.description {
+        d = d.add_paragraph(Paragraph::new().add_run(Run::new().add_text(desc)));
+    }
+
+    for line in &meta_lines {
+        d = d.add_paragraph(
+            Paragraph::new().add_run(Run::new().add_text(line).size(18)), // 9pt
+        );
+    }
+
+    // Empty paragraph as spacer
+    d = d.add_paragraph(Paragraph::new());
+
+    d
+}
+
+fn add_executive_summary(
+    docx: Docx,
+    analysis: &Analysis,
+    findings: &[AnalysisFindingWithConcept],
+) -> Docx {
+    let total = findings.len();
+    let addressed = findings
+        .iter()
+        .filter(|f| f.finding_type == FindingType::Addressed)
+        .count();
+    let partial = findings
+        .iter()
+        .filter(|f| f.finding_type == FindingType::PartiallyAddressed)
+        .count();
+    let gaps = findings
+        .iter()
+        .filter(|f| f.finding_type == FindingType::Gap)
+        .count();
+    let fw_count = analysis.matched_framework_ids.len();
+
+    let summary = format!(
+        "This analysis matched {fw_count} framework(s) and produced {total} total findings: \
+         {addressed} addressed, {partial} partially addressed, {gaps} gaps."
+    );
+
+    docx.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text("Executive Summary"))
+            .style("Heading2"),
+    )
+    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&summary)))
+    .add_paragraph(Paragraph::new()) // spacer
+}
+
+fn add_coverage_heatmap(
+    docx: Docx,
+    findings: &[AnalysisFindingWithConcept],
+    frameworks: &[(String, String)],
+) -> Docx {
+    if frameworks.is_empty() {
+        return docx;
+    }
+
+    let coverages: Vec<(String, f64)> = frameworks
+        .iter()
+        .map(|(fw_id, fw_name)| {
+            let fw_findings: Vec<_> = findings.iter().filter(|f| &f.framework_id == fw_id).collect();
+            let total = fw_findings.len() as f64;
+            let addressed = fw_findings
+                .iter()
+                .filter(|f| f.finding_type == FindingType::Addressed)
+                .count() as f64;
+            let coverage = if total > 0.0 { addressed / total } else { 0.0 };
+            (fw_name.clone(), coverage)
+        })
+        .collect();
+
+    let mut d = docx.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text("Coverage Overview"))
+            .style("Heading2"),
+    );
+
+    match charts::render_coverage_heatmap(&coverages) {
+        Ok(png_bytes) => {
+            d = embed_chart_image(d, &png_bytes);
+        }
+        Err(e) => {
+            tracing::warn!("Failed to render coverage heatmap: {e}");
+            d = add_chart_fallback(d);
+        }
+    }
+
+    d.add_paragraph(Paragraph::new()) // spacer
+}
+
+fn add_framework_section(
+    docx: Docx,
+    fw_id: &str,
+    fw_name: &str,
+    findings: &[AnalysisFindingWithConcept],
+) -> Docx {
+    let fw_findings: Vec<_> = findings.iter().filter(|f| f.framework_id == fw_id).collect();
+    if fw_findings.is_empty() {
+        return docx;
+    }
+
+    let mut d = docx.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text(fw_name))
+            .style("Heading2"),
+    );
+
+    // Radar chart
+    let types = ["Addressed", "Partial", "Gap", "N/A"];
+    let total = fw_findings.len() as f64;
+    let labels: Vec<String> = types.iter().map(|s| s.to_string()).collect();
+    let values: Vec<f64> = vec![
+        fw_findings
+            .iter()
+            .filter(|f| f.finding_type == FindingType::Addressed)
+            .count() as f64
+            / total,
+        fw_findings
+            .iter()
+            .filter(|f| f.finding_type == FindingType::PartiallyAddressed)
+            .count() as f64
+            / total,
+        fw_findings
+            .iter()
+            .filter(|f| f.finding_type == FindingType::Gap)
+            .count() as f64
+            / total,
+        fw_findings
+            .iter()
+            .filter(|f| f.finding_type == FindingType::NotApplicable)
+            .count() as f64
+            / total,
+    ];
+
+    match charts::render_radar_chart(&labels, &values) {
+        Ok(png_bytes) => {
+            d = embed_chart_image(d, &png_bytes);
+        }
+        Err(e) => {
+            tracing::warn!("Failed to render radar chart for {fw_name}: {e}");
+            d = add_chart_fallback(d);
+        }
+    }
+
+    // Findings table
+    d = d.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text("Findings"))
+            .style("Heading3"),
+    );
+
+    // Table header row
+    let header_row = TableRow::new(vec![
+        TableCell::new()
+            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Code").bold().size(18))),
+        TableCell::new()
+            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Concept").bold().size(18))),
+        TableCell::new()
+            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Type").bold().size(18))),
+        TableCell::new().add_paragraph(
+            Paragraph::new().add_run(Run::new().add_text("Priority").bold().size(18)),
+        ),
+        TableCell::new().add_paragraph(
+            Paragraph::new().add_run(Run::new().add_text("Confidence").bold().size(18)),
+        ),
+        TableCell::new().add_paragraph(
+            Paragraph::new().add_run(Run::new().add_text("Recommendation").bold().size(18)),
+        ),
+    ]);
+
+    let mut rows = vec![header_row];
+
+    for finding in &fw_findings {
+        let finding_type_str: String = finding.finding_type.clone().into();
+        let code = finding.concept_code.as_deref().unwrap_or("-");
+        let name = truncate(&finding.concept_name_en, 40);
+        let confidence = format!("{:.0}%", finding.confidence_score * 100.0);
+        let recommendation = finding
+            .recommendation
+            .as_deref()
+            .map(|t| truncate(t, 80))
+            .unwrap_or_default();
+
+        rows.push(TableRow::new(vec![
+            TableCell::new()
+                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(code).size(18))),
+            TableCell::new()
+                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&name).size(18))),
+            TableCell::new().add_paragraph(
+                Paragraph::new().add_run(Run::new().add_text(&finding_type_str).size(18)),
+            ),
+            TableCell::new().add_paragraph(
+                Paragraph::new()
+                    .add_run(Run::new().add_text(&format!("P{}", finding.priority)).size(18)),
+            ),
+            TableCell::new().add_paragraph(
+                Paragraph::new().add_run(Run::new().add_text(&confidence).size(18)),
+            ),
+            TableCell::new().add_paragraph(
+                Paragraph::new().add_run(Run::new().add_text(&recommendation).size(18)),
+            ),
+        ]));
+    }
+
+    let table = Table::new(rows).set_grid(vec![1200, 2400, 1400, 1000, 1200, 2800]);
+
+    d = d.add_table(table);
+    d.add_paragraph(Paragraph::new()) // spacer
+}
+
+fn add_priority_breakdown(docx: Docx, findings: &[AnalysisFindingWithConcept]) -> Docx {
+    if findings.is_empty() {
+        return docx;
+    }
+
+    let mut priority_counts: BTreeMap<i32, i64> = BTreeMap::new();
+    for f in findings {
+        *priority_counts.entry(f.priority).or_insert(0) += 1;
+    }
+
+    let priorities: Vec<(String, i64)> = priority_counts
+        .into_iter()
+        .map(|(p, count)| (format!("P{p}"), count))
+        .collect();
+
+    let mut d = docx.add_paragraph(
+        Paragraph::new()
+            .add_run(Run::new().add_text("Priority Breakdown"))
+            .style("Heading2"),
+    );
+
+    match charts::render_priority_chart(&priorities) {
+        Ok(png_bytes) => {
+            d = embed_chart_image(d, &png_bytes);
+        }
+        Err(e) => {
+            tracing::warn!("Failed to render priority chart: {e}");
+            d = add_chart_fallback(d);
+        }
+    }
+
+    d.add_paragraph(Paragraph::new()) // spacer
+}
+
+fn add_appendix(docx: Docx, analysis: &Analysis) -> Docx {
+    let text = match analysis.extracted_text.as_deref() {
+        Some(t) if !t.is_empty() => t,
+        _ => return docx,
+    };
+
+    let truncated = truncate(text, 2000);
+
+    let mut d = docx
+        .add_paragraph(
+            Paragraph::new()
+                .add_run(Run::new().add_text("Appendix: Extracted Text"))
+                .style("Heading2"),
+        )
+        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&truncated).size(16)));
+
+    if text.chars().count() > 2000 {
+        d = d.add_paragraph(
+            Paragraph::new()
+                .add_run(Run::new().add_text("[Text truncated at 2000 characters]").italic().size(16)),
+        );
+    }
+
+    d
+}
+
+fn embed_chart_image(docx: Docx, png_bytes: &[u8]) -> Docx {
+    let pic = Pic::new(png_bytes).size(
+        CHART_WIDTH_PX * EMU_PER_PX,
+        CHART_HEIGHT_PX * EMU_PER_PX,
+    );
+    docx.add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)))
+}
+
+fn add_chart_fallback(docx: Docx) -> Docx {
+    docx.add_paragraph(
+        Paragraph::new().add_run(Run::new().add_text("[Chart could not be rendered]").italic()),
+    )
+}
+
+fn truncate(s: &str, max_chars: usize) -> String {
+    if s.chars().count() <= max_chars {
+        s.to_string()
+    } else {
+        let truncated: String = s.chars().take(max_chars).collect();
+        format!("{truncated}...")
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::features::analysis::models::{AnalysisStatus, InputType};
+
+    fn make_test_analysis() -> Analysis {
+        Analysis {
+            id: "test-analysis-id".to_string(),
+            name: "Test Security Analysis".to_string(),
+            description: Some("A test description".to_string()),
+            input_type: InputType::Pdf,
+            input_text: None,
+            original_filename: Some("test.pdf".to_string()),
+            file_path: None,
+            extracted_text: Some(
+                "This is extracted test text for the appendix section.".to_string(),
+            ),
+            status: AnalysisStatus::Completed,
+            error_message: None,
+            prompt_template: None,
+            matched_framework_ids: vec!["nist-csf".to_string()],
+            processing_time_ms: Some(1500),
+            token_count: Some(3000),
+            created_by: None,
+            created_at: "2026-03-19 09:00:00".to_string(),
+            updated_at: "2026-03-19 09:01:00".to_string(),
+        }
+    }
+
+    fn make_test_findings() -> Vec<AnalysisFindingWithConcept> {
+        vec![
+            AnalysisFindingWithConcept {
+                id: "f-1".to_string(),
+                analysis_id: "test-analysis-id".to_string(),
+                concept_id: "c-1".to_string(),
+                framework_id: "fw-1".to_string(),
+                finding_type: FindingType::Addressed,
+                confidence_score: 0.9,
+                evidence_text: Some("Evidence for addressed finding".to_string()),
+                recommendation: Some("Continue current approach".to_string()),
+                priority: 1,
+                sort_order: 0,
+                created_at: "2026-03-19 09:00:00".to_string(),
+                concept_code: Some("ID.AM-1".to_string()),
+                concept_name_en: "Asset Management".to_string(),
+                concept_name_nb: "Eiendelsforvaltning".to_string(),
+                concept_definition_en: "Systems and assets are identified".to_string(),
+                concept_definition_nb: None,
+                source_reference: None,
+            },
+            AnalysisFindingWithConcept {
+                id: "f-2".to_string(),
+                analysis_id: "test-analysis-id".to_string(),
+                concept_id: "c-2".to_string(),
+                framework_id: "fw-1".to_string(),
+                finding_type: FindingType::Gap,
+                confidence_score: 0.7,
+                evidence_text: Some("No evidence of risk assessment".to_string()),
+                recommendation: Some("Implement risk assessment process".to_string()),
+                priority: 2,
+                sort_order: 1,
+                created_at: "2026-03-19 09:00:00".to_string(),
+                concept_code: Some("ID.RA-1".to_string()),
+                concept_name_en: "Risk Assessment".to_string(),
+                concept_name_nb: "Risikovurdering".to_string(),
+                concept_definition_en: "Asset vulnerabilities are identified".to_string(),
+                concept_definition_nb: None,
+                source_reference: None,
+            },
+        ]
+    }
+
+    #[test]
+    fn test_generate_docx_returns_bytes() {
+        let analysis = make_test_analysis();
+        let findings = make_test_findings();
+        let frameworks = vec![("fw-1".into(), "Test Framework".into())];
+        let result = generate_docx(&analysis, &findings, &frameworks);
+        assert!(result.is_ok(), "generate_docx failed: {:?}", result.err());
+        let bytes = result.unwrap();
+        assert!(!bytes.is_empty());
+        // DOCX is ZIP format: starts with PK (0x50, 0x4B)
+        assert_eq!(bytes[0], 0x50, "Expected ZIP magic byte P");
+        assert_eq!(bytes[1], 0x4B, "Expected ZIP magic byte K");
+    }
+
+    #[test]
+    fn test_generate_docx_empty_findings() {
+        let analysis = make_test_analysis();
+        let findings = vec![];
+        let frameworks = vec![];
+        let result = generate_docx(&analysis, &findings, &frameworks);
+        assert!(result.is_ok());
+        let bytes = result.unwrap();
+        assert_eq!(bytes[0], 0x50);
+        assert_eq!(bytes[1], 0x4B);
+    }
+
+    #[test]
+    fn test_generate_docx_contains_analysis_name() {
+        let mut analysis = make_test_analysis();
+        analysis.name = "UniqueDocxTitle2026".to_string();
+        let findings = make_test_findings();
+        let frameworks = vec![];
+        let result = generate_docx(&analysis, &findings, &frameworks).unwrap();
+
+        // DOCX is a ZIP containing XML files. Unzip and check word/document.xml.
+        let cursor = std::io::Cursor::new(&result);
+        let mut archive = zip::ZipArchive::new(cursor).expect("Should be valid ZIP");
+        let mut doc_xml = String::new();
+        {
+            use std::io::Read;
+            let mut file = archive
+                .by_name("word/document.xml")
+                .expect("Should contain word/document.xml");
+            file.read_to_string(&mut doc_xml).unwrap();
+        }
+        assert!(
+            doc_xml.contains("UniqueDocxTitle2026"),
+            "document.xml should contain the analysis name"
+        );
+    }
+}
diff --git a/backend/src/features/analysis/mod.rs b/backend/src/features/analysis/mod.rs
index 65f43a9..a10e2cc 100644
--- a/backend/src/features/analysis/mod.rs
+++ b/backend/src/features/analysis/mod.rs
@@ -6,4 +6,5 @@ pub mod routes;
 pub mod tokenizer;
 pub mod upload;
 pub mod charts;
+pub mod export_docx;
 pub mod export_pdf;
