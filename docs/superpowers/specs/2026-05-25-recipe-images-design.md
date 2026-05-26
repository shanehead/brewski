# Recipe Images — Design Spec

**Date:** 2026-05-25
**Status:** Approved

## Overview

Users can upload a photo as the icon/thumbnail for a recipe. The image appears as a small thumbnail in the recipe list sidebar and as a full-width hero in the recipe detail view. Recipes without a photo show an auto-generated SRM color gradient placeholder.

---

## Decisions

| Question | Decision |
|---|---|
| Where images appear | List thumbnail + detail hero |
| No-image placeholder | SRM-derived color gradient |
| Storage | Filesystem + resize on ingest |

---

## Data Model

### Migration: `010_recipe_image.sql`

```sql
ALTER TABLE recipes ADD COLUMN image_path TEXT;
```

Nullable. Stores only the filename (e.g. `abc123.jpg`), not the full path. The full path is always resolved at runtime as `{appDataDir}/images/{filename}`.

### On-disk layout

```
{appDataDir}/
  images/
    <recipe-id>.jpg    ← always JPEG after resize, named by recipe ID
```

One file per recipe, named by recipe ID. Easy orphan cleanup: deleting a recipe deletes its image file.

### OpenAPI schema changes

Add `image_path` to both `Recipe.yaml` and `RecipeSummary.yaml`:

```yaml
image_path:
  type: [string, "null"]
```

Run `just gen` after editing the spec to regenerate `api.gen.ts` and `models.gen.rs`.

---

## Rust Backend

### New Cargo dependency

```toml
image = "0.25"
```

Used for decode, resize, and JPEG encode. No other new dependencies.

### New commands (`src-tauri/src/commands/recipe_image.rs`)

**`upload_recipe_image(recipe_id: String, source_path: String) -> Result<Recipe, AppError>`**

1. Read source image from `source_path` (path returned by file picker / file input)
2. Decode with `image` crate
3. Resize so longest edge ≤ 1200px, preserving aspect ratio
4. Encode as JPEG at 85% quality
5. Create `{appDataDir}/images/` dir if it doesn't exist
6. Save to `{appDataDir}/images/{recipe_id}.jpg`
7. Update `recipes.image_path = '{recipe_id}.jpg'` in DB
8. Return updated `Recipe`

**`delete_recipe_image(recipe_id: String) -> Result<Recipe, AppError>`**

1. Delete `{appDataDir}/images/{recipe_id}.jpg` (ignore if file not found)
2. Set `recipes.image_path = NULL` in DB
3. Return updated `Recipe`

### Cascade on recipe delete

File deletion happens in the `delete_recipe` **command handler** (not the repository), before calling the repository's delete method. The command handler already has access to `AppState` / `AppHandle` for path resolution. Ignore file-not-found errors so a missing image file never blocks recipe deletion.

---

## Frontend

### Image display

**Detail hero** (`RecipeView.svelte`, desktop + mobile):
- Full-width, `height: 120px` desktop / `160px` mobile, `object-fit: cover`
- Use `convertFileSrc(fullPath)` from `@tauri-apps/api/core` to convert the absolute filesystem path to a safe `asset://` URL for the `<img>` tag
- Full path = `{appDataDir}/images/{image_path}` where `appDataDir` is resolved once at app start via `appDataDir()` from `@tauri-apps/api/path` and stored in a shared store

**List thumbnail** (`RecipeList.svelte` desktop, `RecipesHome.svelte` mobile):
- 32×32px rounded square, `object-fit: cover`
- Same `convertFileSrc` resolution

### SRM gradient placeholder

When `image_path` is null, render a CSS gradient derived from the recipe's estimated SRM. The SRM-to-color mapping is already used elsewhere in the app (SRM color display). Use the same utility to produce a two-stop gradient for the hero background and a solid color for the thumbnail.

### Upload trigger

A 📷 camera icon button overlaid top-right on the hero area, always visible.

**Desktop** (`src/lib/desktop/RecipeView.svelte`):
```ts
import { open } from '@tauri-apps/plugin-dialog';
const path = await open({ filters: [{ name: 'Image', extensions: ['jpg','jpeg','png','webp','heic'] }] });
if (path) await uploadRecipeImage(recipe.id, path);
```

**Mobile** (`src/lib/mobile/RecipeView.svelte`):
```html
<input type="file" accept="image/*" bind:this={fileInput} onchange={handleUpload} class="hidden" />
```
`accept="image/*"` on iOS/Android surfaces the native photo picker + camera option automatically.

**Mobile path access:** Tauri 2 injects a `.path` property on `File` objects from file inputs on iOS/Android. Use `(file as any).path` to get the native path and pass it to `uploadRecipeImage`. If `.path` is unavailable on a platform, fall back to reading the file as `ArrayBuffer` and sending bytes to a variant command `upload_recipe_image_bytes(recipe_id: String, bytes: Vec<u8>) -> Result<Recipe, AppError>` — identical logic, just accepts bytes instead of a path. Confirm during implementation which approach Tauri 2 supports on iOS and Android.

### Remove button

A ✕ button overlaid top-right next to the camera button, visible only when `image_path` is set. Calls `deleteRecipeImage(recipe.id)` and updates local state.

### New API wrappers

Add to `docs/openapi/` (new path files) and regenerate:
- `upload_recipe_image(recipe_id, source_path) → Recipe`
- `delete_recipe_image(recipe_id) → Recipe`

---

## File-permission requirements

The `asset://` protocol requires the file path to be in Tauri's allowlist. Add `appDataDir` to the asset scope in `tauri.conf.json`:

```json
"security": {
  "assetProtocol": {
    "enable": true,
    "scope": ["$APPDATA/images/**"]
  }
}
```

---

## Out of scope

- Cropping or repositioning the image after upload
- Multiple images per recipe
- Syncing images to the cloud / between devices
- Images on seeded/baseline recipes
