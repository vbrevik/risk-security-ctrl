# Section 01 Code Review Interview

## Auto-fixes Applied

1. **Strengthened fallback test** — Changed from asserting hex pattern to asserting the returned color equals the index-0 color (fw-a's color).
2. **Moved empty radarData test** — Relocated from top-level `useChartData` describe into the `radarData` describe block.

## Let Go

1. **Duplicated iteration** — radarMap iterates findings separately from fwMap. Merging would reduce readability for negligible perf on <1000 items.
2. **Division by zero guard** — Impossible: radarMap only has entries from iterated findings, so total is always ≥1.
