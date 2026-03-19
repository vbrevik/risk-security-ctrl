# Integration Notes: Opus Review Feedback

## Integrating

### Issue 1: SummaryStats prop changes underspecified
**Integrating.** Will specify that SummaryStats accepts an optional `overrideTypeCounts` prop. When set, only finding-type cards (addressed, gaps, partial, total) use the override. Framework count, processing time, and token count remain analysis-level and do not change.

### Issue 2: concept_id nullability
**Integrating.** `concept_id` is `string` (non-nullable) in the type. Remove the conditional guard — all findings have a concept_id. The clickable check should guard on `concept_code` or `concept_name` being non-null (since those are the visible elements).

### Issue 3: Framework color determinism
**Integrating.** Change the color utility to always take the canonical `matched_framework_ids` array. Pass this array to CoverageHeatmap as a new prop `frameworkIds` alongside the existing `data`.

### Issue 5: useConceptRelationships null guard
**Integrating.** ConceptDrawer will pass `conceptId ?? ""` to the hook, relying on the existing `enabled: !!id` guard.

### Issue 8: Keyboard accessibility
**Integrating.** Add `tabindex="0"`, `role="button"`, `aria-label` (framework name), and keydown handler (Enter/Space) to heatmap bar rects.

### Issue 11: Error state for ConceptDrawer
**Integrating.** Add error state rendering when useConceptRelationships fails — show error message with retry button.

### Issue 13: useCallback dependency
**Integrating.** Use functional updater `setFilters(prev => ...)` to remove `filters` dependency from `handleBarClick`.

## Not Integrating

### Issue 4: Missing i18n keys from spec
**Not integrating.** The spec keys `findings.conceptLink.viewDetails` and `findings.conceptLink.openExplorer` were from the Link-based approach which was replaced by the drawer approach. The plan's i18n keys under `detail.conceptPanel.*` are correct. Will add `detail.conceptPanel.loading` and `detail.conceptPanel.error` keys.

### Issue 6: 1000-finding limit warning
**Not integrating.** Pre-existing issue, out of scope for these enhancements. Worth a separate task but not part of this plan.

### Issue 7: 4-axis diamond shape
**Not integrating.** Acknowledged as a design choice. 4 axes is standard for this data (4 finding types). The angular shape is expected and acceptable.

### Issue 9: Scroll timing
**Not integrating fully.** Will use `requestAnimationFrame` as suggested but this is a minor polish item. The current approach (immediate scroll) is acceptable for MVP.

### Issue 10: D3 callback ref in radar
**Not integrating.** The radar tooltip uses D3 `.on("mouseover")` which captures the `setTooltip` React state setter. Since `setTooltip` is stable (from useState), the stale closure issue does not apply. No ref needed.

### Issue 12: Chart grid width
**Not integrating as plan change.** Will verify visually during implementation. The 220px inner width is sufficient for short bar charts.
