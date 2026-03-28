# Code Review Interview: section-06-i18n-and-tests

## Critical auto-fix: Missing `proof.status.unknown` key — applied
Added `"unknown": "Unknown"` to `proof.status` in `en/ontology.json` and `"unknown": "Ukjent"` to `nb/ontology.json`. `VerificationBadge` calls `t("proof.status.unknown")` for null/unrecognized statuses; without the key the Norwegian locale would show the English fallback "Unknown".

## Moderate: `common.source` duplicates `detail.source` — let go
Both keys translate "Source"/"Kilde". `FrameworkProfile` uses `t("common.source", "Source")` for the framework-level source link while `detail.source` is used in the concept detail panel. Keeping them separate is intentional — different semantic contexts. No change made.

## Minor: Unconditional VerificationBadge rendering — per spec, no change
Badge renders for null-status frameworks in "unknown" style. This is confirmed by the FW_B test which asserts the badge is present. The design intent is to always show verification status, including when unknown. No change.
