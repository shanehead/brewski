# Hide Example Recipes Setting

## Overview

Add a boolean setting that completely suppresses the Example Recipes section from the recipe list on both desktop and mobile. When hidden, no header or recipes from the seeded (`source = 'seeded'`) set are visible anywhere in the UI.

## Data Model

Add `hide_example_recipes?: boolean` to the `AppSettings` interface in `src/lib/stores/settings.ts`.

The existing `saveSetting` function already coerces `"true"`/`"false"` strings to booleans, so no changes are needed to the settings store logic.

## Settings Page

Add a **Recipes** section to `src/routes/settings/+page.svelte` between the Units section and the DatabaseLocation component. The section contains one row: a checkbox labeled "Hide Example Recipes."

Pattern: matches the label/control layout used by Appearance and Units sections.

Handler calls `saveSetting("hide_example_recipes", checked ? "true" : "false")`.

## Desktop Recipe List

In `src/lib/components/RecipeList.svelte`, add `&& !$settings.hide_example_recipes` to the existing guard on the Example Recipes block:

```svelte
{#if $baselineRecipeList.length > 0 && !$settings.hide_example_recipes}
```

When `hide_example_recipes` is true, nothing renders — no section header, no collapse toggle, no recipe rows.

## Mobile Recipe List

Same change in `src/lib/mobile/RecipesHome.svelte`, same guard on the same `{#if $baselineRecipeList.length > 0}` block.

## Interaction with Existing Collapse Setting

`starters_collapsed` is left untouched. If the user later turns off hide, the section comes back in whatever collapsed/expanded state they last set.

## Files Changed

| File | Change |
|---|---|
| `src/lib/stores/settings.ts` | Add `hide_example_recipes?: boolean` to `AppSettings` |
| `src/routes/settings/+page.svelte` | Add Recipes section with hide toggle |
| `src/lib/components/RecipeList.svelte` | Gate example recipes block on setting |
| `src/lib/mobile/RecipesHome.svelte` | Gate example recipes block on setting |
