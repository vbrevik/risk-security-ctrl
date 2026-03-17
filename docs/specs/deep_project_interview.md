# Deep Project Interview: Document Analysis Engine

_Date: 2026-03-17_

## Requirements Source

`docs/specs/2026-03-17-document-analysis-engine-design.md` — Feature spec produced by /spec-feature interview.

## Interview Summary

### Prior Context (from /spec-feature)

The user described a system where real-world documents or scenarios are analyzed against the ontology's risk/security frameworks to produce gap analyses with prioritized recommendations and proper source references. Key decisions made during spec:

- **Input**: Single file (PDF/DOCX) or free-text scenario
- **Matching**: MVP uses deterministic matching (no LLM). Phase 2 adds Claude cloud / Ollama local.
- **Output**: Full gap analysis with prioritized action items, exportable as PDF/DOCX
- **Framework selection**: Fully automatic — system detects relevance from content
- **Entity model**: Standalone analyses, not linked to compliance assessments in MVP
- **Reference validation**: Post-processing pass verifies all concept references exist in ontology DB
- **Audit trail**: Full logging to audit_log table
- **Cost tracking**: Token count + processing time per analysis
- **Prompt template**: JSON-based config, editable by user, with "improve prompt" option
- **Language**: English only for MVP
- **File size limit**: 20MB max
- **Auth**: Skipped for MVP (Sprint 7), stub created_by field

### Deep-Project Interview

**Q: How do you think about the build order — backend as one unit or separate pieces?**

A: **Separate pieces.** The user sees document parsing, matching engine, backend API, and export as distinct subsystems that should be planned independently.

**Q: Frontend visualization scope?**

A: **Need more visualizations** beyond the basic list/create/detail pages. User wants:
- Coverage heatmap (framework/control matrix, color-coded by finding type)
- Framework radar/spider chart (coverage % per framework area)
- Gap priority board (Kanban-style by priority level)
- All of the above — comprehensive analysis dashboard

**Q: Should the matching engine be designed for LLM readiness from day one?**

A: **Yes — design for LLM now.** Define a `MatchingEngine` trait/interface that both the deterministic and future LLM implementations satisfy. This avoids refactoring the pipeline architecture later.

**Q: Should exported reports include rendered charts?**

A: **Yes — include charts.** Render visualizations as images and embed in PDF/DOCX exports. Richer output is preferred over simplicity.

**Q: What deterministic matching approach?**

A: **Both combined.** Two-stage pipeline: FTS5 for candidate retrieval, then keyword overlap scoring (TF-IDF-like) for ranking. Leverages existing SQLite FTS5 infrastructure.

## Key Architecture Decisions

1. **Trait-based matching engine** — `MatchingEngine` trait with `DeterministicMatcher` (MVP) and `LlmMatcher` (Phase 2) implementations
2. **Two-stage matching pipeline** — FTS5 candidate retrieval → keyword overlap scoring
3. **Rich visualization** — Coverage heatmap, radar chart, priority board on the analysis detail page
4. **Charts in exports** — Visualizations rendered as images for PDF/DOCX embedding
5. **Separate subsystems** — Parsing, matching, API/DB, export, and frontend are distinct planning units
