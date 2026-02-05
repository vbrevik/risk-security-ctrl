# Ontology - Risk Management Framework Explorer

A knowledge management system for exploring and applying ISO 31000 series standards and NIST frameworks to improve IT security in governmental organizations.

## Tech Stack

### Backend
- **Runtime:** Rust
- **Framework:** Axum
- **API Documentation:** utoipa (OpenAPI/Swagger)
- **Database:** SQLite3 with SQLx
- **Architecture:** Feature-based modules

### Frontend
- **Build Tool:** Vite
- **Framework:** React 18+ with TypeScript
- **Routing:** TanStack Router (file-based)
- **Styling:** shadcn/ui + Tailwind CSS
- **State:** TanStack Query
- **i18n:** i18next

### Ontology
- **Format:** OWL/RDF for formal ontology definitions
- **Storage:** SQLite3 with graph-like schema
- **Visualization:** D3.js / Cytoscape.js

## Project Structure

```
ontology/
├── backend/                 # Rust REST API
│   ├── src/
│   │   ├── features/        # Feature modules
│   │   │   ├── ontology/    # Ontology CRUD & queries
│   │   │   ├── compliance/  # Compliance tracking
│   │   │   ├── reports/     # Reporting engine
│   │   │   └── auth/        # Authentication (TBD)
│   │   ├── db/              # Database migrations & queries
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/                # Vite + React
│   ├── src/
│   │   ├── features/        # Feature modules
│   │   │   ├── ontology/    # Ontology explorer
│   │   │   ├── compliance/  # Compliance dashboard
│   │   │   └── reports/     # Report generation
│   │   ├── routes/          # TanStack file-based routes
│   │   ├── components/      # Shared UI components
│   │   └── i18n/            # Internationalization
│   └── package.json
├── ontology-data/           # OWL/RDF source files
│   ├── iso31000.owl         # ISO 31000:2018
│   ├── iso31010.owl         # ISO 31010 techniques
│   └── nist.owl             # NIST framework mappings
└── docs/
    ├── PRD.md               # Product requirements
    └── TASKS.md             # Sprint planning
```

## Quick Start

### Prerequisites
- Rust 1.75+
- Node.js 20+
- pnpm 8+

### Backend
```bash
cd backend
cargo run
# API available at http://localhost:3000
# Swagger UI at http://localhost:3000/swagger-ui
```

### Frontend
```bash
cd frontend
pnpm install
pnpm dev
# App available at http://localhost:5173
```

### Database
```bash
cd backend
cargo sqlx database create
cargo sqlx migrate run
```

## Development Commands

### Backend
```bash
cargo build              # Build
cargo run                # Run development server
cargo test               # Run all tests
cargo test <name>        # Run specific test
cargo clippy             # Lint
cargo fmt                # Format
cargo sqlx prepare       # Generate offline query data
```

### Frontend
```bash
pnpm dev                 # Development server
pnpm build               # Production build
pnpm preview             # Preview production build
pnpm test                # Run tests
pnpm lint                # ESLint
pnpm format              # Prettier
pnpm typecheck           # TypeScript check
```

## Ontology-First Development

This project follows an ontology-first approach:

1. **Define concepts** in OWL/RDF format (`ontology-data/`)
2. **Generate schema** from ontology definitions
3. **Build API** endpoints that expose ontology structure
4. **Create UI** that visualizes and navigates the ontology

The ontology captures:
- ISO 31000:2018 risk management principles
- ISO 31010 risk assessment techniques
- IEC 31010 vocabulary terms
- NIST Cybersecurity Framework mappings
- Cross-framework relationships

## License

Proprietary - Government use only
