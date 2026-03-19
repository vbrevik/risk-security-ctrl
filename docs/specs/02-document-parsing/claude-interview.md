# Interview: Document Parsing Pipeline

## Q1: Scanned PDF handling
**Q:** The spec says 'no OCR' for scanned PDFs. What should happen when a user uploads a scanned/image-based PDF?
**A:** Detect and return specific error. Check if extracted text is empty/near-empty and return EmptyDocument with a message about scanned PDFs.

## Q2: Norwegian text processing
**Q:** Should we support Norwegian text processing (stopwords, stemming) now or defer?
**A:** Support both English and Norwegian now. Use language detection or user selection to pick stopwords/stemmer.

## Q3: Document size range
**Q:** What's the expected document size range?
**A:** Medium (5-20MB typical). Larger reports with images — stream to temp file first.

## Q4: Section detection
**Q:** Should section detection preserve document structure (headings, bullet lists, tables)?
**A:** Preserve headings as sections. Split on headings so findings can reference 'Section 3.2' etc.

## Q5: File retention
**Q:** Should we clean up uploaded files after processing or keep them?
**A:** Configurable retention. Keep by default, add a cleanup endpoint or TTL later.
