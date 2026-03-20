# Implementation Plan: Concept Detail API & Matcher Enhancement

## Background

This project extends a risk management framework explorer that helps governmental IT security teams understand and track compliance across standards (ISO 31000, NIST CSF, NIST AI RMF). The backend is Rust + Axum + SQLx (SQLite), with OpenAPI documentation via utoipa.

A prior implementation phase (02-schema-import) added four guidance tables to the database: `concept_guidance`, `concept_actions`, `concept_transparency_questions`, and `concept_references`. It also added an FTS5 virtual table (`concept_guidance_fts`) that indexes concept names, definitions, and guidance about text. The tables are populated for NIST AI RMF action-level concepts with structured playbook data (about sections, suggested actions, transparency questions, academic references).

This plan covers two areas: (1) extending the existing concept detail API to return guidance data in the response, and (2) enhancing the analysis matcher to leverage guidance text for better scoring and more actionable recommendations.

---

## Section 1: Guidance Response Types

### What

Add four new response structs to `backend/src/features/ontology/models.rs` for serializing guidance data in the API response.

### Why

The existing `ConceptWithRelationships` struct returns concept metadata and relationships but no guidance data. Frontend (split 04) needs a well-defined response shape to render guidance panels. The types must derive `Serialize` and `ToSchema` to integrate with the existing OpenAPI/utoipa documentation.

### Design

**`ConceptGuidanceResponse`** — top-level guidance container:
- `source_pdf: String` — PDF filename for provenance
- `source_page: i64` — page number in source PDF
- `about_en: Option<String>` — English guidance summary
- `about_nb: Option<String>` — Norwegian translation
- `suggested_actions: Vec<ActionResponse>` — ordered actions
- `transparency_questions: Vec<QuestionResponse>` — ordered questions
- `references: Vec<ReferenceResponse>` — academic + transparency resources

**`ActionResponse`** — one suggested action:
- `sort_order: i64`
- `text_en: String`
- `text_nb: Option<String>`

**`QuestionResponse`** — one transparency question:
- `sort_order: i64`
- `text_en: String`
- `text_nb: Option<String>`

**`ReferenceResponse`** — one reference entry:
- `reference_type: String` — "academic" or "transparency_resource" (use `#[serde(rename = "type")]`)
- `title: String`
- `authors: Option<String>`
- `year: Option<i64>`
- `venue: Option<String>`
- `url: Option<String>`

**Extending ConceptWithRelationships:**

Add an optional guidance field to the existing `ConceptWithRelationships` struct:
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub guidance: Option<ConceptGuidanceResponse>,
```

This is backward-compatible — clients that don't expect the field will simply ignore it. When the concept has no guidance data (most frameworks), the field is omitted from JSON entirely.

### Testing Strategy

- Deserialization round-trip tests: construct each response type, serialize to JSON, verify structure
- Verify `skip_serializing_if` omits the field when `None`
- Verify `serde(rename = "type")` produces `"type"` in JSON output

---

## Section 2: Concept Detail Handler Extension

### What

Modify the `get_concept_relationships` handler in `backend/src/features/ontology/routes.rs` to query the four guidance tables and assemble a nested `ConceptGuidanceResponse`.

### Why

This is the primary API change that enables the frontend to display guidance data alongside concept details. The handler already fetches concept metadata and relationships; guidance is an additional data dimension from the same concept.

### Design

After the existing relationship queries, add four additional queries:

1. `SELECT source_pdf, source_page, about_en, about_nb FROM concept_guidance WHERE concept_id = ?` (use `fetch_optional`)
2. `SELECT action_text_en, action_text_nb, sort_order FROM concept_actions WHERE concept_id = ? ORDER BY sort_order`
3. `SELECT question_text_en, question_text_nb, sort_order FROM concept_transparency_questions WHERE concept_id = ? ORDER BY sort_order`
4. `SELECT reference_type, title, authors, year, venue, url, sort_order FROM concept_references WHERE concept_id = ? ORDER BY sort_order`

If query 1 returns no row, the guidance field is `None`. If it returns a row, assemble all four queries into `ConceptGuidanceResponse`.

**Optimization:** Since queries 1-4 are independent of each other, run them concurrently using `tokio::join!` (or `tokio::try_join!` for error propagation). This reduces the four sequential round-trips to a single concurrent batch.

All queries use parameterized binds with `sqlx::query()` and `.bind()` (STIG V-222607). The concept_id comes from the URL path parameter, which is already validated by Axum's type extraction.

### OpenAPI Updates

- Update the `#[utoipa::path]` annotation on `get_concept_relationships` to document the enriched response
- Register the four new schema types in the OpenAPI component list in `main.rs`

