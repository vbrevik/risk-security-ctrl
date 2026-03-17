use async_trait::async_trait;
use sqlx::SqlitePool;

use super::models::FindingType;

// ============================================================================
// Error Type
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("No relevant frameworks detected in document")]
    NoFrameworksDetected,

    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    #[error("Invalid prompt template: {0}")]
    InvalidPromptTemplate(String),
}

// ============================================================================
// Matching Result Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct MatchingResult {
    pub matched_framework_ids: Vec<String>,
    pub findings: Vec<NewFinding>,
    pub processing_time_ms: i64,
    pub token_count: i64,
}

#[derive(Debug, Clone)]
pub struct NewFinding {
    pub concept_id: String,
    pub framework_id: String,
    pub finding_type: FindingType,
    pub confidence_score: f64,
    pub evidence_text: Option<String>,
    pub recommendation: Option<String>,
    pub priority: i32,
}

// ============================================================================
// MatchingEngine Trait
// ============================================================================

/// Trait for pluggable analysis implementations.
///
/// MVP: `DeterministicMatcher` (split 03) uses FTS5 + keyword scoring.
/// Phase 2: `LlmMatcher` uses Claude/Ollama for intelligent analysis.
#[async_trait]
pub trait MatchingEngine: Send + Sync {
    /// Analyze extracted text against ontology frameworks.
    ///
    /// - `text`: the document text to analyze
    /// - `prompt_template`: optional JSON config overriding default matching behavior
    /// - `db`: database pool for querying ontology concepts during analysis
    async fn analyze(
        &self,
        text: &str,
        prompt_template: Option<&str>,
        db: &SqlitePool,
    ) -> Result<MatchingResult, AnalysisError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time check that MatchingEngine is dyn-compatible
    fn _assert_dyn_compatible(_: &dyn MatchingEngine) {}

    #[test]
    fn test_analysis_error_no_frameworks_display() {
        let err = AnalysisError::NoFrameworksDetected;
        let msg = format!("{}", err);
        assert!(!msg.is_empty());
        assert!(msg.contains("No relevant frameworks"));
    }

    #[test]
    fn test_analysis_error_from_sqlx() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let err: AnalysisError = sqlx_err.into();
        assert!(matches!(err, AnalysisError::DatabaseError(_)));
    }

    #[test]
    fn test_new_finding_construction() {
        let finding = NewFinding {
            concept_id: "concept-1".to_string(),
            framework_id: "nist-csf".to_string(),
            finding_type: FindingType::Gap,
            confidence_score: 0.85,
            evidence_text: Some("evidence here".to_string()),
            recommendation: None,
            priority: 2,
        };
        assert_eq!(finding.concept_id, "concept-1");
        assert_eq!(finding.priority, 2);
    }

    #[test]
    fn test_matching_result_empty() {
        let result = MatchingResult {
            matched_framework_ids: vec![],
            findings: vec![],
            processing_time_ms: 0,
            token_count: 0,
        };
        assert!(result.findings.is_empty());
        assert!(result.matched_framework_ids.is_empty());
    }
}
