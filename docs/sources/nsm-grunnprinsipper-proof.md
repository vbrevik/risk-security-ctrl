# NSM Grunnprinsipper for IKT-sikkerhet v2.1 — Verification Proof

**Source:** https://nsm.no/regelverk-og-hjelp/rad-og-anbefalinger/ta-i-bruk-grunnprinsippene/grunnprinsipper-for-ikt-sikkerhet-2-1
**Reference:** NSM Grunnprinsipper for IKT-sikkerhet, versjon 2.1
**Verified:** 2026-03-28
**Status:** partially-verified

## How to Verify

1. **NSM official page:** https://nsm.no/regelverk-og-hjelp/rad-og-anbefalinger/ta-i-bruk-grunnprinsippene/grunnprinsipper-for-ikt-sikkerhet-2-1
2. **Category structure:** Compare JSON top-level categories (1–4) and subcategories against NSM page
3. **Control titles:** Each control (e.g. 1.1.1) should match NSM's title exactly
4. **Control definitions:** NSM provides descriptive guidance per control — current JSON definitions are thin (see below)

## Official Structure

NSM Grunnprinsipper v2.1 has **4 categories**, **21 subcategories**, and **118 individual controls (tiltak)**:

| Category | Title (NO) | Subcategories |
|----------|-----------|--------------|
| 1 | Identifisere og kartlegge | 1.1, 1.2, 1.3 |
| 2 | Beskytte og opprettholde | 2.1–2.10 |
| 3 | Oppdage | 3.1–3.4 |
| 4 | Håndtere og gjenopprette | 4.1–4.4 |

### Subcategories

**Category 1 — Identifisere og kartlegge:**
- 1.1 Kartlegg styringsstrukturer, leveranser og understøttende systemer
- 1.2 Kartlegg enheter og programvare
- 1.3 Kartlegg brukere og behov for tilgang

**Category 2 — Beskytte og opprettholde:**
- 2.1 Ivareta sikkerhet i anskaffelses- og utviklingsprosesser
- 2.2 Etabler en sikker IKT-arkitektur
- 2.3 Ivareta en sikker konfigurasjon
- 2.4 Beskytt virksomhetens nettverk
- 2.5 Kontroller dataflyt
- 2.6 Ha kontroll på identiteter og tilganger
- 2.7 Beskytt data i ro og i transitt
- 2.8 Beskytt e-post og nettleser
- 2.9 Etabler evne til gjenoppretting av data
- 2.10 Integrer sikkerhet i prosess for endringshåndtering

**Category 3 — Oppdage:**
- 3.1 Oppdag og fjern kjente sårbarheter og trusler
- 3.2 Etabler sikkerhetsovervåkning
- 3.3 Analyser data fra sikkerhetsovervåkning
- 3.4 Gjennomfør inntrengningstester

**Category 4 — Håndtere og gjenopprette:**
- 4.1 Forbered virksomheten på håndtering av hendelser
- 4.2 Vurder og klassifiser hendelser
- 4.3 Kontroller og håndter hendelser
- 4.4 Evaluer og lær av hendelser

## Verification Results

### Confirmed Correct
- 4 top-level category names, codes, and ordering match official source exactly
- All 21 subcategory names and codes verified against NSM website
- Category hierarchy (category → subcategory → control) is structurally correct
- Total of 143 concepts (4 categories + 21 subcategories + 118 controls) is consistent with the framework

### Issues Found

1. **Control definitions are thin** — All 118 control definitions (`definition_en` and `definition_nb`) simply repeat the control title verbatim rather than providing the descriptive guidance NSM publishes for each control. This is a quality/completeness issue, not a fabrication issue. The titles and codes themselves are accurate.
   - Example: `1.1.1` `definition_nb` = `"Identifiser virksomhetens strategi og prioriterte mål..."` which is just the control title, not an explanation of how to apply it.
   - NSM provides detailed guidance per control on their website and in the full PDF document.

2. **To-do — enrich definitions:** Each control should have a substantive definition explaining the control's intent, not just its title. Source: https://nsm.no/regelverk-og-hjelp/rad-og-anbefalinger/ta-i-bruk-grunnprinsippene/grunnprinsipper-for-ikt-sikkerhet-2-1

### Concept Count
143 concepts — all codes and titles legitimate. Definitions need enrichment.

### Note on Status
Marked `partially-verified` because the structural integrity (category/subcategory hierarchy, all codes) is confirmed correct against the official source, but individual control definitions have not been validated against NSM's detailed guidance text.
