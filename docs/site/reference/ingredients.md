# Ingredients

Brewski ships with hundreds of hops, fermentables, yeasts, and misc ingredients. Access the full library via the flask icon in the left sidebar.

## Categories

### Hops

Each hop entry includes:

| Field | Description |
|---|---|
| **Name** | Hop variety name |
| **Origin** | Country of origin |
| **Alpha %** | Alpha acid percentage, used for IBU calculation |
| **Beta %** | Beta acid percentage |
| **Typical Use** | Bittering, aroma, dual-use |

The library includes cryo hops, experimental varieties, and both classic and modern cultivars.

### Fermentables

Each fermentable entry includes:

| Field | Description |
|---|---|
| **Name** | Fermentable name |
| **Type** | Grain, extract, sugar, or adjunct |
| **Yield %** | How much fermentable sugar it contributes per unit weight |
| **Color** | Color in Lovibond, used for SRM calculation |
| **Supplier** | Typical maltster or supplier |

### Yeast

Each yeast entry includes:

| Field | Description |
|---|---|
| **Name** | Strain name |
| **Lab** | Manufacturer (Wyeast, White Labs, etc.) |
| **Product ID** | Lab's catalog number |
| **Type** | Ale, lager, or wild/mixed fermentation |
| **Attenuation %** | Expected attenuation range, used for FG estimate |
| **Temperature Range** | Recommended fermentation temperature range |

### Misc

Miscellaneous ingredients cover anything that doesn't add fermentable sugars. Examples: Irish Moss, Whirlfloc, lactose, nutrient blends, finings, and flavor adjuncts.

### Water

Brewski includes preset mineral profiles for common water targets:

- Balanced
- Soft
- Burton-on-Trent
- Pilsen
- Dublin

Select a preset as a starting point, or build a custom profile from scratch. See [Water chemistry](/guides/water-chemistry) for details.

## Data sources

Hop and fermentable data comes from publicly available brewing databases. Yeast data comes from Wyeast, White Labs, and dry yeast manufacturer specs. Where manufacturer data varies by lot or revision, Brewski uses the published typical values.

## Adding custom ingredients

You can add your own entries to any category. Custom ingredients work exactly like built-in ones: they show up in recipe search, carry the same fields, and are stored locally in your database.

For a full walkthrough, see [Using the ingredient library](/guides/ingredient-library).
