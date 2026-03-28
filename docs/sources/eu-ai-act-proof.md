# EU AI Act (EU) 2024/1689 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/reg/2024/1689/oj
**Reference:** Regulation (EU) 2024/1689 of 13 June 2024
**Verified:** 2026-03-28
**Status:** verified
## How to Verify

1. **EUR-Lex Full Text (EN):** https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32024R1689
2. **EUR-Lex PDF:** https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:32024R1689
3. **AI Act Explorer (structured):** https://artificialintelligenceact.eu/the-act/
4. **Annex III (High-Risk Areas):** Search "ANNEX III" in the full text

### Verify Article 5 Prohibited Practices
Search "Article 5" in the EUR-Lex text. Paragraphs (a) through (h) list all 8 prohibited practices. Compare against the 8 concepts under `eu-ai-unacceptable` in the JSON.

## Official Structure

The AI Act uses a risk-based classification:
- Title I: General provisions
- Title II: Prohibited AI practices (Article 5)
- Title III: High-risk AI systems (Articles 6-49)
- Title IV: Transparency obligations (Article 50)
- Title V: General-purpose AI models (Articles 51-56)
- Titles VI-XIII: Governance, enforcement, final provisions

## Verification Results

### Confirmed Correct
- Four-tier risk classification (unacceptable/high/limited/minimal) accurately represented
- Annex III high-risk areas 1-8 all present and correctly numbered
- High-risk requirements (Art 9-15, 17) correctly represented
- Transparency obligations correctly reference Art 50 (not old numbering Art 52)
- GPAI rules (Art 51-56) correctly structured
- Penalty amounts correct: EUR 35M/7%, EUR 15M/3%, EUR 7.5M/1%
- GPAI systemic risk threshold >10^25 FLOPs correct
- Deployer obligations (Art 26) correct
- Regulatory sandboxes (Art 60-61) correct

### Issues Found — All Resolved

All 8 Article 5 prohibited practices were present when last verified (items 1-5 below had been added prior to this correction pass). Remaining issues resolved 2026-03-28:

1. ~~**5 of 8 Article 5 prohibited practices missing**~~ — All 8 present: (a) manipulation, (b) vulnerability exploitation, (c) social scoring, (d) predictive policing, (e) facial scraping, (f) emotion recognition, (g) biometric categorisation, (h) real-time biometric identification.
2. **Social scoring definition too narrow** — Fixed: parent category `eu-ai-unacceptable` definition updated to say "public or private actors" (Art 5(1)(c) applies beyond just public authorities).
3. **Art 64 attribution** — Verified correct: Article 64 IS titled "AI Office" in the final regulation. The original `ART-64` code was accurate. Definition updated to align with the article's actual text ("developing Union expertise and capabilities in AI").
4. **Missing Art 8** — Fixed: `eu-ai-art-8` concept added as child of `eu-ai-high-risk`.

### Concept Count
56 concepts — all legitimate, zero fabricated.
