diff --git a/backend/Cargo.lock b/backend/Cargo.lock
index 770deba..58e9f68 100644
--- a/backend/Cargo.lock
+++ b/backend/Cargo.lock
@@ -2568,6 +2568,7 @@ dependencies = [
  "plotters",
  "quick-xml",
  "rand 0.9.2",
+ "regex",
  "serde",
  "serde_json",
  "sha2",
diff --git a/backend/Cargo.toml b/backend/Cargo.toml
index fce2cad..d42a0cc 100644
--- a/backend/Cargo.toml
+++ b/backend/Cargo.toml
@@ -59,6 +59,7 @@ tower_governor = "0.8"
 
 # Document parsing
 pdf-extract = "0.10"
+regex = "1"
 zip = "2"
 quick-xml = "0.37"
 
diff --git a/backend/src/features/extraction/page_offset.rs b/backend/src/features/extraction/page_offset.rs
index 6be1f02..8a407e2 100644
--- a/backend/src/features/extraction/page_offset.rs
+++ b/backend/src/features/extraction/page_offset.rs
@@ -1 +1,178 @@
-// Page offset detection — implemented in section-02
+use regex::Regex;
+
+use super::extractor::PageOffsetSource;
+
+/// Detect the offset between physical PDF page indices and logical page numbers.
+///
+/// `pages` is a slice of (physical_page_index, page_text) pairs.
+/// `manual_override` bypasses auto-detection.
+///
+/// Returns (offset_value, source). The offset is defined such that:
+///   logical_page = physical_page - offset
+pub fn detect_page_offset(
+    pages: &[(usize, String)],
+    manual_override: Option<i32>,
+) -> (i32, PageOffsetSource) {
+    // 1. Manual override takes precedence
+    if let Some(offset) = manual_override {
+        return (offset, PageOffsetSource::Manual);
+    }
+
+    // 2. Primary: TOC pattern scanning
+    let toc_re =
+        Regex::new(r"(GOVERN|MAP|MEASURE|MANAGE)\s+(\d+\.\d+)\s*[.\-\s]+\s*(\d+)").unwrap();
+
+    // Scan first 10 physical pages for TOC entries
+    let scan_pages = &pages[..pages.len().min(10)];
+
+    for (toc_phys_idx, toc_text) in scan_pages {
+        for cap in toc_re.captures_iter(toc_text) {
+            let function_name = &cap[1];
+            let subcategory = &cap[2];
+            let logical_page: i32 = match cap[3].parse() {
+                Ok(p) => p,
+                Err(_) => continue,
+            };
+
+            let concept_code = format!("{function_name} {subcategory}");
+
+            // Search all pages for the actual section header (not a TOC line)
+            for (content_phys_idx, content_text) in pages {
+                // Skip the TOC page itself
+                if content_phys_idx == toc_phys_idx {
+                    continue;
+                }
+
+                // Check if this page has the concept code as a section header
+                // A header line has the concept code but does NOT match the TOC pattern
+                // (i.e., it's not followed by dots/dashes and a page number)
+                for line in content_text.lines() {
+                    let trimmed = line.trim();
+                    if trimmed.contains(&concept_code) && !toc_re.is_match(trimmed) {
+                        let offset = *content_phys_idx as i32 - logical_page;
+                        return (offset, PageOffsetSource::Auto);
+                    }
+                }
+            }
+        }
+    }
+
+    // 3. Secondary: Footer page number scanning
+    let footer_re = Regex::new(r"\n\s*(\d+)\s*$").unwrap();
+    let mut offset_votes: Vec<i32> = Vec::new();
+
+    for (phys_idx, text) in pages {
+        if let Some(cap) = footer_re.captures(text) {
+            if let Ok(footer_num) = cap[1].parse::<i32>() {
+                let offset = *phys_idx as i32 - footer_num;
+                offset_votes.push(offset);
+            }
+        }
+    }
+
+    // Check for consistent offset across 2+ pages
+    if offset_votes.len() >= 2 {
+        let first = offset_votes[0];
+        let consistent_count = offset_votes.iter().filter(|&&v| v == first).count();
+        if consistent_count >= 2 {
+            return (first, PageOffsetSource::Auto);
+        }
+    }
+
+    // 4. Fallback
+    tracing::warn!("Could not auto-detect page offset, defaulting to 0");
+    (0, PageOffsetSource::Default)
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn test_detects_offset_from_toc_and_content_page() {
+        // TOC on physical page 0: "GOVERN 1.1 ..... 4"
+        // Actual GOVERN 1.1 header on physical page 8
+        // Expected offset = 8 - 4 = 4
+        let pages: Vec<(usize, String)> = vec![
+            (0, "Table of Contents\nGOVERN 1.1 ..... 4\nGOVERN 1.2 ..... 6\n".to_string()),
+            (1, "Introduction text here".to_string()),
+            (2, "More intro".to_string()),
+            (3, "Some other content".to_string()),
+            (4, "Yet more content".to_string()),
+            (5, "Preamble stuff".to_string()),
+            (6, "Framework overview".to_string()),
+            (7, "Still framework".to_string()),
+            (8, "GOVERN 1.1\nAbout\nThis is the govern section".to_string()),
+            (9, "GOVERN 1.2\nAbout\nThis is another section".to_string()),
+        ];
+
+        let (offset, source) = detect_page_offset(&pages, None);
+        assert_eq!(offset, 4);
+        assert!(matches!(source, PageOffsetSource::Auto));
+    }
+
+    #[test]
+    fn test_returns_zero_offset_when_no_toc_pattern_found() {
+        let pages: Vec<(usize, String)> = vec![
+            (0, "Just some random text".to_string()),
+            (1, "No table of contents here".to_string()),
+            (2, "Regular page content".to_string()),
+        ];
+
+        let (offset, source) = detect_page_offset(&pages, None);
+        assert_eq!(offset, 0);
+        assert!(matches!(source, PageOffsetSource::Default));
+    }
+
+    #[test]
+    fn test_handles_toc_with_various_separators() {
+        // Dots, spaces, dashes should all parse
+        let toc_text = "GOVERN 1.1 ..... 4\nMAP 1.1    8\nMEASURE 1.1 --- 12\n";
+        let pages: Vec<(usize, String)> = vec![
+            (0, toc_text.to_string()),
+            (1, "some content".to_string()),
+            (6, "GOVERN 1.1\nActual content here".to_string()),
+        ];
+
+        let (offset, source) = detect_page_offset(&pages, None);
+        // GOVERN 1.1 on physical page 6, logical page 4 => offset = 2
+        assert_eq!(offset, 2);
+        assert!(matches!(source, PageOffsetSource::Auto));
+    }
+
+    #[test]
+    fn test_manual_override_takes_precedence() {
+        let pages: Vec<(usize, String)> = vec![
+            (0, "GOVERN 1.1 ..... 4\n".to_string()),
+            (8, "GOVERN 1.1\nContent".to_string()),
+        ];
+
+        // Even though auto-detection would find offset=4,
+        // the manual override of 10 takes precedence
+        let (offset, source) = detect_page_offset(&pages, Some(10));
+        assert_eq!(offset, 10);
+        assert!(matches!(source, PageOffsetSource::Manual));
+    }
+
+    #[test]
+    fn test_records_correct_page_offset_source() {
+        // Auto
+        let pages_auto: Vec<(usize, String)> = vec![
+            (0, "GOVERN 1.1 ..... 4\n".to_string()),
+            (8, "GOVERN 1.1\nContent".to_string()),
+        ];
+        let (_, source) = detect_page_offset(&pages_auto, None);
+        assert!(matches!(source, PageOffsetSource::Auto));
+
+        // Manual
+        let (_, source) = detect_page_offset(&pages_auto, Some(5));
+        assert!(matches!(source, PageOffsetSource::Manual));
+
+        // Default
+        let pages_none: Vec<(usize, String)> = vec![
+            (0, "Nothing useful".to_string()),
+        ];
+        let (_, source) = detect_page_offset(&pages_none, None);
+        assert!(matches!(source, PageOffsetSource::Default));
+    }
+}
diff --git a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
index b418e4d..3df4042 100644
--- a/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
+++ b/docs/specs/05-playbook-enrichment/01-data-extraction/implementation/deep_implement_config.json
@@ -14,7 +14,12 @@
     "section-05-cli-integration",
     "section-06-integration-tests"
   ],
-  "sections_state": {},
+  "sections_state": {
+    "section-01-extractor-trait-and-types": {
+      "status": "complete",
+      "commit_hash": "f539693"
+    }
+  },
   "pre_commit": {
     "present": false,
     "type": "none",
