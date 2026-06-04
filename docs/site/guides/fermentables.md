# Fermentables

Fermentables are anything that adds fermentable sugar to your beer: base malts, specialty malts, adjuncts like corn or rice, table sugar, and malt extract. Together they set your original gravity, your color, and a big chunk of the flavor foundation.

## Adding a fermentable

Open the **Ingredients** tab and find the **Fermentables** card. Hit **Add Fermentable**.

Type to search the library, which includes hundreds of common fermentables from most major maltsters. Select the one you want and it's added to your grain bill with typical defaults already filled in.

If you can't find what you're looking for, you can create a custom fermentable through the Ingredients library (the flask icon in the sidebar). Set your own values there and it'll show up in search from then on.

## Key fields

**Amount** is how much of this fermentable you're using. The unit (kg or lb) follows your settings.

**Yield %** is how much fermentable sugar this contributes per unit of weight. Base malts are typically in the 78–82% range. Pure sugars are close to 100%. If you're pulling a fermentable from the library, this is already set, but you can adjust it if you know your maltster's actual spec.

**Color** is measured in degrees Lovibond. Pale base malts sit around 1–3°L. Crystal malts range from 10°L all the way past 120°L, and roasted malts go higher still. The SRM estimate in the stats sidebar combines all of these together, so you can dial in your target color as you build the grain bill.

**Type** tells Brewski how to treat the fermentable in calculations. The options are grain, liquid extract, dry extract, sugar, and adjunct. Grain goes through the mash. Extracts and sugars get counted differently. Getting this right matters for accurate gravity and efficiency numbers.

**Add after boil** is the checkbox you want for adjuncts or sugars added at flameout or later in the process. Checking it removes that fermentable from your mash efficiency calculation, which is the right call because it never went through the mash.

## What's updating in the sidebar

As you add fermentables, watch the **OG** and **SRM** numbers in the stats sidebar. OG climbs with every addition. SRM shifts as you add darker malts. It's a fast feedback loop that makes it easy to dial things in without doing math by hand.

## Building your grain bill

Base malt is the backbone. For most styles, it makes up 60–80% of the grain bill. Everything else, crystal malts, roasted malts, specialty grains, adjuncts, layers on top of that foundation. Start with your base, add specialty malts a little at a time, and check the OG and SRM as you go.

::: tip Malt extract recipes
If you're brewing with extract, set the fermentable type to **Liquid Extract** or **Dry Extract** and skip the mash profile entirely. Brewski handles extract recipes and all-grain recipes the same way in the stats calculations.
:::
