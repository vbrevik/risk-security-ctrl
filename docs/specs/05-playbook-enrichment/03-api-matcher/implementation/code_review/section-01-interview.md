# Section 01 Code Review Interview

## M1: Null vs absent for nested optional fields
**Decision: Keep as null** (user choice). Nested Option fields serialize as null, not omitted. This is explicit and simpler for API consumers. Only the top-level guidance field uses skip_serializing_if.

## M2: Empty vec assertion (auto-fix)
**Applied.** Added assertions that empty vecs serialize as `[]` in the `includes_guidance_when_some` test. Prevents regression if someone adds skip_serializing_if to Vec fields.

## L1-L3: Let go
- Forward reference: unavoidable per plan placement
- No Clone derive: add later if needed
- Test helper duplication: acceptable for 2 tests
