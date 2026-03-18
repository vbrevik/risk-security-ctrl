Now I have all the context needed. Let me produce the section content.

# Section 03: Create Analysis (Text Input)

## Overview

This section implements the `create_analysis` handler in `backend/src/features/analysis/routes.rs`. The handler accepts a JSON `CreateAnalysisRequest`, validates input, inserts an analysis record into the database, runs the document parser and deterministic matcher pipeline synchronously (with a 30-second timeout), stores findings, and returns an `AnalysisSummary` response.

This is the core orchestration handler for text-based analysis. Section 04 (file upload) reuses the same matching flow established here.

## Dependencies

- **Section 01** must be completed first: `AppState` must have `topics: Vec<Topic>` field, and Cargo dependencies must be in place.
- **Section 02** must be completed first: the route scaffold in `routes.rs` must exist with `create_analysis` as a handler stub, and the router must be wired into `lib.rs`.

## Existing Types (Do Not Recreate)

All of these already exist in `backend/src/features/analysis/models.rs`:

- `CreateAnalysisRequest` -- has fields `name`, `description`, `input_text`, `prompt_template`
- `Analysis` / `AnalysisRow` -- database row and API response types
- `AnalysisSummary` -- `#[serde(flatten)]` on `Analysis` plus `total_findings`, `gap_count`, `addressed_count`, `partially_addressed_count`, `frameworks_matched: Vec<FrameworkFindingSummary>`
- `FrameworkFindingSummary` -- per-framework breakdown with counts

From `backend/src/features/analysis/engine.rs`:

- `MatchingEngine` trait with `async fn analyze(&self, text, prompt_template, db) -> Result<MatchingResult, AnalysisError>`
- `MatchingResult` -- `matched_framework_ids`, `findings: Vec<NewFinding>`, `processing_time_ms`, `token_count`
- `NewFinding` -- `concept_id`, `framework_id`, `finding_type`, `confidence_score`, `evidence_text`, `recommendation`, `priority`

From `backend/src/features/analysis/matcher.rs`:

- `DeterministicMatcher::new(topics: Vec<Topic>)` -- constructor
- `Topic` struct with `id`, `name_en`, `concept_ids`

From `backend/src/features/analysis/parser.rs`:

- `DocumentParser::parse_text(text: &str) -> Result<ParsedDocument, ParsingError>` -- blocking text parser
- `ParsedDocument` -- has `full_text`, `sections`, `word_count`, `token_count_estimate`

The `PaginatedResponse<T>` type is defined in `backend/src/features/compliance/models.rs` (and also in `ontology/models.rs`). Use the compliance one or add an equivalent to analysis models if needed.

## Database Schema

