-- Ontology Framework Explorer - Initial Schema
-- Sprint 0: Database Schema Design

-- ============================================================================
-- ONTOLOGY TABLES
-- ============================================================================

-- Frameworks (ISO 31000, ISO 31010, NIST CSF)
CREATE TABLE IF NOT EXISTS frameworks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT,
    description TEXT,
    source_url TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Concepts (nodes in the ontology graph)
CREATE TABLE IF NOT EXISTS concepts (
    id TEXT PRIMARY KEY,
    framework_id TEXT NOT NULL REFERENCES frameworks(id),
    parent_id TEXT REFERENCES concepts(id),
    concept_type TEXT NOT NULL, -- 'principle', 'framework_component', 'process', 'technique', 'function', 'category', 'subcategory'
    code TEXT, -- e.g., 'ID.AM-1' for NIST
    name_en TEXT NOT NULL,
    name_nb TEXT,
    definition_en TEXT,
    definition_nb TEXT,
    source_reference TEXT, -- e.g., 'ISO 31000:2018 Clause 5.2'
    sort_order INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Relationships (edges between concepts)
CREATE TABLE IF NOT EXISTS relationships (
    id TEXT PRIMARY KEY,
    source_concept_id TEXT NOT NULL REFERENCES concepts(id),
    target_concept_id TEXT NOT NULL REFERENCES concepts(id),
    relationship_type TEXT NOT NULL, -- 'part_of', 'related_to', 'maps_to', 'implements', 'requires', 'supports'
    description TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Concept properties (additional metadata key-value pairs)
CREATE TABLE IF NOT EXISTS concept_properties (
    id TEXT PRIMARY KEY,
    concept_id TEXT NOT NULL REFERENCES concepts(id),
    property_key TEXT NOT NULL,
    property_value TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
);

-- ============================================================================
-- COMPLIANCE TABLES
-- ============================================================================

-- Assessments (compliance assessment projects)
CREATE TABLE IF NOT EXISTS assessments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    framework_id TEXT NOT NULL REFERENCES frameworks(id),
    status TEXT DEFAULT 'draft', -- 'draft', 'in_progress', 'completed', 'archived'
    created_by TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Compliance items (checklist items linked to concepts)
CREATE TABLE IF NOT EXISTS compliance_items (
    id TEXT PRIMARY KEY,
    assessment_id TEXT NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    concept_id TEXT NOT NULL REFERENCES concepts(id),
    status TEXT DEFAULT 'not_started', -- 'not_started', 'in_progress', 'implemented', 'not_applicable'
    notes TEXT,
    updated_by TEXT,
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Evidence attachments
CREATE TABLE IF NOT EXISTS evidence (
    id TEXT PRIMARY KEY,
    compliance_item_id TEXT NOT NULL REFERENCES compliance_items(id) ON DELETE CASCADE,
    evidence_type TEXT NOT NULL, -- 'file', 'url'
    name TEXT NOT NULL,
    url TEXT,
    file_path TEXT,
    mime_type TEXT,
    file_size INTEGER,
    uploaded_by TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- ============================================================================
-- USER & AUTH TABLES
-- ============================================================================

-- Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer', -- 'admin', 'risk_manager', 'specialist', 'viewer'
    is_active INTEGER DEFAULT 1,
    last_login_at TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Sessions
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT UNIQUE NOT NULL,
    expires_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- ============================================================================
-- AUDIT LOG
-- ============================================================================

-- Audit log for tracking all changes
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    user_id TEXT REFERENCES users(id),
    action TEXT NOT NULL, -- 'create', 'update', 'delete', 'login', 'logout'
    entity_type TEXT NOT NULL, -- 'assessment', 'compliance_item', 'evidence', 'user', etc.
    entity_id TEXT,
    old_value TEXT, -- JSON of previous state
    new_value TEXT, -- JSON of new state
    ip_address TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Ontology indexes
CREATE INDEX IF NOT EXISTS idx_concepts_framework ON concepts(framework_id);
CREATE INDEX IF NOT EXISTS idx_concepts_parent ON concepts(parent_id);
CREATE INDEX IF NOT EXISTS idx_concepts_type ON concepts(concept_type);
CREATE INDEX IF NOT EXISTS idx_concepts_code ON concepts(code);
CREATE INDEX IF NOT EXISTS idx_relationships_source ON relationships(source_concept_id);
CREATE INDEX IF NOT EXISTS idx_relationships_target ON relationships(target_concept_id);
CREATE INDEX IF NOT EXISTS idx_relationships_type ON relationships(relationship_type);
CREATE INDEX IF NOT EXISTS idx_concept_properties_concept ON concept_properties(concept_id);

-- Compliance indexes
CREATE INDEX IF NOT EXISTS idx_assessments_framework ON assessments(framework_id);
CREATE INDEX IF NOT EXISTS idx_assessments_status ON assessments(status);
CREATE INDEX IF NOT EXISTS idx_compliance_items_assessment ON compliance_items(assessment_id);
CREATE INDEX IF NOT EXISTS idx_compliance_items_concept ON compliance_items(concept_id);
CREATE INDEX IF NOT EXISTS idx_compliance_items_status ON compliance_items(status);
CREATE INDEX IF NOT EXISTS idx_evidence_item ON evidence(compliance_item_id);

-- User/Auth indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);

-- Audit indexes
CREATE INDEX IF NOT EXISTS idx_audit_log_user ON audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_created ON audit_log(created_at);

-- ============================================================================
-- FULL-TEXT SEARCH
-- ============================================================================

-- FTS table for concept search
CREATE VIRTUAL TABLE IF NOT EXISTS concepts_fts USING fts5(
    name_en,
    name_nb,
    definition_en,
    definition_nb,
    content='concepts',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS concepts_ai AFTER INSERT ON concepts BEGIN
    INSERT INTO concepts_fts(rowid, name_en, name_nb, definition_en, definition_nb)
    VALUES (NEW.rowid, NEW.name_en, NEW.name_nb, NEW.definition_en, NEW.definition_nb);
END;

CREATE TRIGGER IF NOT EXISTS concepts_ad AFTER DELETE ON concepts BEGIN
    INSERT INTO concepts_fts(concepts_fts, rowid, name_en, name_nb, definition_en, definition_nb)
    VALUES ('delete', OLD.rowid, OLD.name_en, OLD.name_nb, OLD.definition_en, OLD.definition_nb);
END;

CREATE TRIGGER IF NOT EXISTS concepts_au AFTER UPDATE ON concepts BEGIN
    INSERT INTO concepts_fts(concepts_fts, rowid, name_en, name_nb, definition_en, definition_nb)
    VALUES ('delete', OLD.rowid, OLD.name_en, OLD.name_nb, OLD.definition_en, OLD.definition_nb);
    INSERT INTO concepts_fts(rowid, name_en, name_nb, definition_en, definition_nb)
    VALUES (NEW.rowid, NEW.name_en, NEW.name_nb, NEW.definition_en, NEW.definition_nb);
END;
