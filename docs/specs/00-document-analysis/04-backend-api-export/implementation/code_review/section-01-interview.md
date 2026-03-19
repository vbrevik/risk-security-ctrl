# Section 01 Code Review Interview

## Auto-fixes Applied

1. **Duplicated topic-loading logic** → Extracted to `pub fn load_topics()` in `lib.rs`. Both `main.rs` and `tests/common/mod.rs` now call this shared function.

2. **Silent data loss via filter_map** → `load_topics()` now logs `tracing::warn!` for each malformed topic entry skipped, with index number.

## User Decisions

3. **genpdf/image version alignment** → User chose to keep `image = "0.25"` despite 3 image crate versions in tree (0.23 via genpdf, 0.24 via pdf-extract, 0.25 for plotters). Needed for chart rendering in section-07.

## Let Go

4. **Relative path fragility** → Matches existing pattern used throughout main.rs for ontology data loading.
5. **Missing dedicated test** → Existing 115 tests verify AppState works correctly.
6. **No analysis route registered** → Expected in section-02.
