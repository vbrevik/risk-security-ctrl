# Section 01: Dependencies and AppState

## Goal

Add the new Cargo dependencies required by the analysis API and export features, extend `AppState` with preloaded topic data, update the test helper to match, and download LiberationSans font files for chart/PDF rendering.

## Background

The analysis feature needs four new crates:

- **genpdf** (with `images` feature) -- PDF generation for export reports
- **plotters** -- chart rendering (heatmaps, radar charts, bar charts) to PNG
- **image** -- PNG encoding from raw bitmap buffers produced by plotters
- **docx-rs** -- DOCX document generation for export reports

The `DeterministicMatcher` (in `backend/src/features/analysis/matcher.rs`) requires a `Vec<Topic>` loaded from `ontology-data/topic-tags.json`. Rather than reading this file on every request, the topics should be loaded once at startup and stored in `AppState`. The `Topic` type used in the matcher is defined in `backend/src/features/analysis/matcher.rs` (fields: `id`, `name_en`, `concept_ids`).

Chart rendering and PDF export require TrueType fonts. LiberationSans (Apache 2.0 licensed) should be downloaded and stored in `backend/fonts/`.

## Tests

There are no dedicated unit tests for this section since it is purely infrastructure (dependency additions, struct changes, font files). Verification is done by confirming the project compiles after the changes. However, the test helper must be updated so that all existing and future integration tests continue to work with the new `AppState` shape.

The following test stub confirms the topics are loaded correctly. Place it in `backend/tests/api_tests.rs` (or a new file `backend/tests/appstate_tests.rs`):

```rust
#[tokio::test]
async fn test_topics_loaded_in_appstate() {
    /// After creating the test app, the AppState should contain a non-empty
    /// list of topics loaded from ontology-data/topic-tags.json.
    /// This test ensures the startup loading logic works correctly.
}
```

Since `AppState` is not directly accessible from the router, the practical verification is that `create_test_app()` does not panic and the router is successfully created. The existing tests in `backend/tests/api_tests.rs` and `backend/tests/compliance_tests.rs` serve as regression checks -- they must continue to pass after the `AppState` change.

## Implementation Steps

### 1. Add Cargo dependencies

**File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml`**

Add to `[dependencies]`:

```toml
# Chart rendering
plotters = "0.3"
image = "0.25"

# PDF export
genpdf = { version = "0.2", features = ["images"] }

# DOCX export
docx-rs = "0.4"
```

These are added alongside the existing dependencies. No version conflicts are expected with the current dependency set.

### 2. Extend AppState with topics

**File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs`**

The current `AppState` struct is:

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}
```

Add the `topics` field using the matcher's `Topic` type:

```rust
use features::analysis::matcher::Topic;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
    pub topics: Vec<Topic>,
}
```

The `Topic` struct in `matcher.rs` already derives `Clone` and `Debug` (line 107 of matcher.rs: `#[derive(Debug, Clone, Deserialize)]`), so this compiles without changes to the matcher module.

### 3. Load topics at startup

**File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs`**

Before constructing `AppState` (around line 137), add topic loading logic. The file format matches the `TopicTagsFile` struct in `backend/src/features/ontology/models.rs`, but the matcher's `Topic` type is a subset (no `name_nb`, `description_en`, `description_nb`). Load from file and map to the matcher's `Topic` type:

```rust
// Load topics from ontology-data/topic-tags.json
let topics = {
    let path = std::path::Path::new("../ontology-data/topic-tags.json");
    if path.exists() {
        let contents = std::fs::read_to_string(path)?;
        let file: serde_json::Value = serde_json::from_str(&contents)?;
        // Extract topics array and map to matcher::Topic
        // Each topic needs: id, name_en, concept_ids
    } else {
        tracing::warn!("topic-tags.json not found, analysis matching will have no topics");
        vec![]
    }
};
```

A cleaner approach is to define a small deserialization struct locally (or reuse `TopicTagsFile` from `ontology/models.rs`) and convert to `Vec<matcher::Topic>`. The key fields to extract are `id`, `name_en`, and `concept_ids`.

Then construct AppState with the topics:

```rust
let state = AppState {
    db,
    config: config.clone(),
    topics,
};
```

### 4. Update test helper

**File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/common/mod.rs`**

The `create_test_app()` function constructs `AppState` directly (line 43-46). It must be updated to include `topics`. Apply the same loading logic as in `main.rs`:

```rust
// Load topics for test state
let topics = {
    let path = std::path::Path::new("../ontology-data/topic-tags.json");
    if path.exists() {
        // same loading logic as main.rs
    } else {
        vec![]
    }
};

let state = AppState {
    db: pool,
    config: config.clone(),
    topics,
};
```

This is critical: without this change, all existing integration tests will fail to compile because `AppState` now requires the `topics` field.

### 5. Download LiberationSans fonts

**Directory: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/fonts/`**

Create the `backend/fonts/` directory and download the following LiberationSans TTF files (Apache 2.0 licensed, from the `liberation-fonts` project):

- `LiberationSans-Regular.ttf`
- `LiberationSans-Bold.ttf`
- `LiberationSans-Italic.ttf`

These can be downloaded from the Fedora release page or GitHub mirror. The font files are used by `genpdf` for PDF rendering and by `plotters` for chart text labels in sections 07, 08, and 09.

The fonts should be committed to the repository (they are small, ~400KB total, and are a build-time dependency that should not require network access).

Add a `.gitkeep` or similar marker if committing an empty directory first, but the fonts themselves should be added in this section.

### 6. Verify compilation

Run `cargo check` from `backend/` to confirm:
- All four new dependencies resolve without conflicts
- `AppState` compiles with the new `topics` field
- `main.rs` compiles with topic loading
- All existing tests compile (the test helper has the updated `AppState` constructor)

Then run `cargo test` to confirm no existing tests regress.

## Files Modified

| File | Change |
|------|--------|
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml` | Add plotters, image, genpdf, docx-rs dependencies |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs` | Add `topics: Vec<Topic>` to `AppState` |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs` | Load topics from JSON at startup, pass to AppState |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/common/mod.rs` | Update `create_test_app()` to include topics in AppState |

## Files Created

| File/Directory | Purpose |
|----------------|---------|
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/fonts/LiberationSans-Regular.ttf` | Regular weight font for charts and PDF |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/fonts/LiberationSans-Bold.ttf` | Bold weight font for headings |
| `/Users/vidarbrevik/projects/risk-security-ctrl/backend/fonts/LiberationSans-Italic.ttf` | Italic font for emphasis |

## Dependencies on Other Sections

None. This is the foundational section that all other sections depend on. Sections 02 through 11 all require the updated `AppState` and/or the new Cargo dependencies added here.

## Notes

- The `Topic` type in `matcher.rs` has fewer fields than the `Topic` type in `ontology/models.rs`. When loading from `topic-tags.json`, only `id`, `name_en`, and `concept_ids` need to be extracted. The other fields (`name_nb`, `description_en`, `description_nb`) are ignored for matching purposes.
- The `docx-rs` crate is distinct from the `zip` + `quick-xml` crates already in Cargo.toml. Those existing crates are used by the document parser (split 02) for reading DOCX files. The `docx-rs` crate is used for writing/generating DOCX export files in section 09.
- If `genpdf` version 0.2 has compatibility issues with `image` 0.25, check for a newer version or pin `image` to the version that `genpdf` expects. The `genpdf` `images` feature internally depends on the `image` crate, so version alignment matters.