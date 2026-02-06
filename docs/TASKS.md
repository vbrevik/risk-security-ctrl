# Sprint Planning & Tasks

## Release Plan

| Sprint | Duration | Focus | Milestone |
|--------|----------|-------|-----------|
| Sprint 0 | 2 weeks | Project Setup & Ontology Foundation | Infrastructure Ready |
| Sprint 1 | 2 weeks | Ontology Data & Basic API | Ontology API Complete |
| Sprint 2 | 2 weeks | Ontology Explorer UI | MVP Ontology Explorer |
| Sprint 3 | 2 weeks | Compliance Tracking Backend | Compliance API Complete |
| Sprint 4 | 2 weeks | Compliance Tracking UI | MVP Compliance Tracker |
| Sprint 5 | 2 weeks | Reporting Backend | Reporting API Complete |
| Sprint 6 | 2 weeks | Reporting UI & Dashboards | MVP Reporting |
| Sprint 7 | 2 weeks | User Management & Auth | Multi-user Ready |
| Sprint 8 | 2 weeks | i18n & Polish | v1.0 Release Candidate |

---

## Sprint 0: Project Setup & Ontology Foundation
**Goal:** Establish project infrastructure and define the core ontology

### Tasks

#### T0.1: Backend Project Setup
- [x] Initialize Rust project with Cargo
- [x] Configure Axum web framework
- [x] Set up utoipa for OpenAPI/Swagger
- [x] Configure SQLx with SQLite
- [x] Create feature-based module structure
- [x] Set up error handling and logging (tracing)
- [x] Configure CORS for frontend development

**Acceptance:** `cargo run` starts server, Swagger UI accessible at `/swagger-ui` ✅

#### T0.2: Frontend Project Setup
- [x] Initialize Vite + React + TypeScript project
- [x] Configure TanStack Router (file-based)
- [x] Set up shadcn/ui and Tailwind CSS
- [x] Configure TanStack Query for API calls
- [x] Set up i18next framework
- [x] Create feature-based folder structure
- [x] Configure ESLint + Prettier

**Acceptance:** `pnpm dev` starts app, routing works, shadcn components render ✅

#### T0.3: Database Schema Design
- [x] Design ontology storage schema (concepts, relationships, properties)
- [x] Design compliance tracking schema (assessments, items, evidence)
- [x] Design user/auth schema
- [x] Design audit log schema
- [x] Create initial SQLx migrations

**Acceptance:** Migrations run successfully, schema supports ontology-first model ✅

#### T0.4: Ontology Definition - ISO 31000:2018
- [x] Define core concepts (Risk, Risk Management, Framework, Process)
- [x] Define principles (8 principles from the standard)
- [x] Define framework components (Leadership, Integration, Design, etc.)
- [x] Define process steps (Communication, Scope, Assessment, Treatment, etc.)
- [x] Create relationships between concepts

**Acceptance:** JSON-LD file validates, covers all ISO 31000:2018 main concepts

#### T0.5: Ontology Definition - ISO 31010
- [x] Define risk assessment techniques (31 techniques)
- [x] Categorize by assessment phase (identification, analysis, evaluation)
- [x] Link techniques to ISO 31000 process steps

**Acceptance:** JSON-LD file covers key ISO 31010 techniques with mappings

#### T0.6: Ontology Definition - NIST CSF
- [x] Define NIST CSF functions (Identify, Protect, Detect, Respond, Recover)
- [x] Define categories and subcategories
- [x] Create cross-walk mappings to ISO 31000

**Acceptance:** JSON-LD concepts linked to ISO 31000 equivalents

---

## Sprint 1: Ontology Data & Basic API
**Goal:** Load ontology into database and expose via REST API

### User Stories
- US1.2: Search for terms
- US1.4: Filter by framework

### Tasks

#### T1.1: Ontology Import Pipeline
- [x] Create JSON-LD parser (custom, imports from ontology-data/)
- [x] Transform ontology to database records
- [x] Auto-import on server startup if DB empty
- [x] Validate imported data integrity

**Acceptance:** Server auto-imports 8 framework files + relationships on startup ✅

#### T1.2: Ontology API - Concepts
- [x] GET /api/ontology/concepts - List all concepts (paginated)
- [x] GET /api/ontology/concepts/:id - Get concept details
- [x] GET /api/ontology/concepts/search?q= - Full-text search
- [x] GET /api/ontology/concepts/:id/relationships - Get related concepts

**Acceptance:** All endpoints documented in Swagger, return correct data ✅

