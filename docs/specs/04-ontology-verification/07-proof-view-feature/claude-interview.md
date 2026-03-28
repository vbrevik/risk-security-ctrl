# Interview Transcript — Split 07: Proof View Feature

## Q1: Where should the verification status badge appear in FrameworkProfile?

Current layout: Header → Stats Strip → Type Breakdown → Connections → Hierarchy.

**Answer:** In the header area (next to name/version). Most visible placement — fits alongside the existing version badge.

---

## Q2: react-markdown is NOT installed. Add it or render as plain text?

**Answer:** Add react-markdown + remark-gfm. Proper markdown rendering with headings, lists, links preferred.

---

## Q3: Should proof content load eagerly or only when user clicks "View Proof"?

**Answer:** Lazy — only fetch when user explicitly opens the proof panel. No wasted requests; small delay on first open is acceptable.

---

## Q4: When a framework has no proof file (null proof_content), what should the proof panel show?

**Answer:** Hide the "View Proof" button entirely. Only show the proof toggle if `proof_content` is non-null. The badge in the header still shows the verification status.

---

## Q5: Should VerificationBadge use icon + color + text (WCAG) or color + text only?

**Answer:** Icon + color + text label (WCAG compliant). Use distinct icon shapes per status (check, triangle, dash, X).

---

## Summary of Decisions

| Decision | Choice |
|----------|--------|
| Badge placement | In FrameworkProfile header, next to name/version badge |
| Markdown rendering | react-markdown + remark-gfm (new deps) |
| Fetch timing | Lazy — triggered by user opening proof panel |
| No proof file UI | Hide "View Proof" button; badge still shows |
| Badge accessibility | Icon + color + text (WCAG 1.4.1 compliant) |
