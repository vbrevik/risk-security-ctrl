# Section 01 Code Review: Config Types

## Important

1. **ScoredCandidate duplicates ConceptCandidate fields** — Composition (`candidate: ConceptCandidate`) would be more idiomatic, but the plan's test code constructs flat fields. Changing would require test updates and affect later sections' field access patterns.

2. **No threshold invariant validation** — `partial_threshold < addressed_threshold` not enforced. Inverted thresholds from JSON could misclassify findings in sections 04-05.

## Minor

3. **Redundant serde defaults** — Per-field `#[serde(default = "fn")]` + manual `Default` impl is verbose. Struct-level `#[serde(default)]` achieves the same with less code.

4. **`confidence_score` range not enforced** — Documented as [0.0, 1.0] but bare `f64`.

## Nitpick

5. **Missing `PartialEq` derive** on candidate structs — will make later test assertions awkward.

6. **`Topic` missing `Serialize`** — minor forward-compatibility concern.

## Verdict

Implementation correct relative to plan. No bugs, all 9 tests pass. Main concerns are design-level issues inherited from the plan.
