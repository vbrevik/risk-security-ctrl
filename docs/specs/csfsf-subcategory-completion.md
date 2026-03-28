# NIST CSF 2.0 Subcategory Completion

## Background

The Risk Security CTRL project's ontology currently includes NIST Cybersecurity Framework (CSF) 2.0, but only partially implemented:

- **Current coverage:** 21 of 106 subcategories (~20%)
- **Missing:** 85 subcategories across all 6 functions and 34 categories

This task is to complete the NIST CSF 2.0 implementation by adding all missing subcategories based on the official framework structure.

## Official NIST CSF 2.0 Structure

NIST CSF 2.0 (released November 2023) consists of:

### 6 Functions
1. **Govern (GV)** - NEW in 2.0
2. **Identify (ID)**
3. **Protect (PR)**
4. **Detect (DE)**
5. **Respond (RS)**
6. **Recover (RC)**

### 34 Categories & 106 Subcategories

**Govern Function (GV)**
- GVOC: Organizational Context - 2 subcategories
- GVP: Governance Policies - 2 subcategories
- GVRR: Risk Management Roles - 2 subcategories

**Identify Function (ID)**
- ID.AM: Asset Management - 3 subcategories
- ID.RA: Risk Assessment - 2 subcategories
- ID.IM: Incident Management Planning - 1 subcategory
- ID.BE: Business Environment - 4 subcategories
- ID.GV: Governance - 5 subcategories

**Protect Function (PR)**
- PR.AT: Identity, Authentication, and Authorization - 3 subcategories
- PR.AA: Access Control and Management - 6 subcategories
- PR.DS: Data Security - 2 subcategories
- PR.GO: Supply Chain Governance - 4 subcategories
- PR.IM: Information Protection Processes and Procedures - 2 subcategories

**Detect Function (DE)**
- DE.CM: Anomaly Detection - 2 subcategories
- DE.CA: Continuous Monitoring - 1 subcategory
- DE.DS: Security Data Quality - 1 subcategory
- DE.AE: Event Analysis - 3 subcategories

**Respond Function (RS)**
- RS.MP: Response Planning - 6 subcategories
- RS.COM: Communication - 2 subcategories
- RS.GO: Incident Governance - 4 subcategories
- RS.CA: Coordination with External Parties - 5 subcategories

**Recover Function (RC)**
- RC.IM: Recovery Planning - 3 subcategories
- RC.IMPROVE: Improvements - 1 subcategory
- RC.MP: Communications - 2 subcategories

## Requirements

### R1: Complete All Missing Subcategories

For each missing subcategory, create a concept entry with:
- Unique ID following existing convention: `nist-csf-[function]-[subcategory-id]`
- Full name from official framework
- Definition text from official documentation
- Appropriate parent-child relationships (subcategory → category → function)
- Tags for framework association

### R2: Maintain Ontology Data Format

All new entries must follow the existing JSON-LD format in `nist-csf.json`:
```json
{
  "id": "nist-csf-[function]-[subcategory-id]",
  "type": "subcategory",
  "name": "Full Subcategory Name",
  "description": "Official definition text",
  "parent_id": "nist-csf-[function]-[category-id]",
  "properties": {},
  "tags": ["nist-csf-2.0"]
}
```

### R3: Validate After Changes

After completing all subcategories:
1. Verify JSON syntax is valid
2. Run `cargo test` to ensure backend tests still pass
3. Check that new concepts appear correctly in the ontology explorer

### R4: Document Coverage

Update documentation to reflect full coverage achieved.

## Constraints

- Use official NIST CSF 2.0 structure from nist.gov/cybersecurityframework as source
- Maintain backward compatibility with existing concept IDs
- Follow existing naming conventions (GV, ID, PR, DE, RS, RC prefixes)
- No custom modifications to official subcategory text
- Complete in a single pass where possible

## Priority

**HIGH** - This is blocking for complete NIST CSF 2.0 coverage and is referenced in the backlog as a prerequisite to adding new frameworks.
