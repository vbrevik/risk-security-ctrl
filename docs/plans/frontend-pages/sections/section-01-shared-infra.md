Now I have all the context needed. Let me produce the section content.

# Section 01: Shared Infrastructure

## Overview

This section sets up all shared types, API hooks, utility functions, URL param utilities, and test infrastructure required by the four new pages (Framework Catalog, Crosswalk Explorer, Regulatory Landscape, Unified Search). Everything in this section is a foundation dependency -- all other sections (02 through 06) depend on it.

No new backend endpoints are needed. All new functionality derives from existing API responses via client-side composition.

## Prerequisites

- Working frontend dev environment (`pnpm dev` runs successfully)
- Existing codebase with TanStack Query hooks in `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/index.ts`
- Existing types in `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts`

## Step 1: Install Test Dependencies

The project has Vitest 4.0 but no React Testing Library packages. Install them as dev dependencies.

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/frontend
pnpm add -D @testing-library/react @testing-library/jest-dom @testing-library/user-event jsdom
```

Vitest needs a browser-like environment for component tests. Since there is no `vitest.config.ts`, add one at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/vitest.config.ts` that extends the existing vite config:

- Set `test.environment` to `"jsdom"`
- Set `test.globals` to `true` (so `describe`/`it`/`expect` are available without imports)
- Set `test.setupFiles` pointing to a setup file
- Configure the `@` path alias to match the vite config

Create a test setup file at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/test-setup.ts` that imports `@testing-library/jest-dom/vitest` to add custom matchers (like `toBeInTheDocument`).

## Step 2: New Types

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts`

Append three new interfaces after the existing type definitions:

### `FrameworkStats`

```typescript
export interface FrameworkStats {
  conceptCount: number;
  conceptTypes: Record<string, number>;
  connectedFrameworks: number;
  relationshipCount: number;
}
```

Used by the Framework Catalog detail panel and potentially the home page. Holds per-framework aggregate statistics derived from concepts and relationships data.

### `CrosswalkCell`

```typescript
export interface CrosswalkCell {
  sourceFrameworkId: string;
  targetFrameworkId: string;
  count: number;
  relationships: Relationship[];
}
```

Represents one cell in the 22x22 crosswalk matrix. Used by the Crosswalk Explorer page.

### `LandscapeProfile`

```typescript
export interface LandscapeProfile {
  sector: string;
  activities: string[];
  applicableFrameworks: string[];
}
```

Represents a user's selected regulatory profile. Used by the Regulatory Landscape page.

## Step 3: Framework Domain Grouping Utility

**File to create:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/frameworkDomains.ts`

Export a function `groupFrameworksByDomain` that takes an array of `Framework` objects and returns an array of domain groups. Each group has a `label` string and a `frameworkIds` string array.

Signature:

```typescript
export function groupFrameworksByDomain(
  frameworks: Framework[]
): { label: string; frameworkIds: string[] }[]
```

The function uses a hardcoded domain-to-framework-ID mapping:

| Domain Label | Framework IDs |
|---|---|
| Risk & Security Standards | iso31000, iso31010, iso27000, iso9000, nist-csf, nist-800-53, nist-rmf |
| AI Governance | eu-ai-act, nist-ai-rmf, iso42001, iso42005, iso23894, google-saif, mitre-atlas |
| EU Regulations | gdpr, nis2, dora, cer-directive |
| Architecture & Models | zero-trust, cisa-ztmm, data-centric, fmn |

Behavior:
- Only include framework IDs that exist in the input `frameworks` array (filter against actual data)
- Return all 4 groups always, even if some are empty (when frameworks array is empty, all groups have empty `frameworkIds`)
- Framework IDs not in the hardcoded mapping should be excluded (no fallback/"Other" group)
- Every one of the 22 known frameworks is assigned to exactly one group

**Update the barrel export** at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/index.ts` to add:

```typescript
export * from "./frameworkDomains";
```

## Step 4: Landscape Mapping Utility

