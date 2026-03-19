# Research: Backend API & Report Generation

## Codebase Patterns

### Route Handler Patterns
- CRUD handlers in `compliance/routes.rs` return `AppResult<Json<T>>` or `AppResult<(StatusCode, Json<T>)>`
- Extractors: `State<AppState>`, `Path<String>`, `Query<T>`, `Json<T>`, `Multipart`
- UUIDs for IDs: `Uuid::new_v4().to_string()`
- Timestamps: `chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()`

### Error Handling (`error.rs`)
- `AppError` enum: Database, NotFound, BadRequest, Unauthorized, Forbidden, Internal
- `AppResult<T> = Result<T, AppError>`
- Auto-converts to JSON `{ "error": "type", "message": "..." }` with appropriate HTTP status

### Audit Logging
- `audit_log` table: id, user_id, action, entity_type, entity_id, old_value, new_value, ip_address, created_at
- Pattern: `INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at) VALUES (...)`
- new_value is JSON serialized via `serde_json::json!(...).to_string()`

### Pagination
- `PaginatedResponse<T>` wrapper: data, total, page, limit, total_pages
- Query struct with `#[serde(default)]` and `IntoParams` derive
- Offset calculation: `(page - 1) * limit`

### Router Registration
- Feature routers composed via `.nest("/path", feature::routes::router())`
- Main app: `Router::new().nest("/api", api_routes()).layer(cors)...with_state(state)`

### OpenAPI (utoipa)
- `#[utoipa::path(method, path, tag, request_body, responses(...))]` on handlers
- `params(QueryType)` for query parameters
- Groups by `tag` in Swagger UI at `/swagger-ui`

### File Upload (existing pattern)
- `Multipart` extractor, iterate fields with `next_field().await`
- Save to `uploads/{entity_id}/{uuid}` via `tokio::fs::write`
- Sanitize filenames with `Path::file_name()`

### AppState
```rust
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}
```

### Testing
- Integration tests in `backend/tests/` using `create_test_app()` helper
- `.oneshot(Request::builder()...)` pattern
- Auto-migration and test data seeding
- JSON assertions via `serde_json::Value`

### Analysis DB Schema
- `analyses` table: id, name, description, input_type, input_text, original_filename, file_path, extracted_text, status, error_message, prompt_template, matched_framework_ids, processing_time_ms, token_count, created_by, created_at, updated_at
- `analysis_findings` table: id, analysis_id, concept_id, framework_id, finding_type, confidence_score, evidence_text, recommendation, priority, sort_order, created_at
- Status enum: pending, processing, completed, failed, deleted

---

## Web Research

### Axum Multipart File Upload

**Key patterns:**
- Built-in `axum::extract::Multipart` — iterate fields, consume bytes
- Default body limit is 2MB — override with `DefaultBodyLimit::max(20 * 1024 * 1024)` for 20MB
- Multipart must be last extractor (consumes body)
- Validate content type against allowlist before processing
- Sanitize filenames with `Path::file_name()` to prevent path traversal

**Pitfalls:**
- Don't hold references to previous fields when calling `next_field()`
- Always validate file size and type server-side, don't trust client headers alone

### genpdf PDF Generation

**Key patterns:**
- `genpdf = "0.2"` with `images` feature for PNG embedding
- Load fonts from TTF directory: `fonts::from_files("./fonts", "LiberationSans", None)`
- `TableLayout::new(vec![1, 2, 1])` for relative column widths
- `FrameCellDecorator` for borders
- Image embedding: `elements::Image::from_dynamic_image(img)` from in-memory `image::DynamicImage`
- Render to bytes: `doc.render(&mut buf)` for HTTP responses
- Measurements in mm (except font sizes in points)
- Font embedding adds ~100-200 KiB per family

**Pitfalls:**
- No alpha transparency support
- Row element count must match column count exactly
- Font files must be bundled — no system font fallback

### plotters Server-Side Chart Rendering

**Key patterns:**
- `BitMapBackend::with_buffer(&mut rgb_buf, (width, height))` for in-memory rendering
- Buffer size must be `width * height * 3` bytes (RGB)
- Must call `root.present()` before reading buffer
- Encode to PNG via `image` crate: `ImageBuffer::from_raw()` → `write_to(Cursor, ImageFormat::Png)`
- No native radar/spider chart — implement manually with `Polygon`, `PathElement`, `Text`
- Heatmap: grid of `Rectangle` elements with interpolated colors
- Color: `RGBColor(r, g, b)`, transparency via `.mix(0.4)`, palette via `Palette99::pick(i)`

**Pitfalls:**
- Dropping backend without `present()` silently swallows errors
- System font dependency on headless servers — bundle TTF files
- Buffer size mismatch panics at runtime

### Chart Strategy for This Project

The spec requires:
1. **Coverage heatmap** — plotters rectangle grid with green-red interpolation
2. **Radar/spider chart** — manual implementation with Polygon + PathElement
3. **Priority breakdown** — bar chart with SegmentedCoord

All rendered to PNG bytes via `BitMapBackend` → `image` crate → embedded in genpdf/docx-rs documents.
