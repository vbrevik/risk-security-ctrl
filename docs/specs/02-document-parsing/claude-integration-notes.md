# Integration Notes: Opus Review Feedback

## Critical Finding: Implementation Already Exists

The Opus reviewer discovered that `parser.rs` (490 lines, 14 tests), `tokenizer.rs` (187 lines, 9 tests), and even `matcher.rs` (1273 lines, 27 tests) already exist from parallel development sessions. The plan must be reframed as incremental improvements.

## Integrating (with changes to plan)

1. **Reframe plan as delta** — List what exists, what needs modification, what's new
2. **Align crate versions** — Cargo.toml already has `zip = "2"` and `quick-xml = "0.37"`, not the plan's `0.6` and `0.31`
3. **Add spawn_blocking** — Sync file I/O should be wrapped for async context
4. **Add magic byte validation** — Check %PDF and PK headers alongside extension
5. **Sanitize analysis_id** — UUID validation before using in filesystem paths
6. **Add RequestBodyLimitLayer** — At router level for proper size limiting
7. **Align term_frequency return type** — Existing returns `HashMap<String, usize>` (raw counts), plan said `f64`. Keep raw counts.
8. **Add EmptyDocument message** — Current enum variant has no String payload, need to add it for scanned PDF detection message

## Not Integrating (with reasons)

1. **Separate error.rs file** — Existing co-location in parser.rs works fine, no need to split
2. **unicode-segmentation for sentence splitting** — Existing manual implementation works, changing to library is a separate concern
3. **stop-words/rust-stemmers crates** — Existing hardcoded stopwords work for MVP. Upgrading to crate-based is a nice-to-have but adds dependencies for marginal gain right now
4. **Norwegian language detection** — Useful but not critical for MVP document parsing. Can be added when the matching engine needs it
5. **Unicode normalization crate** — Overkill for current use case
6. **Password-protected document variants** — Edge case, existing CorruptFile error is adequate
7. **Heading heuristic changes** — Existing page-per-section for PDF is more reliable than aggressive heading detection
