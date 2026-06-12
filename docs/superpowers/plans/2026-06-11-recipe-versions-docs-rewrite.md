# Recipe Versions Docs Rewrite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite `recipe-versions.md` to accurately describe manual versioning and the brew-version modal, and update two sections of `first-batch.md` to match.

**Architecture:** Docs-only change. Two files edited, no code or tests touched. The rewrite fixes a factual error (versions are not auto-created on every edit) and documents the `BrewVersionModal` flow that was missing from both files.

**Tech Stack:** Markdown, VitePress (docs site)

**Spec:** `docs/superpowers/specs/2026-06-11-recipe-versions-docs-rewrite-design.md`

---

### Task 1: Rewrite `recipe-versions.md`

**Files:**
- Modify: `docs/site/guides/recipe-versions.md`

- [ ] **Step 1: Replace the full file content**

Replace the entire contents of `docs/site/guides/recipe-versions.md` with:

```markdown
# Recipe versions

Recipes evolve. Brewski saves your edits automatically, so your live recipe is always current.

Versions are something different. They're checkpoints you create deliberately, when the recipe is in a state you want to be able to get back to. Once saved, a version is a fixed snapshot you can brew from or restore at any time.

## Saving a version

Click the clock icon in the recipe header to open the version history panel.

If your recipe has changed since the last saved version, you'll see a warning at the top of the panel: **un-versioned changes**. This just means your working copy is ahead of the version history. Click **Save as version** to capture it. You can give it a name like "Dropped Mosaic, added Nelson" or "Scaled to 20L", or leave it blank. Either way, the save records the full recipe as it is right now.

If there's no warning, your working copy is already in sync with the latest version.

## Viewing a version

Click any version in the panel to see the recipe as it was at that point. Stats, ingredients, mash, water adjustments, all of it. It's read-only. Click outside the panel to return to the live recipe.

## Branching from a version

Branching resets the live recipe back to match a saved version. Select a version in the panel, then click **Branch from here**.

This is useful when a recipe has taken a direction you want to split off. Say you started with a pale ale and evolved it into an IPA across a few versions. Branch from the original pale ale state to develop that direction as its own independent recipe.

Brewski asks for confirmation before branching. Once confirmed, the live recipe's ingredients and settings are replaced with that version's data. The version history stays intact.

## How batches connect to versions

When you click **Brew this Recipe**, Brewski shows a modal before creating the batch. You have two paths:

**Brew with current changes** saves the recipe as a new version right now, then creates the batch from it. You can name that version if you want. Use this when you've been tweaking the recipe and are ready to brew it as-is.

**Brew a saved version** lets you pick from any version you've already saved. Use this when you want to brew an older iteration, or when you want to brew against a clean known state without auto-saving pending changes.

Either way, the batch is permanently linked to the exact snapshot it was brewed from. Keep developing the recipe and your batch history stays accurate. You'll always know what was actually in the kettle.

## What versions track

A version captures your recipe's formulation: ingredients and their amounts, the mash profile, water sources, water adjustments, and settings like batch size, boil time, and efficiency.

Equipment profiles and water profiles aren't part of the snapshot. Those are shared and live. Editing your equipment profile affects everywhere it's used. What versions do record is which profile is assigned. If you switch a recipe from one equipment profile to another, that switch is part of the version.

## Deleting a version

Select a version in the panel and click **Delete**.

Any version not linked to a batch can be deleted. If a batch was brewed from it, the version stays. The historical record matters more than a tidy list.
```

- [ ] **Step 2: Commit**

```bash
git add docs/site/guides/recipe-versions.md
git commit -m "docs: rewrite recipe-versions guide with accurate versioning model"
```

---

### Task 2: Update two sections in `first-batch.md`

**Files:**
- Modify: `docs/site/getting-started/first-batch.md`

- [ ] **Step 1: Replace the "Starting a batch" section**

Find this block (lines 6-10):

```markdown
## Starting a batch

Open the recipe you want to brew and click the **Batches** tab. Hit **Brew this Recipe** and Brewski links the new batch to the current version of that recipe. If you change the recipe later, your batch still shows exactly what you brewed.

![Batches tab showing the Brew this Recipe button](/screenshots/docs-gs-batch-overview.png)
```

Replace with:

```markdown
## Starting a batch

Open the recipe you want to brew and click the **Batches** tab, then hit **Brew this Recipe**.

Brewski asks you how to version the recipe before creating the batch. If there are unsaved changes, you'll see **Brew with current changes**: this saves a version right now (you can name it), then creates the batch. If you have saved versions, you'll also see **Brew a saved version**, which lets you pick any previous snapshot instead.

Either way, the batch is permanently linked to an exact recipe snapshot. Change the recipe later and your batch record stays untouched.

![Batches tab showing the Brew this Recipe button](/screenshots/docs-gs-batch-overview.png)
```

- [ ] **Step 2: Replace the "The link between batches and recipes" section**

Find this block (near end of file):

```markdown
## The link between batches and recipes

The batch always stays linked to the recipe version you brewed from. Update the recipe next time around and your old batch records are untouched. You'll always know exactly what went into each beer you made.

::: tip Want to go deeper on recipe design?
Check out the [Building a recipe](/guides/building-a-recipe) guide for a thorough walkthrough of every recipe option Brewski offers.
:::
```

Replace with:

```markdown
## The link between batches and recipes

The batch stays linked to the recipe version you brewed from. Update the recipe next time around and your old batch records are untouched. You'll always know exactly what went into each beer you made.

::: tip Want to go deeper on versioning?
Check out the [Recipe versions](/guides/recipe-versions) guide for a full walkthrough of saving, viewing, and branching versions.
:::
```

- [ ] **Step 3: Commit**

```bash
git add docs/site/getting-started/first-batch.md
git commit -m "docs: update first-batch to describe brew-version modal and link to versions guide"
```
