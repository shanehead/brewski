# Delete Recipe Version

**Date:** 2026-05-14

## Summary

Users can delete individual recipe versions from the version history panel. Deletion is safe: it blocks on batch references, re-parents child versions, and cascades cleanup of all snapshot data automatically.

## Backend

### Repository method: `RecipeVersionRepository::delete`

Steps (in order):

1. Load the version by `id` — return `AppError::NotFound` if missing
2. Check `batches` for any row where `recipe_version_id = id` — return `AppError::Conflict` if found
3. Re-parent child versions: set `parent_version_id = <deleted_version.parent_version_id>` for all versions whose `parent_version_id = id`
4. Nullify recipe branch parent: set `branch_parent_id = NULL` on any `recipes` row where `branch_parent_id = id`
5. Delete the version row — SQLite `ON DELETE CASCADE` on all `recipe_version_*` tables handles child snapshot cleanup automatically

### Tauri command: `delete_recipe_version(id: String) -> Result<(), AppError>`

Thin wrapper in `src-tauri/src/commands/batches.rs`, registered in `lib.rs`.

### Error cases

| Condition | Error |
|---|---|
| Version not found | `AppError::NotFound` |
| Version referenced by a batch | `AppError::Conflict("version is referenced by a batch")` |

## Frontend

### API wrapper (`src/lib/api.ts`)

```ts
export const deleteRecipeVersion = (id: string) =>
  invoke<void>("delete_recipe_version", { id });
```

### `VersionHistoryPanel.svelte`

- Add `ondelete: (version: RecipeVersionSummary) => void` prop
- Render a "Delete" button in the expanded version row, next to the existing "Branch from here" button

### `recipe/[id]/+page.svelte`

- `handleDeleteVersion(version)`: shows `confirm()` dialog, calls `deleteRecipeVersion(version.id)` via `ipc`, then calls `refreshRecipe()` (which clears `viewingVersion`)
- Existing `ipc` error store surfaces any conflict error to the user

## Data integrity notes

- All `recipe_version_*` snapshot tables have `ON DELETE CASCADE` — no manual cleanup needed
- `parent_version_id` on `recipe_versions` has no cascade, so re-parenting must be done explicitly before deletion
- `recipes.branch_parent_id` has no cascade, so must be nullified explicitly before deletion
- `batches.recipe_version_id` has no cascade and is `NOT NULL`, so the batch-reference check is a hard block
