Now I have all the context needed. Let me produce the section content.

# Section 06: Regulatory Landscape Page

## Overview

This section implements the Regulatory Landscape page at `/landscape`. The page lets users select their organization's sector and activities, then highlights which of the 22 frameworks apply to them, shows overlap between applicable frameworks, and provides a summary banner. It uses a two-column layout: a selector panel on the left and results on the right.

**Dependencies:** This section depends on section-01 (shared infrastructure: types, `useFrameworkStats` hook, `landscapeMapping.ts` utility, `frameworkDomains.ts` utility, `LandscapeProfile` type) and section-02 (navigation: route placeholder at `/landscape` already registered, two-tier nav with Landscape link). Those must be completed before this section.

## File Inventory

| File | Action | Purpose |
|------|--------|---------|
| `frontend/src/features/ontology/utils/landscapeMapping.ts` | Create (section-01) | Pure function mapping sector+activities to framework IDs |
| `frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts` | Create | Unit tests for applicability logic |
| `frontend/src/features/ontology/components/LandscapeSelector.tsx` | Create | Sector radio buttons + activity checkboxes panel |
| `frontend/src/features/ontology/components/__tests__/LandscapeSelector.test.tsx` | Create | Component tests for selector |
| `frontend/src/features/ontology/components/LandscapeResults.tsx` | Create | Applicable frameworks display with overlap indicators |
| `frontend/src/features/ontology/components/__tests__/LandscapeResults.test.tsx` | Create | Component tests for results |
| `frontend/src/routes/landscape/index.tsx` | Create/Update | Route component wiring selector, results, and URL state |
| `frontend/src/routes/landscape/__tests__/index.test.tsx` | Create | Route-level integration tests |

## Tests (Write First)

All tests use Vitest + @testing-library/react + @testing-library/jest-dom. Test files are colocated in `__tests__/` directories next to the code they test.

### 1. Applicability Logic Tests

**File:** `frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts`

These tests validate the pure function `getApplicableFrameworks(sector, activities)` from `landscapeMapping.ts`.

```typescript
import { describe, it, expect } from "vitest";
import { getApplicableFrameworks } from "../landscapeMapping";

describe("getApplicableFrameworks", () => {
  it("Financial sector returns dora, nis2, iso27000, gdpr plus universals", () => {
    /** result should contain dora, nis2, iso27000, gdpr, iso31000, iso31010, iso9000 */
  });

  it("Financial + Deploying AI adds eu-ai-act, nist-ai-rmf, iso42001, iso23894", () => {
    /** Combines sector base + activity additions, no duplicates */
  });

  it("Critical Infrastructure returns nis2, cer-directive, iso27000, nist-csf plus universals", () => {
    /** sector = 'critical-infrastructure' */
  });

  it("Defense/NATO activity adds fmn, zero-trust, cisa-ztmm", () => {
    /** Any sector + defense-nato activity */
  });

  it("Multiple activities combine without duplicates", () => {
    /** e.g., Financial + processing-personal-data + deploying-ai + financial-services */
  });

  it("Empty selections return only universal frameworks", () => {
    /** sector = '' or undefined, activities = [] => iso31000, iso31010, iso9000 */
  });
});
```

### 2. LandscapeSelector Component Tests

**File:** `frontend/src/features/ontology/components/__tests__/LandscapeSelector.test.tsx`

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { LandscapeSelector } from "../LandscapeSelector";

describe("LandscapeSelector", () => {
  it("renders sector radio buttons", () => {
    /** Expect 6 radio buttons: Financial, Healthcare, Critical Infrastructure,
     *  Government/Public Admin, Technology/AI Provider, General Enterprise */
  });

  it("renders activity checkboxes", () => {
    /** Expect 5 checkboxes: Processing personal data, Deploying AI systems,
     *  Operating critical infrastructure, Financial services, Defense/NATO context */
  });

  it("sector selection updates callback", () => {
    /** Click a sector radio, verify onSectorChange called with correct value */
  });

  it("multiple activities can be selected", () => {
    /** Click two activity checkboxes, verify onActivitiesChange called with both */
  });

  it("clear all button resets activities", () => {
    /** With activities selected, click clear all, verify onActivitiesChange([]) */
  });
});
```

### 3. LandscapeResults Component Tests

**File:** `frontend/src/features/ontology/components/__tests__/LandscapeResults.test.tsx`

```typescript
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { LandscapeResults } from "../LandscapeResults";

