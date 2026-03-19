# Section 05 Code Review

## High Severity

### 1. filteredTypeCounts computed from a capped dataset (limit: 1000)
The `filteredTypeCounts` memo filters `allFindingsData.data`, which comes from `useFindings(id, { limit: 1000 })`. If an analysis has more than 1000 findings, the type counts will be silently wrong.

## Medium Severity

### 2. Plan mentions 4 finding-type cards, implementation only overrides 3
The `partiallyAddressed` and `notApplicable` counts are computed but never displayed. Dead code in filteredTypeCounts.

### 3. Five iterations over the filtered array instead of one
A single `reduce()` pass would be cleaner and more efficient.

## Low Severity

### 4. handleBarClick has empty dependency array
ESLint `react-hooks/exhaustive-deps` may flag this.

### 5. Filter banner clear button creates new function every render
Inconsistent with care taken to memoize handleBarClick.

### 6. Test fragility: asserting absence of '58' and '42'
Could break if those numbers appear in analysis-level cards.

## Failure Condition Check
All 4 failure conditions PASS.
