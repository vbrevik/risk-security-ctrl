# Matching Engine Usage Guide

## Overview

The `DeterministicMatcher` analyzes documents against ontology frameworks using FTS5 full-text search and TF-IDF scoring. It implements the `MatchingEngine` trait.

## Quick Start

```rust
use crate::features::analysis::matcher::{DeterministicMatcher, Topic};
use crate::features::analysis::engine::MatchingEngine;

// 1. Load topics from ontology-data/topic-tags.json
let topics: Vec<Topic> = serde_json::from_str(&topic_json)?;

// 2. Create matcher
let matcher = DeterministicMatcher::new(topics);

// 3. Analyze document text
let result = matcher.analyze(text, prompt_template, &db).await?;

// result.matched_framework_ids - detected frameworks
// result.findings - classified findings (Gap, PartiallyAddressed, Addressed)
// result.processing_time_ms - pipeline duration
// result.token_count - estimated token count
```

## Configuration

Pass JSON as `prompt_template` to customize behavior:

```json
{
  "addressed_threshold": 0.6,
  "partial_threshold": 0.3,
  "min_confidence_threshold": 0.1,
  "max_findings_per_framework": 50,
  "include_addressed_findings": true,
  "boost_terms": {"security": 1.5, "risk": 1.5, "compliance": 1.3, "control": 1.2}
}
```

Pass `None` for defaults.

## Pipeline Stages

1. **Config parsing** - `MatcherConfig::from_json()` with fallback to defaults
2. **Keyword extraction** - `extract_keywords()` + `term_frequency()` from tokenizer
3. **Framework detection** - `detect_frameworks()` via topic matching + direct name matching
4. **FTS5 retrieval** - `retrieve_candidates()` with gap candidate loading
5. **TF-IDF scoring** - `score_candidates()` with boost terms and IDF floor
6. **Classification** - `classify_findings()` with threshold-based types and priority
7. **Reference validation** - Batch verify concept_ids exist in database
8. **Result assembly** - Timing, token count, and final `MatchingResult`

## Key Types

| Type | Location | Purpose |
|------|----------|---------|
| `DeterministicMatcher` | `matcher.rs` | Main entry point, implements `MatchingEngine` |
| `MatcherConfig` | `matcher.rs` | Configurable thresholds and boost terms |
| `Topic` | `matcher.rs` | Topic tag for framework detection |
| `ConceptCandidate` | `matcher.rs` | Raw DB retrieval result |
| `ScoredCandidate` | `matcher.rs` | Candidate + confidence_score |
| `NewFinding` | `engine.rs` | Classified finding with priority |
| `MatchingResult` | `engine.rs` | Full analysis result |

## Test Coverage

34 tests across 7 sections:
- 9 config/type tests
- 4 framework detection tests
- 5 FTS5 retrieval tests (3 integration)
- 4 TF-IDF scoring tests
- 8 classification tests
- 4 integration tests (full pipeline)
