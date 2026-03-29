# Backlog

## Verification Status (2026-03-28)

Splits 01-06 of ontology verification are **complete**. 33 framework JSON files, 32 proof files, 585 cross-framework relationships. All backend tests pass.

| Status | Count | Frameworks |
|--------|-------|------------|
| verified | 19 | cisa-ztmm, cve-cwe, data-centric, eu-ai-act, gdpr, google-saif, iso27000, mitre-atlas, nis2, nist-ai-rmf, nist-c-scrm, nist-csf, nist-privacy-framework, nist-rmf, nist-ssdf, nsl-sikkerhetsloven, nsm-grunnprinsipper, xai-dataops, zero-trust |
| corrected | 8 | cer-directive, dora, iso10015, iso31010, iso42005, iso9000, nist-ai-genai, nist-sp-800-53 |
| structure-verified | 5 | iso23894, iso24028, iso31000, iso42001, mitre-attack |
| partially-verified | 1 | fmn |

## Pending Verification Work

- [ ] **FMN deeper restructuring** — 71% of concepts have fabricated names/codes. 6 service areas should be replaced with 21 official Spiral 6 Service Instruction areas. Architecture component codes need renaming. See `docs/sources/fmn-proof.md`.
- [x] **NSM Grunnprinsipper** — verified 2026-03-28: all 118 control definitions enriched with substantive guidance text from official NSM website.

## New Frameworks to Add

Identified during verification as having unique structural concepts not covered by existing ontology data.

---

# NIST Frameworks

## HIGH Priority

| # | Publication | Est. Concepts | Source | Notes |
|---|------------|--------------|--------|-------|
| ~~1~~ | ~~**NIST Privacy Framework 1.1**~~ | ~~DONE~~ | — | Built as v1.0 (124 concepts: 5 functions, 18 categories, 100 subcategories). v1.1 is IPD only (Apr 2025). |
| ~~2~~ | ~~**NIST SP 800-218 (SSDF) v1.1**~~ | ~~DONE~~ | — | Built: 66 concepts (4 groups, 19 practices, 42 tasks). |
| ~~3~~ | ~~**NIST SP 800-161 Rev 1 (C-SCRM)**~~ | ~~DONE~~ | — | Built: 35 concepts (3 org levels, 3 practice tiers, 8 key practices). |

## MEDIUM Priority

| # | Publication | Est. Concepts | Source | Notes |
|---|------------|--------------|--------|-------|
| 4 | **NIST SP 800-160 Vol 2 Rev 1** | ~30 | csrc.nist.gov | Cyber resiliency: 4 goals, 14 techniques (Adaptive Response, Deception, Diversity, etc.) |
| 5 | **NIST SP 800-63-4** | ~20 | csrc.nist.gov | Digital Identity: IAL/AAL/FAL assurance levels + DIRM framework |
| 6 | **NIST SP 800-53B** | metadata | csrc.nist.gov | Control baselines (Low/Moderate/High/Privacy). Better as properties on existing 800-53 data. |
| 7 | **NIST SP 800-30 Rev 1** | ~25 | csrc.nist.gov | Risk assessment methodology: threat source/event taxonomies. Complements ISO 31010. |

## LOW Priority

| # | Publication | Notes |
|---|------------|-------|
| 8 | SP 800-171 Rev 3 | Subset of 800-53 — model as profile/overlay, not separate framework |
| 9 | SP 800-53A Rev 5 | Assessment procedures — metadata on existing 800-53 controls |
| 10 | SP 800-39 | Overlaps with ISO 31000 + NIST RMF |
| 11 | SP 800-160 Vol 1 Rev 1 | Systems engineering processes — implementation guidance |
| 12 | SP 800-61 Rev 3 | Incident handling — already covered by CSF RS/RC + 800-53 IR |

## ~~Also: Complete NIST CSF 2.0 Subcategories~~ — DONE

All 106 subcategories already present and verified (commit b5fad54). 3 erroneous CSF 1.x codes removed.

---

# ISO/IEC Standards

## HIGH Priority

