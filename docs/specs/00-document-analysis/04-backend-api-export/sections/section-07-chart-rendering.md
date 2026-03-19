I have all the context needed. Let me now produce the section content.

# Section 07: Chart Rendering

## Overview

This section implements `backend/src/features/analysis/charts.rs` -- a module containing three chart rendering functions that produce PNG byte arrays using the `plotters` crate. These charts are consumed by the PDF export (section 08) and DOCX export (section 09) modules. The charts are never served directly via HTTP endpoints; they are embedded into export documents.

The three chart types are:
- **Coverage heatmap** -- a horizontal rectangle grid showing per-framework coverage percentages with green-to-red color interpolation
- **Radar chart** -- a manually drawn polygon chart showing multi-dimensional scores on a radial layout
- **Priority breakdown bar chart** -- color-coded horizontal or vertical bars showing finding counts by priority level

All functions return `Result<Vec<u8>, ChartError>` where the `Vec<u8>` contains valid PNG image bytes.

## Dependencies

**Section 01 (dependencies-and-appstate)** must be completed first. It adds the required Cargo dependencies:

- `plotters = "0.3"` -- chart drawing library with bitmap backend
- `image = "0.25"` -- image encoding (PNG output from raw pixel buffer)

These must be present in `backend/Cargo.toml` before this section can compile.

**Font files:** LiberationSans TTF fonts must exist at `backend/fonts/LiberationSans-Regular.ttf` and `backend/fonts/LiberationSans-Bold.ttf`. Section 01 downloads these. If fonts are missing, chart rendering must return a `ChartError` rather than panicking.

**Module wiring:** Add `pub mod charts;` to `backend/src/features/analysis/mod.rs`.

## Tests First

Create tests in a `#[cfg(test)] mod tests` block at the bottom of `backend/src/features/analysis/charts.rs`. The PNG magic bytes are `[0x89, 0x50, 0x4E, 0x47]` (the string `\x89PNG`).

### test_render_coverage_heatmap_returns_png

Call `render_coverage_heatmap` with sample data such as `[("NIST CSF".into(), 0.85), ("ISO 31000".into(), 0.4)]`. Assert the result is `Ok`, the byte vector is non-empty, and the first four bytes match PNG magic bytes `0x89504E47`.

### test_render_radar_chart_returns_png

Call `render_radar_chart` with labels like `["Identify", "Protect", "Detect", "Respond", "Recover"]` and corresponding values like `[0.9, 0.7, 0.5, 0.8, 0.6]`. Assert `Ok`, non-empty, PNG magic bytes.

### test_render_priority_chart_returns_png

Call `render_priority_chart` with data like `[("P1".into(), 3), ("P2".into(), 7), ("P3".into(), 12), ("P4".into(), 5)]`. Assert `Ok`, non-empty, PNG magic bytes.

### test_render_heatmap_empty_data

Call `render_coverage_heatmap` with an empty slice `&[]`. Assert it returns an `Err(ChartError::...)` or an `Ok` with a valid (possibly minimal) PNG -- it must not panic.

### test_render_radar_empty_labels

Call `render_radar_chart` with empty labels `&[]` and empty values `&[]`. Assert it returns an `Err` -- a radar chart with zero axes is meaningless. Must not panic.

## Implementation Details

### File: `backend/src/features/analysis/charts.rs`

### Error type

Define a `ChartError` enum at the top of the file:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ChartError {
    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("Image encoding error: {0}")]
    Encoding(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Font not found: {0}")]
    FontNotFound(String),
}
```

### Common rendering pipeline

All three chart functions follow the same pipeline:

1. Allocate a pixel buffer: `vec![0u8; (width * height * 3) as usize]` for RGB
2. Create a `BitMapBackend::with_buffer(&mut buffer, (width, height))` drawing area
3. Draw chart elements using plotters API
4. Drop the backend (releases the borrow on `buffer`)
5. Convert raw RGB buffer to PNG bytes using `image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, buffer)` then encode to PNG via `image::codecs::png::PngEncoder` writing to a `Vec<u8>` cursor
6. Return the PNG bytes

Standard chart dimensions: 800x600 pixels for all charts. These are embedded in documents, not displayed interactively, so fixed size is fine.

### Font loading

Attempt to load LiberationSans from `./fonts/LiberationSans-Regular.ttf` (relative to the working directory, which is `backend/`). If the font file is not found, return `ChartError::FontNotFound`. Use plotters' `FontDesc` or the `TTFontFamily` approach depending on what `plotters` version 0.3 supports -- the `("sans-serif", ...)` font family fallback is also acceptable for chart axis labels if bundled fonts are unavailable.

### render_coverage_heatmap

```rust
pub fn render_coverage_heatmap(
    frameworks: &[(String, f64)],  // (name, coverage 0.0..1.0)
) -> Result<Vec<u8>, ChartError>
```

- Return `ChartError::InvalidInput` if `frameworks` is empty
- Draw a horizontal rectangle grid with one row per framework
- Each row is a colored rectangle: interpolate from red (0.0) through yellow (0.5) to green (1.0) using `RGBColor`
- Framework name as text label to the left of each row
- Coverage percentage as text label inside or to the right of each rectangle
- Title: "Framework Coverage"

### render_radar_chart

```rust
pub fn render_radar_chart(
    labels: &[String],
    values: &[f64],  // 0.0..1.0
) -> Result<Vec<u8>, ChartError>
```

- Return `ChartError::InvalidInput` if `labels` is empty or `labels.len() != values.len()`
- This is a manual polygon implementation since plotters does not have a built-in radar chart
- Calculate vertex positions on a unit circle: for each label `i`, angle = `2 * PI * i / n - PI/2` (start at top)
- Draw concentric guide polygons at 0.25, 0.5, 0.75, 1.0 radii using `PathElement` with light gray strokes
- Draw axis lines from center to each vertex
- Place label text at each vertex position (slightly beyond the 1.0 radius)
- Draw the data polygon using `Polygon` with semi-transparent fill (e.g., `RGBAColor(66, 133, 244, 0.3)`) and solid border
- Plot data points as small circles at each vertex of the data polygon
- Title: "Coverage Radar"

### render_priority_chart

```rust
pub fn render_priority_chart(
    priorities: &[(String, i64)],
) -> Result<Vec<u8>, ChartError>
```

- Return `ChartError::InvalidInput` if `priorities` is empty
- Vertical bar chart with one bar per priority level
- Color coding: match on label text -- "P1" or "Critical" -> red `(220, 38, 38)`, "P2" or "High" -> orange `(234, 179, 8)`, "P3" or "Medium" -> yellow `(250, 204, 21)`, "P4" or "Low" -> green `(34, 197, 94)`, anything else -> gray
- Use plotters `ChartBuilder` with `build_cartesian_2d` for a standard bar chart layout
- X-axis: priority labels, Y-axis: count (integer range from 0 to max count)
- Title: "Findings by Priority"

### Module registration

Add to `backend/src/features/analysis/mod.rs`:

```rust
pub mod charts;
```

## File Paths Summary

| File | Action |
|------|--------|
| `backend/src/features/analysis/charts.rs` | Create (new file) |
| `backend/src/features/analysis/mod.rs` | Modify (add `pub mod charts;`) |

## Downstream Consumers

- **Section 08 (PDF export)** will call all three chart functions to embed PNG images in the PDF document
- **Section 09 (DOCX export)** will call all three chart functions to embed PNG images in the DOCX document

Neither section requires changes to the chart API signatures defined here.