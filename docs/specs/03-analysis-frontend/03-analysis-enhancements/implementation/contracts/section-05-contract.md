# Prompt Contract: Section 05 - Page Assembly and Cross-Filter Wiring

## GOAL
Wire all enhancement components (sections 01-04) together in the analysis detail page. Add cross-filter state, filter banner, 3-column chart grid with FrameworkRadar, concept drawer integration, scroll-to-table, and overrideTypeCounts support in SummaryStats.

## CONTEXT
Section 05 of 6 in analysis frontend enhancements. All component pieces exist from previous sections. This section assembles them in the detail page.

## CONSTRAINTS
- Follow existing project patterns (TanStack Router, TanStack Query, i18n, shadcn/ui)
- selectedFrameworkId derived from filters.framework_id (not separate state)
- handleBarClick uses functional updater (no dependency on filters)
- Backward compatible: SummaryStats without overrideTypeCounts works as before
- i18n keys for filter banner (detail.filteredBy, detail.clearFilter) referenced but defined in section 06

## FORMAT
Files to modify:
- `frontend/src/features/analysis/components/SummaryStats.tsx` - add overrideTypeCounts prop
- `frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx` - add 3 override tests
- `frontend/src/routes/analysis/$id.tsx` - full page assembly integration

## FAILURE CONDITIONS
- SHALL NOT break existing SummaryStats behavior when overrideTypeCounts is absent
- SHALL NOT create separate state for selectedFrameworkId (must derive from filters)
- SHALL NOT skip TDD (tests first for SummaryStats changes)
- SHALL NOT hardcode strings (use i18n keys)
