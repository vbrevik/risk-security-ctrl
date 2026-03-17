# Integration Notes

## Integrating (5 issues)

**#1 Crosswalk data flow — concept-to-framework mapping**
Integrating. The plan must explicitly add useAllConcepts() as a dependency for the matrix and describe the lookup map. This is a real blocker.

**#3 Nav contradicts codebase**
Integrating. The plan will reference the current __root.tsx state (already has single-bar links) and specify refactoring to two-tier. The existing links are placeholders I added.

**#4 Accessibility for heatmap**
Integrating. Adding role="grid", keyboard navigation, table-view toggle, and color contrast requirements to the Crosswalk section.

**#5 Error handling for parallel requests**
Integrating. Adding explicit error handling strategy: show partial results, retry failed requests, loading skeleton until all complete.

**#9 Loading states for catalog default**
Integrating. Specifying skeleton sidebar + empty detail panel while loading, fallback to first framework when loaded.

## Partially Integrating (5 issues)

**#2 Search pagination**
Partially integrating. Will increase limit to 500 (matching useConcepts pattern). True faceting on full dataset isn't feasible without backend changes, so noting this as a known limitation. At 500 results, facets will be accurate for most queries.

**#6 Hardcoded mappings**
Noting as maintenance documentation but not changing approach — deriving domains from data would require backend schema changes. Will colocate all hardcoded IDs in a single utility file.

**#10 Comma-separated URL params**
Adding filter(Boolean) to the validateSearch implementations.

**#12 /concepts route 404**
Adding a redirect from /concepts to /concepts/search.

**#13 Testing strategy**
Expanding with specific test case lists per page.

## Not Integrating (5 issues)

**#7 Relationship type passthrough on crosswalk nav**
Not integrating — this is a nice-to-have UX enhancement, not a plan gap. Can be added during implementation.

**#8 i18n note**
Already stated in the spec. Plan uses hardcoded English strings. Not worth a plan change.

**#11 Ontology explorer auto-select verification**
The existing code already handles this (GraphView lines 97-107 auto-pan on selectedConceptId change, OntologyExplorer reads ?concept search param). No plan change needed.

**#14 Search faceting on incomplete data**
Addressed by #2 (increasing limit to 500). At that size, faceting is accurate enough.

**#15 Matrix loading state**
Addressed by #5 (explicit loading strategy). Not a separate issue.
