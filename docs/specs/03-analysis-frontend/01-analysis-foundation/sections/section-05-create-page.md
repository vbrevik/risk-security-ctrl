Now I have all the context needed. Let me generate the section content.

# Section 5: Create Analysis Page

## Overview

This section implements the create analysis page at `/analysis/create`. It consists of two main components: `CreateAnalysisForm` (the page-level form with tabbed input) and `FileDropZone` (a native HTML5 drag-and-drop upload area). The route file at `src/routes/analysis/create.tsx` renders the form within a centered card layout.

## Dependencies

- **section-01-types-and-hooks**: Provides `CreateAnalysisRequest`, `UploadAnalysisInput` types and `useCreateAnalysis()`, `useUploadAnalysis()` hooks from `@/features/analysis/api`
- **section-02-i18n-and-navigation**: Provides the `analysis` i18n namespace with `create.*` translation keys
- **section-03-shadcn-components**: Provides `tabs` and `textarea` shadcn/ui components

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/analysis/components/CreateAnalysisForm.tsx` | Tabbed form with text/upload input modes |
| `frontend/src/features/analysis/components/FileDropZone.tsx` | Native drag-and-drop file upload area |
| `frontend/src/routes/analysis/create.tsx` | Route file for `/analysis/create` |
| `frontend/src/features/analysis/components/__tests__/CreateAnalysisForm.test.tsx` | Form component tests |
| `frontend/src/features/analysis/components/__tests__/FileDropZone.test.tsx` | Drop zone component tests |

## Files to Modify

| File | Change |
|------|--------|
| `frontend/src/features/analysis/index.ts` | Re-export `CreateAnalysisForm` and `FileDropZone` |

---

## Tests

All tests use Vitest + React Testing Library. Follow the project pattern: mock `@/lib/api` with `vi.mock`, mock `react-i18next`, and wrap components requiring TanStack Router in a `renderWithRouter` helper.

### CreateAnalysisForm Tests

**File:** `frontend/src/features/analysis/components/__tests__/CreateAnalysisForm.test.tsx`

```typescript
/**
 * CreateAnalysisForm component tests
 *
 * Mock dependencies:
 * - vi.mock("react-i18next") with t returning keys
 * - vi.mock("@/features/analysis/api") to mock useCreateAnalysis, useUploadAnalysis
 * - Wrap in renderWithRouter helper (TanStack Router memory history)
 *
 * Test cases:
 */

// Test: renders name input and tab toggle
//   - Assert an input with placeholder matching "create.namePlaceholder" exists
//   - Assert both "create.textTab" and "create.uploadTab" tab triggers render

// Test: description field visible on text tab, hidden on upload tab
//   - On default (text tab), the description textarea is in the DOM
//   - Click the upload tab, description textarea is NOT in the DOM
//   - This matches the spec: upload endpoint does not accept description

// Test: submit disabled when name is empty
//   - Render form, leave name empty, assert submit button is disabled

// Test: submit disabled while mutation is pending
//   - Mock useCreateAnalysis to return { isPending: true, mutate: vi.fn() }
//   - Fill in name + text, assert submit button is disabled
//   - Button text should show "create.uploading" or spinner state

// Test: calls createAnalysis mutation on text tab submit
//   - Fill name input, fill textarea with text content
//   - Submit the form
//   - Assert useCreateAnalysis().mutate was called with { name, description, input_text }

// Test: calls uploadAnalysis mutation on upload tab submit
//   - Switch to upload tab
//   - Fill name input, simulate file selection via onFileSelected callback
//   - Submit the form
//   - Assert useUploadAnalysis().mutate was called with { file, name }
```

### FileDropZone Tests

**File:** `frontend/src/features/analysis/components/__tests__/FileDropZone.test.tsx`

```typescript
/**
 * FileDropZone component tests
 *
 * Props: accept: string[], maxSizeMB: number, onFileSelected: (file: File) => void, onError: (msg: string) => void
 *
 * Test cases:
 */

// Test: renders drop zone with browse link
//   - Render FileDropZone with standard props
//   - Assert text matching "create.dropzoneText" is visible
//   - Assert a clickable element with "create.dropzoneBrowse" text exists

