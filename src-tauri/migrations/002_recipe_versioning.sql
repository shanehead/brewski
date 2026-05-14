ALTER TABLE recipe_versions ADD COLUMN parent_version_id TEXT REFERENCES recipe_versions(id);
ALTER TABLE recipes ADD COLUMN branch_parent_id TEXT REFERENCES recipe_versions(id);
