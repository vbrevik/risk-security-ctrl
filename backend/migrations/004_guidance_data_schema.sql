-- ============================================================
-- Migration 004: Guidance Data Schema
-- Stores NIST AI RMF Playbook guidance content (actions,
-- transparency questions, references) linked to concepts.
-- ============================================================

-- Concept guidance: one row per concept with guidance data
CREATE TABLE IF NOT EXISTS concept_guidance (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL UNIQUE REFERENCES concepts(id) ON DELETE CASCADE,
    source_pdf TEXT NOT NULL,
    source_page INTEGER NOT NULL,
    about_en TEXT,
    about_nb TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Suggested actions: ordered list per concept
CREATE TABLE IF NOT EXISTS concept_actions (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    action_text_en TEXT NOT NULL,
    action_text_nb TEXT,
    sort_order INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(concept_id, sort_order)
);

-- Transparency/documentation questions: ordered list per concept
CREATE TABLE IF NOT EXISTS concept_transparency_questions (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    question_text_en TEXT NOT NULL,
    question_text_nb TEXT,
    sort_order INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(concept_id, sort_order)
);

-- References and transparency resources
CREATE TABLE IF NOT EXISTS concept_references (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    reference_type TEXT NOT NULL CHECK(reference_type IN ('academic', 'transparency_resource')),
    title TEXT NOT NULL,
    authors TEXT,
    year INTEGER,
    venue TEXT,
    url TEXT,
    sort_order INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
);

-- ============================================================
-- Indexes
-- ============================================================

CREATE INDEX IF NOT EXISTS idx_concept_actions_concept ON concept_actions(concept_id);
CREATE INDEX IF NOT EXISTS idx_concept_questions_concept ON concept_transparency_questions(concept_id);
CREATE INDEX IF NOT EXISTS idx_concept_references_concept ON concept_references(concept_id);
CREATE INDEX IF NOT EXISTS idx_concept_references_type ON concept_references(reference_type);

-- ============================================================
-- Content view for FTS5 (joins guidance with concept metadata)
-- ============================================================

CREATE VIEW IF NOT EXISTS concept_guidance_search_v AS
SELECT
    cg.rowid AS rowid,
    c.name_en,
    c.definition_en,
    cg.about_en
FROM concept_guidance cg
JOIN concepts c ON c.id = cg.concept_id;

-- ============================================================
-- FTS5 virtual table for full-text search on guidance content
-- ============================================================

CREATE VIRTUAL TABLE IF NOT EXISTS concept_guidance_fts USING fts5(
    name_en,
    definition_en,
    about_en,
    content='concept_guidance_search_v',
    content_rowid='rowid'
);

-- Initial FTS index build (indexes zero rows on first run)
INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild');
