I now have all the context needed. Let me produce the section content.

# Section 6: DeterministicMatcher Implementation

## Overview

This section implements the `DeterministicMatcher` struct, which is the orchestration layer that ties together all preceding stages (framework detection, FTS5 retrieval, scoring, and classification) into a single `MatchingEngine` trait implementation. The struct lives in `backend/src/features/analysis/matcher.rs`.

`DeterministicMatcher` is the only struct in this file that implements the `MatchingEngine` trait defined in `backend/src/features/analysis/engine.rs`. Its `analyze` method is the entry point called by route handlers.

## Dependencies

This section depends on the completion of:

- **Section 01** (config-types): `MatcherConfig`, `Topic`, `ConceptCandidate`, `ScoredCandidate` structs
- **Section 02** (framework-detection): `detect_frameworks()` function
- **Section 03** (fts5-retrieval): `retrieve_candidates()` function
- **Section 04** (scoring): `score_candidates()` function
- **Section 05** (classification): `classify_findings()` function

All of these are defined in the same file: `backend/src/features/analysis/matcher.rs`.

## Existing Trait and Types

The `MatchingEngine` trait is already defined in `backend/src/features/analysis/engine.rs`:

```rust
#[async_trait]
pub trait MatchingEngine: Send + Sync {
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError>;
}
```

`MatchingResult` and `NewFinding` are also defined in that file:

```rust
pub struct MatchingResult {
    pub matched_framework_ids: Vec<String>,
    pub findings: Vec<NewFinding>,
    pub processing_time_ms: i64,
    pub token_count: i64,
}

pub struct NewFinding {
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
}
```

The tokenizer utilities are in `backend/src/features/analysis/tokenizer.rs` and provide `extract_keywords()`, `term_frequency()`, and `sentence_split()`.

## File to Create/Modify

**File:** `backend/src/features/analysis/matcher.rs` (append to file created by sections 01-05)

## Tests First

Place these tests in the `#[cfg(test)] mod tests` block within `matcher.rs`. These are integration tests that require a populated SQLite database.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::analysis::engine::MatchingEngine;

    // Helper: create an in-memory SQLite database with the schema and seed data
    async fn setup_test_db() -> SqlitePool {
        // Create in-memory DB, run migrations, insert at least:
        // - 2 frameworks (e.g., "nist-csf", "iso-31000")
        // - Several concepts per framework with varying depth (root, child, grandchild)
        // - Populate concepts_fts via triggers (automatic on INSERT)
        todo!("Set up test DB with schema and seed concepts")
    }

    fn sample_topics() -> Vec<Topic> {
        // Return topics that map keywords like "risk", "security" to framework concept IDs
        todo!("Create test topics")
    }

    #[tokio::test]
    async fn test_analyze_security_text_returns_findings() {
        /// Given text containing security-related keywords that match seeded concepts,
        /// analyze() should return a non-empty MatchingResult with findings.
        let db = setup_test_db().await;
        let matcher = DeterministicMatcher::new(sample_topics());
        let text = "Our organization implements risk assessment procedures and security controls for identifying threats and vulnerabilities.";
        let result = matcher.analyze(text, None, &db).await.unwrap();
        assert!(!result.findings.is_empty());
        assert!(!result.matched_framework_ids.is_empty());
    }

    #[tokio::test]
    async fn test_analyze_irrelevant_text_returns_no_frameworks_error() {
        /// Given text with no keywords matching any framework or topic,
        /// analyze() should return AnalysisError::NoFrameworksDetected.
        let db = setup_test_db().await;
        let matcher = DeterministicMatcher::new(sample_topics());
        let text = "The weather today is sunny with a chance of rain in the afternoon.";
        let result = matcher.analyze(text, None, &db).await;
        assert!(matches!(result, Err(AnalysisError::NoFrameworksDetected)));
    }

    #[tokio::test]
    async fn test_analyze_validates_references() {
        /// Findings referencing non-existent concept IDs should be dropped.
        /// This is implicitly tested by ensuring all returned findings have
        /// concept_ids that exist in the seeded database.
        let db = setup_test_db().await;
        let matcher = DeterministicMatcher::new(sample_topics());
        let text = "Risk management and security controls are critical for compliance.";
        let result = matcher.analyze(text, None, &db).await.unwrap();
        for finding in &result.findings {
            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concepts WHERE id = ?")
                .bind(&finding.concept_id)
                .fetch_one(&db)
                .await
                .unwrap();
            assert!(count.0 > 0, "Finding references non-existent concept: {}", finding.concept_id);
        }
    }

    #[tokio::test]
    async fn test_analyze_returns_timing_and_token_count() {
        /// The returned MatchingResult should have positive processing_time_ms
        /// and a token_count derived from the input text length.
        let db = setup_test_db().await;
        let matcher = DeterministicMatcher::new(sample_topics());
        let text = "Security risk assessment identifies threats and vulnerabilities in systems.";
        let result = matcher.analyze(text, None, &db).await.unwrap();
        assert!(result.processing_time_ms >= 0);
        assert!(result.token_count > 0);
    }
}
```

## Implementation Details

### DeterministicMatcher Struct

The struct holds topics as a field because the `MatchingEngine` trait signature does not accept topics as a parameter. Route handlers load topics from `ontology-data/topic-tags.json` and pass them at construction time.

```rust
pub struct DeterministicMatcher {
    topics: Vec<Topic>,
}

