# Split 07: Proof View Feature

## Scope

Build a verification provenance feature that lets users see *why* each framework's data is trusted: what source was used, when it was verified, what status it has, and the evidence trail. This is the user-facing capstone of the ontology verification effort (splits 01-06).

## Dependencies

Requires completion of splits 01-06 (all framework verifications and relationship validation). The proof view displays the results of that verification work.

## Zachman Framework Perspectives

### What (Data)

**New database fields** on `frameworks` table:
- `verification_status` TEXT — one of: `verified`, `partially-verified`, `structure-verified`, `unverified`, `corrected`
- `verification_date` TEXT — ISO 8601 date of last verification
- `verification_source` TEXT — URL to authoritative source used
- `verification_notes` TEXT — brief notes on verification depth/limitations

**Proof files** in `docs/sources/`:
- One markdown file per verified framework (e.g., `nist-csf-proof.md`)
- Contains extracted structure, source URLs, verification methodology
- Served read-only via API — never modified by the application at runtime

**API response shape** for proof data:
```
{
  framework_id: string,
  verification_status: string,
  verification_date: string | null,
  verification_source: string | null,
  verification_notes: string | null,
  proof_content: string | null  // rendered markdown from proof file
}
```

### How (Process)

1. **Database migration** adds verification columns to `frameworks` table
2. **Import pipeline** updated to populate verification fields from framework JSON metadata
3. **Proof API endpoint** reads proof file from disk, returns content alongside DB metadata
4. **Frontend route** renders verification status badge + proof content with source links
5. Proof files are static artifacts — no CRUD, no user editing

### Where (Network)

- **Backend**: New endpoint under existing `/api/ontology/` prefix
- **Frontend**: New route under `/ontology/frameworks/:frameworkId/proof` or similar
- **Proof files**: Read from `docs/sources/` on the server filesystem (not served as static files directly)

### Who (People)

- **End users**: View verification status and proof evidence for any framework
- **Developers/maintainers**: Create and update proof files as part of verification workflow
- Authentication required (uses existing auth middleware)

### When (Timing)

- Proof data is static once written — changes only when a framework is re-verified
- No real-time updates, no polling, no subscriptions needed
- Verification metadata imported at DB seed/migration time

### Why (Motivation)

Every ontology concept claim must be traceable to an authoritative source. Without provenance:
- Users cannot trust that framework data is accurate
- Errors like the NSL/NSM GP hallucination problem go undetected
- Compliance auditors have no evidence trail

The proof view makes verification status visible and evidence accessible, enabling users to verify any claim themselves via source links.

## Technical Requirements

### Database Migration (005)

Add columns to `frameworks` table:
```sql
ALTER TABLE frameworks ADD COLUMN verification_status TEXT DEFAULT 'unverified';
ALTER TABLE frameworks ADD COLUMN verification_date TEXT;
ALTER TABLE frameworks ADD COLUMN verification_source TEXT;
ALTER TABLE frameworks ADD COLUMN verification_notes TEXT;
```

### Framework JSON Schema Extension

Add optional verification fields to framework JSON files:
```json
{
  "framework": {
    "id": "...",
    "verification_status": "verified",
    "verification_date": "2026-03-28",
    "verification_source": "https://...",
    "verification_notes": "Full structure verified from official source"
  }
}
```

### API Endpoint

`GET /api/ontology/frameworks/{id}/proof`

Returns verification metadata from DB + proof file content (if exists) as a single response. The proof file path is derived server-side from the framework ID — never accepted from the client.

### Frontend Route

Accessible from the framework detail view. Displays:
- Verification status badge (color-coded)
- Verification date and source link
- Verification notes
- Proof file content rendered as markdown (with headings, lists, links)

### Proof File Convention

Files in `docs/sources/` follow the naming pattern `{framework-id}-proof.md` (e.g., `nist-csf-proof.md`). Existing files like `nsm-grunnprinsipper-v21-extract.md` will be renamed to match the convention.

## STIG Constraints

- V-222607 (CAT II): Proof API errors must not reveal internal file paths or database structure
- V-222610 (CAT II): Proof endpoints require authentication via existing middleware
- V-222604 (CAT II): Internal proof file paths must not be exposed to the client — derive path server-side from framework ID
- V-222571 (CAT II): Sanitize framework_id parameter to prevent path traversal (validate against known framework IDs, not arbitrary strings)

## Deliverables

- [ ] Database migration adding verification columns
- [ ] Backend import logic updated for verification fields
- [ ] API endpoint for proof data
- [ ] Frontend proof view route with status badge and rendered content
- [ ] Proof file naming convention applied to existing files
- [ ] `cargo test` passes
- [ ] `pnpm build` passes
