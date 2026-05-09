ALTER TABLE yeasts ADD COLUMN min_attenuation_pct REAL;
ALTER TABLE yeasts ADD COLUMN max_attenuation_pct REAL;
ALTER TABLE yeasts ADD COLUMN alcohol_tolerance TEXT;
ALTER TABLE yeasts ADD COLUMN flavor_profile TEXT;
ALTER TABLE yeasts ADD COLUMN styles TEXT;
ALTER TABLE yeasts ADD COLUMN substitutes TEXT;
ALTER TABLE yeasts ADD COLUMN species TEXT;
ALTER TABLE yeasts ADD COLUMN pof_positive INTEGER;
ALTER TABLE yeasts ADD COLUMN sta1_positive INTEGER;

DELETE FROM yeasts WHERE id IN (
    'y-us05', 'y-1056', 'y-wlp001', 'y-s04', 'y-1084',
    'y-wlp300', 'y-t58', 'y-w34-70', 'y-s189'
);

-- Re-seed yeasts with new columns populated
INSERT OR IGNORE INTO yeasts (id, name, type, form, laboratory, product_id, min_temperature_c, max_temperature_c, flocculation, attenuation_pct, min_attenuation_pct, max_attenuation_pct, alcohol_tolerance, flavor_profile, styles, substitutes, species, pof_positive, sta1_positive) VALUES
('y-us05', 'American Ale (US-05)', 'ale', 'dry', 'Fermentis', 'US-05', 15.0, 24.0, 'medium', 77.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-1056', 'American Ale (WY1056)', 'ale', 'liquid', 'Wyeast', '1056', 16.0, 22.0, 'medium', 75.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-wlp001', 'California Ale (WLP001)', 'ale', 'liquid', 'White Labs', 'WLP001', 20.0, 23.0, 'medium', 77.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-s04', 'English Ale (S-04)', 'ale', 'dry', 'Fermentis', 'S-04', 15.0, 24.0, 'high', 73.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-1084', 'Irish Ale (WY1084)', 'ale', 'liquid', 'Wyeast', '1084', 16.0, 22.0, 'medium', 72.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-wlp300', 'Hefeweizen (WLP300)', 'wheat', 'liquid', 'White Labs', 'WLP300', 18.0, 23.0, 'low', 74.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-t58', 'Belgian Ale (T-58)', 'ale', 'dry', 'Fermentis', 'T-58', 15.0, 24.0, 'medium', 78.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-w34-70', 'Bohemian Lager (W-34/70)', 'lager', 'dry', 'Fermentis', 'W-34/70', 9.0, 15.0, 'high', 80.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
('y-s189', 'Lager (S-189)', 'lager', 'dry', 'Fermentis', 'S-189', 9.0, 15.0, 'medium', 80.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
