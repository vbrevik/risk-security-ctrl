# Feature Spec: Document Analysis Engine

_Date: 2026-03-17 | Status: Draft_

## Summary

Analyze uploaded documents (PDF/DOCX) or typed scenarios against the ontology's risk/security frameworks, producing a gap analysis with prioritized action items and proper framework references. MVP uses deterministic keyword/concept matching; LLM integration (Claude cloud / Ollama local) is planned for Phase 2.

## Context & Motivation

Users have real-world documents (security policies, incident reports, risk scenarios) that need to be evaluated against regulatory and best-practice frameworks. Today they must manually read a document, then browse the ontology explorer to find relevant concepts — a tedious, error-prone process that requires deep framework expertise.

This feature automates the mapping: upload a document, get back a structured report showing which framework controls are relevant, what gaps exist, and what actions to prioritize. The output includes proper source references (e.g., "ISO 31000:2018 Clause 6.4.2") so recommendations are traceable and auditable.

Without this, the ontology explorer remains a browsing tool rather than an actionable analysis platform.

## In Scope

### MVP (Phase 1: Deterministic Matching)

- **Document input**: Single file upload (PDF, DOCX) or free-text scenario input
- **Text extraction**: Parse PDF and DOCX into plain text
- **Framework detection**: Automatically identify which frameworks are relevant based on document content
- **Concept matching**: Match document content against ontology concepts using keyword extraction, full-text search, and topic tag intersection
- **Gap analysis**: For each matched framework, identify relevant concepts and categorize as addressed/partially addressed/gap
- **Prioritized recommendations**: Rank action items by coverage gap severity and framework importance
- **Reference validation**: Every recommendation includes a verified reference to an actual ontology concept (concept ID, code, name, source_reference)
- **Prompt template**: Configurable matching/analysis prompt template (editable by user, with "improve prompt" option) — prepares for LLM integration
- **Cost tracking**: Track processing metrics (time, tokens when LLM is added) per analysis
- **Export**: Backend-generated PDF and DOCX reports
- **Audit trail**: Full logging of who analyzed what, when, with what parameters
- **Rollback**: Analyses can be deleted (with audit log entry preserved)
- **Standalone entity**: Each analysis is independent — not linked to compliance assessments

### Phase 2 (Backlog — LLM Integration)

- Replace deterministic matching with LLM-powered analysis (Claude cloud preferred, Ollama local fallback)
- LLM auto-selects relevant frameworks and generates nuanced gap assessment
- Post-processing verification pass validates all LLM-generated references against ontology DB
- Token usage and API cost tracking per analysis
- Document chunking for large documents (50+ pages)
- Multi-document analysis (document sets from folder)
- "Convert to assessment" — create compliance assessment pre-populated from analysis results

## Out of Scope

- User authentication/authorization (auth is Sprint 7 — skip for MVP)
- Multi-document sets (Phase 2)
- Real-time streaming of analysis progress
- Image/diagram analysis within documents
- Direct compliance assessment creation from analysis (Phase 2)
- Custom framework upload (frameworks are managed via ontology-data JSON files)

## Technical Design

### Data Model

#### New: `analyses` table

```sql
CREATE TABLE analyses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    input_type TEXT NOT NULL CHECK(input_type IN ('text', 'pdf', 'docx')),
    input_text TEXT,                    -- for text/scenario input
    original_filename TEXT,             -- for file uploads
    file_path TEXT,                     -- stored file location
    extracted_text TEXT,                -- parsed plain text from document
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK(status IN ('pending', 'processing', 'completed', 'failed', 'deleted')),
    error_message TEXT,                 -- if status = 'failed'
    prompt_template TEXT,               -- the matching prompt used
    matched_framework_ids TEXT,         -- JSON array of framework IDs detected
    processing_time_ms INTEGER,         -- wall-clock time for analysis
    token_count INTEGER,                -- document token count (prep for LLM billing)
    created_by TEXT,                    -- user ID (nullable until auth exists)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_analyses_status ON analyses(status);
CREATE INDEX idx_analyses_created_by ON analyses(created_by);
CREATE INDEX idx_analyses_created_at ON analyses(created_at);
```

#### New: `analysis_findings` table

