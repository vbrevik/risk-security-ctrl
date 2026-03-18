# Interview: 02-document-parsing

## Q1: DOCX extraction approach
**Q:** docx-rs is a writer, docx crate unmaintained. Use manual ZIP + quick-xml?
**A:** Yes — ZIP + quick-xml. Full control, actively maintained deps.

## Q2: PDF page-by-page extraction
**Q:** Use extract_text_by_pages() for page-based sections, or flat extract_text()?
**A:** Page-by-page. Gets DocumentSections with page numbers for free.
