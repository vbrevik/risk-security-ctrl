<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-migration
section-02-enums
section-03-models
section-04-engine-trait
section-05-wiring
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-migration | - | section-03, section-05 | Yes |
| section-02-enums | - | section-03, section-04 | Yes |
| section-03-models | section-01, section-02 | section-05 | No |
| section-04-engine-trait | section-02 | section-05 | Yes |
| section-05-wiring | section-03, section-04 | - | No |

## Execution Order

1. section-01-migration, section-02-enums (parallel, no dependencies)
2. section-03-models, section-04-engine-trait (parallel after 01+02)
3. section-05-wiring (after all others)

## Section Summaries

### section-01-migration
Database migration `003_analysis_schema.sql` with `analyses` and `analysis_findings` tables. Includes PRAGMA foreign_keys enablement in `main.rs` pool configuration and foreign key integrity check at startup.

### section-02-enums
Three enums (`InputType`, `AnalysisStatus`, `FindingType`) with serde serialization, `From<String>` / `Into<String>` conversions, and unit tests. Created in `features/analysis/models.rs`.

### section-03-models
All Rust structs: `AnalysisRow`, `Analysis`, `CreateAnalysisRequest`, `AnalysisFindingRow`, `AnalysisFinding`, `AnalysisFindingWithConcept`, `AnalysisSummary`, `FrameworkFindingSummary`, query parameter structs. Includes `From<Row>` conversions and unit tests.

### section-04-engine-trait
`MatchingEngine` async trait using `async-trait` crate, plus `MatchingResult`, `NewFinding`, and `AnalysisError` types. Created in `features/analysis/engine.rs`. Includes dyn-compatibility compile check.

### section-05-wiring
Module registration: `features/analysis/mod.rs`, update `features/mod.rs`, add `async-trait` to `Cargo.toml`. Verify `cargo check` and `cargo test` pass.
