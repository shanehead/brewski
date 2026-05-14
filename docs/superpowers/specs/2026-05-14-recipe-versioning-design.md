# Recipe Versioning Design

**Date:** 2026-05-14
**Status:** Approved

## Background

Brewski already has a recipe snapshot system: every time a batch is brewed, `create_or_reuse` either reuses the last version (if the recipe is unchanged) or creates a new numbered snapshot. The `recipe_versions` table, all its ingredient/mash/water child tables, and the `list_recipe_versions` command are already in place. Batches already store a `recipe_version_id`.

What is missing is the ability for users to manually save named versions, navigate version history inside the recipe editor, and branch from any past version to create a new line of development.

## Goals

- Users can manually save a named version checkpoint at any time.
- Versions form a **tree** (not a linear list): any version can be the parent of a new version.
- Opening a recipe always lands on the **most recent version** (by `created_at`).
- Users can navigate to any past version within the recipe editor and view its data.
- Users can **branch from** any past version, restoring its data as the live recipe and making the next snapshot a child of that version.

## Data Model

### Migration: `002_recipe_versioning.sql`

Two column additions:

```sql
ALTER TABLE recipe_versions ADD COLUMN parent_version_id TEXT REFERENCES recipe_versions(id);
ALTER TABLE recipes ADD COLUMN branch_parent_id TEXT REFERENCES recipe_versions(id);
```

**`recipe_versions.parent_version_id`** — nullable FK to the version this version was derived from. `NULL` on the first version of a recipe. Fully describes the version tree.

**`recipes.branch_parent_id`** — transient pointer. Set when the user branches from a past version; consumed and cleared when the next snapshot is created. Persists across crashes safely — it just means the next snapshot will correctly parent itself to the branched version.

**`version_number`** — stays as a sequential counter across all versions of a recipe (not per-branch). Increments on every new snapshot regardless of branching. Human-readable as "the 5th snapshot ever taken of this recipe."

**"Most recent version"** is defined as the version with the highest `created_at` across all versions of the recipe, regardless of branch.

## Backend

### 1. Update `create_or_reuse`

Read `recipes.branch_parent_id` before comparing against the most recent version:

- If `branch_parent_id` is set: always create a new snapshot (skip the "unchanged = reuse" check), set `parent_version_id = branch_parent_id`, then clear `branch_parent_id` on the recipe.
- If `branch_parent_id` is null: existing behavior — compare against the most recent version, reuse if unchanged, otherwise create new with `parent_version_id` = the version compared against.

### 2. New command: `save_recipe_version(recipe_id: String, name: String)`

Manually creates a new snapshot unconditionally (bypasses the unchanged check). Always creates a new version with the given name. Respects `branch_parent_id` the same way as `create_or_reuse`. Used for named checkpoints without brewing a batch.

### 3. New command: `get_recipe_version(version_id: String) -> RecipeVersion`

Returns the full data for a past version: all scalars, fermentables, hops, yeasts, miscs, water, water adjustments, and mash (sourced from the `recipe_version_*` tables). Returns a new `RecipeVersion` model that mirrors the shape of `Recipe` but is read from version snapshot tables.

### 4. New command: `branch_from_version(recipe_id: String, version_id: String)`

- Loads the full version data via `get_recipe_version`.
- Updates the live `recipes` scalar fields to match the version.
- Deletes all existing ingredient/mash/water additions on the recipe, then re-inserts them from the version snapshot.
- Sets `recipes.branch_parent_id = version_id`.

This restores the past version as the editable live recipe.

## Frontend

### Version history panel

A collapsible panel added to the recipe editor ([src/routes/recipe/[id]/+page.svelte](src/routes/recipe/[id]/+page.svelte)). Loads via `listRecipeVersions` (already wired in `api.ts`). Displayed as a flat list sorted most-recent-first, with child versions indented under their parent to hint at the tree shape.

Each row shows:
- Version number (e.g. "v3")
- Name if set
- Date (`created_at` formatted)
- Batch count badge if any batches were brewed from this version

The currently-viewed version is highlighted.

### Normal editing flow

Unchanged. Opening a recipe loads the live `recipes` data (most recent version's content) and edits as today.

### Viewing a past version

Clicking a version in the history panel switches the recipe editor to **read-only mode**:
- Loads that version's full data via `get_recipe_version`.
- Disables all editing inputs.
- Shows a banner at the top: *"Viewing v3 · March 2025 — Branch from here"*.

Clicking "Branch from here" in the banner:
- If there are unsaved changes to the live recipe, shows a confirmation dialog: *"This will replace your current recipe with v3's data. Continue?"*
- Calls `branch_from_version`.
- Exits read-only mode; the user is now editing the live recipe (which holds v3's data).

### Manual save

A **"Save Version"** button in the recipe editor header opens a small popover with a name text field and a confirm button. Calls `save_recipe_version(recipe_id, name)`. Always creates a new snapshot.

### Batch linkage (small addition)

`BatchOverviewTab` already displays `recipe_version_id` on the batch model. Add a small *"Recipe v3"* link that navigates to the recipe editor with that version pre-selected in the history panel.

## Error handling and edge cases

| Scenario | Behavior |
|---|---|
| Branch with unsaved live edits | Confirmation dialog before overwriting |
| `save_recipe_version` with no changes | Always creates a new snapshot (intentional — user explicitly requested checkpoint) |
| `branch_parent_id` set but app crashes before next snapshot | Persists safely; next snapshot uses it correctly |
| Batch creation after branching | `createBatch` → `create_or_reuse` already respects `branch_parent_id`; no UI change needed |
| First version of a recipe | `parent_version_id = NULL`; root of the tree |

## What is NOT in scope

- Merging two branches
- Diffing versions side-by-side
- Deleting individual versions
- Per-branch version numbering
