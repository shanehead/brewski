-- Recreate batches with updated schema:
--   • status: adds 'conditioning', removes 'complete' (existing 'complete' rows → 'packaged')
--   • conditioning_date: new stage date column
--   • notes: replaces brew_day_notes / fermentation_notes / tasting_notes
CREATE TABLE batches_new (
  id                        TEXT PRIMARY KEY,
  recipe_id                 TEXT NOT NULL REFERENCES recipes(id) ON DELETE RESTRICT,
  recipe_version_id         TEXT NOT NULL REFERENCES recipe_versions(id),
  name                      TEXT,
  status                    TEXT NOT NULL DEFAULT 'planned'
                              CHECK (status IN ('planned', 'brewing', 'fermenting', 'conditioning', 'packaged')),
  brew_date                 INTEGER,
  fermenter_date            INTEGER,
  conditioning_date         INTEGER,
  packaging_date            INTEGER,
  actual_pre_boil_volume_l  REAL,
  actual_post_boil_volume_l REAL,
  actual_batch_size_l       REAL,
  actual_pre_boil_gravity   REAL,
  actual_og                 REAL,
  actual_fg                 REAL,
  notes                     TEXT,
  rating                    INTEGER CHECK (rating IS NULL OR (rating >= 1 AND rating <= 10)),
  created_at                INTEGER NOT NULL,
  updated_at                INTEGER NOT NULL
);

INSERT INTO batches_new (
  id, recipe_id, recipe_version_id, name,
  status,
  brew_date, fermenter_date, conditioning_date, packaging_date,
  actual_pre_boil_volume_l, actual_post_boil_volume_l, actual_batch_size_l,
  actual_pre_boil_gravity, actual_og, actual_fg,
  notes, rating, created_at, updated_at
)
SELECT
  id, recipe_id, recipe_version_id, name,
  CASE WHEN status = 'complete' THEN 'packaged' ELSE status END,
  brew_date, fermenter_date, NULL, packaging_date,
  actual_pre_boil_volume_l, actual_post_boil_volume_l, actual_batch_size_l,
  actual_pre_boil_gravity, actual_og, actual_fg,
  brew_day_notes, rating, created_at, updated_at
FROM batches;

DROP TABLE batches;
ALTER TABLE batches_new RENAME TO batches;

CREATE INDEX IF NOT EXISTS idx_batches_recipe_id ON batches(recipe_id);
CREATE INDEX IF NOT EXISTS idx_batches_recipe_version_id ON batches(recipe_version_id);
