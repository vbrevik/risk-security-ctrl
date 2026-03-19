Now I have everything I need.

# Section 05: Module Wiring and Dependencies

## Overview

This section registers the new `analysis` feature module into the codebase and adds the `async-trait` dependency. It is the final section and depends on all previous sections being complete (section-01 through section-04). No new logic is introduced here -- this is purely structural wiring so that the compiler can find and build the new module.

After completing this section, `cargo check` and `cargo test` must both pass cleanly.

## Dependencies

- **section-01-migration**: The migration file must exist at `backend/migrations/003_analysis_schema.sql`
- **section-02-enums**: Enums must be defined in `backend/src/features/analysis/models.rs`
- **section-03-models**: All structs and `From` impls must be in `backend/src/features/analysis/models.rs`
- **section-04-engine-trait**: The trait, error types, and supporting structs must be in `backend/src/features/analysis/engine.rs`

## Tests First

There are no dedicated unit tests for this section. Validation is performed by confirming:

1. `cargo check` compiles without errors
2. `cargo test` runs all existing tests plus the new tests from sections 02-04
3. The feature module is accessible from other features via `use crate::features::analysis::models::*` and `use crate::features::analysis::engine::*`

A quick smoke test can be added anywhere (e.g., in a doc comment or an existing test file) to confirm accessibility:

```rust
// This compiles only if wiring is correct
use crate::features::analysis::models::InputType;
use crate::features::analysis::engine::MatchingEngine;
```

## Implementation

### 1. Create `backend/src/features/analysis/mod.rs`

This file declares the two sub-modules. It follows the exact pattern used by `backend/src/features/compliance/mod.rs`.

```rust
pub mod models;
pub mod engine;
```

Note: There is no `pub mod routes;` line yet. The routes module does not exist until split 04-backend-api-export. Do NOT add a routes declaration here.

### 2. Modify `backend/src/features/mod.rs`

Add `pub mod analysis;` in alphabetical order, which means it goes **before** the existing `pub mod auth;` line.

The file should become:

```rust
pub mod analysis;
pub mod auth;
pub mod compliance;
pub mod ontology;
pub mod reports;
```

File path: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/mod.rs`

### 3. Add `async-trait` to `backend/Cargo.toml`

Add the `async-trait` crate under `[dependencies]`. Place it in the appropriate alphabetical/logical section. Since the existing Cargo.toml groups dependencies by category with comments, add it under a new comment or alongside error handling:

```toml
# Async trait support
async-trait = "0.1"
```

This can go after the `tower-http` line or in its own logical group. The key requirement is that `async-trait = "0.1"` appears in `[dependencies]`.

File path: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml`

### 4. Verify the build

After all three changes, run from the `backend/` directory:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo check
```

This must compile without errors. If it fails, the most likely causes are:

- Missing `use` imports in `models.rs` or `engine.rs` (e.g., `use async_trait::async_trait;` in engine.rs)
- A `pub mod routes;` line accidentally added to `mod.rs` when no `routes.rs` exists
- Typo in the module name (`analysis` vs `analyses`)

Then run tests:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo test
```

All tests from sections 02 (enum round-trips), 03 (From conversions), and 04 (dyn compatibility, error construction) must pass.

## Files Summary

| File | Action |
|------|--------|
| `backend/src/features/analysis/mod.rs` | **Create** -- declares `pub mod models;` and `pub mod engine;` |
| `backend/src/features/mod.rs` | **Modify** -- add `pub mod analysis;` before `pub mod auth;` |
| `backend/Cargo.toml` | **Modify** -- add `async-trait = "0.1"` to `[dependencies]` |

## Important Notes

- Do NOT add router nesting in `backend/src/lib.rs` or `backend/src/main.rs` for the analysis feature. There are no routes yet -- that comes in a later split (04-backend-api-export).
- The `PaginatedResponse<T>` type from `ontology::models` is already generic and should be reused by the analysis feature when routes are added. No re-export or re-definition is needed in this section.
- The `after_connect` PRAGMA change to `main.rs` is covered by section-01-migration, not this section.