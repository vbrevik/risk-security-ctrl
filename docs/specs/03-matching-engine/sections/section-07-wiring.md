Good, the matcher module does not yet exist. Now I have everything I need to write the section.

# Section 7: Module Wiring

## Overview

This section wires the `matcher` module into the existing `analysis` feature module so that `DeterministicMatcher` and its supporting types are accessible to the rest of the codebase. This is the final section in the matching engine implementation chain.

## Dependencies

- **section-06-matcher-impl** must be completed first. That section creates the file `backend/src/features/analysis/matcher.rs` containing `DeterministicMatcher`, `MatcherConfig`, `Topic`, and all supporting functions.

## Tests

There are no dedicated unit tests for this section. Correctness is validated by:

1. `cargo check` compiling successfully (proves the module declaration is syntactically correct and the matcher module is found).
2. `cargo test` passing (proves the tests written in sections 01-06 inside `matcher.rs` are discovered and executed by the test harness).

If `cargo test` runs and all matcher tests from prior sections pass, the wiring is correct.

## Implementation

### File to modify: `backend/src/features/analysis/mod.rs`

The current contents of this file are:

```rust
pub mod engine;
pub mod models;
pub mod parser;
pub mod tokenizer;
```

Add a single line to register the new matcher module:

```rust
pub mod matcher;
```

This makes the following items available to the rest of the application via `crate::features::analysis::matcher::*`:

- `DeterministicMatcher` -- the struct implementing the `MatchingEngine` trait
- `MatcherConfig` -- prompt template configuration
- `Topic` -- topic tag struct used for framework detection

No other files need to change for this section. Specifically:

- **No `Cargo.toml` changes** are needed. All dependencies (`sqlx`, `serde`, `serde_json`, `async-trait`, `tracing`, `thiserror`) are already declared in the backend crate's `Cargo.toml`.
- **No route handler changes** are needed in this section. Route handlers that call `DeterministicMatcher::new(topics).analyze(text, prompt_template, &db)` belong to a separate split (split 04 in the broader project plan).
- **No re-exports** are needed at the crate root. Consumers import directly from the feature module path.

## Verification Steps

After making the change, run these commands from `backend/`:

1. `cargo check` -- confirms the module compiles and all imports resolve.
2. `cargo test --lib features::analysis::matcher` -- confirms all matcher tests are discovered and pass.
3. `cargo test` -- confirms the new module does not break any existing tests.

## File Paths Summary

| File | Action |
|------|--------|
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs` | Add `pub mod matcher;` line |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs` | Must already exist (created in section-06) |