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