describe("LandscapeResults", () => {
  it("applicable frameworks shown with full styling", () => {
    /** Pass applicableIds + frameworks list, verify applicable ones are NOT faded */
  });

  it("non-applicable frameworks shown faded", () => {
    /** Frameworks not in applicableIds should have opacity/faded class */
  });

  it("overlap indicators show relationship counts between applicable frameworks", () => {
    /** Between two applicable frameworks, show "N shared requirements" badge */
  });

  it("summary banner shows correct counts", () => {
    /** "Based on your profile: N frameworks apply, covering M concepts with K cross-framework relationships" */
  });
});
```

### 4. Route Tests

**File:** `frontend/src/routes/landscape/__tests__/index.test.tsx`

```typescript
import { describe, it, expect } from "vitest";

describe("landscape route", () => {
  it("reads ?sector and ?activities from URL", () => {
    /** Navigate to /landscape?sector=financial&activities=deploying-ai,processing-personal-data
     *  Verify selector shows financial selected and two activities checked */
  });

  it("selecting sector updates URL", () => {
    /** Click a sector radio, verify URL ?sector param changes */
  });

  it("comma-separated activities parsed with filter(Boolean)", () => {
    /** /landscape?activities=a,,b => parsed as ['a','b'] not ['a','','b'] */
  });
});
```

## Implementation Details

### Applicability Logic (`landscapeMapping.ts`)

This file is created in section-01 as shared infrastructure but the full mapping logic is defined here. It is a pure function with no API calls or React dependencies.

**File:** `frontend/src/features/ontology/utils/landscapeMapping.ts`

The function signature:

```typescript
export function getApplicableFrameworks(
  sector: string | undefined,
  activities: string[]
): string[];
```

**Sector-to-frameworks mapping (hardcoded):**

| Sector Key | Base Frameworks |
|------------|----------------|
| `financial` | `dora`, `nis2`, `iso27000`, `gdpr` |
| `healthcare` | `nis2`, `gdpr`, `iso27000` |
| `critical-infrastructure` | `nis2`, `cer-directive`, `iso27000`, `nist-csf` |
| `government` | `nis2`, `gdpr`, `iso27000` |
| `technology-ai` | `gdpr`, `iso27000` |
| `general-enterprise` | `iso27000`, `gdpr` |

**Activity-to-additional-frameworks mapping (hardcoded):**

| Activity Key | Additional Frameworks |
|-------------|----------------------|
| `processing-personal-data` | `gdpr` |
| `deploying-ai` | `eu-ai-act`, `nist-ai-rmf`, `iso42001`, `iso23894` |
| `operating-critical-infrastructure` | `cer-directive`, `nist-csf` |
| `financial-services` | `dora` |
| `defense-nato` | `fmn`, `zero-trust`, `cisa-ztmm` |

**Universal frameworks** always included regardless of selection: `iso31000`, `iso31010`, `iso9000`.

The function collects frameworks from the sector base, adds frameworks from each selected activity, adds universals, then deduplicates using a `Set`. Returns a string array of framework IDs.

Also export the sector and activity definitions (label, key, description) as constants so the selector component can render them without duplicating strings:

```typescript
export const SECTORS: { key: string; label: string; description: string }[];
export const ACTIVITIES: { key: string; label: string }[];
export const UNIVERSAL_FRAMEWORKS: string[];
```

### LandscapeSelector Component

**File:** `frontend/src/features/ontology/components/LandscapeSelector.tsx`

A controlled component receiving props:

```typescript
interface LandscapeSelectorProps {
  sector: string | undefined;
  activities: string[];
  onSectorChange: (sector: string) => void;
  onActivitiesChange: (activities: string[]) => void;
}
```

**Layout and behavior:**

- Renders inside a 320px-wide panel (the parent route handles the two-column layout)
- **Sector selector:** Renders `SECTORS` as styled radio buttons. Each shows the label and a brief description tooltip. Single-select -- clicking one deselects the previous. Calls `onSectorChange` on click.
- **Activity selector:** Renders `ACTIVITIES` as checkboxes. Multiple can be active. Toggling a checkbox adds/removes the activity key from the array and calls `onActivitiesChange` with the new array.
- **Clear all button:** Visible when at least one activity is checked. Calls `onActivitiesChange([])`.
- The visualization auto-updates on every selection change (no separate "Show My Landscape" button needed beyond the auto-update, though the button can be included as a visual anchor).
- Uses shadcn/ui components where available (Button for clear all). Radio buttons and checkboxes can use native HTML inputs styled with Tailwind or shadcn RadioGroup/Checkbox.

### LandscapeResults Component

**File:** `frontend/src/features/ontology/components/LandscapeResults.tsx`

Props:

```typescript
interface LandscapeResultsProps {
  applicableFrameworkIds: string[];
  frameworks: Framework[];
  relationships: Relationship[];
  conceptCountMap: Map<string, number>; // frameworkId -> concept count
  conceptToFramework: Map<string, string>; // conceptId -> frameworkId
}
```

**Sections rendered:**

1. **Summary Banner** at top: "Based on your profile: N frameworks apply, covering M concepts with K cross-framework relationships" where:
   - N = `applicableFrameworkIds.length`
   - M = sum of concept counts for applicable frameworks
   - K = count of relationships where both source and target concepts belong to applicable frameworks (resolved via `conceptToFramework` map)

2. **Compliance Stack** -- ordered list of applicable frameworks:
   - Ordering: sector-specific frameworks first, then activity-specific additions, then universal frameworks (the order can mirror the order returned by `getApplicableFrameworks`)
   - Each framework rendered as a card (reuse `.feature-card` + `.corner-markers` CSS classes) showing:
     - Framework name (bold, monospace)
     - Concept count
     - Why it applies (derive from which sector/activity triggered it -- or simply show a generic label)
     - Count of connections to other applicable frameworks

3. **Overlap Indicators** between adjacent applicable framework cards:
   - A small badge between cards showing the relationship count between that pair
   - E.g., "5 shared requirements" between DORA and NIS2
   - Computed by filtering `relationships` where source concept belongs to framework A and target concept belongs to framework B (using `conceptToFramework` map)

4. **Non-applicable frameworks** shown below the stack as a faded list:
   - Frameworks not in `applicableFrameworkIds` rendered with `opacity-40` or similar faded styling
   - Simple list format (name only, no cards)

### Route Component

**File:** `frontend/src/routes/landscape/index.tsx`

This is the page route that wires everything together.

```typescript
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/landscape/")({
  validateSearch: (search: Record<string, unknown>) => ({
    sector: (search.sector as string) || undefined,
    activities: (search.activities as string) || undefined,
  }),
  component: LandscapePage,
});
```

**URL state:**
- `?sector=financial` -- the selected sector key
- `?activities=deploying-ai,defense-nato` -- comma-separated activity keys

**Parsing `activities` from URL:** Use `.split(',').filter(Boolean)` to handle empty strings from `""` or `"a,,b"` patterns. This is critical for avoiding `[""]` from an empty activities param.

**LandscapePage component behavior:**

1. Read `sector` and `activities` from `Route.useSearch()`
2. Parse activities string into array: `const activityList = activities?.split(',').filter(Boolean) ?? []`
3. Compute applicable framework IDs: `const applicableIds = getApplicableFrameworks(sector, activityList)`
4. Fetch data using existing hooks:
   - `useFrameworks()` for the full framework list
   - `useRelationships()` for overlap indicators
   - `useAllConcepts()` (from section-01) for concept counts and concept-to-framework map
5. Derive `conceptCountMap` and `conceptToFramework` map from `useAllConcepts` data in `useMemo`
6. Update URL on selector changes using `navigate({ search: { sector: newSector, activities: newActivities.join(',') || undefined } })`
7. Render two-column layout:
   - Left: `<LandscapeSelector>` with current sector/activities and change callbacks
   - Right: `<LandscapeResults>` with computed data

**Two-column layout styling:**
```
<div className="flex gap-6">
  <div className="w-80 shrink-0">
    <LandscapeSelector ... />
  </div>
  <div className="flex-1 min-w-0">
    <LandscapeResults ... />
  </div>
