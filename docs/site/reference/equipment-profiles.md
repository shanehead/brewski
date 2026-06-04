# Equipment profiles

Equipment profiles describe your physical brew setup so Brewski can calculate volumes and gravities accurately. Every recipe uses an equipment profile, and the numbers here directly affect your pre-boil volume, strike water, and OG estimates.

Access your profiles via the equipment (kettle) icon in the left sidebar.

## Fields

| Field | What it does |
|---|---|
| **Batch Size** | Your target volume, interpreted based on the **Batch Volume Target** setting below. |
| **Batch Volume Target** | "Fermenter" means **Batch Size** is your target into the fermenter. "Kettle" means **Batch Size** is your target post-boil kettle volume. Most setups use Fermenter. |
| **Evaporation Rate %** | How much water evaporates per hour of boiling. Typical range: 8-12%. Measure yours over a few batches for best accuracy. |
| **Trub/Chiller Loss** | Volume left behind in the kettle after transfer to the fermenter. Typical: 0.5-2 L. |
| **Fermenter Loss** | Volume lost to yeast cake and trub when packaging. Typical: 0.5-1.5 L. |
| **Mash Tun Loss** | Dead space in your mash tun that doesn't drain. Relevant for HERMS, RIMS, and cooler setups. |
| **HLT Deadspace** | Dead space in your hot liquor tank. Relevant for three-vessel setups. |
| **Top-Up Water** | Post-boil water added to the fermenter. Use this for BIAB top-up or partial boil setups. |
| **Efficiency %** | Your expected mash efficiency. Brewski uses this to estimate your OG. Dial this in over a few batches. |
| **Whirlpool Time** | Minutes of whirlpool rest after the boil. This affects the IBU calculation for whirlpool hop additions. |
| **Calc Mash Efficiency** | Toggle to enable separate mash efficiency tracking. |
| **Mash Efficiency %** | Your expected mash efficiency as a separate figure (used when the toggle above is on). |

## Tips

**Start with typical values.** If you're new to your system, start with an evaporation rate of 10% and kettle loss of 1 L. Measure what actually happens over your first few batches and adjust from there.

**Dial in efficiency over time.** Most home systems land between 70-80%. Log your OG readings on brew day and compare to the recipe estimate. After three or four batches you'll have a solid number.

**BIAB setups.** If you're brewing in a bag with a single vessel, set **Mash Tun Loss** and **HLT Deadspace** to 0. Use **Top-Up Water** if you top up the fermenter after the boil.

## Copying a profile

Click the copy icon next to any profile to duplicate it. This is handy if you want to experiment with a variation of your main setup without losing your dialed-in numbers.

## Setting a default profile

The profile selected in **Settings** under **Ingredients** is used automatically when you create a new recipe. You can always swap it out per recipe in the recipe's Overview tab.
