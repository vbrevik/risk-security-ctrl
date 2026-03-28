# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Risk management framework explorer for governmental IT security. Implements an ontology-first approach to help users understand and track compliance with ISO 31000 series standards and NIST CSF.

**Stack:**
- Backend: Rust + Axum + SQLx (SQLite3) + utoipa (Swagger)
- Frontend: Vite + React + TanStack Router (file-based) + TanStack Query + shadcn/ui + Tailwind
- Ontology: JSON-LD definitions in `ontology-data/`

## Architecture

### Ontology-First Approach
1. Domain concepts defined in JSON-LD format (`ontology-data/*.json`)
2. Database schema models the ontology graph (concepts, relationships, properties)
3. API exposes ontology structure for visualization
4. UI renders interactive concept graphs and compliance checklists

### Feature-Based Structure
Both backend and frontend use feature modules:
```
backend/src/features/{ontology,compliance,reports,auth}/
frontend/src/features/{ontology,compliance,reports}/
```

Each feature contains its own routes, handlers, components, and types.

## Development Commands

### Backend (from `backend/`)
```bash
cargo run                    # Start dev server (port 3000)
cargo test                   # Run tests
cargo test <name>            # Run specific test
cargo clippy                 # Lint
cargo fmt                    # Format
cargo sqlx migrate run       # Run migrations
cargo sqlx prepare           # Generate offline query data
```

### Frontend (from `frontend/`)
```bash
pnpm dev                     # Start dev server (port 5173)
pnpm build                   # Production build
pnpm test                    # Run tests
pnpm lint                    # ESLint
pnpm typecheck               # TypeScript check
```

## Key Patterns

### API Conventions
- REST endpoints under `/api/`
- OpenAPI docs at `/swagger-ui`
- All responses use consistent error format
- Pagination via `?page=&limit=` query params

### Frontend Routing
File-based routing with TanStack Router:
- `routes/__root.tsx` - Root layout
- `routes/index.tsx` - Home
- `routes/ontology/` - Ontology explorer
- `routes/compliance/` - Compliance tracking
- `routes/reports/` - Reporting

### State Management
- Server state: TanStack Query with API hooks in `features/*/api/`
- UI state: React context or component state
- No global state library

### Internationalization
- i18next with namespace per feature
- Translation files in `src/i18n/locales/{en,nb}/`
- Use `useTranslation()` hook, never hardcode strings

## Database

SQLite3 with SQLx for compile-time query checking. Key tables:
- `concepts` - Ontology nodes
- `relationships` - Edges between concepts
- `assessments` - Compliance assessments
- `compliance_items` - Assessment checklist items
- `evidence` - Attached evidence files/links
- `users`, `sessions` - Auth

## Domain Context

30 frameworks across EU legislation, NIST publications, ISO standards, MITRE databases, and industry frameworks. Key ones:
- **ISO 31000/31010/27000/42001** — Risk management, information security, AI management
- **NIST CSF/RMF/AI RMF/SP 800-53** — Cybersecurity, risk, AI governance, security controls
- **MITRE ATT&CK/ATLAS/CWE** — Threat intelligence, AI threats, software weaknesses
- **EU AI Act/GDPR/NIS2/DORA** — EU regulatory frameworks
- **Zero Trust (NIST 800-207)/CISA ZTMM** — Zero trust architecture and maturity

Cross-framework relationships (585) link equivalent concepts between standards.
Verification proof files in `docs/sources/{framework-id}-proof.md`.
Full backlog at `docs/BACKLOG.md`.

## Deep Workflow Integration Rules

When running `/deep-project`, `/deep-plan`, or `/deep-implement`, apply prompt contracts at these points:

- **deep-project Step 6** (Spec Generation): Draft a lightweight prompt contract before writing each split's spec.md
- **deep-plan Step 10** (Write Spec): Draft a prompt contract before synthesizing claude-spec.md
- **deep-plan Step 11** (Generate Plan): Draft a prompt contract before writing claude-plan.md
- **deep-implement Step 3** (Implement Section): Draft a prompt contract from the section file before coding
- **deep-implement Step 6** (Code Review): Pass the contract's FAILURE CONDITIONS to the code-reviewer subagent

Every prompt contract MUST run `/stig-compliance guard` after drafting CONSTRAINTS. This is non-blocking and advisory.

## Gotchas

### Ontology Concept ID Naming
NIST AI RMF concept IDs use abbreviated prefixes: `gv-` (Govern), `mp-` (Map), `ms-` (Measure), `mg-` (Manage) — not full words. Always verify actual IDs from the JSON before writing cross-framework relationships.

### Import Pipeline: Framework Files vs Relationship Files
Framework JSON files must be **manually listed** in the `framework_files` array in `backend/src/import.rs`. Relationship files (`relationships-*.json`) are **auto-discovered by glob**. If you add a new framework JSON and its relationship file without adding the framework to `import.rs`, the relationship glob will find and try to import relationships for a framework that was never loaded — causing FK constraint failures.

### FK Ordering in JSON Arrays
SQLite FK constraints require parent concepts to exist before children during INSERT. Concepts in JSON files must be topologically sorted (parents before children in the array). If a framework has hierarchical concepts, verify sort order.

### Stale DB After JSON Changes
After modifying ontology JSON files, always `rm -f ontology.db` before running `cargo test`. The test suite creates and populates the DB from scratch. A stale DB from a prior run will have old data.

### SQLx Offline Mode
When `ontology.db` doesn't exist at compile time, use `SQLX_OFFLINE=true` for builds and tests. To regenerate offline query data after schema changes: `sqlx database create && sqlx migrate run && cargo sqlx prepare`.

### Bulk Ontology JSON Edits
For changes across many JSON files, write a Python script to `/tmp/` rather than editing each file individually. This avoids token limits and is less error-prone.

### Flaky Guidance Tests
The `guidance_tests` integration tests share a single SQLite file across parallel test threads. Two tests (`integration_import_with_real_concept_ids`, `integration_fts5_search_with_real_data`) occasionally fail due to race conditions. They pass reliably with `--test-threads=1`. This is a pre-existing issue, not caused by your changes.
