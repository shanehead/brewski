-- Content fingerprint for change detection (NULL = recompute lazily).
ALTER TABLE recipe_versions ADD COLUMN content_hash TEXT;

-- Freeze hopstand temperatures into snapshots (previously re-resolved/hardcoded,
-- which would make recipes read as permanently changed once hashed).
ALTER TABLE recipe_versions ADD COLUMN hopstand_temp_c REAL;
ALTER TABLE recipe_version_hops ADD COLUMN hopstand_temp_c REAL;