| # | Standard | Est. Concepts | Source | Notes |
|---|----------|--------------|--------|-------|
| 1 | **ISO/IEC 27005:2022** | ~30 | [iso.org](https://www.iso.org/standard/80585.html) | Info security risk management. Bridge between ISO 31000 and 27001. Operationalizes risk management for ISMS. |
| 2 | **ISO/IEC 27701:2019** | ~40 | [iso.org](https://www.iso.org/standard/71670.html) | Privacy information management (PIMS). Extension to 27001/27002. Directly supports GDPR compliance mapping. |
| 3 | **ISO/IEC 27017:2015** | ~25 | [iso.org](https://www.iso.org/standard/43757.html) | Cloud security controls supplementing 27002. Essential for governmental cloud compliance. |
| 4 | **ISO/IEC 27018:2025** | ~20 | [iso.org](https://www.iso.org/standard/27018) | PII protection in public clouds. Links to GDPR + 27001 for cloud PII processing. |
| 5 | **ISO/IEC 27035-1:2023** | ~25 | [iso.org](https://www.iso.org/standard/78973.html) | Incident management. Core for NIS2 compliance (mandatory reporting). Maps to CSF Detect/Respond. |
| 6 | **ISO 22301:2019** | ~30 | [iso.org](https://www.iso.org/standard/75106.html) | Business continuity management. Mandated by NIS2, DORA, CER. Maps to CSF Recover. |
| 7 | **ISO/IEC 38500:2024** | ~15 | [iso.org](https://www.iso.org/standard/81684.html) | IT governance (Evaluate/Direct/Monitor). Top-level governance for governmental IT. |
| 8 | **ISO/IEC 27014:2020** | ~15 | [iso.org](https://www.iso.org/standard/74046.html) | Information security governance. Connects 38500 IT governance to 27001 ISMS. |
| 9 | **ISO 37301:2021** | ~25 | [iso.org](https://www.iso.org/standard/75080.html) | Compliance management systems. Meta-framework for how compliance is managed. |
| 10 | **ISO/IEC 22989:2022** | ~30 | [iso.org](https://www.iso.org/standard/74296.html) | AI concepts and terminology. Foundational vocabulary for all 42xxx AI standards. |
| 11 | **ISO/IEC 5338:2023** | ~25 | [iso.org](https://www.iso.org/standard/81118.html) | AI system lifecycle processes. Connects 42001 management to practical AI development. |
| 12 | **ISO/IEC 23053:2022** | ~20 | [iso.org](https://www.iso.org/standard/74438.html) | ML pipeline framework. Technical architecture concepts for AI/ML systems. |
| 13 | **ISO/IEC 27036 (Parts 1-4)** | ~30 | [iso.org](https://www.iso.org/standard/82890.html) | Supplier relationship security. Supply chain security required by NIS2. |

## MEDIUM Priority

| # | Standard | Est. Concepts | Notes |
|---|----------|--------------|-------|
| 14 | **ISO/IEC 27037:2012** | ~15 | Digital evidence handling. Supports incident response and forensics. |
| 15 | **ISO/IEC TS 27110:2021** | ~10 | Cybersecurity framework development guidelines. Meta-framework validation. |
| 16 | **ISO/IEC 27003:2017** | ~15 | ISMS implementation guidance. Useful for compliance checklists. |
| 17 | **ISO/IEC 27004:2016** | ~15 | ISMS monitoring/measurement. Defines control effectiveness measurement. |
| 18 | **ISO/IEC TR 24027:2021** | ~15 | AI bias assessment. Key EU AI Act requirement for high-risk AI. |
| 19 | **ISO/IEC 25059:2023** | ~20 | AI quality model (SQuaRE extension). Measurable AI quality criteria. |
| 20 | **ISO/IEC TR 24029-1:2021** | ~10 | Neural network robustness assessment. EU AI Act robustness requirement. |
| 21 | **ISO/IEC 5259 (Parts 1-5)** | ~25 | AI/ML data quality. EU AI Act training data requirement. |
| 22 | **ISO/TS 31050:2023** | ~15 | Emerging risk management. Extends ISO 31000 for novel/AI risks. |
| 23 | **ISO 31022:2020** | ~15 | Legal risk management. Regulatory compliance risk tracking. |
| 24 | **ISO 37000:2021** | ~15 | Organizational governance. Top-level context for 38500 and 27014. |
| 25 | **ISO/IEC 27032:2023** | ~15 | Internet security / cybersecurity guidelines. |
| 26 | **ISO/IEC 27031:2011** | ~15 | ICT readiness for business continuity. Bridges 22301 to ICT. |
| 27 | **ISO/IEC 27400:2022** | ~15 | IoT security and privacy. Government IoT deployments. |
| 28 | **ISO/IEC 42006:2025** | ~10 | AI management system certification requirements. |
| 29 | **ISO/IEC 42002** | ~15 | AI governance vocabulary. Terminology alignment for ontology. |

## LOW Priority

| # | Standard | Notes |
|---|----------|-------|
| 30 | ISO/IEC 27034 (multi-part) | Application security — more relevant to dev teams than compliance ontology |
| 31 | ISO/IEC 27033 (multi-part) | Network security — highly technical controls |
| 32 | ISO/IEC TR 24030:2024 | AI use cases — reference material, not normative requirements |
| 33 | ISO/IEC 27041/27042/27043 | Digital forensics trilogy — specialized, not core to governance |
| 34 | ISO/IEC 27557:2022 | Privacy risk management — covered by 27701 + GDPR |
| 35 | ISO/IEC 42003 | AI implementation guidance — under development |

---

# MITRE Frameworks

## HIGH Priority

| # | Framework | Est. Concepts | Source | Notes |
|---|-----------|--------------|--------|-------|
| 1 | **MITRE D3FEND** | ~750 | [d3fend.mitre.org](https://d3fend.mitre.org/) | Defensive countermeasures knowledge graph. OWL ontology. Directly complements ATT&CK with built-in ATT&CK mappings. |
| 2 | **MITRE CAPEC** | ~650 | [capec.mitre.org](https://capec.mitre.org/) | Common Attack Pattern Enumeration. Bridges CWE ↔ ATT&CK with official mappings to both. |
| 3 | **MITRE CREF** | ~200 | [cref.mitre.org](https://cref.mitre.org/) | Cyber Resiliency Engineering Framework. 4 goals, 14 techniques. Maps to NIST CSF + ATT&CK. Based on SP 800-160 Vol 2. |
| 4 | **MITRE System of Trust (SoT)** | ~300 | [sot.mitre.org](https://sot.mitre.org/) | Supply chain risk assessment. Trust categories → risk areas → factors → subfactors. Maps to ISO 31000. |

## MEDIUM Priority

| # | Framework | Est. Concepts | Notes |
|---|-----------|--------------|-------|
| 5 | **MITRE ENGAGE** | ~70 | Adversary engagement/deception framework. Small, ATT&CK-mapped. |
| 6 | **MITRE ATT&CK for ICS** | ~100 | Critical infrastructure threats. Relevant for Sikkerhetsloven compliance. |

## LOW Priority

| # | Framework | Notes |
|---|-----------|-------|
| 7 | MITRE EMB3D | Embedded/IoT threats — specialized |
| 8 | ATT&CK for Mobile | Mobile threats — narrow scope |
| 9 | MITRE FiGHT | 5G threats — very specialized |
| 10 | CVE | Instance data (250K+) — better as data feed, not ontology |

### Cross-Framework Mapping Note
D3FEND and CAPEC ship with built-in MITRE mappings (D3FEND↔ATT&CK, CAPEC↔CWE+ATT&CK), reducing custom crosswalk work.

---

## Industry & Vendor Frameworks (Added 2026-03-28)

### Cloud Security

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | CSA CCM v4 | Cloud Controls Matrix — 197 controls across 17 domains, maps to ISO 27001/NIST 800-53 |
| HIGH | BSI C5 | Cloud Computing Compliance Criteria Catalogue — German federal standard |
| HIGH | FedRAMP | US federal cloud authorization — based on NIST 800-53, 3 impact levels |
| HIGH | EUCS | EU Cloud Services Certification Scheme — ENISA-developed |
| MEDIUM | CSA STAR | Self-assessment/audit/certification layers on top of CCM |

### DevSecOps & Supply Chain

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | SLSA v1.0 | Supply-chain Levels for Software Artifacts — 4 build levels |
| HIGH | OWASP SAMM v2 | Software Assurance Maturity Model — 5 business functions × 3 practices |
| MEDIUM | OWASP Top 10 | Web app security risks — well-known, maps to CWE |
| MEDIUM | OpenSSF Scorecard | Automated supply chain risk checks for open source |

### AI Governance (beyond existing NIST AI RMF)

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | OECD AI Principles | 5 principles + 5 recommendations — referenced by EU AI Act |
| MEDIUM | EU HLEG / ALTAI | Assessment List for Trustworthy AI — 7 requirements |
| MEDIUM | UNESCO AI Ethics | Recommendation on Ethics of AI — 10 principles |
| MEDIUM | Canada AIA | Algorithmic Impact Assessment — 4 impact levels |

### Threat Modeling

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | STRIDE | Microsoft threat model — 6 categories, maps naturally to MITRE ATT&CK |
| HIGH | LINDDUN | Privacy threat modeling — 7 categories, complements STRIDE |
| MEDIUM | PASTA | Process for Attack Simulation and Threat Analysis — 7-stage process |
| MEDIUM | OCTAVE | Operationally Critical Threat, Asset, and Vulnerability Evaluation |

### Privacy

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | ISO/IEC 29100 | Privacy framework — 11 principles, foundation for 27701 |
| MEDIUM | ISO/IEC 29134 | Privacy impact assessment guidelines |
| MEDIUM | NIST SP 800-188 | De-identification of government datasets |

### Sector-Specific

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | IEC 62443 | Industrial automation/control system security — 4-level security assurance |
| HIGH | EUCS | EU cybersecurity certification (NIS2-aligned) |
| MEDIUM | PCI DSS v4.0 | Payment card industry — 12 requirements, 6 goals |
| MEDIUM | NERC CIP | Critical infrastructure protection for energy sector |

### National & Governmental

| Priority | Framework | Notes |
|----------|-----------|-------|
| HIGH | BSI IT-Grundschutz | German federal IT baseline protection — maps to ISO 27001 |
| HIGH | UK NCSC CAF | Cyber Assessment Framework — 4 objectives, 14 principles |
| HIGH | ANSSI EBIOS RM | French risk management method — 5 workshops |
| HIGH | ENISA NIS2 Guidance | EU NIS2 Directive implementation guidance |
| HIGH | Digdir Referansekatalog | Norwegian digital standards catalogue — directly relevant to project scope |

### Deprioritized

| # | Framework | Reason |
|---|-----------|--------|
| 1 | SOC 2 Type II | Audit/attestation framework, not ontological structure |
| 2 | HITRUST CSF | Healthcare-specific, overlaps heavily with NIST 800-53 |
| 3 | TISAX | Automotive-specific (VDA ISA), very narrow scope |

---

## Frontend Proof View Feature — DONE (2026-03-28)

Backend: migration 005 adds `verification_status/date/source/notes` columns, `GET /api/ontology/frameworks/{id}/proof` endpoint returns metadata + markdown. Frontend: all items implemented and committed.

- [x] API hook `useFrameworkProof` for `GET /api/ontology/frameworks/{id}/proof`
- [x] `ProofPanel` component with lazy proof fetching and rendered markdown
- [x] `VerificationBadge` component (WCAG-compliant, color-coded status display)
- [x] Integrated into `FrameworkProfile` page
- [x] i18n keys added for all proof view strings
- [x] `pnpm build` passes

---

# Execution Plan

1. ~~Complete verification splits 01-06~~ DONE (2026-03-28)
2. ~~Frontend proof view feature~~ DONE (2026-03-28)
3. Pending verification work (FMN restructuring, NSL/NSM GP)
4. Build HIGH priority NIST frameworks (Privacy Framework, SSDF, C-SCRM)
4. Build HIGH priority MITRE frameworks (D3FEND, CAPEC, CREF, SoT) — leverage built-in cross-mappings
5. Build HIGH priority ISO standards (27005, 27701, 27017, 27018, 27035, 22301, 38500, etc.)
6. Build HIGH priority cloud security frameworks (CSA CCM, BSI C5, FedRAMP, EUCS)
7. Build HIGH priority DevSecOps frameworks (SLSA, OWASP SAMM)
8. Build HIGH priority threat modeling frameworks (STRIDE, LINDDUN)
9. Build HIGH priority national/governmental frameworks (BSI Grundschutz, NCSC CAF, ANSSI EBIOS, ENISA NIS2, Digdir)
10. Build MEDIUM priority frameworks as needed
11. Add cross-framework relationships for all new frameworks