impl DeterministicMatcher {
    pub fn new(topics: Vec<Topic>) -> Self {
        Self { topics }
    }
}
```

### analyze() Method Orchestration

The `analyze` method implements `MatchingEngine` and orchestrates all pipeline stages in sequence:

```rust
#[async_trait]
impl MatchingEngine for DeterministicMatcher {
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError> {
        let start = std::time::Instant::now();

        // Stage 1: Parse config
        // Stage 2: Extract keywords and term frequencies from text
        // Stage 3: Load frameworks from DB and detect relevant ones
        // Stage 4: Retrieve candidates via FTS5
        // Stage 5: Score candidates
        // Stage 6: Classify into findings
        // Stage 7: Validate references
        // Stage 8: Build and return MatchingResult

        todo!("Implement pipeline orchestration")
    }
}
```

#### Stage 1: Parse Config

Call `MatcherConfig::from_json(prompt_template)` to get configuration. This returns defaults if the template is None or invalid JSON.

#### Stage 2: Extract Keywords and Term Frequencies

Use the tokenizer utilities from `super::tokenizer`:

- `extract_keywords(text)` produces the document keyword list
- `term_frequency(text)` produces the term frequency map

These are passed to subsequent stages.

#### Stage 3: Framework Detection

Query the database for all frameworks:

```sql
SELECT id, name_en FROM frameworks
```

Then call `detect_frameworks(&doc_keywords, &self.topics, frameworks, &config)`.

If the result is empty, return `Err(AnalysisError::NoFrameworksDetected)`.

#### Stage 4: FTS5 Candidate Retrieval

Call `retrieve_candidates(&doc_keywords, &framework_ids, db)` to get `Vec<ConceptCandidate>`.

Note: The `frameworks` table column is `name_en` (not `name`). The SQL query should use the correct column name.

#### Stage 5: Score Candidates

Call `score_candidates(&candidates, &doc_keywords, &term_freq, &config)` to produce `Vec<ScoredCandidate>`.

#### Stage 6: Classify Findings

Call `classify_findings(scored_candidates, &config)` to produce `Vec<NewFinding>`.

The `classify_findings` function from Section 05 needs the original document text for evidence extraction. The function signature should either accept the text directly or evidence extraction should happen here in the orchestrator. The plan specifies evidence extraction happens inside `classify_findings`, so ensure the text is passed through (either as an additional parameter to `classify_findings` or by a separate evidence extraction step here).

#### Stage 7: Reference Validation

For each finding, verify the concept_id exists in the database:

```sql
SELECT COUNT(*) FROM concepts WHERE id = ?
```

Drop any finding where the count is 0. Log a warning with `tracing::warn!` for dropped findings.

This can be batched for efficiency: collect all concept_ids, run a single `SELECT id FROM concepts WHERE id IN (...)` query, then filter findings to only those whose concept_id appears in the result set.

#### Stage 8: Build Result

Compute timing and token count:

- `processing_time_ms`: `start.elapsed().as_millis() as i64`
- `token_count`: `(text.split_whitespace().count() as f64 * 1.33) as i64` (same formula as the parser module)

Construct and return:

```rust
Ok(MatchingResult {
    matched_framework_ids: framework_ids,
    findings: validated_findings,
    processing_time_ms,
    token_count,
})
```

### Error Handling

- Database query failures propagate as `AnalysisError::DatabaseError` via the `From<sqlx::Error>` impl.
- Empty framework detection returns `AnalysisError::NoFrameworksDetected`.
- If FTS5 retrieval returns zero candidates across all frameworks, return an empty findings list (not an error) since this means the document matched frameworks by name but no individual concepts had keyword overlap.

### Required Imports

The implementation needs these imports at the top of `matcher.rs` (some may already be present from earlier sections):

```rust
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::time::Instant;
use tracing::warn;

use super::engine::{AnalysisError, MatchingEngine, MatchingResult, NewFinding};
use super::models::FindingType;
use super::tokenizer::{extract_keywords, term_frequency, sentence_split};
```