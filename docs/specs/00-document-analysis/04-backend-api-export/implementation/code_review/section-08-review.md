# Section 08 Code Review: PDF Export

## Critical

**1. Hardcoded `./fonts` relative path will fail in non-root-CWD deployment** (95% confidence)
- `./fonts` resolves against process CWD, not binary location
- Fix: use `include_bytes!` to embed fonts at compile time, or env var

## Important

**2. 800x600 px charts without scaling overflow A4 page** (90% confidence)
- genpdf Image doesn't auto-scale; charts at 800px > A4 usable width ~482pt
- Fix: add `.with_scale(genpdf::Scale::new(0.5, 0.5))` in `embed_png`

**3. Truncation notice uses byte length but truncate() uses char count** (95% confidence)
- `text.len() > 2000` vs `text.chars().count()` mismatch for multi-byte UTF-8
- Fix: use `text.chars().count() > 2000`

**4. `test_generate_pdf_contains_analysis_name` searches raw binary PDF bytes** (85% confidence)
- Relies on genpdf writing /Title as raw ASCII in metadata - fragile
- Suggestion: add comment documenting the assumption, or use lopdf for parsing

**5. Chart failures leave dangling heading with no fallback text** (82% confidence)
- When chart rendering fails, section heading appears with blank gap
- Fix: add fallback paragraph "[Chart could not be rendered]" on error

## No Issues Found
- ExportError design clean
- Division by zero guarded
- BTreeMap for deterministic ordering correct
- truncate() function itself correct
- mod.rs change minimal
