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

## Mash and sparge water

| Field | What it does |
|---|---|
| **Sparge Method** | "No Sparge", "Batch Sparge", or "Fly Sparge". This controls how Brewski splits your water between the mash and sparge. |
| **Water/Grain Ratio** | Target mash thickness, in L/kg (or qt/lb). A typical infusion mash runs around 2.5 to 3.5 L/kg. |
| **Grain Absorption Rate** | How much water your grain holds back per kg. The default (1.04 L/kg) works for most setups. Adjust if your efficiency calculations are consistently off. |
| **Mash-Tun Deadspace** | The volume trapped below your false bottom or screen that can't drain to the kettle. This gets added to your total water on sparge setups so you're collecting enough wort. On no-sparge setups, deadspace drains fully into the kettle and is not counted as a loss. |
| **Mash Volume Limit** | The maximum mash volume (water plus grain) your tun can hold. When your mash would exceed this, Brewski automatically adjusts water amounts to fit. |

### What happens when the mash is too big

When you've set a **Mash Volume Limit** and your recipe's mash volume would exceed it, Brewski handles it differently depending on your sparge method.

**No sparge:** all your water goes into the mash at once, so there's nowhere else to redirect it. Instead, Brewski reduces mash water and moves the excess to **Top-Up Water** (post-boil fermenter addition). Your total water in doesn't change, just when you add it. The **Water** tab shows a yellow message telling you how much was shifted.

**Batch or fly sparge:** Brewski caps your mash water at the limit and moves the overflow to sparge water. Your total water and pre-boil volume stay the same, it's just redistributed between mash and sparge.

**Kettle batch volume target:** if your equipment profile uses "Kettle" as the batch volume target, the no-sparge redistribution doesn't apply. Pre-boil volume is fixed directly as your batch size, so there's no top-up water to shift into. In that case the **Water** tab shows a red warning if the mash volume is over the limit, and you'll need to adjust your recipe manually.

## Tips

**Start with typical values.** If you're new to your system, start with an evaporation rate of 10% and kettle loss of 1 L. Measure what actually happens over your first few batches and adjust from there.

**Dial in efficiency over time.** Most home systems land between 70-80%. Log your OG readings on brew day and compare to the recipe estimate. After three or four batches you'll have a solid number.

**BIAB setups.** If you're brewing in a bag with a single vessel, set **Mash Tun Loss** and **HLT Deadspace** to 0. Use **Top-Up Water** if you top up the fermenter after the boil.

## Copying a profile

Click the copy icon next to any profile to duplicate it. This is handy if you want to experiment with a variation of your main setup without losing your dialed-in numbers.

## Setting a default profile

The profile selected in **Settings** under **Ingredients** is used automatically when you create a new recipe. You can always swap it out per recipe in the recipe's Overview tab.
