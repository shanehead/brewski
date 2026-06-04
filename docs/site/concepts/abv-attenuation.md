# ABV & attenuation

Two numbers tell you the most about how fermentation went: **ABV** (how much alcohol ended up in your beer) and **attenuation** (how much of the sugar the yeast converted). Both come from your OG and FG readings.

## Calculating ABV

Brewski uses the standard homebrewing formula:

```
ABV = (OG - FG) × 131.25
```

For a beer with OG 1.052 and FG 1.013, that's `(1.052 - 1.013) × 131.25 = 5.1% ABV`.

This formula is an approximation. It's accurate to within about 0.2-0.3% for typical beers (under ~8% ABV). For very high-gravity beers, the error grows slightly because alcohol's effect on density becomes more significant at high concentrations.

## Apparent attenuation

**Apparent attenuation** tells you what percentage of the original fermentable extract the yeast converted:

```
Attenuation = (OG - FG) / (OG - 1.000) × 100
```

For OG 1.052, FG 1.013: `(1.052 - 1.013) / (1.052 - 1.000) × 100 = 75%`.

Yeast strains have typical attenuation ranges. A highly attenuative strain might hit 80-85%. An English ale strain might land at 65-72%, leaving more body and residual sweetness. Mash temperature also plays a big role: a lower mash produces more fermentable wort, and a higher mash leaves more unfermentable dextrins.

## Why it's called "apparent"

The word "apparent" is a precision flag. Alcohol is less dense than water. When yeast converts sugar to alcohol, the FG reading goes down not just because sugar is gone, but also because alcohol is lighter than what it replaced. This makes the FG look lower than the actual remaining sugar content.

**Real attenuation** corrects for this by accounting for the alcohol's effect on density. It's always a few percentage points lower than apparent attenuation. For most homebrewing purposes, apparent attenuation is what you'll use. Real attenuation matters more in commercial quality control contexts.

## Higher vs. lower attenuation

Higher attenuation means drier beer and more alcohol. Lower attenuation means more residual sweetness and body with less alcohol. Neither is better or worse on its own: a West Coast IPA should finish dry, while a milk stout is supposed to have body and sweetness left over.

If your beer finishes higher than expected (lower attenuation than expected), look at mash temperature (too high?), yeast health (underpitched or stressed?), or whether the fermentation stalled early.

## Calories

Brewski estimates calories using the ASBC formula, which accounts for both contributions to caloric content:

- **Alcohol:** roughly 7 kcal per gram
- **Residual carbohydrates:** roughly 4 kcal per gram

The alcohol contribution dominates. A 5% ABV beer has significantly more calories than a 3.5% beer of similar apparent body, even if both finish at the same FG.
