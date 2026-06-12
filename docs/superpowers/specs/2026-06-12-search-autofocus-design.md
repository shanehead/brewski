# Search Fields and Autofocus

**Date:** 2026-06-12
**Status:** Approved

## Overview

Add search filtering to the Equipment page and mobile Recipes list. Add autofocus-on-mount to all full-page views that have a search field. The IngredientPicker modal already handles autofocus and is not changed.

## Scope

| Location | Change |
|---|---|
| `src/routes/equipment/+page.svelte` | Add search input + filter + autofocus |
| `src/lib/mobile/RecipesHome.svelte` | Add search input + filter + autofocus |
| `src/routes/library/+page.svelte` | Add autofocus only (search already exists) |
| `src/lib/components/RecipeList.svelte` | No change (sidebar, not a full-page view) |
| `src/lib/components/ingredients/IngredientPicker.svelte` | No change (already autofocuses) |

## Design

### Filtering logic

All new search fields use a case-insensitive substring match on `name`. No debounce needed — all lists are in-memory. An empty query shows the full list. Non-empty query with no matches shows an inline empty-state message.

### Autofocus mechanism

Use `onMount` with `setTimeout(() => el?.focus(), 0)` and a `bind:this` ref on the input element. This matches the pattern already used in `IngredientPicker.svelte` and fires reliably on every component mount in SvelteKit's client-side navigation model.

Do not use the HTML `autofocus` attribute — it only fires on initial page load, not on subsequent SvelteKit navigations.

### Equipment page

- Add `let query = $state("")` and `const filtered = $derived(...)` that filters `profiles` by name.
- Add `let searchEl = $state<HTMLInputElement | null>(null)`.
- Add `onMount` focus call (alongside the existing `loadSettings` / `listEquipmentProfiles` calls).
- Place the search input between the "Equipment Profiles" section heading and the profile list. Use the same icon+input markup as the library page (magnifying glass SVG, `pl-8` padding, `max-w-xs` width).
- Empty state when filtered is empty and query is non-empty: `<p>No profiles match "{query}"</p>` in place of the profile list.
- The "Add profile" input at the bottom is unaffected by the search state.

### Mobile RecipesHome

- Add `let search = $state("")` and `const filtered = $derived(...)` that filters `$recipeList` by name.
- Add `let searchEl = $state<HTMLInputElement | null>(null)`.
- Add `onMount` focus call (alongside the existing `refreshRecipeList` / `refreshBaselineRecipeList` calls).
- Place the search input inside the existing header block, below the Import BeerXML button.
- The `{#each}` over user recipes uses `filtered` instead of `$recipeList`.
- The Example Recipes section is not filtered — it renders from `$baselineRecipeList` unconditionally, same as the desktop sidebar.
- Empty state when `filtered` is empty and `search` is non-empty: replace the `{:else}` message with "No matches".

### Ingredient Library autofocus

- Add `let searchEl = $state<HTMLInputElement | null>(null)`.
- Add `bind:this={searchEl}` to the existing search `<input>`.
- Add `setTimeout(() => searchEl?.focus(), 0)` inside the existing `onMount`.
- `tick` is already imported; no new imports needed beyond confirming `onMount` is available.
