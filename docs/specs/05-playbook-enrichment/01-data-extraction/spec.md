# 01-data-extraction: Extract Playbook Guidance into Companion JSON

## Summary

Semi-automated extraction of structured guidance data from the NIST AI RMF Playbook PDF (`docs/reference-pdfs/AI_RMF_Playbook.pdf`) for all 75 action-level concepts in the NIST AI RMF ontology. Produces a companion JSON file (`ontology-data/nist-ai-rmf-guidance.json`) with full traceability to source PDF pages.

## Requirements Source

- Parent requirements: `docs/specs/05-playbook-enrichment/requirements.md`
- Interview: `docs/specs/05-playbook-enrichment/deep_project_interview.md`
- Manifest: `docs/specs/05-playbook-enrichment/project-manifest.md`

## What to Build

### Output File: `ontology-data/nist-ai-rmf-guidance.json`

Companion file to `ontology-data/nist-ai-rmf.json`, keyed by concept_id. Structure:

```json
{
  "framework_id": "nist-ai-rmf",
  "source_pdf": "AI_RMF_Playbook.pdf",
  "guidance": [
    {
      "concept_id": "nist-ai-ms-1-1",
      "source_page": 98,
      "about_en": "The development and utility of trustworthy AI systems depends on reliable measurements...",
      "suggested_actions_en": [
        "Establish approaches for detecting, tracking and measuring known risks, errors, incidents or negative impacts.",
        "Identify testing procedures and metrics to demonstrate whether or not the system is fit for purpose and functioning as claimed."
      ],
      "transparency_questions_en": [
        "How will the appropriate performance metrics, such as accuracy, of the AI be monitored after the AI is deployed?",
        "What corrective actions has the entity taken to enhance the quality, accuracy, reliability, and representativeness of the data?"
      ],
      "resources": [
        {
          "title": "GAO-21-519SP: AI Accountability Framework for Federal Agencies & Other Entities",
          "url": null,
          "type": "transparency"
        }
      ],
      "references": [
        {
          "title": "Designing Artificial Intelligence Review Boards: Creating Risk Metrics for Review of AI",
          "authors": "Sara R. Jordan",
          "year": 2019,
          "venue": "2019 IEEE International Symposium on Technology and Society (ISTAS)",
          "url": null
        }
      ]
    }
  ]
}
```

### Extraction Process

1. Read the Playbook PDF table of contents (pages 2-4) to map each concept code (GOVERN 1.1, MAP 1.1, MEASURE 1.1, etc.) to its PDF page number
2. For each of the 75 action-level concepts in `ontology-data/nist-ai-rmf.json` (where `concept_type == "action"`):
   - Match by `code` field (e.g., "MEASURE 1.1") to the Playbook section
   - Extract from that section:
     - **about_en**: The "About" paragraphs (between the section header/definition and "Suggested Actions")
     - **suggested_actions_en**: Each bullet under "Suggested Actions"
     - **transparency_questions_en**: Each bullet under "Transparency & Documentation" → "Organizations can document the following"
     - **resources**: Named items under "AI Transparency Resources" (title only, URLs from PDF may not be extractable)
     - **references**: Citations under "References" (parse author, title, year, venue where possible)
   - Record the PDF page number as `source_page`
3. Cross-reference: verify every `concept_id` in the guidance file exists in `nist-ai-rmf.json`

### Concept Coverage

The 75 action concepts span four functions:
- **GOVERN**: GOVERN 1.1 through GOVERN 6.2 (pages 4-34)
- **MANAGE**: MANAGE 1.1 through MANAGE 4.3 (pages 35-57)
- **MAP**: MAP 1.1 through MAP 5.2 (pages 58-92)
- **MEASURE**: MEASURE 1.1 through MEASURE 4.3 (pages 93-142)

### Quality Criteria

- All 75 action concepts must have entries in the guidance file
- Every entry must have a valid `concept_id` matching `nist-ai-rmf.json`
- Every entry must have `source_page` (integer, matching actual PDF page)
- `about_en` should be the full About section text, not truncated
- `suggested_actions_en` should preserve the exact wording from the PDF
- `references` should parse author/year where the citation format allows

## Key Decisions

- **Companion file, not inline**: Guidance lives in separate `nist-ai-rmf-guidance.json`, not embedded in `nist-ai-rmf.json` (interview decision)
- **Semi-automated**: Claude reads PDF pages and generates JSON; human reviews for accuracy
- **English only**: No Norwegian (`_nb`) content in this phase; schema supports it for later
- **Playbook only**: Only extracting from `AI_RMF_Playbook.pdf`, not the other 3 reference PDFs

## Dependencies

- **Needs**: `docs/reference-pdfs/AI_RMF_Playbook.pdf` (already downloaded)
- **Needs**: `ontology-data/nist-ai-rmf.json` (existing, provides concept IDs and codes)
- **Provides to 02**: The companion JSON file and its structure (defines import format)

## Notes

- The PDF page numbers in the TOC (e.g., "MEASURE 1.1 ..... 93") refer to the page footer numbers, which are offset from the physical PDF page numbers by ~4-5 pages due to cover/TOC pages. The extraction must account for this offset.
- Some concepts span 2-3 pages in the PDF; all content must be captured.
- Reference citations in the PDF vary in format (some APA, some informal). Parse what's parseable, store the rest as raw title strings.