```sql
CREATE TABLE analysis_findings (
    id TEXT PRIMARY KEY,
    analysis_id TEXT NOT NULL REFERENCES analyses(id) ON DELETE CASCADE,
    concept_id TEXT NOT NULL REFERENCES concepts(id),
    framework_id TEXT NOT NULL REFERENCES frameworks(id),
    finding_type TEXT NOT NULL
        CHECK(finding_type IN ('addressed', 'partially_addressed', 'gap', 'not_applicable')),
    confidence_score REAL NOT NULL DEFAULT 0.0,  -- 0.0-1.0, keyword match strength
    evidence_text TEXT,                           -- excerpt from document that matched
    recommendation TEXT,                          -- action item for this finding
    priority INTEGER NOT NULL DEFAULT 3           -- 1=critical, 2=high, 3=medium, 4=low
        CHECK(priority BETWEEN 1 AND 4),
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_analysis_findings_analysis ON analysis_findings(analysis_id);
CREATE INDEX idx_analysis_findings_framework ON analysis_findings(framework_id);
CREATE INDEX idx_analysis_findings_type ON analysis_findings(finding_type);
CREATE INDEX idx_analysis_findings_priority ON analysis_findings(priority);
```

#### New Rust models

```rust
// Analysis entity
struct Analysis {
    id: String,
    name: String,
    description: Option<String>,
    input_type: InputType,          // Text, Pdf, Docx
    input_text: Option<String>,
    original_filename: Option<String>,
    file_path: Option<String>,
    extracted_text: Option<String>,
    status: AnalysisStatus,         // Pending, Processing, Completed, Failed, Deleted
    error_message: Option<String>,
    prompt_template: Option<String>,
    matched_framework_ids: Vec<String>,  // deserialized from JSON
    processing_time_ms: Option<i64>,
    token_count: Option<i64>,
    created_by: Option<String>,
    created_at: String,
    updated_at: String,
}

// Individual finding within an analysis
struct AnalysisFinding {
    id: String,
    analysis_id: String,
    concept_id: String,
    framework_id: String,
    finding_type: FindingType,      // Addressed, PartiallyAddressed, Gap, NotApplicable
    confidence_score: f64,
    evidence_text: Option<String>,
    recommendation: Option<String>,
    priority: i32,                  // 1-4
    sort_order: i32,
    created_at: String,
}

// Finding enriched with concept details for API responses
struct AnalysisFindingWithConcept {
    // finding fields +
    concept_code: Option<String>,
    concept_name: String,
    concept_definition: String,
    source_reference: Option<String>,
}

// Analysis summary for list views
struct AnalysisSummary {
    // analysis fields +
    total_findings: i64,
    gap_count: i64,
    addressed_count: i64,
    frameworks_matched: Vec<FrameworkSummary>,
}
```

### Analysis Pipeline (Deterministic MVP)

```
Input (text/PDF/DOCX)
  |
  v
[1. Text Extraction]        -- pdf-extract, docx-rs
  |
  v
[2. Tokenization]           -- split into sentences/paragraphs, extract keywords
  |
  v
[3. Framework Detection]    -- match keywords against framework names, topic tags,
  |                            concept definitions using FTS5
  v
[4. Concept Matching]       -- for each relevant framework, score every concept
  |                            against document content (TF-IDF-like scoring)
  v
[5. Gap Classification]     -- categorize each matched concept:
  |                            addressed (high match), partial (medium),
  |                            gap (framework relevant but concept not in doc)
  v
[6. Reference Validation]   -- verify every concept_id exists in DB
  |
  v
[7. Priority Ranking]       -- gaps ranked by: framework importance,
  |                            concept level (root > child), coverage score
  v
[8. Report Generation]      -- structured AnalysisFindings stored in DB
```

