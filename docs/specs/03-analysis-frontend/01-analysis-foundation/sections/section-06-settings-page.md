I have enough context now to write the section.

# Section 6: Settings Page

## Overview

This section implements the `/analysis/settings` page, which provides a form for editing the `MatcherConfig` used by the backend analysis engine. It consists of two components (`SettingsForm` and `BoostTermsEditor`) and a route file. The page loads the current configuration via `usePromptTemplate()`, allows editing thresholds, options, and boost terms locally, then persists changes via `useUpdatePromptTemplate()`.

## Dependencies

- **section-01-types-and-hooks**: Provides `MatcherConfig` type, `usePromptTemplate()` hook, `useUpdatePromptTemplate()` hook, and `analysisKeys` (all from `@/features/analysis/api`)
- **section-02-i18n-and-navigation**: Provides the `analysis` i18n namespace with `settings.*` keys
- **section-03-shadcn-components**: The `table` component may be useful for layout but is not strictly required; standard `Input`, `Label`, `Button`, and `Card` from shadcn/ui are the primary primitives

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/analysis/components/SettingsForm.tsx` | Main settings form component |
| `frontend/src/features/analysis/components/BoostTermsEditor.tsx` | Inline key-value editor for boost terms |
| `frontend/src/routes/analysis/settings.tsx` | Route file for `/analysis/settings` |
| `frontend/src/features/analysis/components/__tests__/SettingsForm.test.tsx` | Tests for SettingsForm |
| `frontend/src/features/analysis/components/__tests__/BoostTermsEditor.test.tsx` | Tests for BoostTermsEditor |

## Files to Modify

| File | Change |
|------|--------|
| `frontend/src/features/analysis/components/index.ts` | Add re-exports for `SettingsForm` and `BoostTermsEditor` |

## Tests

Write all tests before implementation. The test files use Vitest and React Testing Library. Mock `@/lib/api` and `react-i18next` as established in the project. Wrap components needing TanStack Query in a `QueryClientProvider` test wrapper.

### SettingsForm Tests

**File:** `frontend/src/features/analysis/components/__tests__/SettingsForm.test.tsx`

Four test cases:

1. **Renders all threshold inputs with current values** -- Mock `usePromptTemplate` to return a `MatcherConfig` with known values. Assert that number inputs for min_confidence, addressed_threshold, partial_threshold, and max_findings_per_framework are present and show the mocked values. Also verify the include_addressed_findings checkbox is rendered.

2. **Renders boost terms list with add/remove** -- Mock `usePromptTemplate` to return config with `boost_terms: { "security": 1.5, "compliance": 2.0 }`. Assert two term rows render. Assert "Add Term" button is present.

3. **Save button calls updatePromptTemplate mutation** -- Fill in values, click save. Assert the mutation function was called with a `MatcherConfig`-shaped object.

4. **Reset button restores default values (does not auto-save)** -- Click the reset button, confirm the dialog/prompt. Assert that form fields now show default values (min_confidence: 0.1, addressed: 0.6, partial: 0.3, max_findings: 50, include_addressed: true). Assert the mutation was NOT called (save is manual).

### BoostTermsEditor Tests

**File:** `frontend/src/features/analysis/components/__tests__/BoostTermsEditor.test.tsx`

Four test cases:

1. **Renders existing terms as rows** -- Pass `value={{ "risk": 1.0, "threat": 2.0 }}` and an `onChange` callback. Assert two rows with the correct term and weight values render.

2. **Add button creates new empty row** -- Click the "Add Term" button. Assert `onChange` is called with an updated array containing a new entry with empty string key and default weight.

3. **Delete button removes a row** -- Render with two terms. Click the delete button on the first row. Assert `onChange` is called with only the second term remaining.

4. **Validates no empty term keys on submit** -- This validation happens at the SettingsForm level. The BoostTermsEditor itself should visually indicate empty keys (e.g., red border). Test that an empty term input receives an error styling class or aria-invalid attribute.

## Implementation Details

### Default Config Values

These are the default `MatcherConfig` values used by the Reset to Defaults button:

```typescript
const DEFAULT_MATCHER_CONFIG: MatcherConfig = {
  version: 1,
  thresholds: {
    min_confidence: 0.1,
    addressed: 0.6,
    partial: 0.3,
  },
  max_findings_per_framework: 50,
  include_addressed_findings: true,
  boost_terms: {},
};
```

### Route File (`settings.tsx`)

Create the route using `createFileRoute("/analysis/settings")`. The route component renders:

1. A back link (`<Link to="/analysis">`) with left arrow icon and back text from i18n
2. Page title "Matcher Configuration" from `t("settings.title")`
3. The `<SettingsForm />` component

Follow the same layout pattern as the compliance page: `<div className="space-y-6 p-6">` wrapper with a header section.

### SettingsForm Component

**Props:** None (fetches its own data via hooks).

**State management:** Use `useState` for form fields, initialized from `usePromptTemplate().data` when it loads. Use a `useEffect` to sync state when the query data first arrives (or changes). The form is fully controlled.

**Structure:**

The form is wrapped in a `<Card>` and divided into three labeled sections:

1. **Thresholds section** (`t("settings.thresholds")` heading):
   - `min_confidence_threshold` -- `<Input type="number">` with min=0, max=1, step=0.05
   - `addressed_threshold` -- `<Input type="number">` with min=0, max=1, step=0.05  
   - `partial_threshold` -- `<Input type="number">` with min=0, max=1, step=0.05
   - `max_findings_per_framework` -- `<Input type="number">` with min=1, max=500, step=1
   - Each input paired with a `<Label>` using the corresponding i18n key

2. **Options section:**
   - `include_addressed_findings` -- A checkbox input (or shadcn Switch if available, otherwise a native checkbox styled consistently). Label from `t("settings.includeAddressed")`.

3. **Boost Terms section** (`t("settings.boostTerms")` heading):
   - Renders `<BoostTermsEditor>` with the current boost terms state and an onChange handler

**Action buttons** (in a flex row at the bottom of the card):
- **Save button** -- Calls `useUpdatePromptTemplate().mutate()` with the assembled `MatcherConfig`. While pending, shows a loading indicator and is disabled. On success, show a brief "Saved" confirmation (can be inline text that appears next to the button, using `t("settings.saved")`).
- **Reset to Defaults button** -- Variant "outline". On click, show a `window.confirm()` dialog with `t("settings.resetConfirm")`. If confirmed, set all local state fields to `DEFAULT_MATCHER_CONFIG` values. Does NOT call the mutation -- the user must explicitly click Save.

**Loading state:** While `usePromptTemplate()` is loading, show a simple loading indicator or skeleton.

**Error state:** If `usePromptTemplate()` returns an error (e.g., corrupt config on backend), display an error message with the text from `t("common.error")`.

### BoostTermsEditor Component

**Props:**
```typescript
interface BoostTermsEditorProps {
  value: Array<{ term: string; weight: number }>;
  onChange: (terms: Array<{ term: string; weight: number }>) => void;
}
```

The parent (`SettingsForm`) is responsible for converting between `Record<string, number>` (the `MatcherConfig` shape) and `Array<{ term: string; weight: number }>` (the editor's internal shape). Conversion logic:

- **Record to Array:** `Object.entries(record).map(([term, weight]) => ({ term, weight }))`
- **Array to Record:** `Object.fromEntries(terms.map(t => [t.term, t.weight]))`

**Rendering:** Each row contains:
- A text `<Input>` for the term key (placeholder from `t("settings.termLabel")`)
- A number `<Input>` for the weight (step=0.1, min=0, placeholder from `t("settings.weightLabel")`)
- A delete button with a Trash2 icon from lucide-react

Below the rows, an "Add Term" `<Button>` (variant "outline", size "sm") adds a new `{ term: "", weight: 1.0 }` entry.

**Validation at save time (in SettingsForm):** Before calling the mutation, check:
- No empty term keys (filter them out or show inline error)
- No duplicate term keys (last one wins, or show error)
- All weights > 0

### i18n Keys Used

These keys must exist in the `analysis` namespace (created in section-02):

- `settings.title`
- `settings.thresholds`
- `settings.minConfidence`
- `settings.addressedThreshold`
- `settings.partialThreshold`
- `settings.maxFindings`
- `settings.includeAddressed`
- `settings.boostTerms`
- `settings.termLabel`
- `settings.weightLabel`
- `settings.addTerm`
- `settings.save`
- `settings.saved`
- `settings.resetDefaults`
- `settings.resetConfirm`
- `common.back`
- `common.error`

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| Settings API returns 500 (corrupt config) | Show error message on settings page, form not rendered |
| Concurrent settings saves | Last write wins (acceptable for single-user) |
| Empty boost terms list | Show only the "Add Term" button, no rows |
| Duplicate boost term keys | Validate before save; show inline error or silently deduplicate |
| User resets then navigates away without saving | Changes are lost (no unsaved-changes guard in this split) |