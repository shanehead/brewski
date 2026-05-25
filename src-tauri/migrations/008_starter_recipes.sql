-- Add source column to recipes (matches 004_user_ingredients.sql pattern)
ALTER TABLE recipes ADD COLUMN source TEXT NOT NULL DEFAULT 'user'
  CHECK (source IN ('seeded', 'user'));

-- ─────────────────────────────────────────────────────────────
-- 1. Pliny the Elder (Russian River) — Double IPA, 5.5 gal
--    Reference: https://homebrewersassociation.org/homebrew-recipe/russian-river-pliny-the-elder-clone/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-pliny-the-elder',
  'Pliny the Elder',
  'All Grain',
  20.8,
  25.2,
  90,
  72.0,
  1.072,
  1.012,
  'Clone of Russian River Pliny the Elder Double IPA. Ferment at 68°F (20°C) for 2 weeks, dry hop in two additions. Reference: https://homebrewersassociation.org/homebrew-recipe/russian-river-pliny-the-elder-clone/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r1-f1', 'bm-recipe-pliny-the-elder', 'American 2-Row Pale Malt', 'grain', 78.0, 1.8, 6.01, 1),
  ('bm-r1-f2', 'bm-recipe-pliny-the-elder', 'Crystal 45L', 'grain', 74.0, 45.0, 0.272, 2),
  ('bm-r1-f3', 'bm-recipe-pliny-the-elder', 'Carapils / Dextrin', 'grain', 72.0, 1.5, 0.272, 3),
  ('bm-r1-f4', 'bm-recipe-pliny-the-elder', 'Corn Sugar (Dextrose)', 'sugar', 100.0, 0.0, 0.340, 4);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r1-h1', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0992, 'Boil', 90, 1),
  ('bm-r1-h2', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0213, 'Boil', 45, 2),
  ('bm-r1-h3', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Boil', 30, 3),
  ('bm-r1-h4', 'bm-recipe-pliny-the-elder', 'Centennial', 10.0, 'Pellet', 0.0284, 'Aroma', 0, 4),
  ('bm-r1-h5', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0709, 'Aroma', 0, 5),
  ('bm-r1-h6', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0567, 'Dry Hop', 7, 6),
  ('bm-r1-h7', 'bm-recipe-pliny-the-elder', 'Centennial', 10.0, 'Pellet', 0.0284, 'Dry Hop', 7, 7),
  ('bm-r1-h8', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Dry Hop', 7, 8);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r1-y1', 'bm-recipe-pliny-the-elder', 'American Ale', 'ale', 'dry', 'Fermentis', 'US-05', 77.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r1-mash', 'bm-recipe-pliny-the-elder', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r1-ms1', 'bm-r1-mash', 'Saccharification Rest', 'Infusion', 66.7, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 2. Heady Topper (The Alchemist) — Double IPA, 5 gal
--    Reference: https://byo.com/recipes/alchemist-heady-topper-clone/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-heady-topper',
  'Heady Topper',
  'All Grain',
  18.9,
  22.7,
  90,
  72.0,
  1.077,
  1.017,
  'Clone of The Alchemist Heady Topper Double IPA. Key: use Vermont Ale (Conan) yeast for authentic character. Heavy dry hop in two split additions. Reference: https://byo.com/recipes/alchemist-heady-topper-clone/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r2-f1', 'bm-recipe-heady-topper', 'British 2-Row Pale Malt', 'grain', 78.0, 2.0, 6.80, 1),
  ('bm-r2-f2', 'bm-recipe-heady-topper', 'CaraVienne', 'grain', 74.0, 20.0, 0.170, 2),
  ('bm-r2-f3', 'bm-recipe-heady-topper', 'Corn Sugar (Dextrose)', 'sugar', 100.0, 0.0, 0.454, 3);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r2-h1', 'bm-recipe-heady-topper', 'Chinook', 13.0, 'Pellet', 0.0284, 'Boil', 60, 1),
  ('bm-r2-h2', 'bm-recipe-heady-topper', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0142, 'Boil', 60, 2),
  ('bm-r2-h3', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0142, 'Boil', 30, 3),
  ('bm-r2-h4', 'bm-recipe-heady-topper', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Boil', 10, 4),
  ('bm-r2-h5', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0284, 'Boil', 10, 5),
  ('bm-r2-h6', 'bm-recipe-heady-topper', 'Amarillo', 9.0, 'Pellet', 0.0567, 'Aroma', 0, 6),
  ('bm-r2-h7', 'bm-recipe-heady-topper', 'Simcoe', 13.0, 'Pellet', 0.0567, 'Dry Hop', 4, 7),
  ('bm-r2-h8', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0284, 'Dry Hop', 4, 8),
  ('bm-r2-h9', 'bm-recipe-heady-topper', 'Citra', 12.0, 'Pellet', 0.0284, 'Dry Hop', 4, 9),
  ('bm-r2-h10', 'bm-recipe-heady-topper', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0142, 'Dry Hop', 4, 10);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r2-y1', 'bm-recipe-heady-topper', 'Vermont Ale (Conan)', 'ale', 'liquid', 'The Yeast Bay', 'Vermont Ale', 74.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r2-mash', 'bm-recipe-heady-topper', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r2-ms1', 'bm-r2-mash', 'Saccharification Rest', 'Infusion', 67.2, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 3. Julius (Tree House Brewing) — NEIPA, 5 gal
--    Reference: https://byo.com/recipe/tree-house-brewing-company-julius-clone/
--    Reference: https://hazyandhoppy.com/marshall-bishops-treehouse-julius-clone-recipe/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-julius',
  'Julius',
  'All Grain',
  18.9,
  22.7,
  60,
  72.0,
  1.065,
  1.014,
  'Clone of Tree House Brewing Julius New England IPA. Use London Ale III or similar hazy yeast. Low-sulfate, high-chloride water. Biotransformation dry hop on day 2 of active fermentation. References: https://byo.com/recipe/tree-house-brewing-company-julius-clone/ and https://hazyandhoppy.com/marshall-bishops-treehouse-julius-clone-recipe/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r3-f1', 'bm-recipe-julius', 'American 2-Row Pale Malt', 'grain', 78.0, 1.8, 4.54, 1),
  ('bm-r3-f2', 'bm-recipe-julius', 'Golden Promise Pale Malt', 'grain', 80.0, 2.5, 0.907, 2),
  ('bm-r3-f3', 'bm-recipe-julius', 'Carafoam (Carapils)', 'grain', 72.0, 1.5, 0.227, 3),
  ('bm-r3-f4', 'bm-recipe-julius', 'Flaked Oats', 'adjunct', 70.0, 1.0, 0.907, 4);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r3-h1', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0142, 'Boil', 60, 1),
  ('bm-r3-h2', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0567, 'Aroma', 0, 2),
  ('bm-r3-h3', 'bm-recipe-julius', 'Mosaic', 11.5, 'Pellet', 0.0567, 'Aroma', 0, 3),
  ('bm-r3-h4', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0851, 'Dry Hop', 4, 4),
  ('bm-r3-h5', 'bm-recipe-julius', 'Mosaic', 11.5, 'Pellet', 0.0567, 'Dry Hop', 4, 5);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r3-y1', 'bm-recipe-julius', 'London Ale III', 'ale', 'liquid', 'Omega Yeast', 'OYL-011', 74.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r3-mash', 'bm-recipe-julius', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r3-ms1', 'bm-r3-mash', 'Saccharification Rest', 'Infusion', 67.2, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 4. Guinness Draught — Irish Dry Stout, 5 gal
--    Reference: https://byo.com/recipe/guinness-draught-clone/
--    Reference: https://homebrewacademy.com/guinness-recipe/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-guinness-draught',
  'Guinness Draught',
  'All Grain',
  18.9,
  22.7,
  90,
  72.0,
  1.044,
  1.011,
  'Clone of Guinness Draught Irish Dry Stout. Key: use unmalted roasted barley (500L) for the characteristic dry, roasted finish. Ferment cool at 64°F (18°C). References: https://byo.com/recipe/guinness-draught-clone/ and https://homebrewacademy.com/guinness-recipe/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r4-f1', 'bm-recipe-guinness-draught', 'English 2-Row Pale Ale Malt', 'grain', 78.0, 3.0, 2.268, 1),
  ('bm-r4-f2', 'bm-recipe-guinness-draught', 'Flaked Barley', 'adjunct', 70.0, 1.5, 1.134, 2),
  ('bm-r4-f3', 'bm-recipe-guinness-draught', 'Roasted Barley (500L)', 'grain', 55.0, 500.0, 0.454, 3);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r4-h1', 'bm-recipe-guinness-draught', 'East Kent Goldings', 5.0, 'Pellet', 0.0425, 'Boil', 60, 1);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r4-y1', 'bm-recipe-guinness-draught', 'Irish Ale', 'ale', 'liquid', 'Wyeast', '1084', 72.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r4-mash', 'bm-recipe-guinness-draught', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r4-ms1', 'bm-r4-mash', 'Saccharification Rest', 'Infusion', 65.6, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 5. Saison Dupont — Belgian Saison, 5 gal