**File to create:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/landscapeMapping.ts`

Export a function `getApplicableFrameworks` that takes a sector string and activities string array and returns a deduplicated array of applicable framework IDs.

Signature:

```typescript
export function getApplicableFrameworks(
  sector: string,
  activities: string[]
): string[]
```

Logic:

1. Start with universal frameworks: `["iso31000", "iso31010", "iso9000"]`
2. Add sector-specific base frameworks:

| Sector | Base Frameworks |
|---|---|
| Financial | dora, nis2, iso27000, gdpr |
| Healthcare | nis2, gdpr, iso27000 |
| Critical Infrastructure | nis2, cer-directive, iso27000, nist-csf |
| Government/Public Admin | nis2, gdpr, iso27000 |
| Technology/AI Provider | gdpr, iso27000 |
| General Enterprise | iso27000, gdpr |

3. Add activity-specific additional frameworks:

| Activity | Additional Frameworks |
|---|---|
| Processing personal data | gdpr |
| Deploying AI systems | eu-ai-act, nist-ai-rmf, iso42001, iso23894 |
| Operating critical infrastructure | cer-directive, nist-csf |
| Financial services | dora |
| Defense/NATO context | fmn, zero-trust, cisa-ztmm |

4. Deduplicate (use a `Set`) and return as array
5. If sector is empty/unknown and no activities selected, return only universal frameworks

**Update the barrel export** at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/index.ts` to add:

```typescript
export * from "./landscapeMapping";
```

## Step 5: URL Param Parsing Utility

**File to create:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/urlParams.ts`

Export a function for parsing comma-separated URL parameter strings into clean arrays. This is used by the Crosswalk, Landscape, and Search pages for multi-value URL params.

Signature:

```typescript
export function parseCommaSeparated(value: string | undefined): string[]
```

Behavior:
- `"a,b,c"` returns `["a", "b", "c"]`
- `""` returns `[]` (not `[""]`)
- `"a,,b"` returns `["a", "b"]` (no empty strings)
- `undefined` returns `[]`

Implementation: split on comma, then `filter(Boolean)`. Handle the `undefined` case by returning `[]` immediately.

**Update the barrel export** at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/index.ts` to add:

```typescript
export * from "./urlParams";
```

## Step 6: New API Hooks

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/index.ts`

Add two new hooks after the existing exports. Both compose existing hooks.

### `useAllConcepts`

```typescript
export function useAllConcepts(): {
  data: Concept[];
  conceptToFramework: Map<string, string>;
  isLoading: boolean;
  errors: Error[];
}
```

Implementation approach:
- Call `useFrameworks()` to get the list of all framework IDs
- Use `useQueries` (from `@tanstack/react-query`) to fire one `useConcepts`-equivalent query per framework, all in parallel (limit=500 per request)
- Combine results from all successful queries into a single flat `Concept[]` array
- Build a `Map<conceptId, frameworkId>` from the combined results using `useMemo`
- Track errors: collect `.error` from each query result into an `errors` array
- `isLoading` is `true` while any query is still pending (`some(q => q.isPending)`)
- Return partial results immediately from successful queries -- do not block on failures
- Failed queries auto-retry per TanStack Query defaults (3 retries with exponential backoff)

Import `useQueries` from `@tanstack/react-query` (add to existing import).

### `useFrameworkStats`

```typescript
export function useFrameworkStats(): {
  data: Map<string, FrameworkStats>;
  isLoading: boolean;
}
```

Implementation approach:
- Call `useFrameworks()`, `useAllConcepts()`, and `useRelationships()`
- Derive per-framework statistics in `useMemo`:
  - `conceptCount`: count concepts with matching `framework_id`
  - `conceptTypes`: group concepts by `concept_type`, count each
  - `connectedFrameworks`: count distinct other framework IDs connected via relationships (requires the `conceptToFramework` map from `useAllConcepts`)
  - `relationshipCount`: count relationships where source or target concept belongs to this framework
- `isLoading` is `true` while any underlying hook is loading
- Return a `Map<frameworkId, FrameworkStats>`

Import the `FrameworkStats` type from `../types`.

## Step 7: Tests

All test files go in `__tests__/` directories colocated with the source files.

### Test file: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts`

Tests for `groupFrameworksByDomain`:

- **Returns 4 groups with correct labels**: Call with all 22 frameworks, verify 4 groups returned with labels "Risk & Security Standards", "AI Governance", "EU Regulations", "Architecture & Models".
- **Each group contains expected framework IDs**: For each domain, verify the exact set of framework IDs matches the hardcoded mapping.
- **All 22 frameworks assigned to exactly one group**: Flatten all group `frameworkIds`, verify length is 22 and no duplicates.
- **Handles empty framework array**: Pass `[]`, verify all 4 groups returned with empty `frameworkIds` arrays.
- **Unknown framework IDs excluded**: Pass frameworks with an ID not in the mapping, verify it does not appear in any group.

