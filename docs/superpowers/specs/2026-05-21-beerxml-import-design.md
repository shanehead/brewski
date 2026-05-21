# BeerXML Import Design

## Goal

Add an "Import BeerXML" button to the Recipes page (desktop and mobile) so users can import recipes from `.xml` files produced by other homebrewing software.

## Background

The backend is already fully implemented:

- **Tauri command:** `create_recipes_from_beerxml(xml: String) -> Vec<RecipeSummary>` in `src-tauri/src/commands/import_export.rs` — parses a BeerXML string, creates all recipes (with fermentables, hops, yeasts, miscs), and returns their summaries.
- **Frontend wrapper:** `createRecipesFromBeerxml(xml: string)` in `src/lib/api.ts`.

No backend changes are needed.

## Architecture

This is a purely frontend change touching two components:

- `src/lib/components/RecipeList.svelte` — desktop sidebar
- `src/lib/mobile/RecipesHome.svelte` — mobile recipe list

## Component Changes

### RecipeList.svelte (desktop)

Add below the existing "+ New Recipe" button in the sidebar header:

```svelte
<input
  type="file"
  accept=".xml"
  bind:this={fileInput}
  onchange={handleImport}
  class="hidden"
/>
<button
  onclick={() => fileInput.click()}
  class="w-full py-1.5 rounded text-sm font-medium transition-colors"
  style="border: 1px solid var(--color-border); color: var(--color-text-secondary); background: transparent;"
>
  Import BeerXML
</button>
```

`handleImport` reads the selected file via the FileReader API, calls `createRecipesFromBeerxml(xml)`, then calls `refreshRecipeList()`. No navigation — the list updates in place.

### RecipesHome.svelte (mobile)

Same button and logic added below "+ New Recipe". Uses the same `<input type="file">` file picker pattern, which surfaces a native file picker sheet on iOS and Android.

## Data Flow

1. User clicks "Import BeerXML"
2. OS file picker opens (filtered to `.xml`)
3. User selects a file
4. FileReader reads the file as text
5. `createRecipesFromBeerxml(xml)` is called
6. Backend parses all `<RECIPE>` blocks and creates recipes
7. `refreshRecipeList()` refreshes the sidebar/list
8. User sees imported recipes in the list; no navigation

## Multi-recipe files

BeerXML files can contain multiple `<RECIPE>` elements. All are imported. After import the list refreshes and the user can see everything that was added.

## Error Handling

Errors (malformed XML, no `<RECIPE>` elements found, etc.) are surfaced through the existing `ipc()` wrapper, which already handles displaying errors to the user. No additional error UI is needed.

## What's Out of Scope

- Import preview / recipe selection from multi-recipe files
- Duplicate detection
- Progress indicator
- Export UI (already exists as a separate feature)