</div>
```

**Loading state:** While `useFrameworks` or `useAllConcepts` is loading, show skeleton placeholders in the results area. The selector can render immediately since it uses only hardcoded data.

**Edge case -- no selections:** When neither sector nor activities are selected, `getApplicableFrameworks` returns only universal frameworks (iso31000, iso31010, iso9000). The results area shows these three as applicable and the rest as faded. The summary banner reflects this minimal profile.

## Existing CSS Classes to Reuse

- `.feature-card` + `.corner-markers` for framework cards in the compliance stack
- `.tech-badge` for framework badges and "why it applies" labels
- `.stat-number` for counts in the summary banner
- `.topo-grid` + `.gradient-mesh` for page background
- `.animate-fadeInUp` for page load animations

No new CSS classes are required for this section.

## Data Flow Summary

```
URL params (?sector, ?activities)
    |
    v
getApplicableFrameworks(sector, activities) --> string[] of framework IDs
    |
    v
useFrameworks() ---------> Framework[] (all 22)
useAllConcepts() --------> Concept[] + Map<conceptId, frameworkId>
useRelationships() ------> Relationship[] (all 382)
    |
    v
LandscapeResults receives:
  - applicableFrameworkIds (from pure function)
  - frameworks (from API)
  - relationships (from API)
  - conceptCountMap (derived)
  - conceptToFramework (derived)
```

## Error Handling

| Scenario | Handling |
|----------|----------|
| No sector/activities selected | Show universal frameworks only, results area shows minimal profile |
| `useAllConcepts` partially fails | Show partial data with warning: "Some frameworks failed to load" (leverages the `errors` array from `useAllConcepts` hook built in section-01) |
| Unknown sector in URL | Treat as no sector selected (universal only) |
| Empty activities string in URL | `filter(Boolean)` produces `[]`, treated as no activities |