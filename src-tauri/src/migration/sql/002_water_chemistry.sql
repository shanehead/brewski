-- Add water source columns to recipes
ALTER TABLE recipes ADD COLUMN mash_water_id TEXT REFERENCES waters(id);
ALTER TABLE recipes ADD COLUMN sparge_water_id TEXT REFERENCES waters(id);

-- Create recipe_water_adjustments table
CREATE TABLE IF NOT EXISTS recipe_water_adjustments (
  id          TEXT PRIMARY KEY,
  recipe_id   TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  addition    TEXT NOT NULL,
  target      TEXT NOT NULL,
  amount      REAL NOT NULL
);

-- Seed water profiles
INSERT INTO waters (id, name, calcium_ppm, magnesium_ppm, sodium_ppm, chloride_ppm, sulfate_ppm, bicarbonate_ppm, ph, notes) VALUES
  ('water-distilled', 'Distilled', 0, 0, 0, 0, 0, 0, 7.0, NULL),
  ('water-ro', 'Reverse Osmosis', 3, 1, 2, 2, 3, 10, 6.5, NULL),
  ('water-pilsen', 'Pilsen', 7, 3, 2, 5, 5, 25, 7.1, NULL),
  ('water-munich', 'Munich', 77, 17, 4, 8, 18, 295, 7.7, NULL),
  ('water-vienna', 'Vienna', 75, 15, 10, 15, 60, 225, 7.5, NULL),
  ('water-london', 'London', 70, 6, 15, 37, 40, 166, 7.4, NULL),
  ('water-edinburgh', 'Edinburgh', 120, 25, 55, 65, 140, 285, 7.2, NULL),
  ('water-dublin', 'Dublin', 118, 4, 12, 19, 54, 315, 7.4, NULL),
  ('water-burton', 'Burton', 275, 40, 30, 35, 725, 270, 6.5, NULL);
