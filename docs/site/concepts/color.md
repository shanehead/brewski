# SRM & EBC: color explained

Beer color is measured on two scales: **SRM** (Standard Reference Method, used in the US) and **EBC** (European Brewery Convention, used in Europe). Both measure the same thing: how much light a beer absorbs. Higher numbers mean darker beer.

## How Brewski calculates color

Brewski uses the **Morey formula** to estimate SRM from your grain bill. Each fermentable contributes color based on its weight (in kg), its color rating (in **Lovibond**), and your batch volume (in liters). The formula accounts for the fact that color compounds stack non-linearly: adding a very dark grain to an already-dark beer doesn't darken it as dramatically as adding that same grain to a pale base.

Lovibond is the unit used to measure the color of individual malts. It's roughly equivalent to SRM in the homebrewing range (pale to medium-dark malts), but they diverge at high color values. Brewski converts using `(1.3546 × Lovibond) - 0.76` when needed.

## SRM scale reference

Here's a rough guide to what SRM values look like in the glass:

| SRM | What you're looking at |
|---|---|
| 1-4 | Pale straw to light yellow. Mass-market lagers, light beers. |
| 5-8 | Golden. Blonde ale, hefeweizen, witbier. |
| 9-14 | Amber to light copper. Amber ale, Marzen, pale ale. |
| 15-22 | Deep copper to brown. Brown ale, English porter. |
| 23-40 | Dark brown to near-black. Stout, robust porter. |
| 40+ | Opaque black. Imperial stout, black IPA. |

## EBC and Lovibond

**EBC = SRM × 1.97.** A beer at 10 SRM is about 20 EBC. If you're used to one system and see a recipe in the other, that conversion handles it.

**Lovibond** is a malt measurement, not a beer measurement. Your pale malt might be 2-3°L. Chocolate malt might be 350°L. Crystal 60 is around 60°L. These grain color values are what Brewski uses to calculate the final beer color.

## Color is an estimate

The SRM number Brewski shows is a calculated estimate based on your grain bill. Actual beer color in the glass depends on factors the formula can't account for: how clear or hazy the beer is, whether it's carbonated, the shape of the glass, and even lighting. A crystal-clear beer in a straight pint looks noticeably darker than the same beer served in a thistle glass. Take the SRM number as a useful guide for hitting your target color range, not as a guarantee of what you'll see in the glass.
