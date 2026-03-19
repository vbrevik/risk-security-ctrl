# Usage Guide: Document Analysis Backend API & Export

## Quick Start

Start the backend server from `backend/`:

```bash
cargo run
```

The analysis API is available at `http://localhost:3000/api/analyses`.

## API Endpoints

### Create Analysis (text input)
```bash
curl -X POST http://localhost:3000/api/analyses \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Security Analysis",
    "input_text": "Our organization implements risk management...",
    "description": "Annual compliance review"
  }'
# Returns: { "id": "<uuid>" }
```

### Upload File for Analysis
```bash
curl -X POST http://localhost:3000/api/analyses/upload \
  -F "file=@document.pdf" \
  -F "name=Uploaded Analysis"
# Returns: { "id": "<uuid>", "status": "completed" }
```

### List Analyses
```bash
curl http://localhost:3000/api/analyses?page=1&limit=20
# Supports ?status=completed filter
```

### Get Analysis Details
```bash
curl http://localhost:3000/api/analyses/<id>
```

### Get Findings
```bash
curl http://localhost:3000/api/analyses/<id>/findings?page=1&limit=50
# Supports ?framework_id=..., ?finding_type=gap, ?priority=1
```

### Export as PDF
```bash
curl -o report.pdf http://localhost:3000/api/analyses/<id>/export/pdf
# Content-Type: application/pdf
```

### Export as DOCX
```bash
curl -o report.docx http://localhost:3000/api/analyses/<id>/export/docx
# Content-Type: application/vnd.openxmlformats-officedocument.wordprocessingml.document
```

### Get Prompt Template (Matcher Config)
```bash
curl http://localhost:3000/api/analyses/prompt-template
# Returns MatcherConfig JSON (defaults if no custom config saved)
```

### Update Prompt Template
```bash
curl -X PUT http://localhost:3000/api/analyses/prompt-template \
  -H "Content-Type: application/json" \
  -d '{
    "version": 1,
    "min_confidence_threshold": 0.15,
    "addressed_threshold": 0.7,
    "partial_threshold": 0.35,
    "max_findings_per_framework": 100,
    "include_addressed_findings": true,
    "boost_terms": { "security": 2.0, "risk": 1.5 }
  }'
```

## Export Report Structure

Both PDF and DOCX exports contain:
1. **Title page** - Analysis name, metadata, creation date
2. **Executive summary** - Total findings, addressed/gap counts, framework count
3. **Coverage heatmap** - Per-framework coverage as horizontal bar chart
4. **Per-framework sections** - Radar chart + findings table (code, concept, type, priority, confidence, recommendation)
5. **Priority breakdown** - Bar chart of findings by priority level
6. **Appendix** - First 2000 characters of extracted text

## Files Created

### Source Files
- `backend/src/features/analysis/routes.rs` - All API route handlers
- `backend/src/features/analysis/upload.rs` - File upload validation and storage
- `backend/src/features/analysis/charts.rs` - Chart rendering (heatmap, radar, priority bars)
- `backend/src/features/analysis/export_pdf.rs` - PDF report generation
- `backend/src/features/analysis/export_docx.rs` - DOCX report generation

### Runtime Files
- `backend/uploads/` - Uploaded analysis files (PDF, DOCX)
- `backend/config/default-prompt-template.json` - Custom matcher configuration (created on first PUT)
- `backend/fonts/LiberationSans-*.ttf` - Fonts for PDF rendering

## Notes

- Export only works for analyses with `status: completed`
- Charts render to PNG and are embedded in both PDF and DOCX
- The matching engine runs synchronously; export generation uses `spawn_blocking`
- Audit log entries are written best-effort (failures logged, don't block the response)
