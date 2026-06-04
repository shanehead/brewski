# IBU: bitterness explained

**IBU** stands for International Bitterness Units. It measures the concentration of iso-alpha acids in your beer, the compounds responsible for hop bitterness. More IBUs means more bitterness potential, but the actual bitterness you perceive is always about balance.

## What IBUs measure

Alpha acids in hops aren't bitter on their own. During the boil, heat transforms them into **iso-alpha acids** through a process called isomerization. IBUs measure the concentration of those iso-alpha acids in the finished beer, in milligrams per liter. A 10 IBU blonde ale and a 70 IBU West Coast IPA have measurably different concentrations, but the perceived bitterness gap can feel even larger because of the malt sweetness each one carries.

## Perceived bitterness vs. measured bitterness

Two beers with the same IBU count can taste completely different. A 40 IBU amber ale with an OG of 1.065 tastes malty and balanced. A 40 IBU session pale ale at 1.040 tastes sharp and dry. The malt sweetness in the amber absorbs the bitterness; the session pale has much less sugar to balance against.

The **BU:GU ratio** (bitterness-to-gravity) captures this balance. Divide your IBU by your gravity in "gravity units," which is `(OG - 1) × 1000`. For OG 1.052, that's `IBU / 52`. Around 1.0 is balanced. Below 0.5 is malt-forward. Above 1.5 is aggressively bitter.

## How Brewski calculates IBUs

Brewski uses the **Tinseth formula**, the most widely used model in homebrewing software. It takes five inputs:

- Hop alpha acid percentage
- Amount of hops (grams)
- Boil time (minutes)
- Wort gravity during the boil
- Post-boil volume (liters)

Longer boil times, lower gravity wort, and smaller batch volumes all increase the IBU contribution from a given hop addition. Higher gravity wort is less efficient at isomerizing alpha acids, so big beers need more hops to hit the same IBU target.

## Whirlpool and hopstand hops

Hops added after the boil in a whirlpool or hopstand still contribute IBUs if the wort is hot enough. Isomerization continues down to around 80°C (176°F). Brewski accounts for this with a configurable utilization percentage based on the hopstand temperature you set. Higher temperatures extract more IBUs along with flavor and aroma. Lower temperatures shift the contribution almost entirely toward aroma.

## Why your actual IBUs will differ

IBU calculations are models, not measurements. A few things will affect where you actually land:

- **Alpha % varies by harvest.** The number on the packet is measured at packaging. Old hops lose potency over time. Use the actual packet value when you have it rather than the variety's typical value.
- **Your chilling speed matters.** The longer wort stays hot after flameout, the more isomerization happens. A fast plate chiller will give you fewer whirlpool IBUs than a slow ice bath.
- **Hop form affects utilization.** Whole leaf hops have roughly 10% lower utilization than pellets. Brewski defaults to pellet utilization. If you're using whole hops, expect slightly lower bitterness for the same amount.
- **Wort pH and kettle geometry** also influence utilization in ways the Tinseth formula doesn't account for.

Use your IBU number as a consistent target and a tool for comparing recipes, not as a precise laboratory value.
