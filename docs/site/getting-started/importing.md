# Importing a recipe (BeerXML)

If you're coming from Brewfather, BeerSmith, BrewUnited, or any other homebrewing tool, you don't have to rebuild your recipes from scratch. Brewski supports BeerXML 1.0, which is the standard format those tools use for recipe export.

## How to import

In the recipe list sidebar, hit **Import BeerXML**. Pick your `.xml` file and Brewski will pull it in.

## What comes through

Brewski imports the core recipe data:

- Recipe name, style, and batch size
- Fermentables: name, yield, and color
- Hops: name, alpha %, use type, and time
- Yeast: name and attenuation
- Mash steps: temperature and time

## What might need a quick review

A few things don't map perfectly from every tool:

**Equipment profile.** BeerXML includes equipment data, but Brewski uses your own saved profiles for volume and efficiency calculations. After importing, pick the right equipment profile for your setup.

**Water chemistry.** Detailed water additions and targets sometimes come through with limited data. Double-check your water profile if that's part of your recipe.

**Custom ingredients.** If an imported ingredient doesn't match anything in Brewski's library, Brewski adds it as a custom entry. The numbers come over as-is, and you can edit the entry later if you need to adjust anything.

## After the import

Your recipe shows up in the recipe list as soon as the import finishes. Open it, look through the ingredients and stats, and make any adjustments. Most recipes come through cleanly and just need the equipment profile set.

::: tip Exporting from Brewfather
In Brewfather, open your recipe and go to **Recipe → Export to BeerXML**. That file is ready to import straight into Brewski.
:::