--    Reference: https://www.beerandbrewing.com/belgian-saison-in-the-style-of-saison-dupont-recipe
--    Reference: https://www.brewersfriend.com/homebrew/recipe/view/438817/saison-dupont-clone
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-saison-dupont',
  'Saison Dupont',
  'All Grain',
  18.9,
  22.7,
  90,
  75.0,
  1.060,
  1.010,
  'Clone of Brasserie Dupont Saison Dupont. Low mash temp for a very dry finish. Use WY3724 or WLP565; ferment warm (80–90°F / 27–32°C) for full attenuation and classic spicy character. Soft, low-mineral water. References: https://www.beerandbrewing.com/belgian-saison-in-the-style-of-saison-dupont-recipe and https://www.brewersfriend.com/homebrew/recipe/view/438817/saison-dupont-clone',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r5-f1', 'bm-recipe-saison-dupont', 'Belgian Pilsner Malt', 'grain', 80.0, 1.6, 4.990, 1),
  ('bm-r5-f2', 'bm-recipe-saison-dupont', 'Vienna Malt', 'grain', 78.0, 3.5, 0.227, 2),
  ('bm-r5-f3', 'bm-recipe-saison-dupont', 'Munich Malt', 'grain', 77.0, 6.0, 0.113, 3),
  ('bm-r5-f4', 'bm-recipe-saison-dupont', 'CaraMunich', 'grain', 74.0, 60.0, 0.227, 4),
  ('bm-r5-f5', 'bm-recipe-saison-dupont', 'Wheat Malt', 'grain', 80.0, 2.0, 0.227, 5);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r5-h1', 'bm-recipe-saison-dupont', 'Styrian Goldings', 5.5, 'Pellet', 0.0284, 'Boil', 90, 1),
  ('bm-r5-h2', 'bm-recipe-saison-dupont', 'East Kent Goldings', 5.0, 'Pellet', 0.0142, 'Boil', 10, 2);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r5-y1', 'bm-recipe-saison-dupont', 'Belgian Saison', 'ale', 'liquid', 'Wyeast', '3724', 78.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r5-mash', 'bm-recipe-saison-dupont', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r5-ms1', 'bm-r5-mash', 'Saccharification Rest', 'Infusion', 63.0, 90, 1);
