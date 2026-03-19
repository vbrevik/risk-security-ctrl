# Opus Review

**Model:** claude-opus-4
**Generated:** 2026-03-19T14:30:00Z

---

## 1. SummaryStats Filtering Approach Is Underspecified and Inconsistent

Section 5 says to pass `filteredTypeCounts` via `useMemo` to `SummaryStats`, but the current `SummaryStats` takes the full `ChartData` object, not a separate `typeCounts` prop. The plan says to modify SummaryStats to "accept filtered type counts" but does not specify what the new prop signature should be.

Moreover, `SummaryStats` also displays `analysis.matched_framework_ids.length` as the framework count. When filtering by a single framework, should this show "1" or still the total? The `processingTime` and `tokenCount` cards are analysis-level and should not change when filtering -- this distinction is not called out.

**Recommendation:** Specify the exact prop changes for `SummaryStats`. Consider splitting the interface to accept an optional `overrideTypeCounts` prop, and clarify which cards are affected by framework filtering and which are not.

## 2. ConceptDrawer: `concept_id` Is Always Present in the Type

Section 4 says "Only make clickable when `finding.concept_id` exists" but `concept_id` is typed as `string` (non-nullable) in `AnalysisFinding`. It is `concept_code` and `concept_name` that are `string | null`.

**Recommendation:** Verify with the backend whether `concept_id` can actually be null. If it cannot, remove the conditional guard from the plan.

## 3. Framework Color Determinism Has a Fragile Assumption

Section 1 says color is determined by "the framework's index in the sorted array mod 10." But different components could pass different arrays, causing the same framework to get different colors. The heatmap does not receive a `frameworkIds` prop.

**Recommendation:** Either always pass the canonical `analysis.matched_framework_ids` to every component, or use a deterministic hash instead of index-based assignment.

## 4. Spec vs. Plan Disagreement on i18n Keys

The spec lists `findings.conceptLink.viewDetails` and `findings.conceptLink.openExplorer` as i18n keys that do not appear in the plan's i18n section. Also missing: `detail.conceptPanel.loading` for skeleton state.

## 5. `useConceptRelationships` Needs an `enabled` Guard for `null`

The hook signature is `useConceptRelationships(id: string)`, not `string | null`. Passing `null` will cause a type error.

**Recommendation:** Pass `conceptId ?? ""` with the `enabled` guard relying on the falsy check.

## 6. The 1000-Finding Limit Is a Silent Data Truncation Risk

Pre-existing issue. Charts and stats operate on potentially truncated dataset. Adding a warning when `total > 1000` would be valuable.

## 7. Radar Chart: 4 Axes Produces a Diamond Shape

Worth noting that with only 4 axes, the visual will be quite angular. This is a design judgment call.

## 8. Missing Keyboard Accessibility for Heatmap Cross-Filter

SVG rects are not focusable by default. Need `tabindex="0"`, `role="button"`, `aria-label`, and keydown handler.

## 9. Scroll-to-Table Timing Issue

`scrollIntoView()` fires before the new query data loads. Consider deferring via `requestAnimationFrame`.

## 10. D3 Callback Ref Pattern

Section 2 correctly identifies stale closure problem. Section 3 (FrameworkRadar) should also address this for tooltip handlers.

## 11. Missing Error State for ConceptDrawer

Plan mentions loading skeleton but not error state for failed `useConceptRelationships`.

## 12. Chart Grid Width

At `xl:grid-cols-3`, each column is ~400px. Heatmap's inner width would be ~220px after margins. Should be visually verified.

## 13. `useCallback` Dependency

`handleBarClick` should use functional updater for `setFilters` to avoid depending on `filters` state, preventing unnecessary re-renders.

---

## Summary

Main issues:
1. **Type mismatch** on `concept_id` nullability -- will cause build error or incorrect behavior
2. **Color determinism fragility** -- different components may assign different colors
3. **Missing keyboard accessibility** -- gap for interactive feature
4. **SummaryStats prop changes underspecified** -- will cause implementation ambiguity
5. **Hook type mismatch** for null conceptId -- will cause TypeScript error
