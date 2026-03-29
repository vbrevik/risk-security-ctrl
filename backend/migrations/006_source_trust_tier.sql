-- Add source trust tier to frameworks table
-- 1 = Primary official source (EUR-Lex, NIST, MITRE, ISO official, government sites)
-- 2 = Legitimate secondary source (official distributors, structured readers, peer summaries)
-- 3 = Weak or unofficial source (unauthorized PDF copies, compliance vendor summaries)
ALTER TABLE frameworks ADD COLUMN source_trust_tier INTEGER;
