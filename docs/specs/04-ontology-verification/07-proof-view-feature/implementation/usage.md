# Usage Guide: Proof View Feature

## Overview

This feature adds verification provenance display to the ontology framework explorer. Users can view badge indicators showing each framework's verification status, and open a proof panel to read the full verification document.

## Quick Start

The feature is integrated into the existing `FrameworkProfile` component. No configuration required — it activates automatically when a framework has `verification_status` set in the database.

### For end users

1. Open the Ontology Explorer and select a framework from the sidebar
2. The framework header now shows a `VerificationBadge` next to the version tag
3. If the framework has been verified, a **View Proof** button appears below the description
4. Click **View Proof** to open the proof panel with:
   - Verification date and status badge
   - Link to the source document
   - Full markdown-rendered proof content
5. Click **Hide Proof** to close the panel

### For developers adding verification data

Verification data is stored in the `frameworks` table (columns added in migration `005_verification_provenance.sql`):

```sql
UPDATE frameworks
SET
  verification_status = 'verified',
  verification_date   = '2025-01-15',
  verification_source = 'https://example.com/proof',
  verification_notes  = NULL
WHERE id = 'iso31000';
```

Proof content (markdown) is served by the `/api/ontology/frameworks/:id/proof` endpoint and loaded lazily when the user opens the panel.

## Public Interfaces

### `VerificationBadge`

```tsx
import { VerificationBadge } from "@/features/ontology/components";

<VerificationBadge status={framework.verification_status} />
```

Renders a colored badge with icon, label, and `aria-label` for any `string | null` status value. Unknown/null statuses render in a neutral "Unknown" style.

### `ProofPanel`

```tsx
import { ProofPanel } from "@/features/ontology/components";

<ProofPanel frameworkId="iso31000" />
```

Self-contained panel that fetches and renders proof data for a given framework. Shows loading skeletons, error state, or the full verification document. Only fetches when mounted.

### `useFrameworkProof`

```typescript
import { useFrameworkProof } from "@/features/ontology/api";

const { data, isLoading, isError } = useFrameworkProof(frameworkId);
// data: FrameworkProof | undefined
// Pass null to disable fetching (query stays idle)
```

TanStack Query hook with `staleTime: Infinity` — proof artifacts are static and never re-fetched within a session.

### `VerificationStatus` type

```typescript
import { toVerificationStatus } from "@/features/ontology/types";

const status = toVerificationStatus(rawString); // "verified" | "partially-verified" | ... | "unknown"
```

## API Reference

`GET /api/ontology/frameworks/:id/proof`

Returns `FrameworkProof`:

```typescript
interface FrameworkProof {
  framework_id: string;
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
  proof_content: string | null; // markdown
}
```

## i18n Keys

All proof-related strings are in the `ontology` namespace under the `proof.*` and `common.source` keys. See `frontend/src/i18n/locales/en/ontology.json` for the full list.
