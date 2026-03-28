# Federated Mission Networking (FMN) — Verification Proof

**Source:** https://www.nato.int/cps/en/natohq/topics_157575.htm
**Reference:** NATO Federated Mission Networking, Spiral 5/6
**Verified:** 2026-03-28
**Status:** partially-verified (significant issues remain)

## How to Verify

1. **NATO ACT overview:** https://www.act.nato.int/activities/federated-mission-networking/
2. **FMN Spiral 5 Standards Profile (NISP):** https://nisp.nw3.dk/capabilityprofile/fmn5-20231123.html
3. **Wikipedia:** https://en.wikipedia.org/wiki/Federated_Mission_Networking
4. **FMN Public Site:** https://coi.nato.int/FMNPublic/SitePages/Home.aspx
5. **Local reference docs:** /Users/vidarbrevik/projects/fmn_security_tests/ — Spiral 5 CSRS, Spiral 6 Service Instructions (21 PDFs)

### Key Verification Points
- FMN Spiral 6 finalized 14 Nov 2025 (confirmed by SI document dates)
- Official structure: Governance, FMN Framework, Mission Networks (NOT numbered principles)
- 21 Service Instruction areas in Spiral 6 (NOT the 6 invented service areas in the file)
- CSRS structured around ISO/IEC 27001:2013 control clauses (3.1-3.14)

## Verification Results

### Issues Found — Corrections Applied
1. **CRITICAL: Fabricated principle codes removed.** FP1-FP4 codes don't exist in official FMN. Source references updated to "FMN design theme (editorial summary)".
2. **CRITICAL: Spiral 5 definition updated.** Removed "current specification" claim.
3. **LOW: Norwegian typo fixed.** "etterretningtjenester" → "etterretningstjenester".

### Issues Documented — Require Deeper Restructuring
4. **CRITICAL: Service areas are fabricated.** The 6 service areas (CIS, GEOINT, COLLAB, VUC, PNT) don't match actual FMN Service Instruction areas. Official Spiral 6 has 21 distinct service areas (Audio/Video Collaboration, CIS Security, Communications Transport, Digital Certificates, Domain Naming, Formal Messaging, Friendly Force Tracking, Geospatial Information, etc.).
5. **MAJOR: Architecture component codes fabricated.** FIDM→should be "Web Authentication" (IAM framework); SD→should be "Service Management and Control" (SMC/FSMC); MDR→not an official component; MSG→should split into "Formal Messaging" and "Informal Messaging"; ISA→should be CSRS/SoC process; CDS→subsumed under Communications Security.
6. **MAJOR: Missing FMN governance concepts.** Security Accreditation Board (SAB), CSRS, Statement of Compliance (SoC), Mission Network Participant (MNP), MNPSOC, C-SOC, SAA — all official named entities absent from ontology.
7. **MEDIUM: Version outdated.** Spiral 6 is current since Nov 2025.

### Confirmed Correct
- Framework name, general description, and overarching themes (federation, interoperability, SOA, security) are all accurate
- Spiral-based development model is correctly represented
- Collaboration and geospatial services exist (though named differently in official docs)

### Concept Count
20 concepts — 4 categories + 4 principles + 6 components + 5 service areas + 1 specification. Principle codes removed but deeper restructuring needed for service areas and components (~71% of concepts use fabricated names/codes).
