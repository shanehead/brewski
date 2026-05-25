-- Wire water profiles into the 5 example (seeded) recipes.
-- Uses existing seeded water profiles where a good match exists;
-- RO water as a neutral base for West Coast and NEIPA styles.
-- Mineral adjustments are per-5-gallon (18.9 L) batch.

-- ── Pliny the Elder — West Coast DIPA ────────────────────────
-- High-sulfate profile accentuates hop bitterness and dryness.
-- RO base + gypsum to reach ~250 ppm SO4, moderate Ca.
UPDATE recipes SET mash_water_id = 'water-ro', sparge_water_id = 'water-ro'
  WHERE id = 'bm-recipe-pliny-the-elder';

INSERT INTO recipe_water_adjustments (id, recipe_id, addition, target, amount) VALUES
  ('bm-r1-wa1', 'bm-recipe-pliny-the-elder', 'gypsum',           'mash',   8.0),
  ('bm-r1-wa2', 'bm-recipe-pliny-the-elder', 'calcium_chloride', 'mash',   2.0),
  ('bm-r1-wa3', 'bm-recipe-pliny-the-elder', 'gypsum',           'sparge', 4.0);

-- ── Heady Topper — DIPA ──────────────────────────────────────
-- Similar West Coast profile; slightly more balanced Cl for
-- fuller mouthfeel while still emphasising hop character.
UPDATE recipes SET mash_water_id = 'water-ro', sparge_water_id = 'water-ro'
  WHERE id = 'bm-recipe-heady-topper';

INSERT INTO recipe_water_adjustments (id, recipe_id, addition, target, amount) VALUES
  ('bm-r2-wa1', 'bm-recipe-heady-topper', 'gypsum',           'mash',   7.0),
  ('bm-r2-wa2', 'bm-recipe-heady-topper', 'calcium_chloride', 'mash',   3.0),
  ('bm-r2-wa3', 'bm-recipe-heady-topper', 'gypsum',           'sparge', 3.5);

-- ── Julius — New England IPA ─────────────────────────────────
-- High-chloride, low-sulfate profile promotes soft, juicy
-- mouthfeel. Cl:SO4 target > 2:1.
UPDATE recipes SET mash_water_id = 'water-ro', sparge_water_id = 'water-ro'
  WHERE id = 'bm-recipe-julius';

INSERT INTO recipe_water_adjustments (id, recipe_id, addition, target, amount) VALUES
  ('bm-r3-wa1', 'bm-recipe-julius', 'calcium_chloride', 'mash',   8.0),
  ('bm-r3-wa2', 'bm-recipe-julius', 'gypsum',           'mash',   2.0),
  ('bm-r3-wa3', 'bm-recipe-julius', 'calcium_chloride', 'sparge', 4.0);

-- ── Guinness Draught — Irish Dry Stout ───────────────────────
-- Dublin water is the authentic match: moderately hard,
-- moderate bicarbonate buffers the roasted barley acidity.
UPDATE recipes SET mash_water_id = 'water-dublin', sparge_water_id = 'water-dublin'
  WHERE id = 'bm-recipe-guinness-draught';

-- No mineral additions needed — Dublin profile is used as-is.

-- ── Saison Dupont — Belgian Saison ───────────────────────────
-- Brasserie Dupont draws from a very soft well; Pilsen water
-- is the closest seeded profile. Low mineral content lets
-- the yeast character shine.
UPDATE recipes SET mash_water_id = 'water-pilsen', sparge_water_id = 'water-pilsen'
  WHERE id = 'bm-recipe-saison-dupont';

-- No mineral additions — Pilsen soft water used as-is.
