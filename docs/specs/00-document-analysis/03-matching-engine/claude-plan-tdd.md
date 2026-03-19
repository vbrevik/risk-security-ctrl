# TDD Plan: 03-matching-engine

## Section 1: Prompt Template Configuration
- Test: MatcherConfig::default() returns expected threshold values (0.6, 0.3, 0.1)
- Test: MatcherConfig::from_json(None) returns default
- Test: MatcherConfig::from_json(Some(valid_json)) parses correctly
- Test: MatcherConfig::from_json(Some(malformed_json)) returns default with warning
- Test: boost_terms contains expected defaults (security=1.5, risk=1.5)

## Section 2: Framework Detection
- Test: detect_frameworks with "risk assessment" keywords matches ISO 31000
- Test: detect_frameworks with "NIST" keyword matches NIST frameworks by name
- Test: detect_frameworks with unrelated keywords returns empty vec
- Test: detect_frameworks orders by match strength (highest first)

## Section 3: FTS5 Candidate Retrieval
- Test: retrieve_candidates returns concepts matching keywords (requires DB)
- Test: retrieve_candidates includes gap candidates (unmatched concepts)
- Test: retrieve_candidates deduplicates results
- Test: FTS5 query sanitizes special characters in keywords
Note: These are integration tests requiring a populated SQLite database.

## Section 4: TF-IDF Scoring
- Test: score_candidates with high overlap → score near 1.0
- Test: score_candidates with no overlap → score 0.0
- Test: score_candidates applies boost_terms (boosted keyword scores higher)
- Test: score_candidates normalizes to 0.0-1.0 range

## Section 5: Gap Classification and Findings
- Test: classify with score 0.8 → Addressed
- Test: classify with score 0.4 → PartiallyAddressed
- Test: classify with score 0.1 → Gap
- Test: classify with score 0.0 → Gap (zero-match gap candidate)
- Test: priority P1 for root concept gap, P2 for child gap
- Test: recommendation text contains concept name and source_reference
- Test: max_findings_per_framework caps output
- Test: include_addressed_findings=false excludes Addressed findings

## Section 6: DeterministicMatcher Implementation
- Test: analyze() with security-related text returns findings (integration)
- Test: analyze() with irrelevant text returns NoFrameworksDetected error
- Test: analyze() validates references (drops fake concept_ids)
- Test: analyze() returns MatchingResult with timing and token count

## Section 7: Module Wiring
No tests. Validated by `cargo check` and `cargo test`.
