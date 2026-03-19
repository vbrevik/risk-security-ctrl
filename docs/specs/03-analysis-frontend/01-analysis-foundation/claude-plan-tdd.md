# TDD Plan: Analysis Frontend Foundation

Testing framework: **Vitest + React Testing Library** (existing project setup).

Patterns: Mock `@/lib/api` with `vi.mock`, wrap hooks in `QueryClientProvider` via `createWrapper()`, mock `react-i18next` for component tests.

---

## Section 1: Types and API Hooks

### Type tests (compile-time only)
- No runtime tests needed for pure type definitions — TypeScript compilation validates correctness

### Hook tests (`features/analysis/api/__tests__/hooks.test.ts`)

**useAnalyses:**
- Test: returns paginated list on successful fetch
- Test: passes status filter as query param
- Test: refetchInterval activates when response contains processing items
- Test: refetchInterval is false when no processing items

**useAnalysis:**
- Test: returns analysis data for valid id
- Test: parses matched_framework_ids from JSON string to array
- Test: disabled when id is empty

**useCreateAnalysis:**
- Test: posts to /api/analyses with request body
- Test: invalidates analysis cache on success

**useUploadAnalysis:**
- Test: posts FormData with file and name fields
- Test: returns progress percentage during upload
- Test: resets progress on settled (both success and error)
- Test: invalidates analysis cache on success

**useDeleteAnalysis:**
- Test: calls DELETE /api/analyses/{id}
- Test: invalidates analysis cache on success

**usePromptTemplate:**
- Test: fetches from /api/analyses/prompt-template
- Test: returns MatcherConfig-shaped data

**useUpdatePromptTemplate:**
- Test: puts to /api/analyses/prompt-template
- Test: invalidates prompt-template cache on success

---

## Section 2: i18n and Navigation

### i18n tests
- Test: analysis namespace is registered and loadable
- Test: key access returns string (not the key itself) for a sample of keys

### Navigation tests (`routes/__tests__/analysis-nav.test.tsx`)
- Test: Analysis link renders in the navigation
- Test: Analysis link points to /analysis

---

## Section 3: Route Files and List Page

### AnalysisList component tests (`features/analysis/components/__tests__/AnalysisList.test.tsx`)
- Test: renders loading skeleton while fetching
- Test: renders analysis cards when data loads
- Test: renders empty state when no analyses exist
- Test: renders error state with retry button on error

### AnalysisCard component tests
- Test: renders analysis name and status badge
- Test: links to /analysis/{id}

### StatusBadge component tests
- Test: renders green badge for completed status
- Test: renders yellow badge with pulse for processing status
- Test: renders red badge for failed status

### List page route tests
- Test: renders page title and New Analysis button
- Test: status filter changes URL search param

---

## Section 4: Create Analysis Page

### CreateAnalysisForm component tests
- Test: renders name input and tab toggle
- Test: description field visible on text tab, hidden on upload tab
- Test: submit disabled when name is empty
- Test: submit disabled while mutation is pending
- Test: calls createAnalysis mutation on text tab submit
- Test: calls uploadAnalysis mutation on upload tab submit

### FileDropZone component tests
- Test: renders drop zone with browse link
- Test: accepts PDF files via input change
- Test: accepts DOCX files via input change
- Test: rejects files exceeding maxSizeMB
- Test: rejects non-PDF/DOCX files
- Test: shows drag highlight on dragEnter, removes on dragLeave
- Test: calls onFileSelected with dropped file

---

## Section 5: Settings Page

### SettingsForm component tests
- Test: renders all threshold inputs with current values
- Test: renders boost terms list with add/remove
- Test: save button calls updatePromptTemplate mutation
- Test: reset button restores default values (does not auto-save)

### BoostTermsEditor component tests
- Test: renders existing terms as rows
- Test: add button creates new empty row
- Test: delete button removes a row
- Test: validates no empty term keys on submit

---

## Section 6: New shadcn/ui Components

No tests needed — shadcn components are pre-tested upstream. Just verify installation via build.