#### T1.3: Ontology API - Frameworks
- [x] GET /api/ontology/frameworks - List frameworks (ISO 31000, NIST, etc.)
- [x] GET /api/ontology/concepts?framework_id= - Concepts by framework (via query param)
- [x] Hierarchy via parent_id field (client-side tree building)

**Acceptance:** Can retrieve framework-specific concept trees ✅

#### T1.4: Ontology API - Relationships
- [x] GET /api/ontology/relationships - List all relationships
- [x] Cross-framework mappings available via relationships endpoint
- [x] Per-concept graph via GET /api/ontology/concepts/:id/relationships

**Acceptance:** Graph queries return nodes and edges for visualization ✅

#### T1.5: API Testing
- [x] Write integration tests for all ontology endpoints
- [x] Set up test database fixtures
- [x] Configure CI test runner

**Acceptance:** `cargo test` passes all API tests ✅ (12 tests passing)

---

## Sprint 2: Ontology Explorer UI
**Goal:** Build interactive ontology visualization and navigation

### User Stories
- US1.1: Visual graph browsing
- US1.3: NIST to ISO mapping view
- US1.5: Concept detail view

### Tasks

#### T2.1: Ontology Explorer Layout
- [x] Create explorer page with sidebar + main area layout
- [x] Implement framework selector (expand/collapse in sidebar)
- [x] Add search input with result dropdown (2+ char trigger)
- [x] Create breadcrumb navigation (in detail view hierarchy)
- [x] Wire up UI strings to i18n (en + nb)
- [x] Wire GraphControls tooltips to i18n (5 tooltip strings)

**Acceptance:** Basic layout renders with navigation elements ✅

#### T2.2: Graph Visualization Component
- [x] Integrate D3.js force-directed graph
- [x] Render nodes and edges from API data (all frameworks dynamically)
- [x] Implement zoom, pan, and fit controls
- [x] Add node click selection
- [x] Style nodes by type/framework (color coding)
- [x] Minimap component

**Acceptance:** Graph renders up to 200 nodes smoothly with interactions ✅

#### T2.3: Concept Detail Panel
- [x] Create detail view (full-page tab, not slide-out)
- [x] Display concept name, definition, source
- [x] List related concepts with clickable links
- [x] Show framework and category tags
- [x] Display cross-framework mappings in detail view (with framework color dots and badges)
- [x] Language toggle (EN/NB) in detail view

**Acceptance:** Clicking a node shows concept information ✅

#### T2.4: Search & Filter
- [x] Implement search with debouncing (300ms)
- [x] Highlight search results in graph (amber highlight + dim others)
- [x] Add framework filter (multi-select checkboxes)
- [x] Add category/type filter (dropdown)
- [x] Persist filters in URL params (view, concept, frameworks, type)

**Acceptance:** Search and filters work together, shareable URLs ✅

#### T2.5: Hierarchy View
- [x] Create tree view component as main-area alternative to graph
- [x] Collapsible/expandable nodes with type-specific icons
- [x] Toggle between graph, tree, detail, and compare views
- [x] Compare view for side-by-side framework comparison

**Acceptance:** Users can switch between graph and tree visualizations ✅

---

## Sprint 3: Compliance Tracking Backend
**Goal:** Implement compliance assessment API

### User Stories
- US2.1: Create assessment
- US2.2: Mark requirement status
- US2.3: Attach evidence

### Tasks

#### T3.1: Assessment API - CRUD
- [x] POST /api/compliance/assessments - Create new assessment (auto-generates items from framework)
- [x] GET /api/compliance/assessments - List assessments (paginated, filterable by framework/status/owner)
- [x] GET /api/compliance/assessments/:id - Get assessment details
- [x] PUT /api/compliance/assessments/:id - Update assessment (partial updates supported)
- [x] DELETE /api/compliance/assessments/:id - Delete assessment (cascading delete)

**Acceptance:** Full CRUD operations with validation ✅

#### T3.2: Compliance Items API
- [x] GET /api/compliance/assessments/:id/items - Get checklist items (paginated, joins concept data)
- [x] PUT /api/compliance/assessments/:id/items/:itemId - Update item status
- [x] POST /api/compliance/assessments/:id/items/:itemId/notes - Add timestamped note
- [x] Generate items from ontology structure on assessment creation

**Acceptance:** Items linked to ontology concepts, status updates persist ✅

