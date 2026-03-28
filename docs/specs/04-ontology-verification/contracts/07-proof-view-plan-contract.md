---
contract: plan
split: 07-proof-view-feature
created: 2026-03-28
---

# Prompt Contract — Split 07 Plan

## GOAL

Produce a `claude-plan.md` with concrete implementation sections covering the **frontend-only** portion of the proof view feature. The backend is already complete (migration 005, import pipeline, `GET /api/ontology/frameworks/{id}/proof` endpoint). Success = a user can click a framework in the sidebar, see its verification status badge in the profile panel, click a "View Proof" control, and read rendered markdown proof content with source links. `pnpm build` passes with no TypeScript errors.

## CONTEXT

Backend deliverables already merged: verification columns on `frameworks`, proof endpoint returns `{ framework_id, verification_status, verification_date, verification_source, verification_notes, proof_content }`. Frontend patterns: TanStack Query hooks in `features/ontology/api/index.ts`, components co-located in `features/ontology/components/`. The `FrameworkProfile` component renders the main framework detail panel. Proof view should integrate as a panel/section within `FrameworkProfile`, not a separate full-page route (backlog note: "panel in detail view"). Verification status values: `verified`, `partially-verified`, `structure-verified`, `corrected`, `unverified`, `needs-correction`.

## CONSTRAINTS

Always:
- Follow existing TanStack Query hook patterns in `api/index.ts`
- Co-locate new components in `features/ontology/components/`
- Use existing shadcn/ui Badge, Card, Tabs components where applicable
- Sanitize rendered markdown — use react-markdown with rehype-sanitize (already a common dep pattern)
- i18next for any user-facing strings

Ask first:
- Adding new npm dependencies
- Creating a full-page route instead of panel integration

Never:
- Accept proof file paths from the client — backend derives path from framework ID
- Render markdown with unsanitized HTML injection
- Add features not listed in the GOAL

## STIG Constraints

- V-222607 (CAT II): API error responses must not reveal internal file paths
- V-222604 (CAT II): Never expose internal proof file path to frontend; backend derives it server-side
- V-222571 (CAT II): Sanitize markdown content before rendering to prevent XSS — use rehype-sanitize with react-markdown

## FORMAT

Sections should cover:
1. `useFrameworkProof` hook in `features/ontology/api/index.ts`
2. `VerificationBadge` component in `features/ontology/components/`
3. `ProofPanel` component (rendered markdown + metadata) in `features/ontology/components/`
4. Integration into `FrameworkProfile` (tab or collapsible section)
5. Type additions to `features/ontology/types.ts`

## FAILURE CONDITIONS

- SHALL NOT create a full-page route when a panel suffices
- SHALL NOT render user-supplied markdown without sanitization (use rehype-sanitize)
- SHALL NOT skip loading and error states in the hook and components
- SHALL NOT hardcode verification status strings (use a type or enum)
- SHALL NOT add features beyond status badge, proof metadata display, and rendered markdown
- SHALL NOT skip TypeScript types on all new function parameters and return values
- SHALL NOT break existing `FrameworkProfile` tests
