ALTER TABLE hops ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE hops ADD COLUMN forked_from_id TEXT REFERENCES hops(id);

ALTER TABLE fermentables ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE fermentables ADD COLUMN forked_from_id TEXT REFERENCES fermentables(id);

ALTER TABLE yeasts ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE yeasts ADD COLUMN forked_from_id TEXT REFERENCES yeasts(id);

ALTER TABLE miscs ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE miscs ADD COLUMN forked_from_id TEXT REFERENCES miscs(id);

ALTER TABLE waters ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE waters ADD COLUMN forked_from_id TEXT REFERENCES waters(id);
