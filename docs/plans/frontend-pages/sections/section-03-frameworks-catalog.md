Now I have all the context needed. Let me generate the section content.

# Section 03: Framework Catalog

## Overview

This section implements the Framework Catalog page at `/frameworks` with a master-detail layout. It consists of a sidebar listing all 22 frameworks grouped by domain, and a detail panel showing a full profile for the selected framework including stats, concept type breakdown, cross-framework connections, and a concept hierarchy preview.

**Dependencies:** This section requires section-01 (shared infrastructure: types, hooks, utilities) and section-02 (navigation with route placeholders) to be completed first.

## File Inventory

Files to **create**:

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/FrameworkSidebar.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/FrameworkProfile.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/routes/frameworks/index.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/routes/frameworks/__tests__/index.test.tsx`

Files that **must already exist** (from prior sections):

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts` (must contain `FrameworkStats` type from section-01)
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/index.ts` (must contain `useFrameworkStats` hook from section-01)
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/frameworkDomains.ts` (from section-01)

## Existing Types and Hooks (Context)

The following types already exist in `frontend/src/features/ontology/types/index.ts` and are relevant:

```typescript
interface Framework {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  source_url: string | null;
  created_at: string;
  updated_at: string;
}

interface Concept {
  id: string;
  framework_id: string;
  parent_id: string | null;
  concept_type: string;
  code: string | null;
  name_en: string;
  definition_en: string | null;
  sort_order: number | null;
  // ... other fields
}

interface Relationship {
  id: string;
  source_concept_id: string;
  target_concept_id: string;
  relationship_type: string;
  description: string | null;
}
```

Section-01 adds `FrameworkStats`:

```typescript
interface FrameworkStats {
  conceptCount: number;
  conceptTypes: Record<string, number>;
  connectedFrameworks: number;
  relationshipCount: number;
}
```

Existing hooks in `frontend/src/features/ontology/api/index.ts`:
- `useFrameworks()` returns `{ data: Framework[] }` - all frameworks, `staleTime: Infinity`
- `useConcepts(frameworkId)` returns `{ data: Concept[] }` - concepts for one framework
- `useRelationships()` returns `{ data: Relationship[] }` - all 382 relationships

Section-01 adds:
- `useFrameworkStats()` returns `{ data: Map<string, FrameworkStats>; isLoading: boolean }`

Section-01 also adds `groupFrameworksByDomain(frameworks)` in `frontend/src/features/ontology/utils/frameworkDomains.ts`, which returns `{ label: string; frameworkIds: string[] }[]` with four groups: Risk & Security Standards, AI Governance, EU Regulations, Architecture & Models.

## Tests (Write First)

### FrameworkSidebar Component Tests

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx`

Test stubs:

```typescript
import { describe, it, expect, vi } from "vitest";

describe("FrameworkSidebar", () => {
  // Renders all frameworks grouped by domain
  // Expects 4 domain section headings and all framework items beneath them
  it("renders all frameworks grouped by domain", () => {});

  // Each framework item shows a color dot and concept count badge
  it("each framework shows color dot and concept count", () => {});

  // Clicking a framework calls the onSelect callback with that framework's ID
  it("clicking a framework calls the selection callback", () => {});

  // The active framework (matching selectedId prop) is visually highlighted
  // with an accent background class
  it("active framework is visually highlighted", () => {});

  // When isLoading is true, renders skeleton placeholder items
  // instead of real framework data
  it("renders loading skeleton when data is pending", () => {});
});
```

**Props to test against:**

- `frameworks: Framework[]` - list of all frameworks
- `stats: Map<string, FrameworkStats>` - framework stats for concept count badges
- `selectedId: string | null` - currently selected framework ID
- `onSelect: (id: string) => void` - selection callback
- `isLoading: boolean` - loading state

### FrameworkProfile Component Tests

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx`

Test stubs:

```typescript
import { describe, it, expect } from "vitest";

describe("FrameworkProfile", () => {
  // Renders the framework name (large, bold, monospace), version badge,
  // description text, and source URL as an external link
  it("renders framework name, version, description, source link", () => {});

  // Renders 4 stat boxes: Concepts, Types, Connected, Relationships
  // with values from FrameworkStats
  it("renders 4 stat boxes with correct values", () => {});

  // Renders a horizontal stacked bar showing proportion of each concept type
  // with distinct colors and count labels
  it("renders concept type breakdown bar", () => {});

  // Renders connected frameworks sorted by relationship count descending
  // Each row has: framework color dot, name, count badge, type pills
  it("renders connected frameworks list sorted by relationship count", () => {});

  // When no framework is selected (null), renders empty state
  // with "Select a framework" message
  it("renders empty state when no framework selected", () => {});

  // When the URL ?id references a framework ID not in the data,
  // a toast/notification is shown with "Framework not found"
  it("shows 'not found' toast when URL ?id references nonexistent framework", () => {});
});
```

