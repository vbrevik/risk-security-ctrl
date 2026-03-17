# Design: CISA ZTMM, Topic Tags, and Sidebar Filters

**Date:** 2026-03-02
**Status:** Approved

## 1. CISA Zero Trust Maturity Model (ZTMM)

New ontology file `ontology-data/cisa-ztmm.json`.

**Structure:**
- Framework ID: `cisa-ztmm`, version "2.0"
- 5 Pillars: Identity, Devices, Networks, Applications & Workloads, Data
- 3 Cross-cutting capabilities: Visibility & Analytics, Automation & Orchestration, Governance
- Maturity levels (Traditional, Advanced, Optimal) stored as concept properties, not separate concepts
- Full EN/NB translations

**Relationships** added to `relationships.json`:
- ZTMM Identity pillar → NIST CSF PR.AA, NIST 800-53 AC/IA, Zero Trust verify-explicitly
- ZTMM Devices pillar → NIST CSF PR.PS, Zero Trust device-security
- ZTMM Networks pillar → NIST 800-53 SC, Zero Trust secure-communication
- ZTMM Applications pillar → NIST CSF PR.PS, NIST 800-53 CM
- ZTMM Data pillar → NIST CSF PR.DS, NIST 800-53 SC/MP, Data Centric principles
- ZTMM Governance → NIST CSF GV, ISO 31000 leadership

**Frontend:** Graph color `#0ea5e9` (sky blue). i18n keys added for both EN and NB.

**Backend:** Add `"cisa-ztmm.json"` to import list in `import.rs`. Auto-imports on startup.

## 2. Topic Tags

New file `ontology-data/topic-tags.json` mapping cross-cutting topics to concept IDs.

**Structure:**
```json
{
  "topics": [
    {
      "id": "identity",
      "name_en": "Identity & Access Management",
      "name_nb": "Identitets- og tilgangsstyring",
      "concept_ids": ["nist-800-53-ac", "nist-800-53-ia", "nist-csf-pr-aa", ...]
    }
  ]
}
```

**Topics (10):**
1. Identity & Access Management
2. Data Protection
3. Incident Response
4. Governance & Risk
5. Supply Chain
6. Monitoring & Detection
7. Physical Security
8. Network Security
9. Awareness & Training
10. Resilience & Recovery

**Backend:** New endpoint `GET /api/ontology/topics` serves the topic-tags.json content. New endpoint `GET /api/ontology/topics/{id}/concepts` returns concept IDs for a topic.

**Import:** `topic-tags.json` loaded at startup alongside frameworks (stored in a new `topic_tags` table, or served directly from the JSON file).

## 3. Sidebar Filter Extension

Add a "Topics" section to the FilterPanel in `Sidebar.tsx`:
- Multi-select checkboxes (like frameworks)
- Placed between framework filters and concept type dropdown
- AND logic: selected topics intersect with framework and type filters
- "Clear all" resets topics too

**State changes:**
- Add `activeTopics: string[]` to `ExplorerState`
- Add `SET_ACTIVE_TOPICS` and `TOGGLE_TOPIC` actions
- Add `setActiveTopics` and `toggleTopic` to context

**i18n:** Add `filters.topics` key to ontology namespace (EN: "Topics", NB: "Emner").

## Data Flow

1. Backend loads `topic-tags.json` at startup
2. Frontend fetches topics via `GET /api/ontology/topics`
3. User selects topics in sidebar
4. Frontend filters concepts client-side by checking if concept ID is in selected topic's `concept_ids`
5. Filtered concepts passed to graph/tree/compare views
