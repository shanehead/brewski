# Recipe versions

Recipes evolve. Brewski saves your edits automatically, so your live recipe is always current.

Versions are something different. They're checkpoints you create deliberately, when the recipe is in a state you want to be able to get back to. Once saved, a version is a fixed snapshot you can brew from or restore at any time.

## Saving a version

Click the clock icon in the recipe header to open the version history panel.

If your recipe has changed since the last saved version, you'll see a warning at the top of the panel: **un-versioned changes**. This just means your live recipe has changes that haven't been saved as a version yet. Click **Save as version** to capture it. You can give it a name like "Dropped Mosaic, added Nelson" or "Scaled to 20L", or leave it blank. Either way, the save records the full recipe as it is right now.

If there's no warning, your working copy is already in sync with the latest version.

## Viewing a version

Click any version in the panel to see the recipe as it was at that point. Ingredients, mash, water adjustments, stats. All of it, read-only. Click outside the panel to return to the live recipe.

## Branching from a version

Branching resets the live recipe back to match a saved version. Select a version in the panel, then click **Branch from here**.

This is useful when a recipe has taken a direction you want to split off. Say you started with a pale ale and evolved it into an IPA across a few versions. Branch from the original pale ale state to develop that direction as its own independent recipe.

Brewski asks for confirmation before branching. Once you confirm, it replaces the live recipe's ingredients and settings with that version's data. The version history stays intact.

## How batches connect to versions

When you click **Brew this Recipe**, Brewski shows a modal. What appears depends on your recipe's state.

If the recipe has unsaved changes, or if it's never been versioned, you'll see **Brew with current changes**. That saves a version right now, with an optional name, then creates the batch from it.

If you have saved versions, **Brew a saved version** also appears. Pick any saved version to brew from an older snapshot instead.

The batch is permanently linked to whichever snapshot you brewed from. Keep developing the recipe and your batch history stays accurate. You'll always know what was actually in the kettle.

## What versions track

A version captures your recipe's formulation: ingredients and their amounts, the mash profile, water sources, water adjustments, and settings like batch size, boil time, and efficiency.

Equipment profiles and water profiles aren't part of the snapshot. They're shared across all your recipes, not recipe-specific. Editing your equipment profile affects everywhere it's used. But versions do record which profile is assigned. If you switch a recipe from one equipment profile to another, that switch is part of the version.

## Deleting a version

Select a version in the panel and click **Delete**.

Any version not linked to a batch can be deleted. If a batch was brewed from it, the version stays. The historical record matters more than a tidy list.
