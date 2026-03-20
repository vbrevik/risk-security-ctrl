# Section 01 Code Review: Guidance Response Types

## Failure Condition Audit (all pass)

1. **SHALL NOT add Deserialize or FromRow** -- PASS
2. **SHALL NOT break existing ConceptWithRelationships serialization** -- PASS
3. **SHALL NOT use "reference_type" as JSON key** -- PASS (serde rename + test assertion)
4. **SHALL NOT include guidance key when None** -- PASS (skip_serializing_if + test)
5. **SHALL NOT skip tests** -- PASS (6 tests)

## Issues

### Medium

**M1: Optional fields serialize as null instead of being omitted.** ActionResponse.text_nb, QuestionResponse.text_nb, and several ReferenceResponse fields are Option<T> without skip_serializing_if. The top-level guidance field omits when None, but nested optionals produce "field": null. Should be a deliberate choice.

**M2: No assertion that empty vecs serialize as [].** The test with empty vecs doesn't assert they appear as [] — if someone adds skip_serializing_if = "Vec::is_empty" later, no test catches the regression.

### Low

**L1:** Forward reference of ConceptGuidanceResponse (used before defined) — compiles fine, unavoidable given plan placement.
**L2:** No Clone derive — may need adding later.
**L3:** Test helper duplication for Concept construction.