### Error Handling

Follow the existing pattern: database errors map to 500 via `.map_err()`. The guidance queries should not change the error behavior — if guidance tables are empty or the concept has no guidance, the response simply omits the field. No new error paths (STIG V-222585, V-222610).

### Testing Strategy

- API test: `GET /api/ontology/concepts/nist-ai-gv-1-1/relationships` should include a `guidance` object with populated fields
- API test: `GET /api/ontology/concepts/iso31000-1/relationships` (non-NIST concept) should NOT include a `guidance` field in the JSON
- API test: concept with guidance row but zero actions/questions/references returns `guidance` object with empty arrays
- Verify actions are ordered by `sort_order`
- Verify references have correct `type` values

---

## Section 3: ConceptCandidate Enrichment

### What

Extend the `ConceptCandidate` struct in `backend/src/features/analysis/matcher.rs` with optional guidance fields, and modify candidate retrieval to populate them.

### Why

The analysis matcher currently scores concepts using only `name_en` and `definition_en`. Adding `about_en` and concatenated action text provides significantly richer vocabulary for TF-IDF matching, especially for NIST AI RMF concepts where the definition alone may not contain domain-specific action keywords.

### Design

**Struct Extension:**

Add two optional fields to `ConceptCandidate`:
- `about_en: Option<String>` — from `concept_guidance.about_en`
- `actions_text: Option<String>` — all action texts concatenated with newlines

**Retrieval Changes:**

In the existing retrieval functions, modify the SQL queries to LEFT JOIN guidance data:

For the FTS5 candidate query and the exact-match fallback query, add:
```sql
LEFT JOIN concept_guidance cg ON cg.concept_id = c.id
```

For actions text, use a nested subquery for guaranteed sort order in SQLite:
```sql
(SELECT GROUP_CONCAT(action_text_en, char(10)) FROM (SELECT action_text_en FROM concept_actions WHERE concept_id = c.id ORDER BY sort_order)) as actions_text
```

Concepts without guidance data will have `NULL` for both fields, which maps to `None` in the struct.

**Important:** Only populate guidance fields for FTS and exact-match candidates (stages 1 and 2). Gap candidates (stage 3) should NOT LEFT JOIN guidance tables — they always score 0.0 and the guidance fields would never be used for scoring or recommendations. This avoids unnecessary joins across hundreds of gap candidate rows.

**Union FTS Tables:**

In `retrieve_candidates()`, add a second FTS query against `concept_guidance_fts` with custom BM25 weights:
```sql
SELECT c.id, ... FROM concept_guidance_fts gf
JOIN concept_guidance cg ON cg.rowid = gf.rowid
JOIN concepts c ON c.id = cg.concept_id
WHERE concept_guidance_fts MATCH ?
AND c.framework_id IN (SELECT value FROM json_each(?))
ORDER BY bm25(concept_guidance_fts, 10.0, 3.0, 5.0)
LIMIT ?
```

Weights: name_en=10.0, definition_en=3.0, about_en=5.0. These prioritize concept names for exact matches while boosting guidance about text over definitions.

Results from both FTS tables are merged by concept_id, keeping the first occurrence (deduplication via the existing `seen_ids` HashSet pattern). Note: BM25 scores from different FTS tables (different column counts and weights) are not directly comparable, so "keep best score" dedup is impractical. First-occurrence dedup is sufficient because the purpose of the union is broader recall — the final ranking is handled by TF-IDF re-scoring in Section 4, which uses a uniform scoring function across all candidates.

FTS5 MATCH input must be sanitized using the existing `sanitize_keyword()` function (STIG V-222602).

### Testing Strategy

- Unit test: candidate from concept WITH guidance has `about_en` and `actions_text` populated
- Unit test: candidate from concept WITHOUT guidance has `None` for both fields
- Unit test: guidance FTS retrieval returns candidates not found by concepts_fts alone
- Update existing test fixtures that construct `ConceptCandidate` literals to include the new optional fields

---

## Section 4: TF-IDF Scoring Enhancement

### What

Modify `score_candidates()` in `matcher.rs` to include guidance text in the keyword extraction and TF-IDF computation.

### Why

