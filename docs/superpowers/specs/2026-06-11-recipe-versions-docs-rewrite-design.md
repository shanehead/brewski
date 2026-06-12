# Recipe Versions Docs Rewrite — Design Spec

**Date:** 2026-06-11

## Overview

The existing `docs/site/guides/recipe-versions.md` contains a factual error: it states that "every time you make a change to a recipe, Brewski saves the previous state as a version." This is wrong. Versions are explicit, manual checkpoints. The live recipe is always auto-saved as a working copy, but that working copy has no version history until the user explicitly saves a version.

Additionally, the batch creation flow changed significantly with the `BrewVersionModal` (introduced in the batch-version-picker feature): clicking "Brew this Recipe" now shows a modal with two paths ("Brew with current changes" or "Brew a saved version"). Neither `first-batch.md` nor `recipe-versions.md` documents this accurately.

## Scope

Two files change:

1. **`docs/site/guides/recipe-versions.md`** — full rewrite
2. **`docs/site/getting-started/first-batch.md`** — two section updates ("Starting a batch" and "The link between batches and recipes")

No code changes. No other doc files touched.

## `recipe-versions.md` — Full Rewrite

### Sections

**Intro**
Establishes the two-state model: auto-saved live copy vs. explicit versions.

> Recipes evolve. Brewski saves your edits automatically, so your live recipe is always current.
>
> Versions are something different. They're checkpoints you create deliberately, when the recipe is in a state you want to be able to get back to. Once saved, a version is a fixed snapshot you can brew from or restore at any time.

**Saving a version**
Open the clock icon panel. If the live recipe has changed since the last saved version, the panel shows an "un-versioned changes" warning. Click **Save as version** with an optional name.

**Viewing a version**
Click any version in the panel to see that snapshot read-only. Click outside to return to the live recipe.

**Branching from a version**
Select a version, hit **Branch from here**. Replaces the live recipe's ingredients and settings with that version's data. Version history stays intact. Brewski confirms before proceeding.

**How batches connect to versions**
Clicking **Brew this Recipe** shows a modal. Two paths:
- **Brew with current changes**: auto-saves the live recipe as a new version (optional name), then creates the batch from it.
- **Brew a saved version**: pick any existing saved version from a dropdown.

Either path permanently links the batch to the exact snapshot brewed from.

**What versions track**
Captures: ingredients and amounts, mash profile, water sources, water adjustments, recipe settings (batch size, boil time, efficiency, etc.).

Does not capture: equipment profile or water profile values. Those are shared and live. What is captured is which profile is assigned — so switching profiles is recorded as a version change.

**Deleting a version**
Select a version, click **Delete**. Any version not linked to a batch can be deleted. If a batch was brewed from it, the version stays.

## `first-batch.md` — Section Updates

### "Starting a batch"

Current text incorrectly describes clicking "Brew this Recipe" as directly creating a batch. Updated to describe the modal:

> Open the recipe you want to brew and click the **Batches** tab, then hit **Brew this Recipe**.
>
> Brewski asks you how to version the recipe before creating the batch. If there are unsaved changes, you'll see **Brew with current changes**: this saves a version right now (you can name it), then creates the batch. If you have saved versions, you'll also see **Brew a saved version**, which lets you pick any previous snapshot instead.
>
> Either way, the batch is permanently linked to an exact recipe snapshot. Change the recipe later and your batch record stays untouched.

### "The link between batches and recipes"

Light update: keep the core message, add a cross-reference tip to `recipe-versions.md`.

> The batch stays linked to the recipe version you brewed from. Update the recipe next time around and your old batch records are untouched. You'll always know exactly what went into each beer you made.
>
> ::: tip Want to go deeper on versioning?
> Check out the [Recipe versions](/guides/recipe-versions) guide for a full walkthrough of saving, viewing, and branching versions.
> :::

## What Does Not Change

- All other docs pages
- Any code or tests
- Screenshots (no UI changed)