### Routes / Endpoints

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| POST | `/api/analyses` | `create_analysis` | Create from text input (JSON body) |
| POST | `/api/analyses/upload` | `upload_analysis` | Create from file upload (multipart) |
| GET | `/api/analyses` | `list_analyses` | List analyses (paginated, filterable) |
| GET | `/api/analyses/:id` | `get_analysis` | Get analysis with summary stats |
| GET | `/api/analyses/:id/findings` | `get_analysis_findings` | Get findings (filterable by framework, type, priority) |
| DELETE | `/api/analyses/:id` | `delete_analysis` | Soft-delete (status → 'deleted') |
| GET | `/api/analyses/:id/export/:format` | `export_analysis` | Export as PDF or DOCX |
| GET | `/api/analyses/prompt-template` | `get_prompt_template` | Get current default prompt template |
| PUT | `/api/analyses/prompt-template` | `update_prompt_template` | Update prompt template |

#### Request/Response Examples

**POST `/api/analyses`** (text input):
```json
{
  "name": "Q1 Security Policy Review",
  "description": "Evaluating our updated security policy",
  "input_text": "Our organization implements multi-factor authentication for all systems...",
  "prompt_template": null  // use default
}
```

**POST `/api/analyses/upload`** (file upload):
- Multipart form: `file` (PDF/DOCX) + `name` (string) + `description` (optional string)

**GET `/api/analyses/:id`** response:
```json
{
  "id": "abc-123",
  "name": "Q1 Security Policy Review",
  "status": "completed",
  "input_type": "text",
  "matched_framework_ids": ["nist-csf", "iso27000", "cisa-ztmm"],
  "processing_time_ms": 1240,
  "token_count": 3400,
  "summary": {
    "total_findings": 47,
    "gap_count": 12,
    "partially_addressed_count": 8,
    "addressed_count": 27,
    "frameworks_matched": [
      { "id": "nist-csf", "name": "NIST Cybersecurity Framework", "finding_count": 18, "gap_count": 5 }
    ]
  },
  "created_at": "2026-03-17T10:00:00Z"
}
```

**GET `/api/analyses/:id/findings?framework_id=nist-csf&finding_type=gap&sort=priority`** response:
```json
{
  "data": [
    {
      "id": "finding-1",
      "concept_id": "nist-csf-de-cm",
      "concept_code": "DE.CM",
      "concept_name": "Continuous Monitoring",
      "concept_definition": "The information system and assets are monitored...",
      "source_reference": "NIST CSF v1.1 DE.CM",
      "framework_id": "nist-csf",
      "finding_type": "gap",
      "confidence_score": 0.85,
      "evidence_text": null,
      "recommendation": "Document does not address continuous monitoring. Consider implementing...",
      "priority": 1
    }
  ],
  "total": 5
}
```

### UI / Components

#### New route: `/analysis`

**Analysis List Page** (`/analysis`):
- Table of past analyses with status, date, framework count, gap count
- "New Analysis" button → opens create dialog
- Filter by status, date range
- Click row → analysis detail page

**Create Analysis Dialog**:
- Tab 1: "Paste Text" — textarea for scenario/policy text
- Tab 2: "Upload File" — drag-and-drop zone for PDF/DOCX
- Name field (required)
- Description field (optional)
- Collapsible "Advanced" section showing prompt template (editable textarea)
- "Analyze" button → starts processing, shows spinner, redirects to detail on completion

**Analysis Detail Page** (`/analysis/:id`):
- Header: name, status badge, date, processing time, document token count
- Framework summary cards: one per matched framework showing finding breakdown (donut chart or bar)
- Findings table: sortable/filterable by framework, type (gap/partial/addressed), priority
- Each finding row shows: priority badge, concept code, concept name, finding type, evidence excerpt, recommendation
- Click concept code → links to ontology explorer (concept detail)
- Export buttons: "Export PDF" / "Export DOCX"
- Delete button (with confirmation)

**Navigation**: Add "Analysis" to the main nav bar between "Compliance Tracking" and "Reports"

### Integration Points

- **Logging/Events**: All analysis CRUD operations logged to `audit_log` table (action: 'analysis_created', 'analysis_completed', 'analysis_deleted', 'analysis_exported')
- **External services**: None for MVP. Phase 2 adds Claude API / Ollama HTTP API
- **Navigation/Discovery**: New "Analysis" nav item in root layout. Findings link to ontology explorer concept detail view.
- **Permissions**: None for MVP. Stub `created_by` field for future auth integration.
- **Ontology integration**: Analysis reads from `concepts`, `frameworks`, `relationships`, `concepts_fts` tables. Read-only — never mutates ontology data.
- **File storage**: Uploaded files stored in `backend/uploads/` directory. Extracted text stored in DB.

