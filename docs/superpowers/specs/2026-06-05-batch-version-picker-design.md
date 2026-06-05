# Batch Version Picker — Design Spec

**Date:** 2026-06-05

## Overview

When creating a new batch from the "New Batch" modal, users can choose which saved recipe version to brew from. The version picker only appears when a recipe has two or more saved versions. Single-version recipes (and recipes with no saved versions) create a batch immediately as today.

## Scope

- "New Batch" modal in `desktop/BatchesHome.svelte` and `mobile/BatchesHome.svelte`
- "Brew this Recipe" button in `BatchesTab.svelte` is **out of scope** — it retains today's auto-snapshot behavior

## User Flow

### Step 1 — Recipe picker (unchanged)

The modal opens showing all recipes. Clicking a recipe triggers a version check:

- **0 or 1 saved versions** → call `createBatch({ recipe_id, name: null })` immediately (no `version_id`), navigate to new batch. No change from today.
- **2+ saved versions** → call `listRecipeVersions(recipeId)`, transition to step 2.

### Step 2 — Version picker (new)

The recipe list is replaced in place by a version list. Layout:

- Header: `← RecipeName` (back link, resets to step 1)
- Version rows in reverse-chronological order (newest first)
- Most recent version is visually pre-highlighted as default
- Each row shows: version number (`v3`), optional name (`"Added Citra dry hop"`), date
- Clicking a version calls `createBatch({ recipe_id, version_id, name: null })`, navigates to new batch

## Backend Changes

### `CreateBatchInput`

Add an optional `version_id` field:

```rust
pub struct CreateBatchInput {
    pub recipe_id: String,
    pub name: Option<String>,
    pub version_id: Option<String>,  // if Some, use this version directly
}
```

### `BatchRepository::create`

- If `version_id` is `Some(id)`: fetch that version record directly (no `create_or_reuse` call), use it as `recipe_version_id` on the new batch.
- If `version_id` is `None`: existing `create_or_reuse` path unchanged.

No database migration required — `recipe_version_id` already exists on the `batches` table.

### OpenAPI / generated types

- Update the `CreateBatchInput` schema in the OpenAPI spec to include `version_id?: string`
- Regenerate `src/lib/api.gen.ts` so the frontend type reflects the new optional field

## Frontend Changes

### State additions (both `BatchesHome` variants)

```ts
let step = $state<'recipe' | 'version'>('recipe');
let pickedRecipe = $state<RecipeSummary | null>(null);
let versions = $state<RecipeVersionSummary[]>([]);
```

### Recipe click handler

```ts
async function handlePickRecipe(recipe: RecipeSummary) {
  const vers = (await ipc(listRecipeVersions(recipe.id))) ?? [];
  if (vers.length >= 2) {
    pickedRecipe = recipe;
    versions = vers; // already sorted newest-first by API
    step = 'version';
  } else {
    // 0 or 1 versions — create immediately
    const batch = await ipc(createBatch({ recipe_id: recipe.id, name: null }));
    if (!batch) return;
    showPicker = false;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }
}
```

### Version click handler

```ts
async function handlePickVersion(version: RecipeVersionSummary) {
  showPicker = false;
  const batch = await ipc(createBatch({
    recipe_id: pickedRecipe!.id,
    version_id: version.id,
    name: null,
  }));
  if (!batch) return;
  await ipc(refreshBatchList());
  goto(`/batches/${batch.id}`);
}
```

### Back link

```ts
function handleBack() {
  step = 'recipe';
  pickedRecipe = null;
  versions = [];
}
```

### Modal rendering

The modal conditionally renders step 1 or step 2 based on `step`. Both desktop and mobile get the same logic with their existing style conventions (desktop: `w-80`, mobile: full-width with `mx-4`).

## What Does Not Change

- `BatchesTab.svelte` ("Brew this Recipe") — unchanged
- Batch data model — no new fields
- Version storage — no new tables or migrations
- `create_or_reuse` logic — untouched; still used when `version_id` is absent
