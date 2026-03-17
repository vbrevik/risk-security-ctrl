# Code Review: Section 02 - Navigation Architecture

## Major Issues

### 1. Crosswalk route deviates from plan spec
The existing crosswalk route uses different params (source, target, level) and imports a full CrosswalkView component. This pre-dates our work — it was created in a parallel session.

### 2. Extra i18n and component changes outside section scope
Diff includes locale additions and component exports from parallel work sessions.

## Moderate Issues

### 3. Tests do not test the actual __root.tsx component
Tests build a separate TestRootLayout rather than importing the real component. Tests are testing a mock.

### 4. validateSearch param names mismatch for crosswalk
Plan specifies fw1/fw2/type but existing route uses source/target/level.

## Minor Issues

### 5. No data-testid on actual __root.tsx
Tests rely on testids that don't exist in the real component.

### 6. Unsafe type assertions in validateSearch
Uses `as string` cast on unknown values instead of typeof checks.

### 7. Missing test for concepts redirect
No test verifying /concepts redirects to /concepts/search.
