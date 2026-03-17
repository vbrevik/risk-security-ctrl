# Opus Review: Framework Explorer Insight Pages

## Priority Issues

1. **Crosswalk data flow incomplete** — Matrix needs concept-to-framework lookup but plan omits useAllConcepts dependency
2. **Search pagination cap at 50** — Backend defaults to 50 results, faceting inaccurate on partial data
3. **Nav contradicts codebase** — Plan says two-tier but __root.tsx already has single-bar Frameworks/Crosswalk links
4. **Accessibility unaddressed for heatmap** — Government tool needs WCAG: keyboard nav, table alternative, color contrast
5. **22 parallel requests** — useAllConcepts fires 22 HTTP requests with no error handling strategy

## Other Issues

6. Hardcoded domain/landscape mappings are maintenance liabilities
7. Missing relationship type passthrough on crosswalk navigation from catalog
8. No i18n for new pages (spec says skip, plan should note explicitly)
9. Missing loading/error states for framework catalog default selection
10. Comma-separated URL params need filter(Boolean) to avoid empty string bugs
11. Concept links to ontology explorer need verification that auto-select works
12. /concepts route has no index (404 at /concepts)
13. Testing strategy too vague — no specific test cases listed
14. Search faceting on incomplete data acknowledged but not solved
15. Matrix loading state unspecified (waiting for 22+ requests)
