-- Normalize enum casing in seeded ingredients to match UI select options.
-- Seeded data was imported with lowercase/hyphenated values; user-created
-- ingredients use title case from the select options.

-- Hops: form (pellet -> Pellet, leaf -> Leaf, plug -> Plug)
UPDATE hops
SET form = UPPER(SUBSTR(form, 1, 1)) || LOWER(SUBSTR(form, 2))
WHERE source = 'seeded';

-- Yeasts: type (ale -> Ale, lager -> Lager, etc.)
UPDATE yeasts
SET type = UPPER(SUBSTR(type, 1, 1)) || LOWER(SUBSTR(type, 2))
WHERE source = 'seeded';

-- Yeasts: form (liquid -> Liquid, dry -> Dry)
UPDATE yeasts
SET form = UPPER(SUBSTR(form, 1, 1)) || LOWER(SUBSTR(form, 2))
WHERE source = 'seeded';

-- Yeasts: flocculation — map to UI select options (Low/Medium/High/Very High)
UPDATE yeasts
SET flocculation = CASE
    WHEN flocculation IN ('low', 'very-low')       THEN 'Low'
    WHEN flocculation IN ('medium', 'medium-low')  THEN 'Medium'
    WHEN flocculation IN ('medium-high', 'high')   THEN 'High'
    WHEN flocculation IN ('very-high', 'very high') THEN 'Very High'
    ELSE NULL
END
WHERE source = 'seeded' AND flocculation IS NOT NULL;