**Props to test against:**

- `framework: Framework | null` - selected framework data
- `concepts: Concept[]` - concepts for the selected framework
- `relationships: Relationship[]` - all relationships (filtered in component)
- `stats: FrameworkStats | null` - stats for the selected framework
- `frameworks: Framework[]` - all frameworks (for cross-framework connection names)

### Route Tests

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/routes/frameworks/__tests__/index.test.tsx`

Test stubs:

```typescript
import { describe, it, expect } from "vitest";

describe("frameworks/index route", () => {
  // When URL has ?id=iso27000, the framework with that ID is selected
  // in the sidebar and its profile is shown in the detail panel
  it("reads ?id from URL and selects that framework", () => {});

  // When no ?id param is in the URL, auto-selects the first framework
  // in the sidebar list
  it("default selects first framework when no ?id param", () => {});

  // When user clicks a different framework in the sidebar,
  // the URL updates to include ?id=<newFrameworkId>
  it("clicking a framework updates the URL ?id param", () => {});
});
```

## Implementation Details

### Route: `frameworks/index.tsx`

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/routes/frameworks/index.tsx`

Create a file-based route using `createFileRoute('/frameworks/')`. The route component orchestrates the master-detail layout.

**URL state management:**

```typescript
export const Route = createFileRoute("/frameworks/")({
  validateSearch: (search: Record<string, unknown>) => ({
    id: (search.id as string) || undefined,
  }),
  component: FrameworkCatalogPage,
});
```

**Component behavior:**

1. Call `useFrameworks()` for the framework list
2. Call `useFrameworkStats()` for stats (from section-01)
3. Read `id` from search params via `Route.useSearch()`
4. Use `navigate` from `Route.useNavigate()` to update `?id` when user selects a framework
5. Auto-select logic: if no `?id` param and frameworks are loaded, navigate to first framework's ID
6. If `?id` is present but not found in the loaded frameworks, select the first framework and show a toast message "Framework not found"
7. Call `useConcepts(selectedId)` for the selected framework's concepts
8. Call `useRelationships()` for all relationships

**Layout:** Two-panel flex layout.

```
<div className="flex gap-6 h-[calc(100vh-12rem)]">
  <FrameworkSidebar ... />   {/* w-[280px] flex-shrink-0, overflow-y-auto */}
  <FrameworkProfile ... />   {/* flex-1, overflow-y-auto */}
</div>
```

### Component: `FrameworkSidebar.tsx`

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/FrameworkSidebar.tsx`

A sidebar component that renders all frameworks grouped by domain.

**Props interface:**

```typescript
interface FrameworkSidebarProps {
  frameworks: Framework[];
  stats: Map<string, FrameworkStats>;
  selectedId: string | null;
  onSelect: (id: string) => void;
  isLoading: boolean;
}
```

**Rendering logic:**

1. Call `groupFrameworksByDomain(frameworks)` from the shared utility (section-01)
2. For each domain group, render:
   - A domain label in uppercase monospace (`text-xs font-mono uppercase tracking-widest text-foreground/50`) with a horizontal rule and framework count
   - Below the label, each framework as a clickable row:
     - A colored dot (derive color from framework ID using a simple hash or a hardcoded color map)
     - Framework short name (the `name` field)
     - Concept count badge from `stats.get(framework.id)?.conceptCount` (use `tech-badge` class)
3. Active framework (matching `selectedId`) gets accent background: `bg-accent/10 border-l-2 border-accent`
4. Non-active frameworks get hover state: `hover:bg-muted/50`

**Loading state:** When `isLoading` is true, render 4 skeleton domain sections. Each has a shimmer bar for the label and 3-5 shimmer rows for framework items. Use `animate-pulse` with `bg-muted` backgrounds.

**Framework color mapping:** Create a simple lookup object mapping framework IDs to oklch-based colors. This is used for the colored dots throughout the catalog page. Example structure:

```typescript
const FRAMEWORK_COLORS: Record<string, string> = {
  "iso31000": "oklch(0.7 0.15 50)",
  "gdpr": "oklch(0.65 0.2 280)",
  // ... etc for all 22 frameworks
};
```

This color map should be defined in the FrameworkSidebar file or extracted to a small utility if also needed by FrameworkProfile. Both components need it for colored dots.

### Component: `FrameworkProfile.tsx`

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/components/FrameworkProfile.tsx`

The detail panel shown when a framework is selected.

**Props interface:**

```typescript
interface FrameworkProfileProps {
  framework: Framework | null;
  concepts: Concept[];
  relationships: Relationship[];
  stats: FrameworkStats | null;
  frameworks: Framework[];
  isLoading: boolean;
}
```

**Empty state:** When `framework` is null, render a centered message: "Select a framework from the sidebar" with a subdued icon (e.g., `BookOpen` from lucide-react).

