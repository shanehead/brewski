# ABV & Calories

The **ABV & Calories** calculator tells you how strong your beer is and how many calories are in a standard 12 oz serving. You'll find it under **Tools** in the left sidebar.

## Inputs

| Field | Description |
|---|---|
| **OG** | Your original gravity, measured before fermentation started |
| **FG** | Your final gravity, measured once fermentation is complete |

## Outputs

| Output | Description |
|---|---|
| **ABV %** | Alcohol by volume |
| **Apparent Attenuation %** | Percentage of the original gravity that fermented |
| **Calories** | Estimated calories per 355 mL (12 oz) serving |

## How it works

### ABV

Brewski uses the Balling formula:

```
ABV = (OG - FG) × 131.25
```

This formula is accurate for most homebrewing purposes, covering typical gravities from session beers up through strong ales and barleywines.

### Apparent attenuation

Apparent attenuation tells you what fraction of the fermentable sugars your yeast consumed:

```
Apparent Attenuation = ((OG - FG) / (OG - 1.000)) × 100
```

A result in the 70-80% range is typical for most ale yeasts. Higher attenuation means a drier beer; lower attenuation means more residual body and sweetness.

### Calories

The calorie estimate uses the ASBC formula, which accounts for two calorie sources separately:

- **Alcohol:** approximately 7 kcal per gram
- **Residual carbohydrates:** approximately 4 kcal per gram

Both contributions are derived from the OG and FG and then scaled to a 355 mL serving size.

## Tips

**Use your measured FG, not the estimated one.** The recipe's estimated FG is a target, not a measurement. For the most accurate ABV and calorie numbers, enter the actual FG from your hydrometer or corrected refractometer reading after fermentation is complete.

**Refractometer readings post-fermentation need correction.** Raw Brix from a refractometer are unreliable once alcohol is present. Run your reading through the [Refractometer](/reference/calculators/refractometer) calculator first to get a corrected FG in SG.
