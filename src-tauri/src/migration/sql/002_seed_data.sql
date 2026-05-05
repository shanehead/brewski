-- BJCP 2021 style subset (American ales)
INSERT OR IGNORE INTO styles (id, name, category, category_number, style_letter, style_guide, type, og_min, og_max, fg_min, fg_max, ibu_min, ibu_max, color_min_srm, color_max_srm, abv_min_pct, abv_max_pct) VALUES
('bjcp-18b', 'American Pale Ale', 'Pale American Ale', '18', 'B', 'BJCP 2021', 'Ale', 1.045, 1.060, 1.010, 1.015, 30, 50, 5, 10, 4.5, 6.2),
('bjcp-21a', 'American IPA', 'IPA', '21', 'A', 'BJCP 2021', 'Ale', 1.056, 1.070, 1.008, 1.014, 40, 70, 6, 14, 5.5, 7.5),
('bjcp-15b', 'American Porter', 'Dark British Beer', '15', 'B', 'BJCP 2021', 'Ale', 1.050, 1.070, 1.012, 1.018, 25, 40, 22, 40, 4.8, 6.5),
('bjcp-20a', 'American Stout', 'American Porter and Stout', '20', 'A', 'BJCP 2021', 'Ale', 1.050, 1.075, 1.010, 1.022, 35, 75, 30, 40, 5.0, 7.0),
('bjcp-1b', 'American Lager', 'Standard American Beer', '1', 'B', 'BJCP 2021', 'Lager', 1.040, 1.050, 1.004, 1.010, 8, 18, 2, 3, 4.2, 5.3),
('bjcp-26c', 'Belgian Tripel', 'Trappist Ale', '26', 'C', 'BJCP 2021', 'Ale', 1.075, 1.085, 1.008, 1.014, 20, 40, 4.5, 7, 7.5, 9.5),
('bjcp-9c', 'Baltic Porter', 'Brown British Beer', '9', 'C', 'BJCP 2021', 'Lager', 1.060, 1.090, 1.016, 1.024, 20, 40, 17, 30, 6.5, 9.5);

-- Common fermentables
INSERT OR IGNORE INTO fermentables (id, name, type, yield_pct, color_lovibond) VALUES
('f-pale-malt', 'Pale Malt (2-row)', 'grain', 78.0, 1.8),
('f-pale-malt-6', 'Pale Malt (6-row)', 'grain', 73.0, 1.8),
('f-pilsner', 'Pilsner Malt', 'grain', 75.0, 1.6),
('f-munich', 'Munich Malt', 'grain', 77.0, 9.0),
('f-vienna', 'Vienna Malt', 'grain', 77.0, 3.5),
('f-crystal-40', 'Crystal/Caramel 40L', 'grain', 74.0, 40.0),
('f-crystal-60', 'Crystal/Caramel 60L', 'grain', 74.0, 60.0),
('f-crystal-120', 'Crystal/Caramel 120L', 'grain', 72.0, 120.0),
('f-chocolate', 'Chocolate Malt', 'grain', 60.0, 350.0),
('f-roasted-barley', 'Roasted Barley', 'grain', 55.0, 300.0),
('f-black-patent', 'Black Patent Malt', 'grain', 53.0, 500.0),
('f-wheat', 'White Wheat Malt', 'grain', 77.0, 2.0),
('f-dme-light', 'Dry Malt Extract - Light', 'dry extract', 95.0, 4.0),
('f-dme-amber', 'Dry Malt Extract - Amber', 'dry extract', 95.0, 10.0),
('f-corn-sugar', 'Corn Sugar (Dextrose)', 'sugar', 96.0, 0.5);

-- Common hops
INSERT OR IGNORE INTO hops (id, name, alpha_pct, form, type, origin) VALUES
('h-cascade', 'Cascade', 5.5, 'pellet', 'aroma', 'US'),
('h-centennial', 'Centennial', 10.0, 'pellet', 'bittering/aroma', 'US'),
('h-chinook', 'Chinook', 13.0, 'pellet', 'bittering', 'US'),
('h-citra', 'Citra', 12.0, 'pellet', 'aroma', 'US'),
('h-columbus', 'Columbus (CTZ)', 15.0, 'pellet', 'bittering', 'US'),
('h-fuggle', 'Fuggle', 4.5, 'pellet', 'aroma', 'UK'),
('h-hallertau', 'Hallertau Mittelfrüh', 4.0, 'pellet', 'aroma', 'Germany'),
('h-magnum', 'Magnum', 14.0, 'pellet', 'bittering', 'Germany'),
('h-mosaic', 'Mosaic', 12.5, 'pellet', 'aroma', 'US'),
('h-saaz', 'Saaz', 3.5, 'pellet', 'aroma', 'Czech Republic'),
('h-simcoe', 'Simcoe', 13.0, 'pellet', 'aroma/bittering', 'US'),
('h-willamette', 'Willamette', 5.0, 'pellet', 'aroma', 'US');

-- Common yeasts
INSERT OR IGNORE INTO yeasts (id, name, type, form, laboratory, product_id, min_temperature_c, max_temperature_c, flocculation, attenuation_pct) VALUES
('y-us05', 'American Ale (US-05)', 'ale', 'dry', 'Fermentis', 'US-05', 15.0, 24.0, 'medium', 77.0),
('y-1056', 'American Ale (WY1056)', 'ale', 'liquid', 'Wyeast', '1056', 16.0, 22.0, 'medium', 75.0),
('y-wlp001', 'California Ale (WLP001)', 'ale', 'liquid', 'White Labs', 'WLP001', 20.0, 23.0, 'medium', 77.0),
('y-s04', 'English Ale (S-04)', 'ale', 'dry', 'Fermentis', 'S-04', 15.0, 24.0, 'high', 73.0),
('y-1084', 'Irish Ale (WY1084)', 'ale', 'liquid', 'Wyeast', '1084', 16.0, 22.0, 'medium', 72.0),
('y-wlp300', 'Hefeweizen (WLP300)', 'wheat', 'liquid', 'White Labs', 'WLP300', 18.0, 23.0, 'low', 74.0),
('y-t58', 'Belgian Ale (T-58)', 'ale', 'dry', 'Fermentis', 'T-58', 15.0, 24.0, 'medium', 78.0),
('y-w34-70', 'Bohemian Lager (W-34/70)', 'lager', 'dry', 'Fermentis', 'W-34/70', 9.0, 15.0, 'high', 80.0),
('y-s189', 'Lager (S-189)', 'lager', 'dry', 'Fermentis', 'S-189', 9.0, 15.0, 'medium', 80.0);

-- Default equipment profile
INSERT OR IGNORE INTO equipment_profiles (id, name, boil_size_l, batch_size_l, boil_time_min, evap_rate_pct_hr, trub_chiller_loss_l, fermenter_loss_l, hop_utilization_pct, efficiency_pct, created_at, updated_at) VALUES
('eq-default', 'Standard 5 Gallon', 27.0, 23.0, 60.0, 10.0, 1.5, 1.0, 100.0, 72.0, 0, 0);

INSERT OR IGNORE INTO settings (key, value) VALUES ('default_equipment_profile_id', 'eq-default')
  ON CONFLICT(key) DO NOTHING;
