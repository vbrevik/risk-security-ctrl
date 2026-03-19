use plotters::prelude::*;
use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum ChartError {
    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("Image encoding error: {0}")]
    Encoding(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn buffer_to_png(buffer: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, ChartError> {
    let img = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, buffer)
        .ok_or_else(|| ChartError::Encoding("failed to create image buffer".into()))?;
    let mut png_bytes = Cursor::new(Vec::new());
    img.write_to(&mut png_bytes, image::ImageFormat::Png)
        .map_err(|e| ChartError::Encoding(e.to_string()))?;
    Ok(png_bytes.into_inner())
}

/// Render a horizontal heatmap showing per-framework coverage percentages.
pub fn render_coverage_heatmap(
    frameworks: &[(String, f64)],
) -> Result<Vec<u8>, ChartError> {
    if frameworks.is_empty() {
        return Err(ChartError::InvalidInput("no frameworks provided".into()));
    }

    let h = (50 * frameworks.len() as u32 + 80).max(HEIGHT);
    let mut buffer = vec![0u8; (WIDTH * h * 3) as usize];

    {
        let root = BitMapBackend::with_buffer(&mut buffer, (WIDTH, h)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError::Rendering(e.to_string()))?;

        // Title
        root.draw(&Text::new(
            "Framework Coverage",
            (WIDTH as i32 / 2 - 80, 10),
            ("sans-serif", 20).into_font().color(&BLACK),
        )).map_err(|e| ChartError::Rendering(e.to_string()))?;

        let bar_height = 30;
        let start_y = 50;
        let label_width = 200;
        let bar_width = (WIDTH - label_width as u32 - 60) as i32;

        for (i, (name, coverage)) in frameworks.iter().enumerate() {
            let y = start_y + (i as i32 * (bar_height + 10));
            let cov = coverage.clamp(0.0, 1.0);

            // Interpolate color: red(0) -> yellow(0.5) -> green(1.0)
            let (r, g, b) = if cov < 0.5 {
                let t = cov * 2.0;
                ((220.0 * (1.0 - t) + 234.0 * t) as u8, (38.0 * (1.0 - t) + 179.0 * t) as u8, 38)
            } else {
                let t = (cov - 0.5) * 2.0;
                ((234.0 * (1.0 - t) + 34.0 * t) as u8, (179.0 * (1.0 - t) + 197.0 * t) as u8, (8.0 * (1.0 - t) + 94.0 * t) as u8)
            };

            // Label
            root.draw(&Text::new(
                name.clone(),
                (10, y + 5),
                ("sans-serif", 14).into_font().color(&BLACK),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;

            // Bar background
            root.draw(&Rectangle::new(
                [(label_width, y), (label_width + bar_width, y + bar_height)],
                ShapeStyle::from(&RGBColor(230, 230, 230)).filled(),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;

            // Bar fill
            let fill_width = (bar_width as f64 * cov) as i32;
            root.draw(&Rectangle::new(
                [(label_width, y), (label_width + fill_width, y + bar_height)],
                ShapeStyle::from(&RGBColor(r, g, b)).filled(),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;

            // Percentage text
            root.draw(&Text::new(
                format!("{:.0}%", cov * 100.0),
                (label_width + fill_width + 5, y + 5),
                ("sans-serif", 14).into_font().color(&BLACK),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;
        }

        root.present().map_err(|e| ChartError::Rendering(e.to_string()))?;
    }

    buffer_to_png(buffer, WIDTH, h)
}

/// Render a radar/spider chart with manually drawn polygons.
pub fn render_radar_chart(
    labels: &[String],
    values: &[f64],
) -> Result<Vec<u8>, ChartError> {
    if labels.is_empty() || labels.len() != values.len() {
        return Err(ChartError::InvalidInput("labels and values must be non-empty and equal length".into()));
    }

    let mut buffer = vec![0u8; (WIDTH * HEIGHT * 3) as usize];

    {
        let root = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError::Rendering(e.to_string()))?;

        let cx = WIDTH as f64 / 2.0;
        let cy = HEIGHT as f64 / 2.0 + 15.0;
        let radius = 220.0;
        let n = labels.len();

        // Title
        root.draw(&Text::new(
            "Coverage Radar",
            ((cx - 60.0) as i32, 10),
            ("sans-serif", 20).into_font().color(&BLACK),
        )).map_err(|e| ChartError::Rendering(e.to_string()))?;

        // Helper to get polygon vertex
        let vertex = |i: usize, r: f64| -> (i32, i32) {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64) - std::f64::consts::FRAC_PI_2;
            ((cx + r * angle.cos()) as i32, (cy + r * angle.sin()) as i32)
        };

        // Draw guide polygons
        for &scale in &[0.25, 0.5, 0.75, 1.0] {
            let points: Vec<(i32, i32)> = (0..n).map(|i| vertex(i, radius * scale)).collect();
            let mut closed = points.clone();
            closed.push(points[0]);
            root.draw(&PathElement::new(
                closed,
                ShapeStyle::from(&RGBColor(220, 220, 220)).stroke_width(1),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;
        }

        // Draw axis lines and labels
        for i in 0..n {
            let (vx, vy) = vertex(i, radius);
            root.draw(&PathElement::new(
                vec![(cx as i32, cy as i32), (vx, vy)],
                ShapeStyle::from(&RGBColor(200, 200, 200)).stroke_width(1),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;

            let (lx, ly) = vertex(i, radius + 20.0);
            root.draw(&Text::new(
                labels[i].clone(),
                (lx - 20, ly - 5),
                ("sans-serif", 12).into_font().color(&BLACK),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;
        }

        // Draw data polygon
        let data_points: Vec<(i32, i32)> = values.iter().enumerate()
            .map(|(i, v)| vertex(i, radius * v.clamp(0.0, 1.0)))
            .collect();
        let mut closed_data = data_points.clone();
        closed_data.push(data_points[0]);

        root.draw(&Polygon::new(
            closed_data.clone(),
            ShapeStyle::from(&RGBAColor(66, 133, 244, 0.3)).filled(),
        )).map_err(|e| ChartError::Rendering(e.to_string()))?;

        root.draw(&PathElement::new(
            closed_data,
            ShapeStyle::from(&RGBColor(66, 133, 244)).stroke_width(2),
        )).map_err(|e| ChartError::Rendering(e.to_string()))?;

        // Draw data points
        for &(px, py) in &data_points {
            root.draw(&Circle::new(
                (px, py), 4,
                ShapeStyle::from(&RGBColor(66, 133, 244)).filled(),
            )).map_err(|e| ChartError::Rendering(e.to_string()))?;
        }

        root.present().map_err(|e| ChartError::Rendering(e.to_string()))?;
    }

    buffer_to_png(buffer, WIDTH, HEIGHT)
}

/// Render a vertical bar chart showing finding counts by priority.
pub fn render_priority_chart(
    priorities: &[(String, i64)],
) -> Result<Vec<u8>, ChartError> {
    if priorities.is_empty() {
        return Err(ChartError::InvalidInput("no priority data provided".into()));
    }

    let mut buffer = vec![0u8; (WIDTH * HEIGHT * 3) as usize];

    {
        let root = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| ChartError::Rendering(e.to_string()))?;

        let max_val = priorities.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);

        let mut chart = ChartBuilder::on(&root)
            .caption("Findings by Priority", ("sans-serif", 20).into_font())
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                0i64..(priorities.len() as i64),
                0i64..(max_val + max_val / 5 + 1),
            )
            .map_err(|e| ChartError::Rendering(e.to_string()))?;

        chart.configure_mesh()
            .disable_x_mesh()
            .x_labels(priorities.len())
            .x_label_formatter(&|x| {
                priorities.get(*x as usize).map(|(l, _)| l.clone()).unwrap_or_default()
            })
            .draw()
            .map_err(|e| ChartError::Rendering(e.to_string()))?;

        chart.draw_series(
            priorities.iter().enumerate().map(|(i, (label, count))| {
                let color = match label.as_str() {
                    "P1" | "Critical" => RGBColor(220, 38, 38),
                    "P2" | "High" => RGBColor(234, 179, 8),
                    "P3" | "Medium" => RGBColor(250, 204, 21),
                    "P4" | "Low" => RGBColor(34, 197, 94),
                    _ => RGBColor(156, 163, 175),
                };
                Rectangle::new(
                    [(i as i64, 0), (i as i64 + 1, *count)],
                    ShapeStyle::from(&color).filled(),
                )
            })
        ).map_err(|e| ChartError::Rendering(e.to_string()))?;

        root.present().map_err(|e| ChartError::Rendering(e.to_string()))?;
    }

    buffer_to_png(buffer, WIDTH, HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG_MAGIC: [u8; 4] = [0x89, 0x50, 0x4E, 0x47];

    #[test]
    fn test_render_coverage_heatmap_returns_png() {
        let data = vec![
            ("NIST CSF".into(), 0.85),
            ("ISO 31000".into(), 0.4),
        ];
        let result = render_coverage_heatmap(&data).unwrap();
        assert!(!result.is_empty());
        assert_eq!(&result[..4], &PNG_MAGIC);
    }

    #[test]
    fn test_render_radar_chart_returns_png() {
        let labels: Vec<String> = vec!["Identify", "Protect", "Detect", "Respond", "Recover"]
            .into_iter().map(String::from).collect();
        let values = vec![0.9, 0.7, 0.5, 0.8, 0.6];
        let result = render_radar_chart(&labels, &values).unwrap();
        assert!(!result.is_empty());
        assert_eq!(&result[..4], &PNG_MAGIC);
    }

    #[test]
    fn test_render_priority_chart_returns_png() {
        let data = vec![
            ("P1".into(), 3i64),
            ("P2".into(), 7),
            ("P3".into(), 12),
            ("P4".into(), 5),
        ];
        let result = render_priority_chart(&data).unwrap();
        assert!(!result.is_empty());
        assert_eq!(&result[..4], &PNG_MAGIC);
    }

    #[test]
    fn test_render_heatmap_empty_data() {
        let result = render_coverage_heatmap(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_radar_empty_labels() {
        let result = render_radar_chart(&[], &[]);
        assert!(result.is_err());
    }
}
