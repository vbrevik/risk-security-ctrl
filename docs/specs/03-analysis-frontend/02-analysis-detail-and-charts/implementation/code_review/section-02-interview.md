# Section 02 Code Review Interview

## Auto-fixes applied

1. **toBeDefined → toBeInTheDocument** — Fixed all SummaryStats test assertions to use the correct matcher.

## Let go

- Hardcoded en-US locale for number formatting — reasonable default, can be localized later
- Loading skeleton reads chartData — contract is fine, parent always provides valid data
- Unknown finding_type silent swallow — TypeScript union prevents this at compile time
- Missing null formatting tests — trivial helpers, low risk
- EMPTY_CHART_DATA mutability — reverted freeze approach due to TypeScript complexity; shared default arrays are never mutated by consumers
