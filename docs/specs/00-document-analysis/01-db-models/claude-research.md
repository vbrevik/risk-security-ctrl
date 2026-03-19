# Research Findings: 01-db-models

## Codebase Patterns

### Migration Style
- File naming: `00N_description.sql` (zero-padded)
- `CREATE TABLE IF NOT EXISTS` consistently
- `TEXT PRIMARY KEY` for IDs (UUID strings)
- Timestamps: `TEXT DEFAULT (datetime('now'))` for both created_at and updated_at
- FK pattern: `TEXT NOT NULL REFERENCES table_name(id)` with optional `ON DELETE CASCADE`
- Index naming: `idx_table_column`
- Section separators with `-- ====` comments

### Model Patterns
- Derives (order): `Debug, Serialize, Deserialize, FromRow, ToSchema` (DB structs), `Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq` (enums)
- Enums: `#[serde(rename_all = "snake_case")]` + manual `From<String>` and `Into<String>` impls for DB mapping
- Row vs Response: Use `Row` suffix for `FromRow` structs, implement `From<XyzRow> for Xyz`
- Query params: `#[derive(Debug, Deserialize, IntoParams)]` with default fns
- PaginatedResponse<T>: Existing generic wrapper with `new(data, total, page, limit)`

### Feature Module Structure
```
features/analysis/
  mod.rs    → pub mod models; pub mod routes;
  models.rs → enums, structs, From impls, tests
  routes.rs → handlers, router() fn
```
Wire in `features/mod.rs` (add `pub mod analysis;`) and `lib.rs` api_routes (add `.nest("/analyses", ...)`).

### Error Handling
- `AppError` enum with `thiserror`: Database, NotFound, BadRequest, Unauthorized, Forbidden, Internal
- `AppResult<T> = Result<T, AppError>`
- Handlers return `AppResult<(StatusCode, Json<T>)>`

### async_trait Status
- **NOT used anywhere** in the codebase currently
- Not in Cargo.toml dependencies
- All async is native Rust async fn

### Testing
- `#[cfg(test)] mod tests` at end of models.rs
- Test enum conversions (String <-> Enum)
- Dev deps: tokio-test, tower (with "util")

### Timestamp Pattern
```rust
let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
```

### sqlx Patterns
- `sqlx::query_as::<_, Model>()` for FromRow results
- `sqlx::query_scalar()` for single values (COUNT)
- `sqlx::query()` for mutations
- `?` placeholders, bind in order

---

## Web Research: Rust Async Trait Patterns

### Native async fn in traits (Rust 1.75+)
- Stabilized Dec 2023. Can write `async fn` directly in trait definitions.
- **Not dyn-compatible** — cannot use `Box<dyn MyTrait>` with async methods.
- Returned futures are NOT guaranteed `Send` — won't work with `tokio::spawn`.

### Options for the MatchingEngine trait

| Approach | Dyn Dispatch | Send | Heap Alloc | Complexity |
|----------|-------------|------|-----------|-----------|
| Native async fn | No | No | No | Lowest |
| `trait_variant` crate | No | Yes | No | Low |
| `async-trait` crate | Yes | Yes | Yes (Box::pin) | Medium |
| Manual boxing | Yes | Yes | Yes | High |
| **Enum dispatch** | N/A | Yes | No | Low |

### Recommendation for MatchingEngine

Since we have a **closed set of implementations** (DeterministicMatcher now, LlmMatcher later):

**Option A: Enum dispatch** (no trait needed)
```rust
enum MatchingEngine {
    Deterministic(DeterministicMatcher),
    Llm(LlmMatcher),
}
```
Pros: No heap allocation, Send by default, simple. Cons: Must modify enum when adding new variants.

**Option B: async-trait crate** (if trait object flexibility desired)
```rust
#[async_trait]
trait MatchingEngine: Send + Sync { ... }
```
Pros: True polymorphism, extensible. Cons: Adds dependency, Box::pin per call.

**Given the spec explicitly calls for a trait interface**, go with `async-trait` crate for the trait definition. The Box::pin overhead is negligible for analysis operations that take seconds.

---

## Web Research: SQLite Schema Design

### Confidence Score Storage
- Use `REAL` type with `CHECK(score BETWEEN 0.0 AND 1.0)`
- SQLite REAL is 8-byte IEEE 754 float — more than sufficient

### Index Strategy for Analysis/Findings

**Composite indexes for common queries:**
```sql
CREATE INDEX idx_findings_analysis_type ON analysis_findings(analysis_id, finding_type);
CREATE INDEX idx_findings_analysis_priority ON analysis_findings(analysis_id, priority);
CREATE INDEX idx_findings_framework ON analysis_findings(framework_id);
```

**Key rule:** Never create two indexes where one is a prefix of the other.

**Partial indexes for common filters:**
```sql
CREATE INDEX idx_findings_gaps ON analysis_findings(analysis_id, priority)
    WHERE finding_type = 'gap';
```

### JSON Column Patterns
- Store arrays as TEXT with `json_array()` or plain JSON string
- Query with `json_each()` for membership tests
- Use `->>` operator (SQLite 3.38.0+) for scalar extraction
- Consider JSONB for performance (SQLite 3.45.0+)

### CASCADE Delete — Critical Notes
1. **MUST enable `PRAGMA foreign_keys = ON`** at connection time (off by default!)
2. **Always index child FK columns** — without index, CASCADE requires full table scan
3. In SQLx, enable via `after_connect` or pool configuration

### Aggregate Performance
- `COUNT(*)` faster than `COUNT(column)` (no NULL check)
- Composite index on `(analysis_id, finding_type)` makes GROUP BY fast
- For dashboards, consider summary table with triggers (overkill for MVP)
- Run `ANALYZE` periodically for optimal query plans

### Sources
- SQLite JSON1: https://www.sqlite.org/json1.html
- SQLite Query Planner: https://www.sqlite.org/queryplanner.html
- SQLite Foreign Keys: https://www.sqlite.org/foreignkeys.html
- Rust Reference: https://doc.rust-lang.org/reference/items/traits.html
