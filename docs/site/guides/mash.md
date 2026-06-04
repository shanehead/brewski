# Mash

The mash is where your grain bill converts from starches into fermentable sugars. The temperature you hold, the amount of water you use, and how long you rest all affect your final beer. Brewski's **Mash** tab is where you define all of it.

## Adding your first mash step

Open the **Mash** tab. If there's no mash step yet, hit **Add Step** and Brewski creates one for you to configure.

For most beers, a single-infusion mash is all you need: one step, one temperature, one water addition, and you're done.

## Mash step fields

**Step name** is just a label. "Saccharification" or "Single Infusion" works fine. Name it whatever makes sense to you.

**Type** is the step method. Infusion steps add hot water to reach your target temperature. Temperature steps (also called decoction steps) assume your system is heating the existing mash to the target. Most homebrewers use infusion steps.

**Temperature** is your target mash temperature in °C (or °F depending on your settings). This is the most important variable in the mash.

- **65–67°C (149–153°F)** favors fermentability. The enzymes active at lower temps produce more fermentable sugars, which means a drier, crisper beer with more alcohol.
- **68–70°C (154–158°F)** favors body. More unfermentable dextrins make it through to the finished beer, giving you a fuller, sweeter result.

**Duration** is how long you hold that step, in minutes. 60 minutes covers most styles. Some brewers go longer for certain grain bills, but most well-modified modern malts convert fully in 45–60 minutes.

**Infuse amount** is how much water you're adding for this step. Brewski divides this by your grain weight to show your water-to-grain ratio. A typical ratio is 2.5–4 L/kg for traditional mash setups. BIAB (brew in a bag) brewers often use the full kettle volume for a thinner, more fluid mash.

## Grain temperature

Set your grain temperature in the mash section. This is the temperature of your grain before you add the strike water, and it matters because cold grain absorbs heat from the water when you add it. If your grain has been sitting in a cool garage, your strike water needs to be hotter to compensate.

Room temperature (around 20°C/68°F) is a reasonable default if you keep your grain indoors.

## Strike temperature

The **strike temperature** appears in the stats sidebar. It's the temperature your water needs to be when you add it to the grain, accounting for heat absorption by the grain and your equipment. You don't calculate it yourself. Brewski does it from your target mash temperature, grain temperature, and infuse amount.

Heat your strike water to that temperature, add it to your grain, stir, and check with a thermometer. You'll land close to target.

## Step mashing

Some styles and some equipment benefit from multiple mash rests. Hit **Add Step** to add more steps.

Common additions include:

- **Protein rest** (50–55°C / 122–131°F) before the saccharification rest, for hazy beers or undermodified malts
- **Mashout** (75–78°C / 167–172°F) at the end to stop enzyme activity and improve lauter flow, useful for thick mashes or fly sparging setups

For most modern homebrew-scale beers, you don't need these. A single infusion at your target saccharification temperature is the right call.

::: tip BIAB and no-sparge brewing
If you're brewing BIAB or no-sparge, set your infuse amount to your full pre-boil volume. Your water-to-grain ratio will be higher than traditional mashing, but that's expected and fine for those methods.
:::