Create mock `Framework` objects with just the `id` field populated (other fields can be stubs).

### Test file: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts`

Tests for `getApplicableFrameworks`:

- **Each sector returns base frameworks**: Test "Financial" returns at least dora, nis2, iso27000, gdpr plus universals.
- **Each activity adds correct frameworks**: Test "Deploying AI systems" adds eu-ai-act, nist-ai-rmf, iso42001, iso23894.
- **Combined sector + activities produces no duplicates**: Test "Financial" + "Financial services" does not duplicate dora.
- **Universal frameworks always included**: For any input, iso31000, iso31010, iso9000 are present.
- **Empty sector + no activities returns only universals**: Pass empty string and `[]`, verify result is exactly `["iso31000", "iso31010", "iso9000"]`.

### Test file: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/urlParams.test.ts`

Tests for `parseCommaSeparated`:

- `"a,b,c"` returns `["a", "b", "c"]`
- `""` returns `[]` (not `[""]`)
- `"a,,b"` returns `["a", "b"]` (no empty strings)
- `undefined` returns `[]`

### Test file: `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/__tests__/hooks.test.ts`

Tests for `useAllConcepts` and `useFrameworkStats`. These are hook tests that require a wrapper with `QueryClientProvider`. Use `renderHook` from `@testing-library/react`.

**`useAllConcepts` tests:**

- **Returns combined concepts from multiple frameworks**: Mock API to return concepts for 2 frameworks, verify combined array contains all concepts.
- **Returns errors array when some queries fail**: Mock one framework's API call to fail, verify `errors` array has one entry and `data` still contains the successful framework's concepts.
- **isLoading is true while any query is pending**: Verify loading state before queries resolve.
- **Builds correct concept-to-framework Map**: Verify the `conceptToFramework` map correctly maps concept IDs to their framework IDs.

**`useFrameworkStats` tests:**

- **Returns correct conceptCount per framework**: Provide mock data, verify counts match.
- **Returns correct conceptTypes breakdown**: Verify per-type counts.
- **Returns correct connectedFrameworks count**: Verify count of distinct connected frameworks.
- **Returns correct relationshipCount**: Verify relationship count per framework.

For hook tests, mock the `api` module (the axios instance at `@/lib/api`) using `vi.mock`. Wrap `renderHook` calls in a `QueryClientProvider` with a fresh `QueryClient` per test (with `retry: false` to speed up tests).

## File Summary

Files to create:
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/vitest.config.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/test-setup.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/frameworkDomains.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/landscapeMapping.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/urlParams.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/__tests__/urlParams.test.ts`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/__tests__/hooks.test.ts`

Files to modify:
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/types/index.ts` (append 3 interfaces)
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/index.ts` (add 2 hooks, update imports)
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/utils/index.ts` (add 3 barrel exports)
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/package.json` (new devDependencies after install)

## Dependencies

This section has no dependencies on other sections. All subsequent sections (02 through 06) depend on this section being complete.

---

## Implementation Notes (Post-Build)

### Deviations from Plan

1. **Pagination support in useAllConcepts:** The plan specified `limit=500` per request. Code review identified silent data loss for frameworks with >500 concepts. Implemented `fetchAllConceptsForFramework()` helper that loops through all pages.

2. **useMemo stability fix:** `useQueries` returns a new array reference on every render. Used `queries.map(q => q.dataUpdatedAt).join(",")` as a stable dependency key for `useMemo` on data and errors.

3. **errors array wrapped in useMemo:** Originally computed inline (new ref each render). Wrapped in `useMemo` for stable identity.

4. **parseCommaSeparated trim:** Added `.map(s => s.trim())` for defensive whitespace handling.

5. **Vitest config standalone:** Plan said to extend vite config. Implemented standalone `vitest.config.ts` with manual `@` alias — simpler and sufficient.

### Test Summary

- 18 tests across 4 test files, all passing
- 3 utility test files (14 pure function tests)
- 1 hook test file (4 async hook tests with mocked API)

### Actual Files Created/Modified

All files match the plan's File Summary, with no additional files created.