#### T3.3: Evidence API
- [x] POST /api/compliance/assessments/:id/items/:itemId/evidence - Add evidence (URL or file_path reference)
- [x] GET /api/compliance/assessments/:id/items/:itemId/evidence - List evidence for item
- [x] DELETE /api/compliance/evidence/:id - Remove evidence
- [x] Multipart file upload handler (POST .../evidence/upload with file, title, description)

**Acceptance:** Evidence metadata stored and retrievable, file upload supported ✅

#### T3.4: Compliance Scoring
- [x] Calculate compliance percentage per section (grouped by top-level concepts)
- [x] Calculate overall compliance score (weighted: full=1.0, partial=0.5)
- [x] Handle "Not Applicable" items in scoring (excluded from denominator)
- [x] API endpoint: GET /api/compliance/assessments/:id/score

**Acceptance:** Scores calculate correctly, match manual verification ✅

#### T3.5: Audit Trail
- [x] Log all assessment modifications (create, update, delete)
- [x] Track user, timestamp, old/new values (JSON snapshots)
- [x] GET /api/compliance/assessments/:id/history - Get change history (includes item + evidence changes)

**Acceptance:** Complete audit trail for compliance changes ✅

#### T3.6: Compliance API Testing
- [x] Write integration tests for compliance endpoints (15 tests)
- [x] Test scoring calculation accuracy
- [x] Test audit trail completeness

**Acceptance:** `cargo test` passes all compliance API tests ✅ (15 tests passing)

---

## Sprint 4: Compliance Tracking UI
**Goal:** Build compliance assessment interface

### User Stories
- US2.4: View compliance score
- US2.5: Compliance trends
- US2.6: Notes and action items

### Tasks

#### T4.1: Assessment List Page
- [ ] Create assessments list view
- [ ] Display assessment name, date, score
- [ ] Add create new assessment button
- [ ] Add filters (status, date range, framework)

**Acceptance:** Users can view and create assessments

#### T4.2: Assessment Detail Page
- [ ] Create hierarchical checklist view
- [ ] Status dropdown for each item
- [ ] Progress bar per section
- [ ] Overall score display

**Acceptance:** Users can navigate and update compliance items

#### T4.3: Item Detail Modal
- [ ] Display item details from ontology
- [ ] Status selector with history
- [ ] Notes section (add/edit/delete)
- [ ] Evidence list with upload

**Acceptance:** Full item management in modal

#### T4.4: Evidence Management
- [ ] File upload component
- [ ] URL input for external references
- [ ] Evidence list with download/delete
- [ ] File type icons and metadata

**Acceptance:** Users can attach and manage evidence

#### T4.5: Compliance Dashboard Widget
- [ ] Compliance score gauge chart
- [ ] Section breakdown bar chart
- [ ] Recent activity list
- [ ] Link to full assessment

**Acceptance:** At-a-glance compliance status

---

## Sprint 5: Reporting Backend
**Goal:** Implement report generation API

### User Stories
- US3.2: Generate compliance report
- US3.3: Export PDF/Excel
- US3.5: Gap analysis report

### Tasks

#### T5.1: Report Templates
- [ ] Define report template structure (JSON schema)
- [ ] Create Compliance Summary template
- [ ] Create Gap Analysis template
- [ ] Create Audit Report template

**Acceptance:** Templates defined and validated

#### T5.2: Report Generation API
- [ ] POST /api/reports/generate - Generate report
- [ ] GET /api/reports - List generated reports
- [ ] GET /api/reports/:id - Get report data
- [ ] Support date range and filter parameters

**Acceptance:** Reports generate with correct data

#### T5.3: PDF Export
- [ ] Integrate PDF generation library (printpdf or similar)
- [ ] Implement Compliance Summary PDF
- [ ] Implement Gap Analysis PDF
- [ ] Add header/footer with branding

**Acceptance:** Professional PDF reports downloadable

#### T5.4: Excel Export
- [ ] Integrate XLSX library (rust_xlsxwriter)
- [ ] Implement compliance data export
- [ ] Multiple sheets for sections
- [ ] Formatted cells and charts

**Acceptance:** Excel files open correctly with formatted data

#### T5.5: Report Scheduling (Optional)
- [ ] Create scheduled report configuration
- [ ] Background job for report generation
- [ ] Email delivery integration

**Acceptance:** Reports auto-generate on schedule

---

## Sprint 6: Reporting UI & Dashboards
**Goal:** Build reporting interface and executive dashboard

### User Stories
- US3.1: Executive dashboard
- US3.4: Compliance trends

### Tasks

