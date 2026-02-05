# Product Requirements Document (PRD)

## Overview

**Product Name:** Ontology - Risk Management Framework Explorer
**Version:** 1.0
**Target Users:** Governmental IT security and risk management personnel
**Domain:** ISO 31000 series + NIST Framework applied to IT security

---

## Problem Statement

Governmental organizations struggle to understand and implement risk management frameworks (ISO 31000, NIST) for IT security. The standards are complex, interconnected, and difficult to navigate. Staff need a tool to:
- Understand framework concepts and their relationships
- Track compliance against framework requirements
- Generate reports for management and auditors

---

## User Personas

### 1. Risk Manager (Ragnhild)
- **Role:** Organizational risk coordinator
- **Goals:** Oversee risk management program, ensure framework alignment
- **Needs:** High-level dashboards, compliance status, audit reports

### 2. IT Security Specialist (Eirik)
- **Role:** Technical security implementation
- **Goals:** Understand which controls map to which risks, implement treatments
- **Needs:** Detailed ontology navigation, control mappings, technical documentation

### 3. Department Director (Dag)
- **Role:** Executive oversight
- **Goals:** Understand organizational risk posture, make informed decisions
- **Needs:** Executive summaries, trend reports, KPI dashboards

---

## Features

### F1: Ontology Explorer
Interactive visualization and navigation of risk management concepts.

#### User Stories
- **US1.1:** As Eirik, I want to browse ISO 31000 concepts in a visual graph so that I can understand relationships between principles, framework, and process.
- **US1.2:** As Eirik, I want to search for specific terms and see their definitions and relationships so that I can quickly find relevant information.
- **US1.3:** As Ragnhild, I want to see how NIST CSF maps to ISO 31000 so that I can align our existing NIST work with ISO requirements.
- **US1.4:** As Eirik, I want to filter the ontology by framework (ISO 31000, ISO 31010, NIST) so that I can focus on relevant standards.
- **US1.5:** As any user, I want to view concept details including definitions, examples, and related concepts so that I can deeply understand each term.

#### Acceptance Criteria
- Graph visualization with zoom, pan, and node selection
- Full-text search across all ontology concepts
- Filtering by framework, category, and relationship type
- Concept detail panel with definition, source reference, and relationships
- Cross-framework mapping visualization

---

### F2: Compliance Tracking
Track organizational compliance against framework requirements.

#### User Stories
- **US2.1:** As Ragnhild, I want to create a compliance assessment for our organization so that I can track our ISO 31000 implementation status.
- **US2.2:** As Eirik, I want to mark specific requirements as implemented, partial, or not implemented so that we can track progress.
- **US2.3:** As Ragnhild, I want to attach evidence documents to compliance items so that we can demonstrate compliance during audits.
- **US2.4:** As Ragnhild, I want to see an overall compliance score and breakdown by framework section so that I can identify gaps.
- **US2.5:** As Dag, I want to see compliance trends over time so that I can assess improvement trajectory.
- **US2.6:** As Eirik, I want to add notes and action items to compliance items so that we can track remediation work.

#### Acceptance Criteria
- Create/edit/delete compliance assessments
- Hierarchical checklist matching ontology structure
- Status options: Not Started, In Progress, Implemented, Not Applicable
- Evidence attachment (file upload or URL reference)
- Compliance score calculation (percentage by section and overall)
- Historical tracking with timestamps and user attribution
- Audit trail for all changes

---

### F3: Reporting & Dashboards
Generate reports and visualize organizational risk posture.

#### User Stories
- **US3.1:** As Dag, I want an executive dashboard showing key compliance metrics so that I can quickly assess our status.
- **US3.2:** As Ragnhild, I want to generate a compliance report for auditors so that I can demonstrate our framework implementation.
- **US3.3:** As Ragnhild, I want to export reports in PDF and Excel format so that I can share with stakeholders without system access.
- **US3.4:** As Dag, I want to see compliance trends across multiple assessments so that I can track improvement over time.
- **US3.5:** As Eirik, I want to generate a gap analysis report so that I can prioritize remediation efforts.

#### Acceptance Criteria
- Executive dashboard with KPIs, charts, and status indicators
- Report templates: Compliance Summary, Gap Analysis, Audit Report
- Export to PDF and XLSX
- Customizable date ranges and filters
- Scheduled report generation (email delivery)
- Data visualization: bar charts, trend lines, heatmaps

---

### F4: User Management & Authentication
Multi-user support with role-based access control.

#### User Stories
- **US4.1:** As an admin, I want to create user accounts so that team members can access the system.
- **US4.2:** As an admin, I want to assign roles (Admin, Risk Manager, Specialist, Viewer) so that users have appropriate access levels.
- **US4.3:** As a user, I want to log in securely so that my data is protected.
- **US4.4:** As an admin, I want to see an audit log of user actions so that I can monitor system usage.

#### Acceptance Criteria
- User CRUD operations
- Role-based access control (RBAC)
- Authentication mechanism (TBD: local/SSO)
- Session management
- Audit logging for sensitive operations

---

### F5: Internationalization (i18n)
Multi-language support starting with English and Norwegian.

#### User Stories
- **US5.1:** As any user, I want to switch the interface language so that I can use the system in my preferred language.
- **US5.2:** As Ragnhild, I want ontology terms available in Norwegian so that I can communicate using local terminology.

#### Acceptance Criteria
- Language switcher in UI
- All UI strings externalized
- Initial languages: English (en), Norwegian Bokmål (nb)
- Ontology terms with multi-language labels

---

## Non-Functional Requirements

### Performance
- Page load < 2 seconds
- Ontology graph renders < 1 second for up to 500 nodes
- API response time < 200ms for standard queries

### Security
- HTTPS only
- Input validation and SQL injection prevention
- XSS protection
- CSRF protection
- Rate limiting

### Accessibility
- WCAG 2.1 AA compliance
- Keyboard navigation
- Screen reader support

### Data
- SQLite database with daily backups
- Data export capability
- GDPR compliance for user data

---

## Technical Constraints

- **Database:** SQLite3 (single-file, easy deployment for government environments)
- **Backend:** Rust with Axum framework
- **Frontend:** Vite + React + TanStack Router + shadcn/ui
- **Deployment:** Single binary + static files (portable)
- **Authentication:** Abstracted for future SSO integration

---

## Out of Scope (v1.0)

- Risk assessment workflow (guided risk identification/analysis)
- Treatment planning and tracking
- Integration with external GRC tools
- Real-time collaboration features
- Mobile application
- Offline mode

---

## Success Metrics

1. **Adoption:** 80% of target users actively using the system within 3 months
2. **Compliance Tracking:** At least one complete compliance assessment per department
3. **Knowledge Access:** Average 5+ ontology searches per user per week
4. **Report Generation:** Monthly executive reports generated for all departments
