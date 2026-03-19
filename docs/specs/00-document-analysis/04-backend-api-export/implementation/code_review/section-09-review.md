# Section 09 Code Review: DOCX Export

## Critical

No critical issues found.

## Important

### 1. Findings table `set_grid` total width overflows A4 content area (83% confidence)

- File: `export_docx.rs`, line ~232
- Six column widths sum to 10,000 twips (1200+2400+1400+1000+1200+2800). 10,000 twips = 6.94 inches. A4 with standard 1-inch margins provides ~6.27 inches. Table overflows right margin.
- Fix: Reduce total to ~8,900 twips, e.g., `vec![1000, 2200, 1200, 900, 1100, 2500]`.

### 2. Coverage heatmap PNG aspect ratio distorted at 11+ frameworks (82% confidence)

- File: `export_docx.rs`, `embed_chart_image` function
- `charts::render_coverage_heatmap` produces variable-height PNGs. `embed_chart_image` forces all charts into fixed 400x300 bounding box, squishing tall heatmaps. PDF module avoids this with `with_scale(0.5)` preserving original aspect ratio.
- Fix: Compute embed height from actual PNG dimensions instead of using constants.

### 3. `truncate` duplicated verbatim from `export_pdf.rs` (80% confidence)

- File: `export_docx.rs`, `truncate` function
- Identical function in both modules. Bug fix to one won't propagate to the other.
- Fix: Move to a shared `util.rs` module. Not blocking for this section.

## No Issues Found

- `DocxExportError` well-structured and consistent with PDF module
- Division-by-zero guarded at every chart callsite
- Appendix correctly uses `chars().count()` for truncation notice
- Chart failures handled gracefully with fallback text
- BTreeMap ensures deterministic priority ordering
- EMU arithmetic safe within u32 range
- No user input reaches unsafe code
- Test structure mirrors PDF test suite