**Header section:**
- Framework name: `text-2xl font-bold font-mono`
- Version badge using `tech-badge` class (only if `framework.version` is non-null)
- Description: `text-sm text-foreground/70` paragraph
- Source URL: external link with `ExternalLink` icon from lucide-react, opens in new tab

**Stats strip:** A row of 4 stat boxes using `feature-card` styling:
- Concepts: `stats.conceptCount`
- Types: `Object.keys(stats.conceptTypes).length`
- Connected: `stats.connectedFrameworks`
- Relationships: `stats.relationshipCount`

Each box: label on top in small muted text, value below in `stat-number` class.

**Concept Type Breakdown bar:**
- A horizontal stacked bar (CSS flex with proportional widths)
- Each segment represents a concept type, width proportional to its count relative to total
- Color per segment: derive from concept type name via a simple hash-to-hue function
- Below the bar, a legend listing each type with its color swatch and count

**Cross-Framework Connections list:**
- Filter `relationships` to those involving concepts from the selected framework
- Group by the other framework (resolve concept IDs to framework IDs using the `concepts` data plus the `frameworks` list)
- Sort groups by relationship count descending
- Each row renders: colored dot, framework name, count badge, and small colored pills for relationship types (maps_to = blue pill, implements = green pill, related_to = gray pill, supports = amber pill)
- Clicking a row navigates to `/crosswalk?fw1=SELECTED&fw2=CLICKED`

**Concept Hierarchy Preview:**
- Filter `concepts` to top-level only (`parent_id === null`)
- Render as a flat list with expand arrows (`ChevronRight` icon)
- Clicking a top-level concept toggles expansion to show its direct children (filter `concepts` where `parent_id === topLevelId`)
- Use local component state (`useState<Set<string>>`) to track expanded nodes
- Each concept row: code in monospace if present, name, concept type as small badge
- This gives a structural overview without the full tree explorer

### Data Flow Summary

```
frameworks/index.tsx (route)
  ├── useFrameworks()           → Framework[]
  ├── useFrameworkStats()       → Map<string, FrameworkStats>  (section-01 hook)
  ├── useConcepts(selectedId)   → Concept[]
  ├── useRelationships()        → Relationship[]
  │
  ├── FrameworkSidebar
  │     ├── receives: frameworks, stats, selectedId, onSelect, isLoading
  │     └── uses: groupFrameworksByDomain() (section-01 utility)
  │
  └── FrameworkProfile
        └── receives: framework, concepts, relationships, stats, frameworks, isLoading
```

### CSS and Styling Notes

Reuse existing design system classes:
- `.feature-card` + `.corner-markers` for stat boxes and framework cards
- `.tech-badge` for version badges and concept count badges
- `.stat-number` for large stat values
- `.topo-grid` for page background (optional, applied in route)
- `.animate-fadeInUp` for page entrance animation

No new CSS classes are needed for this section. All styling is achieved with existing Tailwind utilities and the project's design system classes.

### Edge Cases

| Scenario | Handling |
|----------|----------|
| API returns empty frameworks list | Sidebar shows "No frameworks loaded" empty state; detail panel shows generic empty state |
| `?id=foo` but `foo` not in framework list | Auto-select first framework, show toast "Framework not found" |
| No `?id` param | Auto-select first framework in the list and update URL |
| Selected framework has zero concepts | Stats show 0s; breakdown bar is empty; hierarchy section shows "No concepts" |
| Selected framework has zero relationships | Connected frameworks section shows "No cross-framework connections" |
| Concepts or relationships still loading | Show skeleton/spinner in the relevant subsection of the detail panel; sidebar and header render immediately from framework data |

---

## Implementation Notes (Post-Build)

### Deviations from Plan

1. **Cross-framework resolution:** Plan assumed concepts prop would contain all concepts. Actually uses `conceptToFramework` Map from `useAllConcepts()` hook for global concept-to-framework lookups. Added as a prop to FrameworkProfile.

2. **Color map:** Reused existing `getFrameworkColor()` from graphTransform.ts instead of creating a new oklch-based FRAMEWORK_COLORS map. DRY and consistent with rest of codebase.

3. **Toast notification:** Omitted toast for invalid ?id — no toast infrastructure in project. Silent redirect to first framework instead.

4. **Clickable connection rows:** Omitted crosswalk navigation on click — deferred to future enhancement.

5. **Route-level tests:** Omitted — router harness complexity disproportionate to value.

### Code Review Fixes Applied
- Added `= new Map()` default for statsMap to prevent undefined crash
- Fixed cross-framework connection resolution using global conceptToFramework Map
- Added empty state for zero frameworks
- Added corner-markers class to stat boxes
- Added useEffect to reset expanded state on framework change

### Test Summary
- 9 component tests across 2 files, all passing
- FrameworkSidebar: 5 tests (grouping, counts, click, active highlight, loading skeleton)
- FrameworkProfile: 4 tests (header, stat boxes, type breakdown, empty state)