The `analyses` table (from `backend/migrations/003_analysis_schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS analyses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    input_type TEXT NOT NULL CHECK(input_type IN ('text', 'pdf', 'docx')),
    input_text TEXT,
    original_filename TEXT,
    file_path TEXT,
    extracted_text TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    error_message TEXT,
    prompt_template TEXT,
    matched_framework_ids TEXT,  -- JSON array
    processing_time_ms INTEGER,
    token_count INTEGER,
    created_by TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS analysis_findings (
    id TEXT PRIMARY KEY,
    analysis_id TEXT NOT NULL REFERENCES analyses(id) ON DELETE CASCADE,
    concept_id TEXT NOT NULL REFERENCES concepts(id),
    framework_id TEXT NOT NULL REFERENCES frameworks(id),
    finding_type TEXT NOT NULL,
    confidence_score REAL NOT NULL,
    evidence_text TEXT,
    recommendation TEXT,
    priority INTEGER NOT NULL,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now'))
);
```

## Tests First

All tests go in `backend/tests/analysis_tests.rs` (new file). They follow the existing integration test pattern using `create_test_app()` and `.oneshot()`.

**Important:** After section 01 adds `topics` to `AppState`, the `create_test_app()` helper in `backend/tests/common/mod.rs` must be updated to load topics and include them in `AppState`. This is a prerequisite.

### test_create_analysis_text_input

POST `/api/analyses` with a valid JSON body containing `name` and `input_text` fields. Assert response status is 201. Deserialize the response body and assert it contains an `id` (non-empty string) and a `status` field (either `"completed"` or `"failed"` -- both are valid since creation succeeded).

### test_create_analysis_empty_name_rejected

POST `/api/analyses` with `name: ""` and a valid `input_text`. Assert response status is 400.

### test_create_analysis_empty_text_rejected

POST `/api/analyses` with a valid `name` but `input_text: ""`. Assert response status is 400.

### test_create_analysis_oversized_text_rejected

POST `/api/analyses` with `input_text` containing more than 500KB of data (e.g., generate a string of 512 * 1024 bytes). Assert response status is 400.

### test_create_analysis_produces_findings

POST `/api/analyses` with security-related text that contains keywords like "risk assessment", "access control", "incident response". Assert response status is 201 and that `matched_framework_ids` in the response body is a non-empty array.

### test_create_analysis_failed_sets_status

This test verifies graceful failure handling. POST with text that would trigger a processing error (or mock a failure scenario). Assert response status is 201, `status` is `"failed"`, and `error_message` is present (non-null).

### test_create_analysis_creates_audit_entry

POST a valid analysis, then query `audit_log` table directly for an entry with `action = 'analysis_created'` and `entity_type = 'analysis'`. Assert the entry exists and `entity_id` matches the returned analysis ID.

## Implementation Details

### File: `backend/src/features/analysis/routes.rs`

Implement the `create_analysis` handler function. The handler signature:

```rust
pub async fn create_analysis(
    State(state): State<AppState>,
    Json(req): Json<CreateAnalysisRequest>,
) -> AppResult<(StatusCode, Json<AnalysisSummary>)>
```

### Orchestration Flow

1. **Validate input:**
   - `req.name` must not be empty (after trimming). Return `AppError::BadRequest` if empty.
   - `req.input_text` must not be empty (after trimming). Return `AppError::BadRequest` if empty.
   - `req.input_text.len()` must be <= 500 * 1024 bytes. Return `AppError::BadRequest` if exceeded.

2. **Generate ID and insert initial record:**
   - Generate `analysis_id = Uuid::new_v4().to_string()`
   - Generate timestamp with `chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")`
   - INSERT into `analyses` with `status='pending'`, `input_type='text'`, `input_text` set to the request text

3. **Parse text via spawn_blocking:**
   - Clone the input text for the blocking closure
   - Call `tokio::task::spawn_blocking(move || DocumentParser::parse_text(&text_clone))`
   - Await the result. On parsing error, UPDATE the analysis with `status='failed'` and `error_message`, then continue to step 9 (still return 201).

4. **UPDATE analysis with extracted_text** from `ParsedDocument::full_text`

5. **Get topics from `state.topics`** (already loaded at startup by section 01)

6. **Run the matcher with 30-second timeout:**
   ```rust
   let matcher = DeterministicMatcher::new(state.topics.clone());
   let result = tokio::time::timeout(
       Duration::from_secs(30),
       matcher.analyze(&extracted_text, req.prompt_template.as_deref(), &state.db)
   ).await;
   ```
   - If timeout: UPDATE analysis with `status='failed'`, `error_message = "Analysis timed out after 30s"`
   - If `AnalysisError`: UPDATE with `status='failed'`, `error_message = err.to_string()`

7. **On matcher success:** INSERT each `NewFinding` into `analysis_findings`:
   - Generate a UUID for each finding ID
   - Set `sort_order` from the enumeration index
   - Convert `FindingType` to string for the DB column

8. **UPDATE analysis to completed:**
   - `status = 'completed'`
   - `matched_framework_ids` = JSON-serialized array of framework IDs from `MatchingResult`
   - `processing_time_ms` from `MatchingResult`
   - `token_count` from `MatchingResult`
   - `updated_at` = current timestamp

9. **Audit logging:** INSERT into `audit_log` with:
   - `action = 'analysis_created'`
   - `entity_type = 'analysis'`
   - `entity_id = analysis_id`
   - `new_value` = JSON with analysis name, status, input_type

10. **Build and return response:**
    - Fetch the analysis row fresh from DB (to get all updated fields)
    - Convert to `Analysis` via `From<AnalysisRow>`
    - Query `analysis_findings` grouped by `framework_id` to build `Vec<FrameworkFindingSummary>` with counts per finding type
    - Compute aggregate counts (`total_findings`, `gap_count`, `addressed_count`, `partially_addressed_count`)
    - Return `(StatusCode::CREATED, Json(AnalysisSummary { ... }))`

### Key Imports

The handler needs these imports in `routes.rs`:

```rust
use std::time::Duration;
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;
use crate::error::{AppError, AppResult};
use crate::AppState;
use super::models::*;
use super::parser::DocumentParser;
use super::matcher::DeterministicMatcher;
use super::engine::MatchingEngine;
```

### Audit Log Pattern

Follow the existing pattern from `backend/src/features/compliance/routes.rs`:

```rust
let audit_id = Uuid::new_v4().to_string();
let new_value = serde_json::json!({
    "id": analysis_id,
    "name": req.name,
    "input_type": "text",
    "status": final_status
});
sqlx::query(
    r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
       VALUES (?, NULL, 'analysis_created', 'analysis', ?, ?, ?)"#
)
.bind(&audit_id)
.bind(&analysis_id)
.bind(new_value.to_string())
.bind(&now)
.execute(&state.db)
.await?;
```

### Error Handling

- Parsing failures: catch `ParsingError` from `spawn_blocking`, set `status='failed'`, return 201 with the failed analysis summary
- Matcher failures: catch `AnalysisError`, set `status='failed'`, return 201
- Timeout: set `status='failed'` with message "Analysis timed out after 30s", return 201
- The handler always returns 201 once the initial INSERT succeeds -- the analysis was created even if processing failed. The status field in the response body indicates success or failure.
- Validation failures (empty name, empty text, oversized text) return 400 before any database writes

### Helper: Build AnalysisSummary from DB

Consider extracting a helper function (reused by section 05's `get_analysis` handler):

```rust
async fn build_analysis_summary(
    db: &SqlitePool,
    analysis_id: &str,
) -> AppResult<AnalysisSummary>
```

This function queries the analysis row, converts to `Analysis`, then runs an aggregation query on `analysis_findings` grouped by `framework_id` joined with the `frameworks` table to get framework names. It builds `FrameworkFindingSummary` entries and computes the aggregate counts.

### Test Helper Updates

The `backend/tests/common/mod.rs` file must be updated (as part of section 01's AppState changes) so that `create_test_app()` loads topics into `AppState`. Without this, the `create_analysis` handler will not have access to `state.topics`. Verify this is done before running integration tests.