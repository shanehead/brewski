# Hide Recipe/Batch Sidebar on Detail View

**Date:** 2026-06-12
**Status:** Approved

## Problem

When a recipe or batch is open, the 224px list sidebar remains visible alongside the content. Combined with the 56px icon rail, 280px of left chrome is consumed before any content starts. Once you've selected an item, the list is dead weight.

## Solution

Auto-hide the list sidebar when a recipe or batch detail view is open. The list returns when navigating back to the section home (`/` or `/batches`). Navigation back is via the existing "← Recipes" button already in the recipe toolbar, or by clicking the section icon in the rail.

Mobile is unaffected — it already uses full-screen list → detail navigation.

## Changes

### `src/lib/desktop/RecipeView.svelte`

Remove `<RecipeList selectedId={id} />` from the template. The `RecipeList` import can also be removed if it's only used there.

RecipeView already has a "← Recipes" button (`onclick={() => goto("/")}`) in its toolbar header — this remains as the primary back affordance.

### `src/lib/desktop/BaselineRecipeView.svelte`

Remove `<RecipeList selectedId={id} />` from the template. The `RecipeList` import can also be removed. The existing "← Recipes" button in its toolbar already handles navigation back.

### `src/lib/desktop/BatchView.svelte`

Remove the entire `<aside>` block (the `w-56` sidebar containing the "+ New Batch" button and `<BatchList />`). Remove the `BatchList` import. Remove the `batchList` store import and all `refreshBatchList()` calls — it's called in both `onMount` and `handleUpdate`, and both were only there to keep the sidebar list current.

### `src/lib/desktop/AppShell.svelte`

Update the Recipes and Batches rail buttons so that clicking them while already in that section navigates to the section home (the list), not to the last visited detail route.

**Recipes button** — change from:
```ts
onclick={() => goto($settings.last_route_recipes ?? "/")}
```
to:
```ts
onclick={() => goto(isRecipes ? "/" : ($settings.last_route_recipes ?? "/"))}
```

**Batches button** — change from:
```ts
onclick={() => goto($settings.last_route_batches ?? "/batches")}
```
to:
```ts
onclick={() => goto(isBatches ? "/batches" : ($settings.last_route_batches ?? "/batches"))}
```

Cross-section behavior is preserved: switching to Tools and back still restores the last recipe or batch directly.

## Navigation Flow

| Action | Result |
|---|---|
| Click recipe in list | Full-width recipe editor, list hidden |
| Click "← Recipes" in toolbar | Returns to `/` (recipe list) |
| Click Recipes icon in rail (while on a recipe) | Returns to `/` (recipe list) |
| Click Recipes icon in rail (from another section) | Restores last recipe directly |
| Click batch in list | Full-width batch view, list hidden |
| Click Batches icon in rail (while on a batch) | Returns to `/batches` (batch list) |
| Click Batches icon in rail (from another section) | Restores last batch directly |

## Out of Scope

- No changes to mobile (`src/lib/mobile/`)
- No changes to `RecipesHome.svelte` or `BatchesHome.svelte` (the list views)
- No new UI elements — existing "← Recipes" button and rail icon provide navigation
- No animation or transition on sidebar removal
