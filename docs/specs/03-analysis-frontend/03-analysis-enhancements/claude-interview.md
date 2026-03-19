# Interview Transcript: Analysis Detail Page Enhancements

## Q1: Radar Chart Axis Design

**Question:** For the radar chart, should each axis represent a finding type (addressed, partial, gap, n/a) showing counts, or should axes represent frameworks showing overall coverage percentage?

**Answer:** Finding-type axes, normalized % — 4 axes showing percentage (0-100%) instead of raw counts. Better comparison across frameworks of different sizes.

## Q2: Cross-Filter Scope

**Question:** When clicking a heatmap bar to filter findings, should the filter also sync with the radar chart or only affect the findings table?

**Answer:** Filter everything — bar click filters table, highlights radar, and updates summary stats to show only that framework.

## Q3: Concept Link Target

**Question:** For the concept links in the findings table, should clicking a concept open the ontology explorer in the same tab, new tab, or a side panel?

**Answer:** Side panel / drawer — show concept details in a slide-out panel on the analysis page itself. Most integrated approach.

## Q4: Chart Grid Layout

**Question:** For the chart grid layout with 3 charts, which arrangement?

**Answer:** 3-column equal on desktop — all three charts side by side on wide screens (xl breakpoint). Compact but each chart gets equal space.

## Q5: Concept Side Panel Scope

**Question:** What information should the concept side panel show? Lightweight basics, full ontology ContextPanel reuse, or basics + link?

**Answer:** Full context — reuse the ontology ContextPanel component which shows hierarchy, relationships, and cross-mappings. Requires concept API call.

## Q6: Overall Scope

**Question:** Should we scope to just the 3 enhancements or add anything else?

**Answer:** Just the 3 enhancements. Keep it focused.

---

## Summary of Design Decisions

1. **Radar chart:** 4 finding-type axes (addressed, partial, gap, n/a) with normalized percentages. Each framework = one polygon overlay. Max 8 frameworks.

2. **Cross-filtering:** Clicking a heatmap bar filters everything — findings table, radar chart highlights, and summary stats recalculate for selected framework only. Toggle behavior (click again to clear).

3. **Concept interaction:** Side panel/drawer reusing the ontology ContextPanel. Opens when clicking a concept in the findings table. Shows full concept context (hierarchy, relationships, cross-mappings). Requires `useConceptRelationships` API call.

4. **Chart layout:** 3-column equal grid on xl breakpoint, 2-column on lg, stacked on mobile.

5. **Scope:** Only these 3 enhancements.
