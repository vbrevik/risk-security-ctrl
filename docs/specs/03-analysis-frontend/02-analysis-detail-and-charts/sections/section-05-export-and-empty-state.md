No tooltip component is installed yet. Now I have all the context needed.

# Section 05: Export Buttons and Empty State

## Overview

This section covers two small, self-contained UI components:

1. **ExportButtons** -- Two buttons (PDF/DOCX) with disabled-with-tooltip behavior when the analysis is not completed, loading state during download, and error handling via toast.
2. **EmptyFindings** -- A centered placeholder shown when an analysis is completed but produced zero findings, with guidance text and a link to the settings page.

Both components depend on i18n keys added in section-01. They are consumed by the page assembly in section-06.

## Dependencies

- **Section 01 (Prerequisites and i18n):** Provides the i18n keys under `export.*` and `findings.empty.*` namespaces in both `en/analysis.json` and `nb/analysis.json`.
- **Existing code:** The `exportAnalysis` function in `frontend/src/features/analysis/api/index.ts` handles the actual blob download. The `AnalysisStatus` type is in `frontend/src/features/analysis/types/index.ts`.
- **shadcn/ui components:** `Button` (`@/components/ui/button`). Note: a Tooltip component is **not** currently installed. You will need to install it via `pnpm dlx shadcn@latest add tooltip` before implementing the disabled tooltip pattern.

## Required i18n Keys (reference)

These keys should already exist from section-01. Confirm they are present before testing:

```
export.pdf            -- "Export PDF"
export.docx           -- "Export DOCX"
export.disabled       -- "Analysis must be completed to export"
export.downloading    -- "Downloading..."
export.error          -- "Export failed. Please try again."
findings.empty.title  -- "No compliance findings detected"
findings.empty.description -- "Try adjusting the matcher thresholds or boost terms in settings."
findings.empty.settingsLink -- "Go to Settings"
```

## Tests

All test files use Vitest + React Testing Library. Mock `react-i18next` to return keys as-is. Mock `@/features/analysis/api` for `exportAnalysis`.

### File: `frontend/src/features/analysis/components/__tests__/ExportButtons.test.tsx`

```tsx
/**
 * Tests for ExportButtons component.
 *
 * Setup:
 * - vi.mock("react-i18next") to return translation keys
 * - vi.mock("@/features/analysis/api") to mock exportAnalysis
 * - Render with props: { analysisId, analysisName, status }
 */

describe("ExportButtons", () => {
  // Test: Renders PDF and DOCX export buttons
  // Expect two buttons with text matching export.pdf and export.docx keys

  // Test: Buttons disabled when status is not "completed"
  // Render with status "processing", assert both buttons have disabled attribute

  // Test: Disabled buttons show tooltip text
  // Render with status "processing", hover over button, expect tooltip with export.disabled text
  // NOTE: Requires shadcn Tooltip + TooltipProvider in test wrapper

  // Test: Clicking PDF button calls exportAnalysis with "pdf" format
  // Render with status "completed", click PDF button
  // Assert exportAnalysis called with (analysisId, "pdf")

  // Test: Clicking DOCX button calls exportAnalysis with "docx" format
  // Same pattern, assert format is "docx"

  // Test: Shows loading spinner on clicked button during export
  // Mock exportAnalysis to return a pending promise
  // Click PDF button, assert button shows downloading text/spinner
  // Assert DOCX button does NOT show spinner

  // Test: Both buttons disabled while any export is in progress
  // Mock exportAnalysis to return a pending promise
  // Click PDF button, assert both buttons are disabled

  // Test: Shows error state when export fails
  // Mock exportAnalysis to reject
  // Click PDF button, await rejection
  // Assert error toast/message appears with export.error text
});
```

### File: `frontend/src/features/analysis/components/__tests__/EmptyFindings.test.tsx`

```tsx
/**
 * Tests for EmptyFindings component.
 *
 * Setup:
 * - vi.mock("react-i18next") to return translation keys
 * - Render without props (component is self-contained)
 */

describe("EmptyFindings", () => {
  // Test: Renders "No compliance findings detected" heading
  // Assert heading element with findings.empty.title text

  // Test: Renders suggestion text
  // Assert findings.empty.description text is present

  // Test: Renders link to /analysis/settings
  // Assert an anchor/link element with href="/analysis/settings"
  // Assert link text matches findings.empty.settingsLink

  // Test: Link navigates to settings page
  // Use TanStack Router test utilities or assert the Link component's `to` prop
});
```

## Implementation Details

### Install Tooltip (prerequisite)

Run from `frontend/`:
```bash
pnpm dlx shadcn@latest add tooltip
```

This creates `frontend/src/components/ui/tooltip.tsx` with `Tooltip`, `TooltipTrigger`, `TooltipContent`, and `TooltipProvider` exports.

### ExportButtons Component

**File:** `frontend/src/features/analysis/components/ExportButtons.tsx`

**Props interface:**

```tsx
interface ExportButtonsProps {
  analysisId: string;
  analysisName: string;
  status: AnalysisStatus;
}
```

