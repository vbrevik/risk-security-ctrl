# Interview Transcript: 03-api-matcher

## Q1: API endpoint design — where should guidance be added?

**Question:** The current get_concept handler returns a plain Concept (no relationships). There's a separate get_concept_relationships handler. Should guidance be added to the plain get_concept endpoint, or to the relationships endpoint, or should we create a new combined endpoint?

**Answer:** Extend get_concept_relationships — add guidance to the existing enriched endpoint that already has relationships.

## Q2: Matcher FTS strategy — union or replace?

**Question:** For the matcher enhancement: should we query the new concept_guidance_fts table as an ADDITIONAL candidate source (union with existing concepts_fts), or REPLACE the existing concepts_fts search with the guidance FTS?

**Answer:** Union both FTS tables — query concepts_fts AND concept_guidance_fts, merge results.

## Q3: Recommendation detail level

**Question:** When generating actionable recommendations with specific suggested actions, how many actions should be referenced per finding?

**Answer:** All actions for the concept — list every suggested action for the matched concept.

## Q4: ConceptCandidate struct extension

**Question:** The ConceptCandidate struct is used throughout the matcher. Adding about_en + action text means fetching more data per candidate. Should we add these fields to ConceptCandidate, or keep it lean and do a separate query during scoring?

**Answer:** Add to ConceptCandidate — simpler code, all candidates carry guidance data.

## Q5: FTS column weights

**Question:** Should the guidance FTS results use the existing bm25 column weights, or should we configure custom weights favoring about_en over definition_en?

**Answer:** Custom weights — about=5, definition=3, name=10. Tune for guidance-enriched search.