### New Dependencies

**Backend (Cargo.toml)**:
- `pdf-extract` — PDF text extraction
- `docx-rs` or `docx` — DOCX text extraction
- `genpdf` — PDF report generation
- `docx-rs` — DOCX report generation (reuse)
- `multer` — already available via axum multipart support

**Frontend (package.json)**:
- No new dependencies expected — existing shadcn/ui components (Table, Dialog, Tabs, Badge, Card) cover the UI needs

## Edge Cases & Error Handling

| Edge Case | Handling |
|-----------|----------|
| LLM unavailable (Phase 2) | Return clear error: "Analysis service unavailable. Check Claude API key or Ollama status." Do not queue. |
| Invalid/corrupt PDF/DOCX | Return 400 with message: "Could not extract text from file. Please check the file is a valid PDF/DOCX." |
| Empty document / no text extracted | Return 400: "No text content found in the uploaded document." |
| Irrelevant document (no framework matches) | Complete analysis with 0 findings. UI shows: "No relevant frameworks detected. Please verify the correct document was uploaded or try describing your scenario in text." |
| Very large document (50+ pages) | Process anyway for MVP (deterministic matching is fast). Add warning in response. Phase 2 introduces chunking for LLM. |
| Concept reference doesn't exist in DB | Drop the finding silently during validation step. Log warning. Never show unverified references. |
| Analysis deleted | Soft-delete: status → 'deleted'. Audit log entry preserved. Can be excluded from list queries. Uploaded file retained for audit (configurable). |
| Concurrent analysis requests | Each analysis is independent. No concurrency issues. |
| Duplicate document upload | Allowed — each analysis is standalone. No dedup. |

## Testing Plan

- [ ] Happy path: Upload a security policy PDF, get analysis with findings across multiple frameworks, verify concept references exist in ontology DB
- [ ] Text input: Submit scenario text, get analysis with relevant framework matches
- [ ] DOCX input: Upload DOCX file, verify text extraction and analysis completion
- [ ] Empty state: Analysis list shows "No analyses yet" with create CTA
- [ ] Irrelevant document: Upload non-security document, verify "no frameworks detected" response
- [ ] Corrupt file: Upload invalid PDF, verify 400 error response
- [ ] Export PDF: Generate PDF report, verify it contains findings with proper references
- [ ] Export DOCX: Generate DOCX report, verify it contains findings with proper references
- [ ] Delete: Soft-delete analysis, verify it disappears from list, audit log entry created
- [ ] Reference validation: Inject a fake concept_id in matching pipeline, verify it gets filtered out
- [ ] Prompt template: Update prompt template, run analysis, verify new template is used
- [ ] Large document: Upload 50+ page PDF, verify it completes without timeout

## Tradeoffs & Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| MVP matching engine | Deterministic (FTS5 + keyword) | No API costs, no hallucination, works offline, forces clean pipeline architecture. LLM drops in later as a matching engine upgrade. |
| Analysis entity | Standalone (not linked to assessments) | Keeps blast radius small. Each analysis is disposable. Phase 2 can add "convert to assessment" bridge. |
| Framework detection | Fully automatic | User shouldn't need framework expertise to use this. System detects relevance from content. |
| Export side | Backend-generated | Consistent formatting, works without browser, easier to template. |
| File storage | Local filesystem + DB text | Simple for MVP. Phase 2 could add S3/MinIO if needed. |
| Prompt template | Editable per-analysis with default | Prepares for LLM integration. Users can tune matching behavior now. |
| Soft delete | Status flag, not hard delete | Audit trail preserved. Rollback = change status back. |
| Cost tracking | Token count + processing time | Meaningful now (document size metric), essential for Phase 2 LLM billing. |

## Resolved Questions

1. **Default prompt template content** — JSON-based config for keyword extraction and matching rules. Structured so it naturally evolves into a natural language prompt when LLM integration is added in Phase 2.
2. **PDF generation style** — Clean and professional layout. No specific government branding for MVP.
3. **Upload file size limit** — 20MB max.
4. **Language support** — English only for MVP. Norwegian matching deferred to Phase 2.
