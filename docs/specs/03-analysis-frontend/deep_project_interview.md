# Deep Project Interview: Analysis Frontend

## Context
Building a frontend feature module for the document analysis system. The backend API is fully implemented with endpoints for CRUD, file upload, findings, export (PDF/DOCX), and prompt template configuration.

The existing frontend has React 19 + TanStack Router/Query + shadcn/ui + Tailwind + i18n (en/nb). The compliance feature serves as the closest pattern reference.

## Q1: Create analysis flow
**Q:** How should creating a new analysis work?
**A:** Dedicated page (`/analysis/create`) — more space for the upload area.

## Q2: Findings depth
**Q:** How much depth for viewing findings on the detail page?
**A:** Table with expand — sortable/filterable table, click row to expand inline for evidence, recommendation, concept definition.

## Q3: Charts
**Q:** Should the detail page render charts (heatmap, radar, priority)?
**A:** Charts + tables — render coverage heatmap and priority breakdown as interactive frontend charts, plus the data tables and stats.

## Q4: Prompt template UI
**Q:** Expose MatcherConfig management in the frontend?
**A:** Yes, include a settings page (`/analysis/settings`) with a form to edit thresholds, boost terms, etc.

## Summary of Decisions
- **Create flow:** Dedicated page at `/analysis/create` with text input and file upload modes
- **Detail page:** Summary stats cards + interactive charts (coverage heatmap, priority bars) + expandable findings table with framework/type/priority filters
- **Export:** PDF and DOCX download buttons on the detail page
- **Settings:** Dedicated settings page for MatcherConfig editing
- **Patterns:** Follow compliance feature structure, use TanStack Query, shadcn/ui, i18n
