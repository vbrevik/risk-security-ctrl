# Interview: 03-matching-engine

## Q1: Gap scope
**Q:** Include all concepts from detected frameworks or only top 2 levels?
**A:** All concepts. Comprehensive analysis even if it means 100+ gaps.

## Q2: Topics loading
**Q:** Read topic-tags.json from disk in matcher, or pass via parameter?
**A:** Pass via parameter. Route handler loads topics and passes them in. Cleaner separation, testable.
