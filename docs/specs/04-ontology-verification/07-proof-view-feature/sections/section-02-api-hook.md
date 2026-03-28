Now I have all the context I need. Let me generate the section content.

# Section 02: API Hook

## Overview

This section implements the `useFrameworkProof` TanStack Query hook that lazily fetches proof/verification data for a given framework. It also extends the `ontologyKeys` query key hierarchy with a `proof` entry.

**Depends on:** section-01-types-and-deps (the `FrameworkProof` interface must exist before this hook is implemented)

**Blocks:** section-04-proof-panel (ProofPanel calls this hook internally), section-05-framework-profile

---

## File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/index.ts`

---

## Tests First

Write these tests before implementing the hook. Add them to the existing file:

**`/Users/vidarbrevik/projects/risk-security-ctrl/frontend/src/features/ontology/api/__tests__/hooks.test.ts`**

The existing file already sets up:
- `vi.mock("@/lib/api", ...)` mocking `api.get`
- `createWrapper()` returning a `QueryClientProvider` with `retry: false`
- `renderHook` + `waitFor` from `@testing-library/react`

Add a new `describe("useFrameworkProof", ...)` block after the existing describes. Import `useFrameworkProof` in the import statement at the top.

### Test stubs

```typescript
import { useFrameworkProof } from "../index";

describe("useFrameworkProof", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("fetches GET /api/ontology/frameworks/{id}/proof when frameworkId is provided", async () => {
    // Arrange: mock api.get to return a FrameworkProof payload for "nist-csf"
    // Act: renderHook(() => useFrameworkProof("nist-csf"), { wrapper: createWrapper() })
    // Assert: waitFor isLoading false, then check data.framework_id === "nist-csf"
    //         and verify mockedApi.get was called with "/ontology/frameworks/nist-csf/proof"
  });

  it("does NOT call api.get when frameworkId is null (skipToken)", async () => {
    // Act: renderHook(() => useFrameworkProof(null), { wrapper: createWrapper() })
    // Assert: after short wait, mockedApi.get was NOT called
    //         and result.current.data is undefined
  });

  it("passes through null proof_content from API response", async () => {
    // Arrange: mock returns { proof_content: null, verification_status: "verified", ... }
    // Assert: result.current.data.proof_content === null
  });

  it("passes through null verification_status from API response", async () => {
    // Arrange: mock returns { proof_content: "# Proof", verification_status: null, ... }
    // Assert: result.current.data.verification_status === null
  });

  it("isLoading is true while fetch is in flight, false after resolution", async () => {
    // Arrange: mock api.get returns a promise that resolves after a tick
    // Assert: result.current.isLoading is initially true, then false after waitFor
  });
});
```

The mock API response shape for a `FrameworkProof`:

```typescript
{
  framework_id: "nist-csf",
  verification_status: "verified",
  verification_date: "2025-01-15",
  verification_source: "https://example.com/nist-csf",
  verification_notes: "Verified against official publication",
  proof_content: "# NIST CSF Proof\n\nVerification details..."
}
```

---

## Implementation

### 2.1 Extend `ontologyKeys`

Add a `proof` entry to the existing `ontologyKeys` object in `api/index.ts`. It must nest under `framework(id)` in the key hierarchy so that invalidating a framework also invalidates its proof:

```typescript
proof: (id: string) => [...ontologyKeys.framework(id), "proof"] as const,
```

The full updated `ontologyKeys` object gains this as a new property alongside the existing `frameworks`, `framework`, `concepts`, etc. keys.

### 2.2 Add import for `FrameworkProof` type

Add `FrameworkProof` to the import from `"../types"`. This type is defined in section-01 and represents the proof endpoint's response shape.

Also import `skipToken` from `@tanstack/react-query`:

```typescript
import { useQuery, useQueries, skipToken } from "@tanstack/react-query";
```

### 2.3 Implement `useFrameworkProof`

Add the hook after the existing hooks in `api/index.ts`:

```typescript
/**
 * Lazily fetches proof and verification metadata for a framework.
 * Only fires when frameworkId is non-null (user has opened the proof panel).
 * Uses skipToken (TanStack Query v5) for type-safe conditional fetching.
 * staleTime: Infinity because proof files are static verification artifacts.
 */
export function useFrameworkProof(frameworkId: string | null) {
  return useQuery({
    queryKey: frameworkId ? ontologyKeys.proof(frameworkId) : [],
    queryFn: frameworkId
      ? async () => {
          const { data } = await api.get<FrameworkProof>(
            `/ontology/frameworks/${frameworkId}/proof`
          );
          return data;
        }
      : skipToken,
    staleTime: Infinity,
  });
}
```

**Key design decisions:**

- `skipToken` when `frameworkId` is null — this is the idiomatic TanStack Query v5 pattern for conditional queries. It gives better TypeScript narrowing than `enabled: !!frameworkId`.
- `staleTime: Infinity` — proof files are static verification artifacts produced by an intentional re-verification process. They should never be refetched automatically.
- The hook is called by `ProofPanel` (section-04), not by `FrameworkProfile` directly. This means the fetch only fires once the user opens the proof panel, not when they select a framework.
- The `queryKey` uses `[]` as fallback when `frameworkId` is null, but since `queryFn` is `skipToken`, the key is never used for a real fetch in that branch.

### Return type

The hook returns `UseQueryResult<FrameworkProof>`. Import `UseQueryResult` from `@tanstack/react-query` if needed for explicit typing elsewhere, but the hook's return type is inferred automatically.

---

## Backend Endpoint Reference

The hook calls:

```
GET /api/ontology/frameworks/{id}/proof
Authorization: Bearer <token>  (handled by the api client)
```

Response shape (matches `FrameworkProof` interface from section-01):

```
{
  framework_id: string,
  verification_status: string | null,
  verification_date: string | null,
  verification_source: string | null,
  verification_notes: string | null,
  proof_content: string | null        // null when no proof file exists yet
}
```

`proof_content` is raw markdown. It is null for frameworks that have no proof file. The hook passes this through as-is — null handling is done in `ProofPanel`.

---

## Checklist

- [x] Import `skipToken` from `@tanstack/react-query` in `api/index.ts`
- [x] Import `FrameworkProof` from `"../types"` in `api/index.ts`
- [x] Add `proof: (id: string) => [...]` to `ontologyKeys`
- [x] Implement and export `useFrameworkProof(frameworkId: string | null)`
- [x] Write five hook tests in `api/__tests__/hooks.test.ts`
- [x] Add `useFrameworkProof` to the import in the test file
- [x] `pnpm test` passes (266 tests)

## Deviations from Plan

- `queryKey` for null case uses `["__disabled__"]` instead of `[]` — empty array is a degenerate shared key; sentinel avoids cache collisions.
- Updated `FW_A`/`FW_B` fixtures in hooks.test.ts to include new verification fields from the extended `Framework` interface.
- Null-path test uses synchronous `fetchStatus === 'idle'` assertion instead of `setTimeout(50ms)` — more reliable since skipToken makes the query synchronously idle.