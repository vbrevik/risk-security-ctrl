# Interview Transcript: 01-db-models

## Q1: MatchingEngine trait design approach

**Question:** Research shows two viable approaches: async-trait crate (true trait with dyn dispatch) vs enum dispatch (simpler, zero-cost). The spec calls for a trait, and we have a closed set of implementations. Which approach?

**Answer:** async-trait crate. True trait interface with dyn dispatch. Matches the spec's design and is extensible.

## Q2: PRAGMA foreign_keys = ON

**Question:** The codebase doesn't currently enable `PRAGMA foreign_keys = ON`. CASCADE deletes on analysis_findings won't work without it. Should we add it globally to the SQLite pool configuration?

**Answer:** Yes, enable globally. Add to SQLite pool `after_connect`. Safe since existing tables already have FK constraints defined.

## Q3: JSON array storage for matched_framework_ids

**Question:** Should we use SQLite's json functions for queries against the matched_framework_ids field, or deserialize in Rust?

**Answer:** Rust-side deserialization. Store as plain TEXT, deserialize with serde in Rust. Simpler, no SQL json functions needed.

## Q4: Aggregate computation for summaries

**Question:** Should AnalysisSummary and FrameworkFindingSummary be computed on-the-fly with SQL or maintained in a summary table with triggers?

**Answer:** On-the-fly SQL. Compute with COUNT/GROUP BY at query time. Simpler for MVP scale.
