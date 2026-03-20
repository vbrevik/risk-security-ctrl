# TDD Plan: Concept Detail API & Matcher Enhancement

Testing uses Rust's built-in test framework with `#[tokio::test]` for async tests. Existing patterns: `create_test_app()` for HTTP tests, `create_test_pool()` for direct DB, `setup_db()` for in-memory SQLite. Test runner: `cargo test`.

---

## Section 1: Guidance Response Types

### Tests to Write First

```rust
// Test: ConceptGuidanceResponse serializes to expected JSON structure
// Test: ActionResponse serializes sort_order and text_en correctly
// Test: QuestionResponse serializes sort_order and text_en correctly
// Test: ReferenceResponse uses "type" (not "reference_type") in JSON via serde(rename)
// Test: ConceptWithRelationships with guidance=None omits guidance field from JSON
// Test: ConceptWithRelationships with guidance=Some includes the nested object
```

---

## Section 2: Concept Detail Handler Extension

### Tests to Write First

```rust
// Test: GET /api/ontology/concepts/nist-ai-gv-1-1/relationships returns guidance object
// Test: guidance.source_pdf, source_page, about_en match imported data
// Test: guidance.suggested_actions are ordered by sort_order
// Test: guidance.references contain both "academic" and "transparency_resource" types
// Test: GET /api/ontology/concepts/iso31000-1/relationships has no guidance field in JSON
// Test: concept with guidance row but zero actions returns guidance with empty suggested_actions array
// Test: existing relationship fields still present and correct in enriched response
```

---

## Section 3: ConceptCandidate Enrichment

### Tests to Write First

```rust
// Test: candidate retrieved for concept WITH guidance has about_en populated
// Test: candidate retrieved for concept WITH guidance has actions_text populated (newline-separated)
// Test: candidate retrieved for concept WITHOUT guidance has None for about_en and actions_text
// Test: guidance FTS query with custom weights returns results matching about_en content
// Test: union of both FTS tables returns broader candidate set than concepts_fts alone
// Test: gap candidates do NOT have guidance fields populated (None)
// Test: deduplication across FTS tables keeps first occurrence
// Test: existing ConceptCandidate test fixtures updated with new optional fields
```

---

## Section 4: TF-IDF Scoring Enhancement

### Tests to Write First

```rust
// Test: concept with guidance text scores higher when query keywords match actions
// Test: concept without guidance scores same as before (no regression)
// Test: scoring uses all four text fields: name_en + definition_en + about_en + actions_text
// Test: normalization still produces scores in [0.0, 1.0] range with enriched text
```

---

## Section 5: Actionable Recommendations

### Tests to Write First

```rust
// Test: finding for concept with actions_text includes "Suggested Actions:" section
// Test: all actions from actions_text appear in recommendation (split by newline)
// Test: actions formatted with concept code and action number
// Test: finding for concept without actions_text has no actions section in recommendation
// Test: classify_findings remains synchronous (no async, no DB access)
```

---

## Section 6: Integration Tests and OpenAPI Verification

### Tests to Write First

```rust
// Test: full analysis pipeline with NIST AI RMF concepts produces guidance-enriched scores
// Test: analysis recommendations include suggested action text for matched concepts
// Test: non-guidance frameworks (ISO 31000, NIST CSF) produce unchanged analysis output
// Test: OpenAPI JSON includes ConceptGuidanceResponse schema definition
// Test: existing API tests still pass (no regression)
// Test: existing matcher tests still pass (no regression)
```
