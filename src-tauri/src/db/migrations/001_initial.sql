CREATE TABLE styles (
  id               TEXT PRIMARY KEY,
  name             TEXT NOT NULL,
  category         TEXT NOT NULL,
  category_number  TEXT NOT NULL,
  style_letter     TEXT NOT NULL,
  style_guide      TEXT NOT NULL,
  type             TEXT NOT NULL,
  og_min           REAL NOT NULL,
  og_max           REAL NOT NULL,
  fg_min           REAL NOT NULL,
  fg_max           REAL NOT NULL,
  ibu_min          REAL NOT NULL,
  ibu_max          REAL NOT NULL,
  color_min_srm    REAL NOT NULL,
  color_max_srm    REAL NOT NULL,
  carb_min_vols    REAL,
  carb_max_vols    REAL,
  abv_min_pct      REAL,
  abv_max_pct      REAL,
  notes            TEXT,
  profile          TEXT,
  ingredients      TEXT,
  examples         TEXT
);

CREATE TABLE equipment_profiles (
  id                    TEXT PRIMARY KEY,
  name                  TEXT NOT NULL,
  notes                 TEXT,
  boil_size_l           REAL NOT NULL,
  batch_size_l          REAL NOT NULL,
  calc_boil_volume      INTEGER NOT NULL DEFAULT 1,
  tun_volume_l          REAL,
  tun_weight_kg         REAL,
  tun_specific_heat     REAL,
  lauter_deadspace_l    REAL DEFAULT 0,
  top_up_kettle_l       REAL DEFAULT 0,
  trub_chiller_loss_l   REAL DEFAULT 0,
  evap_rate_pct_hr      REAL DEFAULT 10,
  boil_time_min         REAL NOT NULL DEFAULT 60,
  top_up_water_l        REAL DEFAULT 0,
  fermenter_loss_l      REAL DEFAULT 0,
  hop_utilization_pct   REAL DEFAULT 100,
  efficiency_pct        REAL NOT NULL DEFAULT 72,
  created_at            INTEGER NOT NULL,
  updated_at            INTEGER NOT NULL
);

CREATE TABLE fermentables (
  id                        TEXT PRIMARY KEY,
  name                      TEXT NOT NULL,
  type                      TEXT NOT NULL,
  yield_pct                 REAL NOT NULL,
  color_lovibond            REAL NOT NULL,
  origin                    TEXT,
  supplier                  TEXT,
  notes                     TEXT,
  add_after_boil            INTEGER DEFAULT 0,
  coarse_fine_diff_pct      REAL,
  moisture_pct              REAL,
  diastatic_power_lintner   REAL,
  protein_pct               REAL,
  max_in_batch_pct          REAL,
  recommend_mash            INTEGER,
  ibu_gal_per_lb            REAL
);

CREATE TABLE hops (
  id                  TEXT PRIMARY KEY,
  name                TEXT NOT NULL,
  alpha_pct           REAL NOT NULL,
  beta_pct            REAL,
  form                TEXT NOT NULL DEFAULT 'pellet',
  type                TEXT,
  origin              TEXT,
  year                TEXT,
  notes               TEXT,
  substitutes         TEXT,
  hsi_pct             REAL,
  humulene_pct        REAL,
  caryophyllene_pct   REAL,
  cohumulone_pct      REAL,
  myrcene_pct         REAL
);

CREATE TABLE yeasts (
  id                TEXT PRIMARY KEY,
  name              TEXT NOT NULL,
  type              TEXT NOT NULL,
  form              TEXT NOT NULL,
  laboratory        TEXT,
  product_id        TEXT,
  min_temperature_c REAL,
  max_temperature_c REAL,
  flocculation      TEXT,
  attenuation_pct   REAL,
  notes             TEXT,
  best_for          TEXT,
  max_reuse         INTEGER,
  add_to_secondary  INTEGER DEFAULT 0
);

-- BeerXML requires use and time_min on the library record as suggested defaults.
-- recipe_addition_miscs holds the actual use/time_min values for a specific recipe.
CREATE TABLE miscs (
  id                TEXT PRIMARY KEY,
  name              TEXT NOT NULL,
  type              TEXT NOT NULL,
  use               TEXT NOT NULL,
  time_min          REAL NOT NULL,
  notes             TEXT,
  use_for           TEXT,
  amount_is_weight  INTEGER DEFAULT 0
);

CREATE TABLE waters (
  id              TEXT PRIMARY KEY,
  name            TEXT NOT NULL,
  calcium_ppm     REAL NOT NULL,
  bicarbonate_ppm REAL NOT NULL,
  sulfate_ppm     REAL NOT NULL,
  chloride_ppm    REAL NOT NULL,
  sodium_ppm      REAL NOT NULL,
  magnesium_ppm   REAL NOT NULL,
  ph              REAL,
  notes           TEXT
);