// Test: accepts PDF files via input change
//   - Find the hidden file input
//   - Fire change event with a File({ type: "application/pdf" }) under 25MB
//   - Assert onFileSelected was called with the file

// Test: accepts DOCX files via input change
//   - Fire change event with a File({ type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document" })
//   - Assert onFileSelected was called

// Test: rejects files exceeding maxSizeMB
//   - Create a File mock with size > 25 * 1024 * 1024
//   - Fire change event
//   - Assert onError was called with size error message
//   - Assert onFileSelected was NOT called

// Test: rejects non-PDF/DOCX files
//   - Fire change event with a File({ type: "text/plain" })
//   - Assert onError was called with file type error message
//   - Assert onFileSelected was NOT called

// Test: shows drag highlight on dragEnter, removes on dragLeave
//   - Find the drop zone container
//   - Fire dragEnter event
//   - Assert container has the drag-active CSS class (e.g. "border-primary")
//   - Fire dragLeave event
//   - Assert the highlight class is removed

// Test: calls onFileSelected with dropped file
//   - Fire drop event with dataTransfer containing a valid PDF file
//   - Assert onFileSelected was called with that file
```

---

## Implementation Details

### Route File: `src/routes/analysis/create.tsx`

Use `createFileRoute("/analysis/create")` following the project pattern. The component function renders:

- A container `div` with `max-w-2xl mx-auto p-6 space-y-6`
- A back link using TanStack Router `<Link to="/analysis">` with `{t("common.back")}` text
- The page title `<h1>` with `{t("create.title")}`
- The `<CreateAnalysisForm />` component

On successful creation, the form navigates to `/analysis/$id` using the router's `useNavigate()` hook. The route file itself is minimal -- all logic lives in the form component.

### Component: `CreateAnalysisForm.tsx`

**Location:** `frontend/src/features/analysis/components/CreateAnalysisForm.tsx`

**Imports:** shadcn `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`, `Input`, `Label`, `Button`, `Textarea`. Hooks: `useCreateAnalysis`, `useUploadAnalysis` from `@/features/analysis/api`. Also `useNavigate` from TanStack Router and `useTranslation` from react-i18next.

**State management:**

- `activeTab`: `"text" | "upload"` -- controlled by the Tabs component's `value`/`onValueChange`
- `name`: string
- `description`: string
- `inputText`: string
- `selectedFile`: `File | null`
- `fileError`: `string | null` -- validation errors from FileDropZone
- `nameError`: `boolean` -- shown on submit attempt with empty name

Both `useCreateAnalysis()` and `useUploadAnalysis()` hooks are called unconditionally (hooks cannot be conditional). The `progress` value is destructured from `useUploadAnalysis()` return.

**Tab behavior:** The `Tabs` component uses `value={activeTab}` and `onValueChange` to track the active tab. Crucially, switching tabs does NOT clear state -- the text content and selected file are preserved independently.

**Description field visibility:** The description `<Textarea>` (with its `<Label>`) renders only when `activeTab === "text"`. The upload endpoint does not accept a description field.

**Submit handler (`handleSubmit`):**

```
function handleSubmit signature:
  (e: React.FormEvent) => void

  1. e.preventDefault()
  2. Validate name is non-empty (trim). If empty, set nameError = true, return.
  3. If activeTab === "text":
     a. Validate inputText is non-empty. If empty, return.
     b. Call createMutation.mutate({ name, description (if non-empty), input_text: inputText })
     c. In onSuccess callback: navigate({ to: "/analysis/$id", params: { id: data.id } })
  4. If activeTab === "upload":
     a. Validate selectedFile is not null. If null, return.
     b. Call uploadMutation.mutate({ file: selectedFile, name })
     c. In onSuccess callback: navigate to the new analysis detail page
