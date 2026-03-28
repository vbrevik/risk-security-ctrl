# MITRE ATLAS — Verification Proof

**Source:** https://atlas.mitre.org/
**Reference:** MITRE ATLAS v5.4.0 — Adversarial Threat Landscape for Artificial Intelligence Systems
**Verified:** 2026-03-28
**Status:** verified (rebuilt from official data)

## How to Verify

1. **ATLAS Website:** https://atlas.mitre.org/ — official site with all tactics and techniques
2. **ATLAS GitHub:** https://github.com/mitre-atlas/atlas-data — official data repository
3. **ATLAS YAML (raw data):** https://raw.githubusercontent.com/mitre-atlas/atlas-data/main/dist/ATLAS.yaml
4. **Individual techniques:** https://atlas.mitre.org/techniques/{ID} (e.g., https://atlas.mitre.org/techniques/AML.T0051)
5. **Individual tactics:** https://atlas.mitre.org/tactics/{ID} (e.g., https://atlas.mitre.org/tactics/AML.TA0005)

### Key Verification Points
- ATLAS v5.4.0 has 16 tactics (non-sequential IDs: AML.TA0000-AML.TA0015)
- Uses "AI" terminology throughout (not "ML" — changed in v4.x→v5.x transition)
- Techniques use AML.T0000-AML.T0075 range (non-sequential, with gaps)
- Kill chain order: Reconnaissance → Resource Development → Initial Access → AI Model Access → Execution → Persistence → Privilege Escalation → Defense Evasion → Credential Access → Discovery → Lateral Movement → Collection → AI Attack Staging → Command and Control → Exfiltration → Impact

## Verification Results

### Issues Found — COMPLETE REBUILD

The original file (v4.6.1, 55 concepts) had severe data quality issues:

1. **CRITICAL: Fabricated tactic IDs.** Original used sequential AML.TA0001-AML.TA0012 (12 tactics). Official ATLAS has 16 tactics with non-sequential IDs (AML.TA0000-AML.TA0015).
2. **CRITICAL: Wrong technique names.** Many technique names were fabricated (e.g., "Compromise ML Development Environment" doesn't exist in ATLAS).
3. **CRITICAL: Wrong technique IDs.** Multiple technique IDs mapped to wrong techniques.
4. **MAJOR: Outdated terminology.** Used "ML" throughout instead of "AI" (ATLAS switched to AI terminology).
5. **MAJOR: Missing 4 tactics.** Privilege Escalation (TA0012), Credential Access (TA0013), Command and Control (TA0014), Lateral Movement (TA0015) were entirely absent.
6. **MAJOR: Missing tactic "AI Model Access" (TA0000).** A distinctive ATLAS tactic not present in ATT&CK.

### Rebuild Details
- **Source:** Official ATLAS YAML from GitHub (v5.4.0)
- **Method:** All 16 tactic IDs and names verified against official YAML. All 64 top-level techniques verified with correct IDs, names, and tactic assignments.
- **Excluded:** Subtechniques (e.g., AML.T0051.000, AML.T0051.001) — only top-level techniques included to match ontology granularity pattern.

### Concept Count
81 concepts (rebuilt from 55) — 1 category + 16 tactics + 64 techniques. All verified against official ATLAS v5.4.0 data.
