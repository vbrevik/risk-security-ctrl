use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

use super::extractor::PageOffsetSource;

static TOC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(GOVERN|MAP|MEASURE|MANAGE)\s+(\d+\.\d+)\s*[.\-\s]+\s*(\d+)").unwrap()
});

static FOOTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\n\s*(\d+)\s*$").unwrap()
});

/// Detect the offset between physical PDF page indices and logical page numbers.
///
/// `pages` is a slice of (physical_page_index, page_text) pairs.
/// `manual_override` bypasses auto-detection.
///
/// Returns (offset_value, source). The offset is defined such that:
///   logical_page = physical_page - offset
pub fn detect_page_offset(
    pages: &[(usize, String)],
    manual_override: Option<i32>,
) -> (i32, PageOffsetSource) {
    // 1. Manual override takes precedence
    if let Some(offset) = manual_override {
        return (offset, PageOffsetSource::Manual);
    }

    // 2. Primary: TOC pattern scanning
    let scan_pages = &pages[..pages.len().min(10)];
    let mut detected_offsets: Vec<i32> = Vec::new();

    for (toc_phys_idx, toc_text) in scan_pages {
        for cap in TOC_RE.captures_iter(toc_text) {
            let function_name = &cap[1];
            let subcategory = &cap[2];
            let logical_page: i32 = match cap[3].parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let concept_code = format!("{function_name} {subcategory}");

            // Search all pages for the actual section header (not a TOC line)
            for (content_phys_idx, content_text) in pages {
                if content_phys_idx == toc_phys_idx {
                    continue;
                }

                for line in content_text.lines() {
                    let trimmed = line.trim();
                    if trimmed.contains(&concept_code) && !TOC_RE.is_match(trimmed) {
                        let offset = *content_phys_idx as i32 - logical_page;
                        detected_offsets.push(offset);
                        break;
                    }
                }
                if detected_offsets.last().is_some() {
                    break;
                }
            }
        }
    }

    if !detected_offsets.is_empty() {
        let first = detected_offsets[0];
        // Warn if multiple TOC entries produce inconsistent offsets
        if detected_offsets.iter().any(|&o| o != first) {
            tracing::warn!(
                "Inconsistent page offsets detected from TOC entries: {:?}. Using first: {}",
                detected_offsets,
                first
            );
        }
        return (first, PageOffsetSource::Auto);
    }

    // 3. Secondary: Footer page number scanning
    let mut offset_votes: Vec<i32> = Vec::new();

    for (phys_idx, text) in pages {
        if let Some(cap) = FOOTER_RE.captures(text) {
            if let Ok(footer_num) = cap[1].parse::<i32>() {
                let offset = *phys_idx as i32 - footer_num;
                offset_votes.push(offset);
            }
        }
    }

    // Use mode (most common value) across 2+ pages
    if offset_votes.len() >= 2 {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for &v in &offset_votes {
            *counts.entry(v).or_insert(0) += 1;
        }
        if let Some((&mode_offset, &count)) = counts.iter().max_by_key(|(_, &c)| c) {
            if count >= 2 {
                return (mode_offset, PageOffsetSource::Auto);
            }
        }
    }

    // 4. Fallback
    tracing::warn!("Could not auto-detect page offset, defaulting to 0");
    (0, PageOffsetSource::Default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_offset_from_toc_and_content_page() {
        let pages: Vec<(usize, String)> = vec![
            (0, "Table of Contents\nGOVERN 1.1 ..... 4\nGOVERN 1.2 ..... 6\n".to_string()),
            (1, "Introduction text here".to_string()),
            (2, "More intro".to_string()),
            (3, "Some other content".to_string()),
            (4, "Yet more content".to_string()),
            (5, "Preamble stuff".to_string()),
            (6, "Framework overview".to_string()),
            (7, "Still framework".to_string()),
            (8, "GOVERN 1.1\nAbout\nThis is the govern section".to_string()),
            (9, "GOVERN 1.2\nAbout\nThis is another section".to_string()),
        ];

        let (offset, source) = detect_page_offset(&pages, None);
        assert_eq!(offset, 4);
        assert!(matches!(source, PageOffsetSource::Auto));
    }

    #[test]
    fn test_returns_zero_offset_when_no_toc_pattern_found() {
        let pages: Vec<(usize, String)> = vec![
            (0, "Just some random text".to_string()),
            (1, "No table of contents here".to_string()),
            (2, "Regular page content".to_string()),
        ];

        let (offset, source) = detect_page_offset(&pages, None);
        assert_eq!(offset, 0);
        assert!(matches!(source, PageOffsetSource::Default));
    }

    #[test]
    fn test_handles_toc_with_various_separators() {
        let toc_text = "GOVERN 1.1 ..... 4\nMAP 1.1    8\nMEASURE 1.1 --- 12\n";
        let pages: Vec<(usize, String)> = vec![
            (0, toc_text.to_string()),
            (1, "some content".to_string()),
            (6, "GOVERN 1.1\nActual content here".to_string()),
        ];

        let (offset, source) = detect_page_offset(&pages, None);
        assert_eq!(offset, 2);
        assert!(matches!(source, PageOffsetSource::Auto));
    }

    #[test]
    fn test_manual_override_takes_precedence() {
        let pages: Vec<(usize, String)> = vec![
            (0, "GOVERN 1.1 ..... 4\n".to_string()),
            (8, "GOVERN 1.1\nContent".to_string()),
        ];

        let (offset, source) = detect_page_offset(&pages, Some(10));
        assert_eq!(offset, 10);
        assert!(matches!(source, PageOffsetSource::Manual));
    }

    #[test]
    fn test_records_correct_page_offset_source() {
        // Auto
        let pages_auto: Vec<(usize, String)> = vec![
            (0, "GOVERN 1.1 ..... 4\n".to_string()),
            (8, "GOVERN 1.1\nContent".to_string()),
        ];
        let (_, source) = detect_page_offset(&pages_auto, None);
        assert!(matches!(source, PageOffsetSource::Auto));

        // Manual
        let (_, source) = detect_page_offset(&pages_auto, Some(5));
        assert!(matches!(source, PageOffsetSource::Manual));

        // Default
        let pages_none: Vec<(usize, String)> = vec![
            (0, "Nothing useful".to_string()),
        ];
        let (_, source) = detect_page_offset(&pages_none, None);
        assert!(matches!(source, PageOffsetSource::Default));
    }

    #[test]
    fn test_footer_fallback_detects_offset() {
        // Pages with footer page numbers: physical 0 has footer "1", physical 1 has footer "2"
        // Offset = 0 - 1 = -1 (consistent)
        let pages: Vec<(usize, String)> = vec![
            (0, "Some content on page one\n  1  ".to_string()),
            (1, "More content on page two\n  2  ".to_string()),
            (2, "Even more content\n  3  ".to_string()),
        ];

        let (offset, source) = detect_page_offset(&pages, None);
        assert_eq!(offset, -1);
        assert!(matches!(source, PageOffsetSource::Auto));
    }
}
