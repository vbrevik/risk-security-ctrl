# Research Findings: 03-api-matcher

## Codebase Analysis

### Concept Model & Response Types (`models.rs`)

- **`Concept`**: Base struct with `#[derive(FromRow, Serialize, ToSchema)]`. Fields: id, framework_id, parent_id, concept_type, code, name_en/nb, definition_en/nb, source_reference, sort_order, timestamps.
- **`ConceptWithRelationships`**: Uses `#[serde(flatten)]` on a `Concept` field + `related_concepts: Vec<RelatedConcept>`.
- All response types derive `ToSchema` for utoipa/Swagger.

### Concept Detail Handler (`routes.rs`)

The current `get_concept` handler at line 205 returns `Result<Json<Concept>, StatusCode>` — a plain `Concept` without relationships. There's a separate `get_concept_relationships` handler that returns `ConceptWithRelationships` with outgoing + incoming relationships via two JOIN queries.

**Error pattern**: `Result<Json<T>, StatusCode>` with `.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)` and `.ok_or(StatusCode::NOT_FOUND)`. There's also an `AppError` enum with structured error responses, but the concept handlers use the simpler StatusCode pattern.

### Analysis Matcher (`matcher.rs`)

**ConceptCandidate** struct: id, framework_id, parent_id, name_en, definition_en, code, source_reference, concept_type. No guidance fields.

**Three-stage retrieval** in `retrieve_candidates()`:
1. FTS5 MATCH query on `concepts_fts` (the concepts FTS table, NOT the guidance FTS table)
2. Exact match on name_en and code via LIKE
3. Gap candidates: all unmatched concepts from detected frameworks

**TF-IDF scoring** in `score_candidates()`:
- Extracts keywords from `name_en + definition_en`
- Computes overlap with document keywords
- Formula: `tf * idf * boost` per keyword, normalized to [0.0, 1.0]
- IDF floor of 0.1, boost_terms configurable (security: 1.5, risk: 1.5, etc.)

**Key insight**: The scorer only uses `name_en` and `definition_en`. Adding `about_en` and action text would significantly enrich the vocabulary for matching.

### Guidance Data Tables (Migration 004)

- `concept_guidance`: id, concept_id (UNIQUE), source_pdf, source_page, about_en, about_nb
- `concept_actions`: id, concept_id, action_text_en/nb, sort_order — UNIQUE(concept_id, sort_order)
- `concept_transparency_questions`: id, concept_id, question_text_en/nb, sort_order
- `concept_references`: id, concept_id, reference_type CHECK('academic','transparency_resource'), title, authors, year, venue, url, sort_order
- `concept_guidance_fts`: FTS5 virtual table indexing name_en, definition_en, about_en via content view

### Test Patterns

- `create_test_pool()` → pool with migrations + ontology data
- `create_test_app()` → full Axum Router for HTTP testing via `oneshot()`
- Pattern: build Request, call oneshot, assert status + parse JSON body
- Guidance tests use both in-memory `setup_db()` and integration `create_test_pool()`

### OpenAPI/utoipa Patterns

- `#[derive(ToSchema)]` on all response structs
- `#[utoipa::path(get, path="...", tag="...", params(...), responses(...))]` on handlers
- Components registered in `main.rs` OpenAPI builder
- `#[derive(IntoParams)]` for query parameters

---

## Web Research: Axum Nested JSON Responses

### Optional Nested Objects

Best practice: use `#[serde(skip_serializing_if = "Option::is_none")]` to omit fields when `None` rather than serializing as `null`. This is backward-compatible — existing clients ignore unknown fields.

For many optional fields, `serde_with::skip_serializing_none` applies container-level.

### Extending Response Types

**Pattern A (recommended for this case)**: Add optional fields to existing struct. Since `guidance` is `Option<T>`, clients without guidance data simply don't see the field.

**Pattern B**: `#[serde(flatten)]` for composition — already used in `ConceptWithRelationships`. Caveat: incompatible with `deny_unknown_fields`.

### Sources
- [serde field attrs](https://serde.rs/field-attrs.html)
- [serde_with skip_serializing_none](https://docs.rs/serde_with/latest/serde_with/attr.skip_serializing_none.html)

---

## Web Research: SQLite FTS5 + TF-IDF Scoring

### BM25 in FTS5

FTS5's built-in `bm25()` is already a BM25 (evolved TF-IDF). Key details:
- Returns **negative** scores — lower (more negative) = better match
- `ORDER BY rank` is more efficient than `ORDER BY bm25()` for LIMIT queries
- Column weights are positional: `bm25(table, w1, w2, w3)`

### Multi-Field Scoring

```sql
-- Weight: definition=10.0, about=5.0, actions=1.0
SELECT rowid, bm25(guidance_fts, 10.0, 5.0, 1.0) AS score
FROM guidance_fts WHERE guidance_fts MATCH ? ORDER BY score;
```

### Two-Phase Search Pattern (recommended)

1. **Phase 1**: FTS5 retrieves candidates with BM25 scores (fast, uses inverted index)
2. **Phase 2**: Application-level re-ranking with additional text fields not in FTS

This avoids needing all text in the FTS index while still getting multi-signal scoring.

### Persistent Rank Weights

```sql
INSERT INTO guidance_fts(guidance_fts, rank) VALUES('rank', 'bm25(10.0, 5.0, 1.0)');
-- Then: SELECT * FROM guidance_fts WHERE ... ORDER BY rank;
```

### Sources
- [SQLite FTS5 docs](https://www.sqlite.org/fts5.html)
- [Simon Willison: Search relevance with SQLite](https://simonwillison.net/2019/Jan/7/exploring-search-relevance-algorithms-sqlite/)
