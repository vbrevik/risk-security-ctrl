# Section 02 Code Review: Framework Detection

## Critical

1. **Abbreviation extraction not a distinct mechanism** — Currently works by accident via tokenizer producing "nist"/"iso" (3 chars, passes min-length filter). Plan intended explicit abbreviation extraction for resilience.

## Important

2. **Framework ID prefix collision** — `starts_with("nist")` matches both "nist" and "nist-csf" concept IDs. Need delimiter guard.

3. **Direct name match bonus is flat** — Single 2.0 bonus regardless of how many name tokens match. Loses discriminating power.

4. **Duplicate concept_ids inflate scores** — Overlapping topics can contribute same concept_id twice.

## Minor

5. **Topic overlap count computed but discarded** — Binary matched/unmatched, doesn't weight by match strength.

7. **Test ordering flake** — `test_detect_frameworks_ordered_by_strength` scores iso31000=2.0 and nist-csf=2.0, making order non-deterministic.

## Nitpick

8. **frameworks param by value** — Could be `&[(String, String)]`.

9. **No tracing in detect_frameworks** — Could add debug logging.
