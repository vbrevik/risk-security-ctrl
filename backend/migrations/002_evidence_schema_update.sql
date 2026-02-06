-- Rebuild evidence table to align with application code
-- Changes: rename 'name' to 'title', add 'description' and 'updated_at'

CREATE TABLE IF NOT EXISTS evidence_new (
    id TEXT PRIMARY KEY,
    compliance_item_id TEXT NOT NULL REFERENCES compliance_items(id) ON DELETE CASCADE,
    evidence_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    url TEXT,
    file_path TEXT,
    mime_type TEXT,
    file_size INTEGER,
    uploaded_by TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Copy any existing data (map 'name' to 'title' if old table exists)
INSERT OR IGNORE INTO evidence_new (id, compliance_item_id, evidence_type, title, url, file_path, mime_type, file_size, uploaded_by, created_at, updated_at)
SELECT id, compliance_item_id, evidence_type, name, url, file_path, mime_type, file_size, uploaded_by, created_at, created_at
FROM evidence;

DROP TABLE IF EXISTS evidence;

ALTER TABLE evidence_new RENAME TO evidence;

CREATE INDEX IF NOT EXISTS idx_evidence_item ON evidence(compliance_item_id);
