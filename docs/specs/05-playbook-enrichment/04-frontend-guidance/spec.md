# 04-frontend-guidance: Display Concept Guidance in Frontend

## Summary

Add guidance display to the concept detail view in the frontend. Shows playbook-sourced about sections, suggested actions (as a checklist-style list), transparency questions, and references — with source PDF traceability badges. Gracefully handles concepts without guidance data.

## Requirements Source

- Parent requirements: `docs/specs/05-playbook-enrichment/requirements.md`
- Interview: `docs/specs/05-playbook-enrichment/deep_project_interview.md`
- Manifest: `docs/specs/05-playbook-enrichment/project-manifest.md`

## What to Build

### API Types

Add TypeScript types matching the API response from split 03:

```typescript
interface ConceptGuidance {
  source_pdf: string;
  source_page: number;
  about_en: string | null;
  suggested_actions: Array<{ sort_order: number; text_en: string }>;
  transparency_questions: Array<{ sort_order: number; text_en: string }>;
  references: Array<{
    type: "academic" | "transparency_resource";
    title: string;
    authors: string | null;
    year: number | null;
    venue: string | null;
    url: string | null;
  }>;
}
```

### TanStack Query Hook

Update or extend the existing concept detail query hook to include guidance data. The API response shape changes (adds optional `guidance` field) so the existing hook may just need its type updated.

### ConceptGuidance Component

New component `frontend/src/features/ontology/components/ConceptGuidance.tsx` with collapsible sections:

#### Source Badge
- Shows "Source: AI_RMF_Playbook.pdf p.98" as a small badge/chip
- Communicates traceability — users can verify against the PDF

#### About Section
- Renders the `about_en` text as paragraphs
- Collapsible, expanded by default

#### Suggested Actions
- Ordered list of action items
- Visually distinct from the about text (e.g., numbered list or checklist style)
- These are informational, not interactive checkboxes (compliance tracking is in the assessment flow)
- Consider connecting to compliance items in the future (note in UI: "Track these actions in an assessment")

#### Transparency & Documentation Questions
- Bulleted list of questions
- Collapsible section, collapsed by default (secondary importance)

#### References
- Two groups: "AI Transparency Resources" (type=transparency_resource) and "Academic References" (type=academic)
- Display: title, author (if available), year (if available)
- Link to URL if available
- Collapsible, collapsed by default

### Integration Point

The `ConceptGuidance` component renders inside the existing concept detail view/page. It should appear below the definition and above or alongside the relationships section. If `guidance` is null (concept has no guidance data), the component renders nothing.

### Internationalization

- Add i18n keys for section headings and labels:
  - `ontology:guidance.source` → "Source"
  - `ontology:guidance.about` → "About"
  - `ontology:guidance.suggestedActions` → "Suggested Actions"
  - `ontology:guidance.transparencyQuestions` → "Transparency & Documentation"
  - `ontology:guidance.references` → "References"
  - `ontology:guidance.academicReferences` → "Academic References"
  - `ontology:guidance.transparencyResources` → "AI Transparency Resources"
- English values populated; Norwegian keys added with English fallback (translation deferred)

### Edge Cases

- Concept has no guidance → component renders nothing, no empty state
- Guidance has about_en but no actions → show about, hide actions section
- Reference has no URL → render title without link
- Reference has no year/authors → render what's available

## Key Decisions

- **Frontend included in this project** (interview decision)
- **Collapsible sections**: About expanded by default, others collapsed — avoids overwhelming on load
- **Source badge**: Every guidance block shows its PDF source for trust/traceability
- **No interactive checkboxes**: Suggested actions are informational; compliance tracking is separate

## Dependencies

- **Needs from 03**: API returning `guidance` object in `GET /api/concepts/:id`
- **Provides**: User-visible enriched concept detail view

## Existing Code Reference

- Frontend routing: `frontend/src/routes/ontology/` (file-based TanStack Router)
- Feature components: `frontend/src/features/ontology/components/`
- API hooks: `frontend/src/features/ontology/api/`
- i18n: `frontend/src/i18n/locales/{en,nb}/ontology.json`
- UI components: shadcn/ui (Collapsible, Badge, Card, etc.)
