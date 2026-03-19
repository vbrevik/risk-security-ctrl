# Prompt Contract: Section 06 - i18n and Tests

## GOAL
Add all missing i18n keys for radar chart, concept drawer, and cross-filter features. Extend i18n validation tests.

## CONTEXT
Final section (6 of 6). Most component tests already exist from sections 01-05. This section focuses on i18n keys and validation.

## CONSTRAINTS
- Follow existing i18n JSON structure (nested objects under charts, detail)
- Norwegian translations must be natural Bokmal, not machine-translated
- i18n validation tests should use it.each pattern for conciseness

## FORMAT
Files to modify:
- `frontend/src/i18n/locales/en/analysis.json` - add radar, conceptPanel, cross-filter keys
- `frontend/src/i18n/locales/nb/analysis.json` - add Norwegian equivalents
- `frontend/src/i18n/__tests__/analysis-namespace.test.ts` - add enhancement key validation

## FAILURE CONDITIONS
- SHALL NOT break existing i18n keys
- SHALL NOT leave any new key without both en and nb translations
- SHALL NOT use empty strings for translation values
