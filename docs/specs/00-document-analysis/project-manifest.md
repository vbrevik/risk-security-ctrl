<!-- SPLIT_MANIFEST
01-db-models
02-document-parsing
03-matching-engine
04-backend-api-export
05-frontend-dashboard
END_MANIFEST -->

# Project Manifest: Document Analysis Engine

## Overview

Decomposes the Document Analysis Engine feature into 5 planning units, ordered by dependency. The system analyzes uploaded documents against risk/security framework ontologies, producing gap analyses with prioritized recommendations.

**Source spec:** `docs/specs/2026-03-17-document-analysis-engine-design.md`

## Split Structure

### 01-db-models — Database Schema & Core Types

**Purpose:** Foundation layer. New SQLite migration for `analyses` and `analysis_findings` tables. Rust model structs, enums (`AnalysisStatus`, `FindingType`, `InputType`), and the `MatchingEngine` trait that defines the interface for both deterministic (MVP) and LLM (Phase 2) matching implementations.

**Outputs:**
- Migration `003_analysis_schema.sql`
- `backend/src/features/analysis/models.rs` — structs, enums, serialization
- `backend/src/features/analysis/engine.rs` — `MatchingEngine` trait definition
- `backend/src/features/analysis/mod.rs` — module wiring

**Dependencies:** None (foundational)

**Estimated complexity:** Low-medium

---

### 02-document-parsing — Text Extraction Pipeline

**Purpose:** Extract plain text from PDF and DOCX files. Handle encoding, multi-page documents, error cases (corrupt files, empty docs, password-protected). Returns structured text with section boundaries where possible.

**Outputs:**
- `backend/src/features/analysis/parser.rs` — `DocumentParser` with `parse_pdf()`, `parse_docx()`, `parse_text()`
- File upload handling (multipart, 20MB limit, temp storage in `backend/uploads/`)
- Text tokenization utilities (sentence splitting, keyword extraction)

**Dependencies:**
- **schemas:** Uses `InputType` enum from 01-db-models
- New Cargo deps: `pdf-extract`, `docx-rs`

**Estimated complexity:** Medium

---

### 03-matching-engine — Framework Detection & Concept Scoring

**Purpose:** The analytical core. Implements the `MatchingEngine` trait with `DeterministicMatcher`. Two-stage pipeline: (1) FTS5 candidate retrieval to find potentially relevant concepts, (2) keyword overlap scoring (TF-IDF-like) to rank and classify matches. Detects which frameworks are relevant automatically. Classifies findings as addressed/partially_addressed/gap. Validates all references against ontology DB. Generates prioritized recommendations.

**Outputs:**
- `backend/src/features/analysis/matcher.rs` — `DeterministicMatcher` implementing `MatchingEngine`
- Framework detection logic (keyword → topic tag → framework mapping)
- Concept scoring algorithm (FTS5 retrieval + TF-IDF ranking)
- Gap classification rules (thresholds for addressed/partial/gap)
- Reference validation pass (verify concept_id exists in DB)
- Priority ranking algorithm
- JSON-based prompt template system (configurable matching parameters)

**Dependencies:**
- **models:** `MatchingEngine` trait, `AnalysisFinding`, `FindingType` from 01-db-models
- **APIs:** Reads from `concepts`, `frameworks`, `concepts_fts`, `relationships` tables
- **patterns:** Existing FTS5 search infrastructure from ontology feature

**Estimated complexity:** High (core algorithmic work)

---

### 04-backend-api-export — REST API & Report Generation

**Purpose:** HTTP layer. CRUD endpoints for analyses, file upload endpoint, findings query endpoint with filtering/sorting, PDF and DOCX export with embedded chart images. Audit logging. Cost/metrics tracking.

**Outputs:**
- `backend/src/features/analysis/routes.rs` — All REST endpoints per spec
- File upload handler (multipart → parser → matcher → DB)
- PDF report generator (using `genpdf`) with embedded chart images
- DOCX report generator (using `docx-rs`) with embedded chart images
- Audit log integration (analysis_created, completed, deleted, exported)
- Prompt template CRUD (get/update default template)

**Dependencies:**
- **models:** All types from 01-db-models
- **APIs:** `DocumentParser` from 02-document-parsing
- **APIs:** `DeterministicMatcher` from 03-matching-engine
- **schemas:** `analyses`, `analysis_findings` tables from 01-db-models
- New Cargo deps: `genpdf` (PDF generation)

**Estimated complexity:** High (orchestration + export rendering)

---

### 05-frontend-dashboard — Analysis UI & Visualizations

**Purpose:** Frontend feature module. Analysis list page, create dialog (text/file upload tabs), analysis detail page with rich visualizations (coverage heatmap, framework radar chart, gap priority board), findings table with filtering, export buttons, delete with confirmation. Navigation integration.

**Outputs:**
- `frontend/src/features/analysis/` — Full feature module (api/, components/, types/)
- `frontend/src/routes/analysis/` — Route pages (list, detail)
- API hooks: `useAnalyses`, `useAnalysis`, `useAnalysisFindings`, `useCreateAnalysis`, `useUploadAnalysis`, `useDeleteAnalysis`, `useExportAnalysis`
- Components: `AnalysisList`, `CreateAnalysisDialog`, `AnalysisDetail`, `CoverageHeatmap`, `FrameworkRadar`, `GapPriorityBoard`, `FindingsTable`
- i18n: `src/i18n/locales/{en}/analysis.json`
- Nav bar update: Add "Analysis" between "Compliance Tracking" and "Reports"

**Dependencies:**
- **APIs:** All backend endpoints from 04-backend-api-export must be available
- **patterns:** Follows existing frontend patterns (TanStack Query, shadcn/ui, i18next)
- Chart library: D3 (already installed) or recharts for radar/heatmap

**Estimated complexity:** High (3 custom visualizations + full CRUD UI)

---

## Dependency Graph

```
01-db-models (foundation)
    |
    +---> 02-document-parsing (needs InputType enum)
    |
    +---> 03-matching-engine (needs trait + models)
              |
              +---> 04-backend-api-export (needs parser + matcher)
                        |
                        +---> 05-frontend-dashboard (needs API endpoints)
```

## Execution Order

### Sequential chain (critical path):
```
01-db-models → 03-matching-engine → 04-backend-api-export → 05-frontend-dashboard
```

### Parallel opportunity:
```
After 01-db-models completes:
  - 02-document-parsing (parallel)
  - 03-matching-engine (parallel)

Both must complete before 04-backend-api-export starts.
```

## Cross-Cutting Concerns

- **Error handling:** All splits use existing `AppError`/`AppResult` patterns
- **Audit logging:** 04-backend-api-export integrates with existing `audit_log` table
- **i18n:** 05-frontend uses existing i18next setup, English only for MVP
- **Testing:** Each split should include unit tests. 04 needs integration tests for the full pipeline.
- **LLM readiness:** The `MatchingEngine` trait (01) is designed so 03's `DeterministicMatcher` can later be swapped for an `LlmMatcher` without changing 04's orchestration code.

## Next Steps

Run /deep-plan for each split in order:
```
/deep-plan @docs/specs/01-db-models/spec.md
/deep-plan @docs/specs/02-document-parsing/spec.md
/deep-plan @docs/specs/03-matching-engine/spec.md
/deep-plan @docs/specs/04-backend-api-export/spec.md
/deep-plan @docs/specs/05-frontend-dashboard/spec.md
```
