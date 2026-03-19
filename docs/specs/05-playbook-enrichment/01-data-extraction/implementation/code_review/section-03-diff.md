diff --git a/backend/src/features/extraction/playbook/mod.rs b/backend/src/features/extraction/playbook/mod.rs
index eac71aa..5203a67 100644
--- a/backend/src/features/extraction/playbook/mod.rs
+++ b/backend/src/features/extraction/playbook/mod.rs
@@ -1 +1,511 @@
-// Playbook extractor — implemented in section-03
+use std::path::Path;
+use std::sync::LazyLock;
+
+use chrono::Utc;
+use regex::Regex;
+
+use super::extractor::{
+    build_concept_code_map, read_pdf_pages, resolve_concept_id, ExtractionConfig, ExtractionError,
+    ExtractionResult, ExtractedSection, PdfExtractor, Subsection, SubsectionKind,
+};
+use super::page_offset::detect_page_offset;
+use super::validation::ValidationReport;
+
+static HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
+    Regex::new(r"(?m)^(GOVERN|MAP|MEASURE|MANAGE)\s+(\d+\.\d+)").unwrap()
+});
+
+/// Concrete extractor for the NIST AI RMF Playbook PDF.
+pub struct PlaybookExtractor;
+
+impl PdfExtractor for PlaybookExtractor {
+    fn name(&self) -> &str {
+        "NIST AI RMF Playbook"
+    }
+
+    fn framework_id(&self) -> &str {
+        "nist-ai-rmf"
+    }
+
+    fn extract(
+        &self,
+        pdf_path: &Path,
+        config: &ExtractionConfig,
+    ) -> Result<ExtractionResult, ExtractionError> {
+        // 1. Read pages
+        let pages = read_pdf_pages(pdf_path)?;
+        if pages.is_empty() {
+            return Err(ExtractionError::NoSectionsFound);
+        }
+
+        // 2. Detect page offset
+        let (offset, offset_source) =
+            detect_page_offset(&pages, config.page_offset_override);
+
+        // 3. Normalize page text for header scanning
+        let normalized: Vec<(usize, String)> = pages
+            .iter()
+            .map(|(idx, text)| (*idx, normalize_spaced_headers(text)))
+            .collect();
+
+        // 4. Scan for concept headers
+        let mut headers: Vec<(String, usize, usize)> = Vec::new(); // (code, phys_page, char_offset)
+        let mut seen_codes = std::collections::HashSet::new();
+
+        for (phys_idx, norm_text) in &normalized {
+            for m in HEADER_RE.find_iter(norm_text) {
+                let full_match = m.as_str();
+                let code = full_match.to_string();
+
+                // Deduplicate: only first occurrence per concept code
+                if seen_codes.contains(&code) {
+                    continue;
+                }
+                seen_codes.insert(code.clone());
+
+                headers.push((code, *phys_idx, m.start()));
+            }
+        }
+
+        if headers.is_empty() {
+            return Err(ExtractionError::NoSectionsFound);
+        }
+
+        // Sort by (physical_page, char_offset)
+        headers.sort_by_key(|(_, page, offset)| (*page, *offset));
+
+        // 5. Build concept code map for ID resolution
+        let ontology_path = Path::new(env!("CARGO_MANIFEST_DIR"))
+            .parent()
+            .unwrap()
+            .join("ontology-data/nist-ai-rmf.json");
+        let code_map = build_concept_code_map(&ontology_path).unwrap_or_default();
+
+        // 6. Extract section text and split subsections
+        let mut sections: Vec<ExtractedSection> = Vec::new();
+
+        for (i, (code, phys_page, _char_offset)) in headers.iter().enumerate() {
+            // Determine text boundaries: from this header to the next (or end)
+            let raw_text = extract_section_text(&pages, &headers, i);
+
+            if raw_text.len() < 50 {
+                tracing::warn!(
+                    "Section {} has suspiciously short text ({} chars)",
+                    code,
+                    raw_text.len()
+                );
+            }
+
+            let subsections = split_subsections(&raw_text, code);
+            let concept_id = resolve_concept_id(code, &code_map);
+            let logical_page = (*phys_page as i32 - offset).max(0) as usize;
+
+            sections.push(ExtractedSection {
+                concept_code: code.clone(),
+                concept_id,
+                physical_page: *phys_page,
+                logical_page,
+                raw_text,
+                subsections,
+            });
+        }
+
+        Ok(ExtractionResult {
+            framework_id: self.framework_id().to_string(),
+            source_pdf: pdf_path.display().to_string(),
+            extracted_at: Utc::now(),
+            sections,
+            page_offset_detected: offset,
+            page_offset_source: offset_source,
+        })
+    }
+
+    fn validate(&self, result: &ExtractionResult, ontology_path: &Path) -> ValidationReport {
+        // Stub — implemented in section-04
+        let _ = (result, ontology_path);
+        ValidationReport {
+            total_expected: 0,
+            total_extracted: result.sections.len(),
+            missing_concepts: Vec::new(),
+            unmatched_sections: Vec::new(),
+            warnings: Vec::new(),
+        }
+    }
+}
+
+// ── Internal helpers ────────────────────────────────────────
+
+/// Collapse spaced-out characters like "G O V E R N  1 . 1" into "GOVERN 1.1".
+/// Only transforms lines where most characters are followed by a space.
+fn normalize_spaced_headers(text: &str) -> String {
+    text.lines()
+        .map(|line| {
+            let trimmed = line.trim();
+            if trimmed.len() < 6 {
+                return line.to_string();
+            }
+
+            // Heuristic: if >40% of chars are spaces and the line has
+            // uppercase letters, it's likely a spaced-out heading
+            let space_count = trimmed.chars().filter(|c| *c == ' ').count();
+            let has_upper = trimmed.chars().any(|c| c.is_ascii_uppercase());
+            let ratio = space_count as f64 / trimmed.len() as f64;
+
+            if has_upper && ratio > 0.35 {
+                // Collapse: remove spaces between single characters
+                let chars: Vec<char> = trimmed.chars().collect();
+                let mut result = String::new();
+                let mut i = 0;
+                while i < chars.len() {
+                    result.push(chars[i]);
+                    // If current is non-space, next is space, and char after is non-space single char
+                    if i + 2 < chars.len()
+                        && chars[i] != ' '
+                        && chars[i + 1] == ' '
+                        && chars[i + 2] != ' '
+                    {
+                        // Check if this is a pattern of alternating char-space
+                        if i + 3 >= chars.len() || chars[i + 3] == ' ' {
+                            // Skip the space
+                            i += 2;
+                            continue;
+                        }
+                    }
+                    i += 1;
+                }
+                result
+            } else {
+                line.to_string()
+            }
+        })
+        .collect::<Vec<_>>()
+        .join("\n")
+}
+
+/// Extract the raw text for a section, concatenating pages from the header
+/// position to the next header position.
+fn extract_section_text(
+    pages: &[(usize, String)],
+    headers: &[(String, usize, usize)],
+    section_idx: usize,
+) -> String {
+    let (_code, start_page, start_offset) = &headers[section_idx];
+
+    // Find the end boundary
+    let (end_page, end_offset) = if section_idx + 1 < headers.len() {
+        let (_, next_page, next_offset) = &headers[section_idx + 1];
+        (*next_page, Some(*next_offset))
+    } else {
+        (pages.last().map(|(i, _)| *i).unwrap_or(0), None)
+    };
+
+    let mut result = String::new();
+
+    for (page_idx, page_text) in pages {
+        if *page_idx < *start_page || *page_idx > end_page {
+            continue;
+        }
+
+        let text_slice = if *page_idx == *start_page && *page_idx == end_page {
+            // Same page: slice between offsets
+            let end = end_offset.unwrap_or(page_text.len());
+            &page_text[*start_offset..end.min(page_text.len())]
+        } else if *page_idx == *start_page {
+            &page_text[*start_offset..]
+        } else if *page_idx == end_page {
+            if let Some(end_off) = end_offset {
+                &page_text[..end_off.min(page_text.len())]
+            } else {
+                page_text.as_str()
+            }
+        } else {
+            // Full intermediate page
+            page_text.as_str()
+        };
+
+        if !result.is_empty() && !text_slice.is_empty() {
+            // Rejoin hyphenated words at page breaks
+            if result.ends_with('-') {
+                let next_char = text_slice.chars().next();
+                if let Some(c) = next_char {
+                    if c.is_ascii_lowercase() {
+                        result.pop(); // remove hyphen
+                        result.push_str(text_slice);
+                        continue;
+                    }
+                }
+            }
+            result.push(' ');
+        }
+        result.push_str(text_slice);
+    }
+
+    result
+}
+
+/// Split section text into subsections based on heading markers.
+fn split_subsections(raw_text: &str, concept_code: &str) -> Vec<Subsection> {
+    let mut subsections: Vec<(SubsectionKind, usize)> = Vec::new();
+
+    for (line_start, line) in line_positions(raw_text) {
+        let trimmed = line.trim();
+
+        if trimmed.starts_with(concept_code) || trimmed == "About" {
+            subsections.push((SubsectionKind::About, line_start));
+        } else if trimmed.starts_with("Suggested Actions") {
+            subsections.push((SubsectionKind::SuggestedActions, line_start));
+        } else if trimmed.starts_with("Transparency")
+            || trimmed.contains("Organizations can document")
+        {
+            subsections.push((SubsectionKind::TransparencyQuestions, line_start));
+        } else if trimmed.starts_with("AI Transparency Resources") {
+            subsections.push((SubsectionKind::Resources, line_start));
+        } else if trimmed.starts_with("References") && !line.contains("See References") {
+            subsections.push((SubsectionKind::References, line_start));
+        }
+    }
+
+    if subsections.is_empty() {
+        // No subsections detected — return entire text as About
+        return vec![Subsection {
+            kind: SubsectionKind::About,
+            text: raw_text.to_string(),
+        }];
+    }
+
+    let mut result = Vec::new();
+    for (i, (kind, start)) in subsections.iter().enumerate() {
+        let end = if i + 1 < subsections.len() {
+            subsections[i + 1].1
+        } else {
+            raw_text.len()
+        };
+
+        // Skip the heading line itself
+        let section_text = &raw_text[*start..end];
+        let text = section_text
+            .lines()
+            .skip(1) // skip heading line
+            .collect::<Vec<_>>()
+            .join("\n")
+            .trim()
+            .to_string();
+
+        result.push(Subsection {
+            kind: kind.clone(),
+            text,
+        });
+    }
+
+    result
+}
+
+/// Returns (byte_offset, line_str) pairs for each line in the text.
+fn line_positions(text: &str) -> Vec<(usize, &str)> {
+    let mut positions = Vec::new();
+    let mut offset = 0;
+    for line in text.split('\n') {
+        positions.push((offset, line));
+        offset += line.len() + 1; // +1 for the \n
+    }
+    positions
+}
+
+// ── Tests ───────────────────────────────────────────────────
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    // --- Header detection ---
+
+    #[test]
+    fn detects_govern_header_at_start() {
+        let text = "GOVERN 1.1\nLegal and regulatory requirements...";
+        let normalized = normalize_spaced_headers(text);
+        assert!(HEADER_RE.is_match(&normalized));
+        let caps = HEADER_RE.captures(&normalized).unwrap();
+        assert_eq!(&caps[1], "GOVERN");
+        assert_eq!(&caps[2], "1.1");
+    }
+
+    #[test]
+    fn detects_header_with_extra_whitespace() {
+        let text = "MEASURE  2.3\nSome content here";
+        assert!(HEADER_RE.is_match(text));
+    }
+
+    #[test]
+    fn detects_spaced_out_header() {
+        let text = "G O V E R N  1 . 1\nSome content";
+        let normalized = normalize_spaced_headers(text);
+        assert!(
+            HEADER_RE.is_match(&normalized),
+            "Normalized text was: {normalized}"
+        );
+    }
+
+    #[test]
+    fn does_not_match_subcategory_without_decimal() {
+        let text = "GOVERN 1\nSubcategory content";
+        assert!(!HEADER_RE.is_match(text));
+    }
+
+    #[test]
+    fn does_not_match_inline_reference() {
+        // "see GOVERN 1.1" is mid-line, not at start
+        let text = "For details see GOVERN 1.1 which describes...";
+        let matches: Vec<_> = HEADER_RE.find_iter(text).collect();
+        assert!(matches.is_empty(), "Should not match mid-line reference");
+    }
+
+    #[test]
+    fn orders_detected_sections_by_page() {
+        let pages: Vec<(usize, String)> = vec![
+            (0, "GOVERN 1.1\nContent A".to_string()),
+            (1, "MAP 1.1\nContent B".to_string()),
+            (2, "MEASURE 1.1\nContent C".to_string()),
+        ];
+
+        let mut headers: Vec<(String, usize)> = Vec::new();
+        for (idx, text) in &pages {
+            for m in HEADER_RE.find_iter(text) {
+                headers.push((m.as_str().to_string(), *idx));
+            }
+        }
+        headers.sort_by_key(|(_, page)| *page);
+
+        assert_eq!(headers[0].0, "GOVERN 1.1");
+        assert_eq!(headers[1].0, "MAP 1.1");
+        assert_eq!(headers[2].0, "MEASURE 1.1");
+    }
+
+    // --- Subsection splitting ---
+
+    #[test]
+    fn splits_all_five_subsections() {
+        let text = "GOVERN 1.1\nSome about text\n\
+            Suggested Actions\n• Action one\n• Action two\n\
+            Transparency Questions\nQ1? Q2?\n\
+            AI Transparency Resources\nResource link\n\
+            References\nRef 1\nRef 2";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        assert_eq!(subs.len(), 5);
+        assert_eq!(subs[0].kind, SubsectionKind::About);
+        assert_eq!(subs[1].kind, SubsectionKind::SuggestedActions);
+        assert_eq!(subs[2].kind, SubsectionKind::TransparencyQuestions);
+        assert_eq!(subs[3].kind, SubsectionKind::Resources);
+        assert_eq!(subs[4].kind, SubsectionKind::References);
+    }
+
+    #[test]
+    fn splits_partial_subsections() {
+        let text = "GOVERN 1.1\nAbout text here\n\
+            Suggested Actions\n• Do something";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        assert_eq!(subs.len(), 2);
+        assert_eq!(subs[0].kind, SubsectionKind::About);
+        assert_eq!(subs[1].kind, SubsectionKind::SuggestedActions);
+    }
+
+    #[test]
+    fn line_anchored_suggested_actions() {
+        let text = "GOVERN 1.1\nThis text mentions Suggested Actions in a sentence\n\
+            Suggested Actions\n• Real action item";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        // "Suggested Actions" mid-sentence is body text, only the line-start one triggers split
+        assert_eq!(subs.len(), 2);
+        assert!(subs[0].text.contains("mentions Suggested Actions"));
+        assert!(subs[1].text.contains("Real action item"));
+    }
+
+    #[test]
+    fn line_anchored_references() {
+        let text = "GOVERN 1.1\nSee References for more info\n\
+            References\nActual reference 1";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        assert_eq!(subs.len(), 2);
+        // "See References" should NOT be filtered — it starts with "See References"
+        // but the check is on "See References" occurring on the SAME line
+        // The About should contain the "See References" line
+        assert!(subs[0].text.contains("See References"));
+        assert_eq!(subs[1].kind, SubsectionKind::References);
+    }
+
+    #[test]
+    fn preserves_subsection_content() {
+        let text = "GOVERN 1.1\nAbout content line 1\nAbout content line 2\n\
+            Suggested Actions\n• Action A\n• Action B\n• Action C";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        assert!(subs[0].text.contains("About content line 1"));
+        assert!(subs[0].text.contains("About content line 2"));
+        assert!(subs[1].text.contains("Action A"));
+        assert!(subs[1].text.contains("Action C"));
+    }
+
+    #[test]
+    fn handles_empty_subsection_body() {
+        let text = "GOVERN 1.1\n\
+            Suggested Actions\n\
+            References\nRef 1";
+
+        let subs = split_subsections(text, "GOVERN 1.1");
+        assert_eq!(subs.len(), 3);
+        assert_eq!(subs[0].kind, SubsectionKind::About);
+        assert!(subs[0].text.is_empty());
+        assert_eq!(subs[1].kind, SubsectionKind::SuggestedActions);
+        assert!(subs[1].text.is_empty());
+        assert_eq!(subs[2].kind, SubsectionKind::References);
+    }
+
+    // --- Multi-page concatenation ---
+
+    #[test]
+    fn joins_pages_with_space() {
+        let pages: Vec<(usize, String)> = vec![
+            (0, "GOVERN 1.1\nFirst page content".to_string()),
+            (1, "continued on second page".to_string()),
+        ];
+        let headers = vec![("GOVERN 1.1".to_string(), 0_usize, 0_usize)];
+        let text = extract_section_text(&pages, &headers, 0);
+        assert!(text.contains("content continued"), "Got: {text}");
+    }
+
+    #[test]
+    fn rejoins_hyphenated_words_at_page_break() {
+        let pages: Vec<(usize, String)> = vec![
+            (0, "GOVERN 1.1\nThis is an exam-".to_string()),
+            (1, "ple of hyphenation".to_string()),
+        ];
+        let headers = vec![("GOVERN 1.1".to_string(), 0_usize, 0_usize)];
+        let text = extract_section_text(&pages, &headers, 0);
+        assert!(text.contains("example"), "Got: {text}");
+        assert!(!text.contains("exam-"), "Hyphen should be removed");
+    }
+
+    #[test]
+    fn records_starting_page() {
+        // Verify that physical_page records where the header was found
+        let pages: Vec<(usize, String)> = vec![
+            (5, "GOVERN 1.1\nStart of section".to_string()),
+            (6, "Continuation of section".to_string()),
+            (7, "GOVERN 1.2\nNext section".to_string()),
+        ];
+
+        let headers = vec![
+            ("GOVERN 1.1".to_string(), 5_usize, 0_usize),
+            ("GOVERN 1.2".to_string(), 7_usize, 0_usize),
+        ];
+
+        let text = extract_section_text(&pages, &headers, 0);
+        // Section spans pages 5-6 but physical_page should be 5 (start)
+        assert_eq!(headers[0].1, 5);
+        assert!(text.contains("Start of section"));
+        assert!(text.contains("Continuation"));
+    }
+}
diff --git a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
index 3df4042..fe0c16a 100644
--- a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
+++ b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
@@ -18,6 +18,10 @@
     "section-01-extractor-trait-and-types": {
       "status": "complete",
       "commit_hash": "f539693"
+    },
+    "section-02-page-offset-detection": {
+      "status": "complete",
+      "commit_hash": "10b2469"
     }
   },
   "pre_commit": {
