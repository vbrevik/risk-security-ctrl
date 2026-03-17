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

The ontology covers:
- **ISO 31000:2018** - Risk management principles, framework, process
- **ISO 31010** - Risk assessment techniques (FMEA, fault trees, etc.)
- **NIST CSF** - Cybersecurity Framework (Identify, Protect, Detect, Respond, Recover)

Cross-framework mappings link equivalent concepts between standards.

## Gotchas

### Ontology Concept ID Naming
NIST AI RMF concept IDs use abbreviated prefixes: `gv-` (Govern), `mp-` (Map), `ms-` (Measure), `mg-` (Manage) — not full words. Always verify actual IDs from the JSON before writing cross-framework relationships.
