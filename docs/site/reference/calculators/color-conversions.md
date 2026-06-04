# Color Conversions

The **Color Conversions** calculator converts between the three color scales used in brewing. Enter a value in any unit and get the equivalent in all three. You'll find it under **Tools** in the left sidebar.

## Input

Enter your color value in whichever unit you have. The calculator accepts SRM, EBC, or Lovibond.

## Output

| Output | Description |
|---|---|
| **SRM** | Standard Reference Method |
| **EBC** | European Brewing Convention |
| **Lovibond** | Degrees Lovibond |

## The three scales

| Scale | Common usage | Reference points |
|---|---|---|
| **SRM** | US homebrewing and commercial brewing standard. Higher numbers mean darker beer. | Pale lager: 2-4. Amber ale: 10-18. Stout: 40+. |
| **EBC** | European equivalent of SRM. Used on European malt spec sheets. | EBC is approximately SRM × 1.97. |
| **Lovibond** | An older scale, still found on some malt spec sheets and color wheels. | Numerically close to SRM in the homebrewing range. |

## Conversion formulas

**SRM and EBC** convert with a direct multiplier:

```
EBC = SRM × 1.97
SRM = EBC / 1.97
```

**Lovibond and SRM** use the Morey formula constants, the same polynomial used when Brewski calculates recipe color from your grain bill. At homebrewing gravities the difference between SRM and Lovibond is small, but it's not 1:1.

## Tips

**Check which scale your malt spec sheet uses.** Most UK and European maltsters report color in EBC. Most North American maltsters report in Lovibond. When you're entering a custom fermentable, make sure you're entering the value in the correct unit.

**Recipe SRM estimates are just estimates.** Actual beer color depends on your boil, pH, and process. The conversion calculator gives you exact unit math, but your finished beer may look slightly different from what the recipe predicts.