**Behavior:**

- Renders two `Button` components side by side (variant `"outline"`, size `"sm"`).
- The PDF button has a file/download icon (use `lucide-react` FileDown or Download icon). The DOCX button has the same or similar icon.
- When `status !== "completed"`, both buttons are disabled. Wrap each disabled button in a `Tooltip` that shows the `export.disabled` i18n text on hover. Use the shadcn `TooltipProvider > Tooltip > TooltipTrigger > TooltipContent` pattern. Note: for disabled buttons inside TooltipTrigger, wrap the button in a `<span>` so pointer events still fire for the tooltip.
- On click, call `exportAnalysis(analysisId, format)` from `@/features/analysis/api`. The existing `exportAnalysis` function creates a temporary anchor and triggers download. It needs a small modification: accept an optional `filename` parameter so the download uses `analysisName` instead of `analysis-{id}`. This modification is described below.
- Manage loading state with `useState` for each format (`pdfLoading`, `docxLoading`). Set to true before calling `exportAnalysis`, set to false in the finally block.
- While either export is loading, disable both buttons.
- While a specific button is loading, replace its icon with a spinner (use `lucide-react` `Loader2` with `animate-spin` class) and change text to the `export.downloading` i18n key.
- On error, show a toast notification. If a toast system is not yet set up, use `window.alert` or `console.error` as a fallback and leave a TODO comment for toast integration.

**Modification to `exportAnalysis` in `frontend/src/features/analysis/api/index.ts`:**

The existing function hardcodes the download filename as `analysis-${id}.${format}`. Add an optional third parameter `filename?: string`:

```tsx
export async function exportAnalysis(id: string, format: string = "pdf", filename?: string) {
  // ... existing blob fetch logic ...
  a.download = filename ? `${filename}.${format}` : `analysis-${id}.${format}`;
  // ... rest unchanged ...
}
```

The ExportButtons component passes `analysisName` as the filename.

### EmptyFindings Component

**File:** `frontend/src/features/analysis/components/EmptyFindings.tsx`

**Props:** None -- this is a self-contained presentational component.

**Structure:**

- Outer container: `flex flex-col items-center justify-center py-16 text-center`
- Icon: A search or document icon from `lucide-react` (e.g., `SearchX` or `FileSearch`), rendered at a large size (`w-12 h-12`) with muted color (`text-muted-foreground`).
- Heading: `<h3>` with `text-lg font-semibold mt-4` using `t("findings.empty.title")`.
- Description: `<p>` with `text-muted-foreground mt-2 max-w-md` using `t("findings.empty.description")`.
- Settings link: A TanStack Router `<Link>` component pointing to `/analysis/settings`. Style it as a button using shadcn `Button` with `variant="outline"` and `className="mt-4"`. Text from `t("findings.empty.settingsLink")`.

### Barrel Export Update

Add exports for both components to `frontend/src/features/analysis/index.ts`:

```tsx
export { ExportButtons } from "./components/ExportButtons";
export { EmptyFindings } from "./components/EmptyFindings";
```

## File Manifest

### New Files
| File | Purpose |
|------|---------|
| `frontend/src/features/analysis/components/ExportButtons.tsx` | Export PDF/DOCX buttons with disabled tooltip and loading states |
| `frontend/src/features/analysis/components/EmptyFindings.tsx` | Empty state shown when analysis has zero findings |
| `frontend/src/features/analysis/components/__tests__/ExportButtons.test.tsx` | Tests for ExportButtons |
| `frontend/src/features/analysis/components/__tests__/EmptyFindings.test.tsx` | Tests for EmptyFindings |
| `frontend/src/components/ui/tooltip.tsx` | shadcn Tooltip component (generated by CLI) |

### Modified Files
| File | Change |
|------|--------|
| `frontend/src/features/analysis/api/index.ts` | Add optional `filename` parameter to `exportAnalysis` function |
| `frontend/src/features/analysis/index.ts` | Add barrel exports for ExportButtons and EmptyFindings |

## Edge Cases

| Case | Handling |
|------|----------|
| Export while another export is in progress | Both buttons disabled while either is loading |
| Export fails (network error, 404, etc.) | Catch error, show toast/alert, re-enable buttons |
| Status transitions from processing to completed | Buttons automatically become enabled via reactive props |
| Tooltip on disabled button | Wrap button in `<span>` inside `TooltipTrigger` so pointer events work |
| Analysis name with special characters | The `filename` parameter is used as-is in the download attribute; the browser handles sanitization |
| Empty `analysisName` | Fall back to the existing `analysis-${id}` pattern if name is falsy |

## Implementation Checklist

1. Install shadcn tooltip component
2. Write ExportButtons tests
3. Write EmptyFindings tests
4. Modify `exportAnalysis` to accept optional filename parameter
5. Implement ExportButtons component
6. Implement EmptyFindings component
7. Update barrel exports in `frontend/src/features/analysis/index.ts`
8. Run tests to verify all pass