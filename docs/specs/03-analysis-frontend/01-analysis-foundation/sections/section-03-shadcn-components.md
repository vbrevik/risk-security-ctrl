I now have all the context needed. Here is the section content:

# Section 3: Install New shadcn/ui Components

## Overview

This section installs three shadcn/ui primitives that are not yet present in the project: **table**, **tabs**, and **textarea**. These are required by downstream sections before any component development begins.

**Currently installed shadcn/ui components** (in `frontend/src/components/ui/`): badge, button, card, dialog, input, label, select.

**Components to add:**
- **tabs** -- Used by the text/upload toggle on the create page (section 05)
- **textarea** -- Used for the text input mode on the create page (section 05)
- **table** -- Used by the findings table in the follow-up split (02-analysis-detail-and-charts), and potentially useful for settings layout (section 06)

## Dependencies

- No dependencies on other sections. This section can be executed in parallel with section 01 (types-and-hooks) and section 02 (i18n-and-navigation).
- Sections 04, 05, and 06 all depend on this section being completed first.

## Tests

No dedicated tests are needed for this section. shadcn/ui components are pre-tested upstream. Verification is done by confirming the build succeeds after installation.

## Implementation Steps

### Step 1: Install the components

Run the following command from the `frontend/` directory (`/Users/vidarbrevik/projects/risk-security-ctrl/frontend`):

```bash
npx shadcn@latest add table tabs textarea
```

The project uses the **new-york** style, **no RSC**, **TSX**, **lucide** icons, and aliases `@/components/ui` for the UI directory. The `components.json` configuration file at `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/components.json` already has these settings, so `shadcn` will place files correctly.

### Step 2: Verify new files exist

After installation, confirm that three new files have been created:

- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/components/ui/table.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/components/ui/tabs.tsx`
- `/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/components/ui/textarea.tsx`

### Step 3: Check for new dependencies

The `shadcn add` command may install additional Radix UI packages (e.g., `@radix-ui/react-tabs`). If the command adds new entries to `package.json`, that is expected. The project already has `radix-ui` as a dependency. Ensure `pnpm install` has been run (the `shadcn` CLI typically handles this automatically).

### Step 4: Verify build

Run from the `frontend/` directory:

```bash
pnpm build
```

A successful build (exit code 0) confirms the components are properly installed and compatible with the existing TypeScript and Tailwind configuration.

### Step 5: Verify test suite still passes

Run from the `frontend/` directory:

```bash
pnpm test --run
```

Existing tests should remain green. The new components are additive and do not modify any existing files.

## Expected Component Exports

After installation, each component file will export the following (standard shadcn/ui exports):

- **table.tsx**: `Table`, `TableHeader`, `TableBody`, `TableFooter`, `TableHead`, `TableRow`, `TableCell`, `TableCaption`
- **tabs.tsx**: `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`
- **textarea.tsx**: `Textarea`

These will be imported by downstream sections as:

```typescript
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
```

## Troubleshooting

If `npx shadcn@latest add` fails or prompts for configuration:

1. Ensure you are running from the `frontend/` directory where `components.json` exists.
2. The project uses Tailwind CSS v4 (`tailwindcss: ^4.1.18`). If the shadcn CLI has issues with v4, check that `components.json` has an empty `tailwind.config` field (it does -- this is correct for Tailwind v4 which uses CSS-based config).
3. If prompted about overwriting, select "no" -- none of these three components should already exist.