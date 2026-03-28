# Code Review: section-03-verification-badge

## Critical Issues
None.

## Moderate Issues
1. `aria-label={config.label}` uses hardcoded English string — violates project rule "never hardcode strings". Screen reader would announce English even in Norwegian locale, and aria-label shadows the visible translated span text.

## Minor Issues
None.