#### T6.1: Executive Dashboard Page
- [ ] Create dashboard layout with grid
- [ ] Compliance score KPI cards
- [ ] Trend chart (last 6 assessments)
- [ ] Framework coverage heatmap
- [ ] Recent activity feed

**Acceptance:** Dashboard loads in < 2 seconds with real data

#### T6.2: Report Generator UI
- [ ] Report type selector
- [ ] Assessment/date range picker
- [ ] Filter configuration
- [ ] Generate button with progress indicator

**Acceptance:** Users can configure and generate reports

#### T6.3: Report Viewer
- [ ] In-app report preview
- [ ] Download buttons (PDF, XLSX)
- [ ] Share/email functionality
- [ ] Report history list

**Acceptance:** Reports viewable and downloadable

#### T6.4: Charts & Visualizations
- [ ] Compliance trend line chart
- [ ] Section comparison bar chart
- [ ] Gap analysis radar chart
- [ ] Framework coverage donut chart

**Acceptance:** Charts render correctly with animations

#### T6.5: Dashboard Customization
- [ ] Widget selection/arrangement
- [ ] Save dashboard configuration per user
- [ ] Default dashboard templates

**Acceptance:** Users can personalize their dashboard

---

## Sprint 7: User Management & Auth
**Goal:** Implement multi-user support with RBAC

### User Stories
- US4.1: Create user accounts
- US4.2: Assign roles
- US4.3: Secure login
- US4.4: Audit log

### Tasks

#### T7.1: Auth Foundation
- [ ] Implement session management
- [ ] Create login/logout endpoints
- [ ] Password hashing (argon2)
- [ ] JWT or session token generation
- [ ] Auth middleware for protected routes

**Acceptance:** Users can log in and access protected resources

#### T7.2: User Management API
- [ ] POST /api/users - Create user
- [ ] GET /api/users - List users (admin)
- [ ] GET /api/users/:id - Get user
- [ ] PUT /api/users/:id - Update user
- [ ] DELETE /api/users/:id - Deactivate user

**Acceptance:** Full user CRUD with validation

#### T7.3: Role-Based Access Control
- [ ] Define roles: Admin, Risk Manager, Specialist, Viewer
- [ ] Define permissions per role
- [ ] Implement authorization middleware
- [ ] Apply to all API endpoints

**Acceptance:** Users restricted based on role

#### T7.4: Auth UI
- [ ] Login page
- [ ] Password reset flow
- [ ] User profile page
- [ ] Admin user management page

**Acceptance:** Complete auth UI flow

#### T7.5: Audit Logging
- [ ] Log authentication events
- [ ] Log data modifications with user context
- [ ] Admin audit log viewer
- [ ] Export audit log

**Acceptance:** Comprehensive audit trail viewable

---

## Sprint 8: i18n & Polish
**Goal:** Internationalization and release preparation

### User Stories
- US5.1: Language switcher
- US5.2: Norwegian ontology terms

### Tasks

#### T8.1: Frontend i18n
- [ ] Extract all UI strings to translation files
- [ ] Create English (en) translations
- [ ] Create Norwegian Bokmål (nb) translations
- [ ] Implement language switcher component
- [ ] Persist language preference

**Acceptance:** UI fully translatable, switcher works

#### T8.2: Ontology i18n
- [ ] Add Norwegian labels to ontology concepts
- [ ] API support for language parameter
- [ ] Frontend displays terms in selected language

**Acceptance:** Ontology terms available in both languages

#### T8.3: Accessibility Audit
- [ ] Run automated accessibility tests
- [ ] Manual keyboard navigation testing
- [ ] Screen reader testing
- [ ] Fix identified issues

**Acceptance:** WCAG 2.1 AA compliance verified

#### T8.4: Performance Optimization
- [ ] Frontend bundle optimization
- [ ] API query optimization
- [ ] Add caching where appropriate
- [ ] Load testing with realistic data

**Acceptance:** Performance metrics meet requirements

#### T8.5: Documentation & Deployment
- [ ] Complete API documentation
- [ ] User guide documentation
- [ ] Deployment guide
- [ ] Create release build scripts
- [ ] Final QA testing

**Acceptance:** v1.0 release ready

---

## Backlog (Future Sprints)

### Risk Assessment Workflow
- Guided risk identification process
- Risk analysis and evaluation
- Risk treatment planning
- Risk monitoring and review

### Integration
- SSO/OIDC authentication
- External GRC tool integration
- API for third-party access

### Advanced Features
- Real-time collaboration
- Custom ontology extensions
- AI-assisted gap analysis
- Mobile responsive enhancements
