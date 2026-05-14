-- src-tauri/migrations/005_batches.sql

CREATE TABLE IF NOT EXISTS recipe_versions (
  id                  TEXT PRIMARY KEY,
  recipe_id           TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  version_number      INTEGER NOT NULL,
  name                TEXT,
  type                TEXT NOT NULL,
  brewer              TEXT,
  batch_size_l        REAL NOT NULL,
  boil_size_l         REAL NOT NULL,
  boil_time_min       REAL NOT NULL,
  efficiency_pct      REAL,
  style_id            TEXT,
  mash_water_id       TEXT,
  sparge_water_id     TEXT,
  notes               TEXT,
  og                  REAL,
  fg                  REAL,
  primary_age_days    REAL,
  primary_temp_c      REAL,
  secondary_age_days  REAL,
  secondary_temp_c    REAL,
  asst_brewer           TEXT,
  fermentation_stages   INTEGER DEFAULT 1,
  tertiary_age_days     REAL,
  tertiary_temp_c       REAL,
  age_days              REAL,
  age_temp_c            REAL,
  forced_carbonation    INTEGER DEFAULT 0,
  priming_sugar_name    TEXT,
  carbonation_temp_c    REAL,
  priming_sugar_equiv   REAL,
  keg_priming_factor    REAL,
  equipment_profile_id  TEXT,
  carbonation_vols    REAL,
  created_at          INTEGER NOT NULL,
  UNIQUE(recipe_id, version_number)
);

CREATE TABLE IF NOT EXISTS recipe_version_fermentables (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  fermentable_id      TEXT REFERENCES fermentables(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  yield_pct           REAL NOT NULL,
  color_lovibond      REAL NOT NULL,
  amount_kg           REAL NOT NULL,
  add_after_boil      INTEGER DEFAULT 0,
  addition_order      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_version_hops (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  hop_id              TEXT REFERENCES hops(id),
  name                TEXT NOT NULL,
  alpha_pct           REAL NOT NULL,
  form                TEXT NOT NULL DEFAULT 'pellet',
  amount_kg           REAL NOT NULL,
  use                 TEXT NOT NULL,
  time_min            REAL NOT NULL,
  addition_order      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_version_yeasts (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  yeast_id            TEXT REFERENCES yeasts(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  form                TEXT NOT NULL,
  laboratory          TEXT,
  product_id          TEXT,
  attenuation_pct     REAL,
  amount              REAL,
  amount_is_weight    INTEGER DEFAULT 0,
  add_to_secondary    INTEGER DEFAULT 0,
  times_cultured      INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_version_miscs (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  misc_id             TEXT REFERENCES miscs(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  use                 TEXT NOT NULL,
  amount              REAL NOT NULL,
  amount_is_weight    INTEGER DEFAULT 0,
  time_min            REAL NOT NULL,
  addition_order      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_version_waters (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  water_id            TEXT REFERENCES waters(id),
  name                TEXT NOT NULL,
  amount_l            REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe_version_water_adjustments (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  addition            TEXT NOT NULL,
  target              TEXT NOT NULL,
  amount              REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe_version_mash (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL UNIQUE REFERENCES recipe_versions(id) ON DELETE CASCADE,
  name                TEXT NOT NULL DEFAULT 'Single Infusion',
  grain_temp_c        REAL NOT NULL DEFAULT 21,
  tun_temp_c          REAL,
  sparge_temp_c       REAL,
  ph                  REAL,
  notes               TEXT,
  ratio_l_per_kg      REAL,
  tun_weight_kg       REAL,
  tun_specific_heat   REAL,
  equip_adjust        INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_version_mash_steps (
  id                      TEXT PRIMARY KEY,
  recipe_version_mash_id  TEXT NOT NULL REFERENCES recipe_version_mash(id) ON DELETE CASCADE,
  name                    TEXT NOT NULL,
  type                    TEXT NOT NULL DEFAULT 'infusion',
  infuse_amount_l         REAL,
  step_temp_c             REAL NOT NULL,
  step_time_min           INTEGER NOT NULL,
  ramp_time_min           INTEGER,
  end_temp_c              REAL,
  step_order              INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS batches (
  id                        TEXT PRIMARY KEY,
  recipe_id                 TEXT NOT NULL REFERENCES recipes(id) ON DELETE RESTRICT,
  recipe_version_id         TEXT NOT NULL REFERENCES recipe_versions(id),
  name                      TEXT,
  status                    TEXT NOT NULL DEFAULT 'planned' CHECK (status IN ('planned', 'brewing', 'fermenting', 'packaged', 'complete')),
  brew_date                 INTEGER,
  fermenter_date            INTEGER,
  packaging_date            INTEGER,
  actual_pre_boil_volume_l  REAL,
  actual_post_boil_volume_l REAL,
  actual_batch_size_l       REAL,
  actual_pre_boil_gravity   REAL,
  actual_og                 REAL,
  actual_fg                 REAL,
  brew_day_notes            TEXT,
  fermentation_notes        TEXT,
  tasting_notes             TEXT,
  rating                    INTEGER CHECK (rating IS NULL OR (rating >= 1 AND rating <= 10)),
  created_at                INTEGER NOT NULL,
  updated_at                INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS batch_gravity_readings (
  id          TEXT PRIMARY KEY,
  batch_id    TEXT NOT NULL REFERENCES batches(id) ON DELETE CASCADE,
  recorded_at INTEGER NOT NULL,
  gravity     REAL NOT NULL,
  temp_c      REAL,
  notes       TEXT
);

CREATE INDEX IF NOT EXISTS idx_batches_recipe_id ON batches(recipe_id);
CREATE INDEX IF NOT EXISTS idx_batches_recipe_version_id ON batches(recipe_version_id);
CREATE INDEX IF NOT EXISTS idx_batch_gravity_readings_batch_id ON batch_gravity_readings(batch_id);
