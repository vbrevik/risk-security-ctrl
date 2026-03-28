# Split 06: Cross-Framework Relationship Validation

## Scope

After all framework verifications complete, validate all relationship JSON files to ensure concept IDs are valid and relationships are semantically correct.

## Dependencies

Requires completion of splits 01-05 (all framework verifications).

## Relationship Files

All `ontology-data/relationships-*.json` files, including:
- relationships-nsl-sikkerhetsloven.json (already rebuilt)
- relationships-nsm-grunnprinsipper.json (already rebuilt)
- Any other relationship files in ontology-data/

## Validation Procedure

1. **Collect all concept IDs** from all framework JSON files
2. **For each relationship file:**
   - Check every `source_concept_id` exists in the concept pool
   - Check every `target_concept_id` exists in the concept pool
   - Flag any orphaned references
3. **For any broken references:**
   - If concept was renamed: update the reference
   - If concept was removed: remove the relationship
   - If concept never existed (hallucinated): remove the relationship
4. **Semantic review:** Verify relationship_type (maps_to, implements, related_to) is appropriate

## Deliverables

- [ ] All relationship files validated
- [ ] Broken references fixed or removed
- [ ] Validation report documenting changes
- [ ] `cargo test` passes
