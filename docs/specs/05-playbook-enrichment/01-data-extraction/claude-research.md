# Research: Playbook Data Extraction

## Codebase Research

### Ontology Data Structure

**Location:** `ontology-data/` — 29 JSON files containing framework definitions.

**NIST AI RMF schema** (`nist-ai-rmf.json`, 130KB):
```json
{
  "framework": {
    "id": "nist-ai-rmf",
    "name": "NIST AI Risk Management Framework",
    "version": "AI 100-1",
    "source_url": "https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.100-1.pdf"
  },
  "concepts": [{
    "id": "nist-ai-unique-id",
    "framework_id": "nist-ai-rmf",
    "parent_id": "parent-concept-id",
    "concept_type": "category|principle|subcategory|function|process|action",
    "code": "GOVERN" | "gv-1" | "mp-3" | null,
    "name_en": "...",
    "name_nb": "...",
    "definition_en": "...",
    "definition_nb": "...",
    "source_reference": "NIST AI 100-1 Section X",
    "sort_order": 1
  }]
}
```

**Key patterns:**
- Functions use full words as codes: `"GOVERN"`, `"MAP"`, `"MEASURE"`, `"MANAGE"`
- Subcategories use abbreviated prefix + number: `"GOVERN 1"`, `"MP-3"`, `"GV-1"`
- Action concepts (the 75 targets) have `concept_type == "action"`
- Other frameworks follow the same schema: `iso31000.json`, `nist-csf.json`, `gdpr.json`, etc.
- Cross-framework mappings in `relationships.json` (185KB) with types: `maps_to`, `related_to`, `supports`

### Existing PDF Processing Infrastructure

The backend already has PDF extraction in Rust (`backend/src/features/analysis/`):

| Module | Purpose |
|--------|---------|
| `upload.rs` | File validation (magic bytes, size), UUID-namespaced storage |
| `parser.rs` | `DocumentParser::parse()` using `pdf_extract::extract_text_by_pages()` |
| `tokenizer.rs` | `sentence_split()`, `extract_keywords()`, stopword filtering |
| `matcher.rs` | Deterministic concept matching with configurable thresholds |
| `engine.rs` | `MatchingEngine` trait producing `MatchingResult` with findings |

**Output structure from parser:**
```rust
pub struct ParsedDocument {
    pub full_text: String,
    pub sections: Vec<DocumentSection>,
    pub word_count: usize,
    pub token_count_estimate: usize,
}

pub struct DocumentSection {
    pub heading: Option<String>,
    pub text: String,
    pub page_number: Option<usize>,
}
```

**Rust PDF dependency:** `pdf-extract = "0.10"` in `backend/Cargo.toml`

### Testing Setup

**Backend:** Rust native `#[tokio::test]`, test app factory in `backend/tests/common/mod.rs` using real SQLite with migrations. Test data auto-imports from `../ontology-data/`.

**Frontend:** Vitest with jsdom, @testing-library/react, 40+ test files. Config at `frontend/vitest.config.ts`.

**No Python test infrastructure currently exists** — would need to be set up for any Python extraction scripts.

---

## Web Research

### PDF Structured Extraction (2025 Best Practices)

**Recommended: pymupdf4llm**
- Extension of PyMuPDF outputting clean Markdown/JSON/TXT
- Fastest high-quality option (~0.12s per document), no GPU needed
- Uses PDF's Table of Contents for automatic header level detection
- `pymupdf4llm.to_markdown(path, page_chunks=True)` returns per-page dicts with text + metadata
- `IdentifyHeaders(doc, max_level=3)` for explicit header control

**Alternative: pdfplumber**
- Best for coordinate-based extraction and table detection
- Character-level access (value, font name, font size, x/y position)
- Good for custom section-detection based on font size or bold styling

**Page offset handling:**
- PDFs define `/PageLabels` catalog entry for logical numbering
- PyMuPDF: `page.get_label()` returns logical page label; `doc[page_index]` for physical
- **Best practice:** Store both physical page index (0-based) and logical label string
- `pagelabels-py` library can read/manipulate PDF page labels independently

**Sources:**
- [PyMuPDF4LLM Docs](https://pymupdf.readthedocs.io/en/latest/pymupdf4llm/)
- [2025 PDF Extractor Comparison](https://onlyoneaman.medium.com/i-tested-7-python-pdf-extractors-so-you-dont-have-to-2025-edition-c88013922257)
- [PDF Page Labels - PDF Association](https://pdfa.org/pdf-ux-page-labels/)

### NIST AI RMF Playbook Structure

The Playbook organizes around 4 core functions:

| Function | Prefix | Pages (approx) |
|----------|--------|-----------------|
| GOVERN | GV | 4-34 |
| MANAGE | MG | 35-57 |
| MAP | MP | 58-92 |
| MEASURE | MS | 93-142 |

**Per-subcategory layout (consistent across all 75 actions):**
1. **Header** (blue-shaded) — outcome statement, e.g., "GOVERN 1.1: Legal and regulatory requirements..."
2. **About** — contextual narrative explaining intent
3. **Suggested Actions** — bulleted voluntary tactical actions
4. **Transparency & Documentation** — recommended documentation practices
5. **AI Transparency Resources** — external resource links
6. **References** — supporting citations

**Design principles:** Voluntary, modular, not a checklist. GOVERN is cross-cutting.

**Sources:**
- [NIST AI RMF Playbook - Official](https://airc.nist.gov/airmf-resources/playbook/)
- [Holistic AI Deep Dive](https://www.holisticai.com/blog/nist-ai-risk-management-framework-playbook)

### JSON Companion File Patterns

**Naming:** Use qualifier suffix — `nist-ai-rmf.json` (ontology) alongside `nist-ai-rmf-guidance.json` (guidance). Keeps files co-located and sortable.

**Traceability per item:**
```json
{
  "source_page": 12,
  "source_page_label": "10",
  "extraction_method": "pymupdf4llm-0.0.17",
  "extracted_at": "2026-03-19T14:00:00Z"
}
```

Key fields: physical page index (programmatic), logical label (human citation), section ID, extraction tool + version, timestamp.

**W3C PROV-O** provides formal provenance vocabulary but adds overhead. For project-internal companion files, a simpler `sourceRef` object is sufficient.

**Sources:**
- [W3C JSON-LD Best Practices](https://w3c.github.io/json-ld-bp/)
- [PROV-JSONLD](https://openprovenance.org/prov-jsonld/)
- [Sidematter Format](https://github.com/jlevy/sidematter-format)
