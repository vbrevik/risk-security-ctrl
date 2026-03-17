# Interview Transcript

## Q1: Framework Catalog navigation pattern
**Question:** For the Framework Catalog (/frameworks), should clicking a framework card navigate to a separate detail page (/frameworks/$id) or expand the card inline on the same page?

**Answer:** Both (master-detail) — Catalog stays visible as a sidebar, detail fills the main area — like the ontology explorer pattern.

## Q2: Crosswalk Matrix ordering
**Question:** For the Crosswalk Matrix, how should the 22 frameworks be ordered?

**Answer:** By domain cluster — Group by domain (Risk & Security, AI Governance, EU Regulations, Architecture) — shows within-domain density.

## Q3: Regulatory Landscape URL state
**Question:** For the Regulatory Landscape, should the sector/activity selection persist in the URL?

**Answer:** Yes, URL state — Selections encoded in URL params — shareable links like /landscape?sector=financial&activities=ai,personal-data.

## Q4: Priority order
**Question:** What's the priority order for these 4 pages?

**Answer:** All at once — Ship all 4 together as a single release.

## Q5: Search result navigation
**Question:** For the Unified Search, should search results link directly to the ontology explorer or the framework detail page?

**Answer:** Both options on each result — Primary click goes to ontology explorer, secondary link to framework detail.

## Q6: Navigation layout
**Question:** Should the 4 new pages be added to the main navigation bar, or organized differently?

**Answer:** Two-tier: primary + secondary — Primary: Home, Ontology, Compliance, Reports. Secondary row: Frameworks, Crosswalk, Landscape, Search.