Currently the scorer extracts keywords from only `name_en + definition_en`. By also including `about_en` and `actions_text`, concepts with guidance data get a richer keyword profile that better reflects their real-world scope. A concept about "measurement approaches" may have actions mentioning "testing procedures", "performance metrics", and "bias detection" — all valuable matching signals.

### Design

In the scoring loop where keywords are extracted from each candidate, change from:
- `name_en + " " + definition_en`

To:
- `name_en + " " + definition_en + " " + about_en.unwrap_or("") + " " + actions_text.unwrap_or("")`

This is purely additive — concepts without guidance continue to score using only name + definition. The normalization denominator naturally accounts for the richer text since it's computed from the same expanded corpus.

No changes to the IDF computation, boost terms, or score normalization formula.

### Testing Strategy

- Unit test: concept with guidance scores higher than same concept without guidance when query keywords match action text
- Unit test: scoring still works correctly for concepts without guidance (no regression)
- Integration test: analysis with real NIST AI RMF concepts produces findings that reference guidance-enriched scores

---

## Section 5: Actionable Recommendations

### What

When generating gap findings/recommendations, include the concept's suggested actions as specific next steps in the recommendation text.

### Why

Currently, gap findings say things like "You have a gap in MEASURE 1.1." With guidance data, we can say "Consider: Establish approaches for detecting, tracking and measuring known risks (MEASURE 1.1, Action 1)" — giving users concrete, actionable steps directly from the NIST AI RMF Playbook.

### Design

In the recommendation generation logic (wherever findings are assembled for the response), check if the matched `ScoredCandidate`'s `ConceptCandidate` has `actions_text` populated (from Section 3's retrieval). If yes:

1. Split `actions_text` by newlines to recover individual actions
2. Format each action as: `"- {action_text} ({concept_code}, Action {n})"`
3. Append the formatted action list to the recommendation text under a "Suggested Actions:" heading

This uses the pre-fetched `actions_text` from `ConceptCandidate` — no additional DB query needed. The `classify_findings` function remains synchronous.

Include **all actions** for the concept (per interview decision). The frontend will handle display/truncation.

For concepts without guidance data (`actions_text` is `None`), the recommendation text is unchanged (no actions section).

### Testing Strategy

- Unit test: finding for concept with actions includes all action texts in recommendation
- Unit test: finding for concept without actions has no actions section
- Integration test: analysis output includes actionable recommendations for NIST AI RMF concepts

---

## Section 6: Integration Tests and OpenAPI Verification

### What

End-to-end tests validating the full flow: guidance-enriched API responses, matcher with guidance scoring, and OpenAPI schema correctness.

### Why

Individual section tests verify components in isolation. Integration tests verify the full pipeline: data imported → API returns enriched response → matcher uses guidance for scoring → recommendations include actions. Also verifies that the OpenAPI schema is updated and valid.

### Design

**API Integration Tests:**
- Verify `GET /api/ontology/concepts/nist-ai-gv-1-1/relationships` returns guidance object with actions, questions, references
- Verify response matches the `ConceptGuidanceResponse` schema
- Verify list endpoints still don't include guidance

**Matcher Integration Tests:**
- Run a full analysis against a test document targeting NIST AI RMF concepts
- Verify scored candidates include guidance-enriched scores
- Verify recommendations contain suggested action text

**OpenAPI Verification:**
- Fetch `/swagger-ui` or the OpenAPI JSON and verify new schema types are registered
- Verify the concept detail endpoint documents the guidance response shape

**No-Regression Tests:**
- Existing API tests continue to pass
- Existing matcher tests continue to pass
- Analysis output for non-guidance frameworks is unchanged

### Testing Strategy

All tests follow the existing patterns in `backend/tests/api_tests.rs` and `backend/tests/guidance_tests.rs`:
- Use `create_test_app()` for HTTP endpoint testing
- Use `create_test_pool()` for direct database testing
- Clean up test data after integration tests to avoid pollution

---

## Compliance Notes

All sections must adhere to these STIG controls:
- **V-222607 (CAT I)**: Parameterized SQL only — all new queries use `sqlx::query().bind()`
- **V-222602 (CAT I)**: FTS5 MATCH input sanitized via existing `sanitize_keyword()` function
- **V-222606 (CAT I)**: All input validated — concept IDs from path params, search terms sanitized
- **V-222609 (CAT I)**: Malformed input handled gracefully — no panics, no unwraps on user input
- **V-222610 (CAT II)**: Error responses generic — database errors mapped to 500 without details
- **V-222585 (CAT I)**: Fail secure — missing data returns None/empty, not errors