```

**Progress bar:** Rendered below the FileDropZone when `activeTab === "upload"` and `uploadMutation.isPending`. Use a simple `<div>` with a colored inner `<div>` whose width is `${progress}%`. Show the percentage as text.

**Submit button state:**
- Disabled when `createMutation.isPending || uploadMutation.isPending || !name.trim()`
- Text changes to `{t("create.uploading")}` with a spinner icon when pending

### Component: `FileDropZone.tsx`

**Location:** `frontend/src/features/analysis/components/FileDropZone.tsx`

**Props interface:**

```typescript
interface FileDropZoneProps {
  accept: string[];         // e.g. [".pdf", ".docx"]
  maxSizeMB: number;        // e.g. 25
  onFileSelected: (file: File) => void;
  onError: (msg: string) => void;
}
```

**Implementation approach -- native HTML5 drag-and-drop, no external library:**

- A container `<div>` with `onDragEnter`, `onDragOver`, `onDragLeave`, `onDrop` handlers
- `isDragging` state (boolean) for visual highlight
- **Drag counter ref** (`useRef<number>(0)`) -- increment on `dragEnter`, decrement on `dragLeave`. Set `isDragging = true` when counter > 0, false when counter reaches 0. This prevents flicker caused by drag events firing on child elements.
- `onDragOver`: `e.preventDefault()` (required to allow drop)
- `onDrop`: `e.preventDefault()`, reset drag counter to 0 and `isDragging` to false, extract file from `e.dataTransfer.files[0]`, validate, call `onFileSelected`
- Hidden `<input type="file" accept=".pdf,.docx">` with an `id` linked to a visible `<label>` for keyboard accessibility and the "Browse" click target
- On `<input>` change: extract file, validate, call `onFileSelected`

**Validation function (shared between drop and input change):**

```
validateFile(file: File): boolean
  1. Check file extension against accept list AND check MIME type:
     - PDF: extension .pdf, MIME application/pdf
     - DOCX: extension .docx, MIME application/vnd.openxmlformats-officedocument.wordprocessingml.document
  2. If invalid type: call onError(t("create.invalidFileType")), return false
  3. Check file.size <= maxSizeMB * 1024 * 1024
  4. If too large: call onError(t("create.fileTooLarge")), return false
  5. Return true
```

**Visual states:**

- Default: dashed border, muted text color, icon (Upload or FileText from lucide-react)
- Dragging (`isDragging`): highlighted border (e.g. `border-primary bg-primary/5`), prompt text
- File selected: show filename, formatted file size (e.g. "2.4 MB"), and a "Remove" button that clears the selection. The remove button calls a callback or the parent manages the `selectedFile` state.

**After file selection:** The parent (`CreateAnalysisForm`) stores the file in its `selectedFile` state. The FileDropZone should also display the selected file info. Two approaches work: either the parent passes `selectedFile` back as a prop for display, or the component manages its own display state. The cleaner approach: add an optional `selectedFile: File | null` prop and an `onClear: () => void` prop so the parent remains the single source of truth. Update the props interface accordingly:

```typescript
interface FileDropZoneProps {
  accept: string[];
  maxSizeMB: number;
  selectedFile: File | null;
  onFileSelected: (file: File) => void;
  onClear: () => void;
  onError: (msg: string) => void;
}
```

When `selectedFile` is non-null, render the file info display instead of the drop prompt.

### Re-exports

Add `CreateAnalysisForm` and `FileDropZone` to the barrel export in `frontend/src/features/analysis/index.ts`.

---

## i18n Keys Used

These keys are expected to exist in `src/i18n/locales/{en,nb}/analysis.json` (created in section-02):

- `create.title`, `create.nameLabel`, `create.namePlaceholder`
- `create.descriptionLabel`
- `create.textTab`, `create.uploadTab`
- `create.textPlaceholder`
- `create.dropzoneText`, `create.dropzoneBrowse`
- `create.uploading`, `create.submit`
- `create.maxFileSize`, `create.invalidFileType`, `create.fileTooLarge`
- `create.success`
- `common.back`

## Edge Cases

| Scenario | Behavior |
|----------|----------|
| Failed upload (network error) | Show error message from mutation, keep file selection and form state for retry |
| File too large (>25MB) | Client-side validation in FileDropZone, error via `onError` callback, upload never initiated |
| Invalid file type | Client-side validation, show "Only PDF/DOCX supported" via `onError` |
| Tab switch preserves state | Switching between text/upload does not clear the other tab's data |
| Upload progress stalls | No timeout implemented; user sees stalled progress bar, can navigate away |
| Name empty on submit | Inline error shown, form not submitted |
| Text empty on text tab submit | Prevented by validation, no mutation call |
| No file on upload tab submit | Prevented by validation, no mutation call |