# Refractometer

The **Refractometer** calculator converts raw Brix readings into accurate specific gravity values. It handles two situations: a pre-fermentation OG reading and a post-fermentation FG correction. You'll find it under **Tools** in the left sidebar.

## OG mode

Use this before fermentation starts to convert your Brix reading to SG.

### Inputs

| Field | Description |
|---|---|
| **Brix Reading** | The value shown on your refractometer |
| **Wort Correction Factor (WCF)** | A multiplier that accounts for your wort's optical properties |

### Output

| Output | Description |
|---|---|
| **Corrected SG** | Your actual original gravity in standard SG format |

### How it works

Pure sugar solution and wort refract light slightly differently. The WCF compensates for this. The default value is **1.04**, which is accurate for most worts. Check your refractometer's manual: some instruments are already pre-corrected for wort and the WCF should be set to 1.00 for those.

## FG correction mode

Use this after fermentation is complete when you want to use your refractometer to measure final gravity.

### Inputs

| Field | Description |
|---|---|
| **OG (Brix)** | Your original Brix reading taken before fermentation |
| **FG (Brix)** | Your current raw Brix reading from the refractometer |
| **WCF** | Wort Correction Factor, same as above |

### Output

| Output | Description |
|---|---|
| **Corrected FG** | Your actual final gravity in SG |

### Why correction is needed

Once alcohol is present in your beer, it changes how light refracts through the sample. A raw Brix reading post-fermentation is not accurate on its own: the alcohol makes the reading lower than the real sugar content. Without correction, a post-fermentation Brix number is meaningless.

Brewski uses the Novotný formula, which accounts for both the residual sugars and the alcohol's effect on refraction. It's widely accepted as the most accurate correction available for post-fermentation refractometer readings.

## Tips

**Always take your OG reading before fermentation.** The FG correction requires your original pre-fermentation Brix reading as an input. If you forgot, check your recipe's estimated OG or calculate it from your measured SG if you have it.

**For critical FG measurements, confirm with a hydrometer.** The Novotný formula is accurate for typical fermentations, but a hydrometer is still the gold standard for final gravity. Use the refractometer correction as a convenient option, not a replacement.
