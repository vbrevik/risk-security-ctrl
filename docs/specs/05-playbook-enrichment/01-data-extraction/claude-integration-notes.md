# Integration Notes: Opus Review Feedback

## Integrating

1. **Fix clap assumption** — Correct. `main.rs` has no argument parsing. Must add `clap` dependency and refactor `main.rs` with backward-compatible default. **Integrating.**

2. **pdf-extract limitations** — Valid concern. Adding a known-limitations section covering text ordering, ligatures, and no `/PageLabels` access. **Integrating.**

3. **Tighten section/subsection detection** — Good point about fragile regex. Will specify multi-line scanning, fuzzy whitespace matching, and line-context anchoring for subsection headers. **Integrating.**

4. **Concept ID resolution via ontology lookup** — Correct. The plan should be explicit that we build a HashMap from the ontology JSON's `code` → `id` mapping, not string transformation. **Integrating.**

5. **Guidance file integration** — Valid gap. The guidance file should NOT be auto-imported by the existing ontology importer (different schema). It's consumed by a future API endpoint (spec 02-schema-import). **Integrating.**

6. **Define ExtractionError** — Yes, should use `thiserror`. **Integrating.**

7. **Commit to test fixture strategy** — Will commit to string fixtures as primary (raw text samples), with a note that a small PDF fixture is optional/supplementary. **Integrating.**

8. **Rename `trait.rs`** — Good catch. Will rename to `extractor.rs`. **Integrating.**

9. **Use `chrono::DateTime<Utc>`** — Correct, `chrono` is already a dependency. **Integrating.**

10. **Clarify STIG path boundary** — The CLI is a developer tool; any readable PDF is acceptable. Canonicalization + `.pdf` extension + magic bytes is sufficient. **Integrating.**

## Not Integrating

- **Async/sync clarification (#2)** — Minor concern. The extraction is CPU-bound and runs as a CLI command, not a server handler. It will run inside `#[tokio::main]` but this is fine for a blocking CLI subcommand. Adding a brief note but not refactoring.

- **Memory/performance (#8)** — For all 4 target PDFs (< 200 pages each), memory is not a concern. Not adding a performance section for this scope.

- **Phase 1/Phase 2 `_en` suffix (#13)** — Phase 1 outputs raw extraction keys without suffixes; Phase 2 (Claude structuring) adds `_en` suffixes. This is the intended division. Adding a clarifying note.
