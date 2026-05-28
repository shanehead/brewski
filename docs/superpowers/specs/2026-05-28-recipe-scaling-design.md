# Recipe Scaling — Design Spec

**Date:** 2026-05-28
**Status:** Approved

## Overview

Add a "Scale Recipe" action to the recipe detail view. The user enters a target batch size; the backend creates a new recipe with all scalable amounts adjusted proportionally and navigates to it. The original recipe is untouched.

## Backend

### New IPC command: `scale_recipe`

```
scale_recipe(recipe_id: String, new_batch_size_l: f64) → Recipe
```

Steps:
1. Load the recipe and all its additions (fermentables, hops, yeast, misc, water, mash steps).
2. Compute ratio: `new_batch_size_l / recipe.batch_size_l`.
3. Duplicate the recipe using the existing `source_id` duplication path with the following overrides:
   - `batch_size_l` → `new_batch_size_l`
   - `boil_size_l` → `boil_size_l * ratio`
   - `name` → `"{original name} (scaled)"`
4. Scale all ingredient amounts by the same ratio:
   - Fermentable additions: `amount_kg * ratio`
   - Hop additions: `amount_kg * ratio`
   - Water additions: `amount_l * ratio`
   - Misc additions: `amount * ratio`
   - Yeast additions: `amount * ratio` (when present)
   - Mash step `infuse_amount_l * ratio`
5. Return the new `Recipe`.

**What does NOT scale:** `efficiency_pct`, hop timing (minutes), mash step temps and times, carbonation target, style, notes, yeast strain.

## Frontend

### New API function (`src/lib/api.ts`)

```ts
export const scaleRecipe = (recipeId: string, newBatchSizeL: number) =>
  invoke<Recipe>("scale_recipe", { recipeId, newBatchSizeL });
```

### Scale modal component (`src/lib/components/ScaleRecipeModal.svelte`)

A small modal accepting a target batch size:

- Props: `recipeId: string`, `currentBatchSizeL: number`, `onClose: () => void`
- Volume input labeled "Target Batch Size", pre-filled with the current batch size in the user's preferred unit (`lToGal(currentBatchSizeL)` for imperial, `currentBatchSizeL` for metric)
- Unit suffix label: "gal" or "L" per `$settings.units`
- On confirm: convert input to liters (`galToL` if imperial), call `scaleRecipe(recipeId, newBatchSizeL)`, navigate to the new recipe via SvelteKit's `goto`
- On cancel: call `onClose`
- Disabled/loading state while the IPC call is in flight
- Error state if the call fails (show inline message, stay open)

### Recipe detail header changes

Add a "Scale" button to the actions area in both:
- `src/lib/desktop/RecipeView.svelte` — desktop recipe header
- `src/lib/mobile/RecipeView.svelte` — mobile recipe header

Both follow the same pattern: a state variable `showScaleModal = $state(false)`, a button that sets it to `true`, and a `<ScaleRecipeModal>` rendered conditionally.

After scaling, `goto(`/recipe/${newRecipe.id}`)` navigates to the new recipe. The modal receives the new recipe's id from the `scaleRecipe` return value.

## Unit Conversion

Volume display and input use the existing utilities from `src/lib/units.ts`:
- `lToGal(l)` — liters → gallons for display
- `galToL(gal)` — gallons → liters before calling IPC

No new unit conversion IPC command is needed. These are pure multiplications already used throughout the app.

## Out of Scope

- Scaling from the recipe list view (follow-on).
- Per-ingredient scaling overrides (e.g., keeping yeast at 1 pack).
- Scaling baseline/read-only recipes.
- Renaming the scaled recipe before creation (user can rename after navigating to it).
