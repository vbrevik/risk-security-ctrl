# TDD Plan: Database Schema & Import Pipeline

**Testing framework:** Rust `#[tokio::test]`, following existing patterns in `backend/tests/`.

---

## 2. Database Migration

```rust
// Test: migration 004 creates concept_guidance table
// Test: migration 004 creates concept_actions table with UNIQUE(concept_id, sort_order)
// Test: migration 004 creates concept_transparency_questions table
// Test: migration 004 creates concept_references table with CHECK on reference_type
// Test: migration 004 creates concept_guidance_search_v view
// Test: migration 004 creates concept_guidance_fts virtual table
// Test: indexes exist on child tables (query sqlite_master)
// Test: ON DELETE CASCADE removes guidance when concept is deleted
```

## 3. Import Types

```rust
// Test: GuidanceFile deserializes from valid JSON string
// Test: GuidanceEntry with all optional fields as null deserializes
// Test: ResourceEntry deserializes with "type" field mapped to resource_type
// Test: ReferenceEntry with partial fields (only title) deserializes
// Test: unknown fields in JSON are silently ignored (no deny_unknown_fields)
```

## 4. Import Function

```rust
// Test: import_guidance_file with 2-3 valid entries populates all four tables
// Test: concept_guidance row has correct source_pdf, source_page, about_en
// Test: concept_actions rows have correct action_text_en and sort_order (1-based)
// Test: concept_transparency_questions rows ordered correctly
// Test: concept_references rows split resources→"transparency_resource" and references→"academic"
// Test: invalid concept_id is skipped with warning, other entries still imported
// Test: re-import same file produces no duplicate rows (upsert + delete-reinsert)
// Test: child rows are replaced on re-import (not appended)
// Test: transaction rolls back on failure for a single concept entry
```

## 5. Wiring into import_all_ontologies()

```rust
// Test: import_all_ontologies imports *-guidance.json files after frameworks
// Test: guidance file with unknown framework_id still imports (framework_id is metadata, not FK)
// Test: no *-guidance.json files in directory does not cause error
```

## 7. FTS5 Tests

```rust
// Test: after import + rebuild, MATCH query on about_en text returns results
// Test: MATCH query on concept name_en returns results via the content view
// Test: FTS results join back to concept_guidance and concepts via rowid
```

## Integration Tests

```rust
// Test: full pipeline — create_test_app() with guidance JSON, verify data queryable
// Test: existing API endpoints still work after migration 004 (no regressions)
```
