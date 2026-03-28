-- Add verification provenance columns to frameworks table
ALTER TABLE frameworks ADD COLUMN verification_status TEXT DEFAULT 'unverified';
ALTER TABLE frameworks ADD COLUMN verification_date TEXT;
ALTER TABLE frameworks ADD COLUMN verification_source TEXT;
ALTER TABLE frameworks ADD COLUMN verification_notes TEXT;
