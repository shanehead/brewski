# Hops

Hops do three things: bitterness, flavor, and aroma. Which one you get most of depends almost entirely on when and how you add them. Brewski tracks all of it and calculates your IBUs using the Tinseth formula.

## Adding a hop

Open the **Ingredients** tab and find the **Hops** card. Hit **Add Hop**.

Search by variety name. Select the hop and it's added with a default use type and time. Change those to match what you're actually doing.

## Use types

**Boil** hops go in during the rolling boil. The longer they're in, the more bitterness they contribute. Aroma isomerizes out over a long boil, so bittering additions are typically 60 minutes. Shorter additions, 10–15 minutes, give more flavor with some aroma. Watch the IBU number in the stats sidebar climb as you add boil hops or increase their time.

**Whirlpool** hops go in after the heat is off, while the wort is still circulating. The wort stays hot enough to extract compounds but cools enough that you preserve more aroma. You get more flavor and aroma than a standard boil addition, with less bitterness relative to the weight you're using.

**Hopstand** is a hot steep after flameout at a specific temperature you set. The temperature matters because it affects isomerization: higher hopstand temps contribute more IBU, lower temps shift the contribution toward aroma. Brewski lets you set the hopstand temperature, and it uses that to estimate the IBU contribution correctly.

**Dry Hop** additions go into the fermenter during fermentation or conditioning. They contribute zero bitterness. All you're getting is aroma, and it's the freshest, most aromatic expression of a hop variety. Time and temperature during dry hopping affect how much you extract and how long before the aroma fades.

## Alpha %

Alpha acids are what isomerize into bitterness during the boil. The alpha % is how concentrated they are in the hop you're using. Higher alpha means fewer grams are needed to hit your IBU target.

Don't leave this field at zero. It directly drives the IBU calculation. Alpha % varies by variety and crop year, so check the actual packet if you have it rather than relying on a typical value.

## Hop form

Pellets are the default and what most homebrewers use. If you're using cryo hops (concentrated lupulin pellets), set the form to **Cryo**. Cryo hops have higher utilization than standard pellets, so the same weight contributes more. Brewski adjusts the calculation accordingly and you'll typically use less weight to hit the same IBU target.

## A note on IBU accuracy

IBU calculations are estimates. Tinseth is well-regarded and works well in practice, but your actual bitterness depends on wort pH, hop freshness, and your system's utilization. Over time, you'll learn how your setup compares to the model. Use the IBU number as a target, not a guarantee.

::: tip New to a hop variety?
Look up typical usage rates for that variety and style before adding it. Some varieties are extremely potent at low additions, others need a heavier hand. The alpha % on the packet is your most important data point.
:::