CREATE TABLE recipes (
  id                    TEXT PRIMARY KEY,
  name                  TEXT NOT NULL,
  type                  TEXT NOT NULL DEFAULT 'all_grain',
  brewer                TEXT,
  asst_brewer           TEXT,
  batch_size_l          REAL NOT NULL,
  boil_size_l           REAL NOT NULL,
  boil_time_min         REAL NOT NULL DEFAULT 60,
  efficiency_pct        REAL,
  equipment_profile_id  TEXT REFERENCES equipment_profiles(id),
  style_id              TEXT REFERENCES styles(id),
  notes                 TEXT,
  taste_notes           TEXT,
  taste_rating          REAL,
  og                    REAL,
  fg                    REAL,
  fermentation_stages   INTEGER DEFAULT 1,
  primary_age_days      REAL,
  primary_temp_c        REAL,
  secondary_age_days    REAL,
  secondary_temp_c      REAL,
  tertiary_age_days     REAL,
  tertiary_temp_c       REAL,
  age_days              REAL,
  age_temp_c            REAL,
  carbonation_vols      REAL,
  forced_carbonation    INTEGER DEFAULT 0,
  priming_sugar_name    TEXT,
  carbonation_temp_c    REAL,
  priming_sugar_equiv   REAL,
  keg_priming_factor    REAL,
  date                  TEXT,
  created_at            INTEGER NOT NULL,
  updated_at            INTEGER NOT NULL
);

CREATE TABLE recipe_addition_fermentables (
  id              TEXT PRIMARY KEY,
  recipe_id       TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  fermentable_id  TEXT REFERENCES fermentables(id),
  name            TEXT NOT NULL,
  type            TEXT NOT NULL,
  yield_pct       REAL NOT NULL,
  color_lovibond  REAL NOT NULL,
  amount_kg       REAL NOT NULL,
  add_after_boil  INTEGER DEFAULT 0,
  addition_order  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_hops (
  id             TEXT PRIMARY KEY,
  recipe_id      TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  hop_id         TEXT REFERENCES hops(id),
  name           TEXT NOT NULL,
  alpha_pct      REAL NOT NULL,
  form           TEXT NOT NULL DEFAULT 'pellet',
  amount_kg      REAL NOT NULL,
  use            TEXT NOT NULL,
  time_min       REAL NOT NULL,
  addition_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_yeasts (
  id               TEXT PRIMARY KEY,
  recipe_id        TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  yeast_id         TEXT REFERENCES yeasts(id),
  name             TEXT NOT NULL,
  type             TEXT NOT NULL,
  form             TEXT NOT NULL,
  laboratory       TEXT,
  product_id       TEXT,
  attenuation_pct  REAL,
  amount           REAL,
  amount_is_weight INTEGER DEFAULT 0,
  add_to_secondary INTEGER DEFAULT 0,
  times_cultured   INTEGER DEFAULT 0
);

CREATE TABLE recipe_addition_miscs (
  id               TEXT PRIMARY KEY,
  recipe_id        TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  misc_id          TEXT REFERENCES miscs(id),
  name             TEXT NOT NULL,
  type             TEXT NOT NULL,
  use              TEXT NOT NULL,
  amount           REAL NOT NULL,
  amount_is_weight INTEGER DEFAULT 0,
  time_min         REAL NOT NULL,
  addition_order   INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_waters (
  id        TEXT PRIMARY KEY,
  recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  water_id  TEXT REFERENCES waters(id),
  name      TEXT NOT NULL,
  amount_l  REAL NOT NULL
);

CREATE TABLE mashes (
  id                TEXT PRIMARY KEY,
  recipe_id         TEXT NOT NULL UNIQUE REFERENCES recipes(id) ON DELETE CASCADE,
  name              TEXT NOT NULL DEFAULT 'Single Infusion',
  grain_temp_c      REAL NOT NULL DEFAULT 21,
  tun_temp_c        REAL,
  sparge_temp_c     REAL,
  ph                REAL,
  tun_weight_kg     REAL,
  tun_specific_heat REAL,
  equip_adjust      INTEGER DEFAULT 0,
  notes             TEXT
);

CREATE TABLE mash_steps (
  id              TEXT PRIMARY KEY,
  mash_id         TEXT NOT NULL REFERENCES mashes(id) ON DELETE CASCADE,
  name            TEXT NOT NULL,
  type            TEXT NOT NULL DEFAULT 'infusion',
  infuse_amount_l REAL,
  step_temp_c     REAL NOT NULL,
  step_time_min   INTEGER NOT NULL,
  ramp_time_min   INTEGER,
  end_temp_c      REAL,
  step_order      INTEGER NOT NULL
);

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

INSERT INTO settings (key, value) VALUES
  ('theme', 'midnight'),
  ('units', 'metric');
