# Brewing Icons Design

**Date:** 2026-05-10
**Status:** Approved

## Goal

Add icons throughout the app to make ingredient interfaces easier to scan — hop sections get a hop icon, fermentable sections get a grain icon, tabs get icons, etc. Start with emoji; design the abstraction so swapping to custom SVGs later requires changing only one file.

## Scope

Icons appear in every place that identifies an ingredient type or a recipe tab:

- Recipe tab bar (Overview, Ingredients, Mash, Fermentation, Notes)
- Ingredient section headings in IngredientsTab (Fermentables, Hops, Yeast)
- IngredientPicker dialog header ("Add Hop", "Add Fermentable", "Add Yeast")

The stats sidebar is excluded — stats are already labeled and icons would add noise there.

## Emoji assignments

| `BrewingIconName` | Emoji | Locations |
|---|---|---|
| `fermentable` | 🌾 | FermentablesTable heading, IngredientPicker header |
| `hop` | 🍃 | HopsTable heading, IngredientPicker header |
| `yeast` | 🧫 | YeastsTable heading, IngredientPicker header |
| `overview` | 📋 | Tab bar |
| `ingredients` | 🛒 | Tab bar |
| `mash` | 🌡️ | Tab bar |
| `fermentation` | 🍺 | Tab bar |
| `notes` | ✏️ | Tab bar |

## Architecture

### `src/lib/icons.ts` — central registry

```ts
export type BrewingIconName =
  | 'fermentable' | 'hop' | 'yeast'
  | 'overview' | 'ingredients' | 'mash' | 'fermentation' | 'notes';

export const ICONS: Record<BrewingIconName, string> = {
  fermentable: '🌾',
  hop: '🍃',
  yeast: '🧫',
  overview: '📋',
  ingredients: '🛒',
  mash: '🌡️',
  fermentation: '🍺',
  notes: '✏️',
};
```

This is the **only file that changes** when swapping emoji for custom SVGs.

### `src/lib/components/BrewingIcon.svelte` — single render point

```svelte
<script lang="ts">
  import { ICONS, type BrewingIconName } from '$lib/icons';
  let { name }: { name: BrewingIconName } = $props();
</script>
<span aria-hidden="true">{ICONS[name]}</span>
```

When switching to SVGs: replace the `{ICONS[name]}` render with a `{#if}` / component-per-type block. Zero changes at call sites.

## Call-site updates

### Tab bar — `src/routes/recipe/[id]/+page.svelte`

Add an `icon: BrewingIconName` field to each entry in the `TABS` array and render `<BrewingIcon name={tab.icon} />` before the tab label text.

```ts
const TABS = [
  { key: 'overview',     label: 'Overview',     icon: 'overview'     },
  { key: 'ingredients',  label: 'Ingredients',  icon: 'ingredients'  },
  { key: 'mash',         label: 'Mash',         icon: 'mash'         },
  { key: 'fermentation', label: 'Fermentation', icon: 'fermentation' },
  { key: 'notes',        label: 'Notes',        icon: 'notes'        },
] as const;
```

Tab button markup adds `<BrewingIcon name={tab.icon} />` inline before `{tab.label}`.

### Ingredient section headings

In `FermentablesTable.svelte`, `HopsTable.svelte`, `YeastsTable.svelte` — add `<BrewingIcon name="fermentable|hop|yeast" />` inline before the existing `<h3>` text. No structural changes; the heading row already uses `flex items-center`.

### IngredientPicker dialog — `src/lib/components/ingredients/IngredientPicker.svelte`

Add a header row at the top of the dialog body using the `type` prop to select the icon name:

```ts
const iconName: BrewingIconName =
  type === 'hop' ? 'hop' : type === 'fermentable' ? 'fermentable' : 'yeast';

const dialogTitle =
  type === 'hop' ? 'Add Hop' : type === 'fermentable' ? 'Add Fermentable' : 'Add Yeast';
```

Rendered as `<BrewingIcon name={iconName} /> {dialogTitle}` inside the `<dialog>` element, before the `<div style="display: flex; height: 100%;">` that wraps the search/detail split.

## Future SVG swap

When custom SVGs are ready:

1. Update `ICONS` in `icons.ts` — or replace the string values with a sentinel and ignore them.
2. Update `BrewingIcon.svelte` to import and render per-type SVG components based on `name`.
3. No other files change.
