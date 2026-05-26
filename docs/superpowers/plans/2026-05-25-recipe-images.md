# Recipe Images Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Users can upload a photo as the icon/thumbnail for a recipe, displayed as a 32×32 thumbnail in the recipe list and a full-width hero in the recipe detail view, with an SRM color gradient as the no-image placeholder.

**Architecture:** Images are stored on the filesystem in `{appDataDir}/images/{recipe_id}.jpg`, resized to ≤1200px longest edge and recompressed as JPEG on ingest. The DB stores only the filename. The frontend resolves image URLs via `convertFileSrc` from Tauri's asset protocol. Placeholder gradients are computed from the recipe's estimated SRM color using a shared `srmToHex` utility.

**Tech Stack:** Rust (`image = "0.25"`, `tauri::AppHandle`), SvelteKit + Svelte 5, `@tauri-apps/api/path`, `@tauri-apps/api/core`, `@tauri-apps/plugin-dialog`, SQLite migration, OpenAPI spec → codegen.

---

## File Map

| Action | File | Purpose |
|---|---|---|
| Create | `src-tauri/migrations/010_recipe_image.sql` | Add `image_path` column to `recipes` |
| Create | `src-tauri/src/commands/recipe_image.rs` | `upload_recipe_image` + `delete_recipe_image` commands |
| Create | `src/lib/utils/srm.ts` | Shared `srmToHex` utility |
| Create | `src/lib/components/RecipeHero.svelte` | Hero display component (image or SRM gradient + camera/remove buttons) |
| Create | `docs/openapi/paths/commands/upload_recipe_image.yaml` | OpenAPI path for upload command |
| Create | `docs/openapi/paths/commands/delete_recipe_image.yaml` | OpenAPI path for delete command |
| Modify | `src-tauri/Cargo.toml` | Add `image = "0.25"` dependency |
| Modify | `src-tauri/src/commands/mod.rs` | Expose `recipe_image` module |
| Modify | `src-tauri/src/lib.rs` | Register new commands + asset scope |
| Modify | `src-tauri/src/commands/recipes.rs` | Cascade delete image file in `delete_recipe` |
| Modify | `src-tauri/tauri.conf.json` | Enable asset protocol with `$APPDATA/images/**` scope |
| Modify | `docs/openapi/components/schemas/Recipe.yaml` | Add `image_path` field |
| Modify | `docs/openapi/components/schemas/RecipeSummary.yaml` | Add `image_path` field |
| Modify | `docs/openapi/openapi.yaml` | Register new command paths |
| Modify | `src/lib/components/RecipeList.svelte` | Add 32×32 thumbnail next to each recipe row |
| Modify | `src/lib/mobile/RecipesHome.svelte` | Add 32×32 thumbnail next to each recipe row |
| Modify | `src/lib/desktop/RecipeView.svelte` | Insert `RecipeHero` below toolbar; dialog-based upload |
| Modify | `src/lib/mobile/RecipeView.svelte` | Insert `RecipeHero` below header; file-input-based upload |
| Modify | `src/lib/components/StatsSidebar.svelte` | Import `srmToHex` from shared util |
| Modify | `src/lib/desktop/IngredientPicker.svelte` | Import `srmToHex` from shared util |
| Modify | `src/lib/components/ingredients/IngredientPicker.svelte` | Import `srmToHex` from shared util |

---

## Task 1: Extract `srmToHex` to a shared utility

`srmToHex` is currently duplicated in three files. Extract it before any new code depends on it.

**Files:**
- Create: `src/lib/utils/srm.ts`
- Modify: `src/lib/components/StatsSidebar.svelte`
- Modify: `src/lib/desktop/IngredientPicker.svelte`
- Modify: `src/lib/components/ingredients/IngredientPicker.svelte`

- [ ] **Step 1: Create `src/lib/utils/srm.ts`**

```typescript
const SRM_STOPS: [number, string][] = [
  [1, "#FFE699"], [2, "#FFD878"], [3, "#FFCA5A"], [4, "#FFBF42"],
  [6, "#FBB123"], [8, "#F8A600"], [10, "#F39C00"], [13, "#EA8F00"],
  [17, "#D77200"], [20, "#CF6900"], [24, "#BB5100"], [29, "#A13600"],
  [35, "#8D1D00"], [40, "#611200"],
];

export function srmToHex(srm: number): string {
  const clamp = Math.min(Math.max(srm, 1), 40);
  for (let i = SRM_STOPS.length - 1; i >= 0; i--) {
    if (clamp >= SRM_STOPS[i][0]) return SRM_STOPS[i][1];
  }
  return "#FFE699";
}
```

- [ ] **Step 2: Update `StatsSidebar.svelte`**

In the `<script lang="ts" module>` block, delete the entire `SRM_STOPS` constant. In the `<script lang="ts">` block, replace the inline `srmToHex` function with an import:

```typescript
import { srmToHex } from "$lib/utils/srm";
```

Remove the duplicate `srmToHex` function body from the script block.

- [ ] **Step 3: Update `src/lib/desktop/IngredientPicker.svelte`**

Find the `SRM_STOPS` constant and `srmToHex` function (around line 209). Delete both and add to imports:

```typescript
import { srmToHex } from "$lib/utils/srm";
```

- [ ] **Step 4: Update `src/lib/components/ingredients/IngredientPicker.svelte`**

Find the `SRM_STOPS` constant and `srmToHex` function (around line 116). Delete both and add to imports:

```typescript
import { srmToHex } from "$lib/utils/srm";
```

- [ ] **Step 5: Verify the app still compiles**

```bash
just check
```

Expected: No TypeScript errors.

- [ ] **Step 6: Commit**

```bash
git add src/lib/utils/srm.ts src/lib/components/StatsSidebar.svelte src/lib/desktop/IngredientPicker.svelte src/lib/components/ingredients/IngredientPicker.svelte
git commit -m "refactor: extract srmToHex into shared utility"
```

---

## Task 2: DB migration + OpenAPI schema changes

**Files:**
- Create: `src-tauri/migrations/010_recipe_image.sql`
- Modify: `docs/openapi/components/schemas/Recipe.yaml`
- Modify: `docs/openapi/components/schemas/RecipeSummary.yaml`
- Create: `docs/openapi/paths/commands/upload_recipe_image.yaml`
- Create: `docs/openapi/paths/commands/delete_recipe_image.yaml`
- Modify: `docs/openapi/openapi.yaml`

- [ ] **Step 1: Create migration**

Create `src-tauri/migrations/010_recipe_image.sql`:

```sql
ALTER TABLE recipes ADD COLUMN image_path TEXT;
```

- [ ] **Step 2: Add `image_path` to `Recipe.yaml`**

In `docs/openapi/components/schemas/Recipe.yaml`, add the following property after the `mash` property at the bottom of the `properties` block:

```yaml
  image_path:
    type:
      - string
      - "null"
    description: Filename of the recipe's image in {appDataDir}/images/
```

- [ ] **Step 3: Add `image_path` to `RecipeSummary.yaml`**

In `docs/openapi/components/schemas/RecipeSummary.yaml`, add after the `source` property:

```yaml
  image_path:
    type:
      - string
      - "null"
    description: Filename of the recipe's image in {appDataDir}/images/
```

- [ ] **Step 4: Create `upload_recipe_image.yaml`**

Create `docs/openapi/paths/commands/upload_recipe_image.yaml`:

```yaml
post:
  operationId: uploadRecipeImage
  summary: Upload and set a recipe's image
  tags:
    - Recipes
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - recipe_id
            - source_path
          properties:
            recipe_id:
              type: string
            source_path:
              type: string
              description: Absolute path to the source image file
  responses:
    "200":
      description: Updated recipe
      content:
        application/json:
          schema:
            $ref: "../../components/schemas/Recipe.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 5: Create `delete_recipe_image.yaml`**

Create `docs/openapi/paths/commands/delete_recipe_image.yaml`:

```yaml
post:
  operationId: deleteRecipeImage
  summary: Remove a recipe's image
  tags:
    - Recipes
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - recipe_id
          properties:
            recipe_id:
              type: string
  responses:
    "200":
      description: Updated recipe
      content:
        application/json:
          schema:
            $ref: "../../components/schemas/Recipe.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 6: Register new paths in `openapi.yaml`**

In `docs/openapi/openapi.yaml`, add the two new paths inside the `paths:` block alongside the other recipe command paths:

```yaml
  /commands/upload_recipe_image:
    $ref: "./paths/commands/upload_recipe_image.yaml"
  /commands/delete_recipe_image:
    $ref: "./paths/commands/delete_recipe_image.yaml"
```

- [ ] **Step 7: Lint and regenerate**

```bash
just lint-openapi
just gen
```

Expected: No lint errors. `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` are updated with `image_path` on `Recipe` and `RecipeSummary`, and `uploadRecipeImage` / `deleteRecipeImage` wrapper functions appear in `api.gen.ts`.

- [ ] **Step 8: Commit**

```bash
git add src-tauri/migrations/010_recipe_image.sql docs/openapi/ src/lib/api.gen.ts src-tauri/src/models.gen.rs
git commit -m "feat: add image_path to recipe schema and OpenAPI commands"
```

---

## Task 3: Rust image commands

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/commands/recipe_image.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add `image` crate to `Cargo.toml`**

In the `[dependencies]` section of `src-tauri/Cargo.toml`, add:

```toml
image = "0.25"
```

- [ ] **Step 2: Write failing tests for `upload_recipe_image`**

Create `src-tauri/src/commands/recipe_image.rs` with tests first:

```rust
use crate::error::AppError;
use crate::models::Recipe;
use crate::repositories::recipe::RecipeRepository;
use sea_orm::DatabaseConnection;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

pub fn images_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(base.join("images"))
}

pub fn image_path(app: &AppHandle, recipe_id: &str) -> Result<PathBuf, AppError> {
    Ok(images_dir(app)?.join(format!("{recipe_id}.jpg")))
}

pub fn write_image(src: &Path, dest: &Path) -> Result<(), AppError> {
    let img = image::open(src).map_err(|e| AppError::Internal(format!("image decode: {e}")))?;
    let resized = resize_to_fit(img, 1200);
    std::fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| AppError::Internal(format!("create images dir: {e}")))?;
    let file = std::fs::File::create(dest)
        .map_err(|e| AppError::Internal(format!("create image file: {e}")))?;
    let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(
        std::io::BufWriter::new(file),
        85,
    );
    enc.encode_image(&resized)
        .map_err(|e| AppError::Internal(format!("image encode: {e}")))?;
    Ok(())
}

fn resize_to_fit(img: image::DynamicImage, max_px: u32) -> image::DynamicImage {
    let (w, h) = (img.width(), img.height());
    if w <= max_px && h <= max_px {
        return img;
    }
    let scale = max_px as f32 / w.max(h) as f32;
    let new_w = (w as f32 * scale) as u32;
    let new_h = (h as f32 * scale) as u32;
    img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
}

#[tauri::command]
pub async fn upload_recipe_image(
    app: AppHandle,
    state: tauri::State<'_, crate::AppState>,
    recipe_id: String,
    source_path: String,
) -> Result<Recipe, AppError> {
    let dest = image_path(&app, &recipe_id)?;
    write_image(Path::new(&source_path), &dest)?;
    let filename = format!("{recipe_id}.jpg");
    RecipeRepository::new(&state.db)
        .set_image_path(&recipe_id, Some(&filename))
        .await
}

#[tauri::command]
pub async fn delete_recipe_image(
    app: AppHandle,
    state: tauri::State<'_, crate::AppState>,
    recipe_id: String,
) -> Result<Recipe, AppError> {
    let path = image_path(&app, &recipe_id)?;
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| AppError::Internal(format!("remove image: {e}")))?;
    }
    RecipeRepository::new(&state.db)
        .set_image_path(&recipe_id, None)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    fn make_test_image(path: &Path, width: u32, height: u32) {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, _y| {
            Rgb([x as u8, 100, 200])
        });
        img.save(path).unwrap();
    }

    #[test]
    fn test_resize_to_fit_large_image() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("images/out.jpg");

        make_test_image(&src, 3000, 2000);

        write_image(&src, &dest).unwrap();

        let result = image::open(&dest).unwrap();
        assert!(result.width() <= 1200);
        assert!(result.height() <= 1200);
    }

    #[test]
    fn test_resize_to_fit_small_image_unchanged() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("images/out.jpg");

        make_test_image(&src, 800, 600);

        write_image(&src, &dest).unwrap();

        let result = image::open(&dest).unwrap();
        assert_eq!(result.width(), 800);
        assert_eq!(result.height(), 600);
    }

    #[test]
    fn test_write_image_creates_dir() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.png");
        let dest = dir.path().join("nested/dir/out.jpg");

        make_test_image(&src, 100, 100);

        write_image(&src, &dest).unwrap();

        assert!(dest.exists());
    }
}
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
cargo test -p brewski recipe_image 2>&1 | head -30
```

Expected: Compile error — `RecipeRepository::set_image_path` doesn't exist yet.

- [ ] **Step 4: Add `set_image_path` to `RecipeRepository`**

In `src-tauri/src/repositories/recipe.rs`, add after the `delete` method (around line 455):

```rust
pub async fn set_image_path(
    &self,
    id: &str,
    filename: Option<&str>,
) -> Result<Recipe, AppError> {
    use sea_orm::ActiveValue::Set;
    let model = recipes::Entity::find_by_id(id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;
    let mut active: recipes::ActiveModel = model.into();
    active.image_path = Set(filename.map(|s| s.to_owned()));
    active.update(self.db).await?;
    self.get(id).await
}
```

Also add `image_path` to the `recipes::ActiveModel` field — this requires the SeaORM entity to have the column. After running the migration in step below, re-generate entities. **Do not proceed with this step until after `just migrate && just gen-entities` is run in the next task.** For now, just add the repository method.

- [ ] **Step 5: Apply the migration and regenerate entities**

```bash
just migrate
just gen-entities
```

Expected: `src-tauri/src/entities/recipes.rs` now has an `image_path` column.

- [ ] **Step 6: Run tests again**

```bash
cargo test -p brewski recipe_image 2>&1
```

Expected: All 3 tests pass — `test_resize_to_fit_large_image`, `test_resize_to_fit_small_image_unchanged`, `test_write_image_creates_dir`.

- [ ] **Step 7: Expose module and register commands**

In `src-tauri/src/commands/mod.rs`, add:

```rust
pub mod recipe_image;
```

In `src-tauri/src/lib.rs`, add to the `invoke_handler` list alongside the other recipe commands:

```rust
commands::recipe_image::upload_recipe_image,
commands::recipe_image::delete_recipe_image,
```

- [ ] **Step 8: Build to confirm no compile errors**

```bash
cargo build -p brewski 2>&1 | grep -E "^error" | head -20
```

Expected: No errors.

- [ ] **Step 9: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/commands/recipe_image.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs src-tauri/src/repositories/recipe.rs src-tauri/src/entities/recipes.rs
git commit -m "feat(backend): add upload_recipe_image and delete_recipe_image commands"
```

---

## Task 4: Cascade delete image on recipe delete

**Files:**
- Modify: `src-tauri/src/commands/recipes.rs`

- [ ] **Step 1: Write failing test**

In `src-tauri/src/commands/recipe_image.rs`, add to the `#[cfg(test)]` block:

```rust
#[test]
fn test_delete_nonexistent_image_does_not_error() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("images/ghost.jpg");
    // File does not exist — should not panic
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    // No error expected
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p brewski test_delete_nonexistent_image_does_not_error
```

Expected: PASS (trivially — just confirms the pattern compiles).

- [ ] **Step 3: Update `delete_recipe` command in `src-tauri/src/commands/recipes.rs`**

Replace:

```rust
#[tauri::command]
pub async fn delete_recipe(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    RecipeRepository::new(&state.db).delete(&id).await
}
```

With:

```rust
#[tauri::command]
pub async fn delete_recipe(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let image = crate::commands::recipe_image::image_path(&app, &id)?;
    if image.exists() {
        std::fs::remove_file(&image)
            .map_err(|e| AppError::Internal(format!("remove image on delete: {e}")))?;
    }
    RecipeRepository::new(&state.db).delete(&id).await
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test -p brewski 2>&1 | tail -10
```

Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/recipes.rs src-tauri/src/commands/recipe_image.rs
git commit -m "feat(backend): delete image file when recipe is deleted"
```

---

## Task 5: Enable asset protocol in `tauri.conf.json`

The `<img>` tag cannot load files from the filesystem directly — Tauri's asset protocol must be enabled and scoped.

**Files:**
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Add asset protocol scope**

In `src-tauri/tauri.conf.json`, inside the `"app"` → `"security"` object, add:

```json
"assetProtocol": {
  "enable": true,
  "scope": ["$APPDATA/images/**"]
}
```

The full `"security"` block should look like:

```json
"security": {
  "csp": null,
  "assetProtocol": {
    "enable": true,
    "scope": ["$APPDATA/images/**"]
  }
}
```

- [ ] **Step 2: Confirm app still builds**

```bash
cargo build -p brewski 2>&1 | grep "^error" | head -5
```

Expected: No errors.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json
git commit -m "feat: enable asset protocol for recipe images"
```

---

## Task 6: `RecipeHero` frontend component

This component handles both the image/gradient display and the upload/remove button UI. The parent views supply `onUploadClick` and `onRemoveClick` callbacks — this component has no upload logic of its own.

**Files:**
- Create: `src/lib/components/RecipeHero.svelte`

- [ ] **Step 1: Create `src/lib/components/RecipeHero.svelte`**

```svelte
<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { srmToHex } from "$lib/utils/srm";
  import type { RecipeSummary } from "$lib/api";

  let {
    recipe,
    appDataDir,
    height = "120px",
    onUploadClick,
    onRemoveClick,
  }: {
    recipe: Pick<RecipeSummary, "id" | "name" | "image_path"> & { srm?: number | null };
    appDataDir: string;
    height?: string;
    onUploadClick: () => void;
    onRemoveClick: () => void;
  } = $props();

  const srm = $derived(recipe.srm ?? 4);
  const color1 = $derived(srmToHex(srm));
  const color2 = $derived(srmToHex(Math.min(srm + 12, 40)));

  const imageSrc = $derived(
    recipe.image_path
      ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
      : null
  );
</script>

<div
  class="relative w-full flex-shrink-0 overflow-hidden"
  style="height: {height};"
>
  {#if imageSrc}
    <img
      src={imageSrc}
      alt={recipe.name}
      class="absolute inset-0 w-full h-full object-cover"
    />
  {:else}
    <div
      class="absolute inset-0"
      style="background: linear-gradient(135deg, {color1} 0%, {color2} 100%);"
    ></div>
  {/if}

  <!-- Gradient overlay so name text is always readable -->
  <div
    class="absolute inset-0"
    style="background: linear-gradient(to top, rgba(0,0,0,0.55) 0%, rgba(0,0,0,0.05) 60%, transparent 100%);"
  ></div>

  <!-- Buttons -->
  <div class="absolute top-2 right-2 flex gap-1.5 z-10">
    {#if recipe.image_path}
      <button
        onclick={onRemoveClick}
        aria-label="Remove photo"
        class="w-7 h-7 rounded-full flex items-center justify-center text-xs"
        style="background: rgba(0,0,0,0.45); border: 1px solid rgba(255,255,255,0.25); color: rgba(255,255,255,0.85); backdrop-filter: blur(4px);"
      >✕</button>
    {/if}
    <button
      onclick={onUploadClick}
      aria-label="Upload photo"
      class="w-7 h-7 rounded-full flex items-center justify-center"
      style="background: rgba(0,0,0,0.45); border: 1px solid rgba(255,255,255,0.25); backdrop-filter: blur(4px);"
    >
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
        <circle cx="12" cy="13" r="4"/>
      </svg>
    </button>
  </div>
</div>
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
just check
```

Expected: No errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/RecipeHero.svelte
git commit -m "feat(frontend): add RecipeHero component with SRM gradient and upload trigger"
```

---

## Task 7: Wire up desktop `RecipeView.svelte`

**Files:**
- Modify: `src/lib/desktop/RecipeView.svelte`

- [ ] **Step 1: Add imports and `appDataDir` state**

At the top of the `<script lang="ts">` block in `src/lib/desktop/RecipeView.svelte`, add:

```typescript
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { uploadRecipeImage, deleteRecipeImage } from "$lib/api";
import RecipeHero from "$lib/components/RecipeHero.svelte";

let appDataDir = $state("");

onMount(async () => {
  appDataDir = await getAppDataDir();
  // ... existing onMount body continues
});
```

If `onMount` already exists, just add `appDataDir = await getAppDataDir();` as the first line of the existing `onMount` callback, and add the three new imports to the existing import block.

- [ ] **Step 2: Add upload and remove handlers**

Add these two functions to the script block, after the existing handlers:

```typescript
async function handleImageUpload() {
  if (!recipe) return;
  const path = await open({
    filters: [{ name: "Image", extensions: ["jpg", "jpeg", "png", "webp", "heic"] }],
  });
  if (!path || typeof path !== "string") return;
  recipe = await ipc(uploadRecipeImage({ recipe_id: recipe.id, source_path: path })) ?? recipe;
}

async function handleImageRemove() {
  if (!recipe) return;
  recipe = await ipc(deleteRecipeImage({ recipe_id: recipe.id })) ?? recipe;
}
```

- [ ] **Step 3: Insert `RecipeHero` into the template**

In the template, find the header toolbar `<div>` (the one with `class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0"`). Insert `RecipeHero` **immediately after** that closing `</div>`, before the tab content area:

```svelte
{#if recipe}
  <RecipeHero
    {recipe}
    {appDataDir}
    height="120px"
    onUploadClick={handleImageUpload}
    onRemoveClick={handleImageRemove}
  />
{/if}
```

The `recipe` object from `get_recipe` is a full `Recipe` which includes `srm` via `RecipeStats`. However `Recipe` itself doesn't have `srm` — that's on `RecipeStats`. Pass the calculated `srm` from stats:

```svelte
<RecipeHero
  recipe={{ ...recipe, srm: stats?.srm ?? null }}
  {appDataDir}
  height="120px"
  onUploadClick={handleImageUpload}
  onRemoveClick={handleImageRemove}
/>
```

- [ ] **Step 4: Verify TypeScript compiles**

```bash
just check
```

Expected: No errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/desktop/RecipeView.svelte
git commit -m "feat(desktop): add recipe image hero with upload/remove"
```

---

## Task 8: Wire up mobile `RecipeView.svelte`

Mobile uses a hidden file input instead of the dialog plugin.

**Files:**
- Modify: `src/lib/mobile/RecipeView.svelte`

- [ ] **Step 1: Add imports, state, and handlers**

At the top of the `<script lang="ts">` block in `src/lib/mobile/RecipeView.svelte`, add:

```typescript
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
import { uploadRecipeImage, deleteRecipeImage } from "$lib/api";
import RecipeHero from "$lib/components/RecipeHero.svelte";

let appDataDir = $state("");
let fileInput: HTMLInputElement;

onMount(async () => {
  appDataDir = await getAppDataDir();
});
```

Add the `onMount` import to the existing `import { onMount } from "svelte"` if not already present.

- [ ] **Step 2: Add upload and remove handlers**

```typescript
async function handleImageUpload() {
  fileInput?.click();
}

async function handleFileSelected(event: Event) {
  if (!recipe) return;
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  const path = (file as File & { path?: string }).path;
  if (!path) return;
  recipe = await ipc(uploadRecipeImage({ recipe_id: recipe.id, source_path: path })) ?? recipe;
  fileInput.value = "";
}

async function handleImageRemove() {
  if (!recipe) return;
  recipe = await ipc(deleteRecipeImage({ recipe_id: recipe.id })) ?? recipe;
}
```

- [ ] **Step 3: Add hidden file input and `RecipeHero` to template**

In the template, find the header bar `<div>` (the one with `flex items-center gap-3 px-4 py-3 border-b`). Insert **immediately after** its closing `</div>`:

```svelte
<input
  type="file"
  accept="image/*"
  bind:this={fileInput}
  onchange={handleFileSelected}
  class="hidden"
/>

{#if recipe}
  <RecipeHero
    recipe={{ ...recipe, srm: stats?.srm ?? null }}
    {appDataDir}
    height="160px"
    onUploadClick={handleImageUpload}
    onRemoveClick={handleImageRemove}
  />
{/if}
```

`stats` is already loaded in `mobile/RecipeView.svelte` (see existing `getRecipeStats` call and `let stats = $state<RecipeStats | null>(null)`). No extra changes needed for stats.

- [ ] **Step 4: Verify TypeScript compiles**

```bash
just check
```

Expected: No errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipeView.svelte
git commit -m "feat(mobile): add recipe image hero with upload/remove"
```

---

## Task 9: List thumbnails

Add 32×32 thumbnails to recipe list rows on both desktop and mobile.

**Files:**
- Modify: `src/lib/components/RecipeList.svelte`
- Modify: `src/lib/mobile/RecipesHome.svelte`

- [ ] **Step 1: Update `RecipeList.svelte` (desktop sidebar)**

Add imports to the `<script>` block:

```typescript
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
import { srmToHex } from "$lib/utils/srm";

let appDataDir = $state("");

onMount(async () => {
  appDataDir = await getAppDataDir();
  // ... existing onMount body
});
```

Add a helper function:

```typescript
function thumbnailSrc(recipe: RecipeSummary): string | null {
  return recipe.image_path
    ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
    : null;
}
```

In the template, find each recipe list row (the `<a>` inside `{#each filtered as recipe}`). The row currently starts with:

```svelte
<a href="/recipe/{recipe.id}" class="flex flex-col px-3 py-2 pr-7 ...">
  <span class="text-sm font-medium truncate" ...>{recipe.name}</span>
```

Change it to include a thumbnail:

```svelte
<a href="/recipe/{recipe.id}" class="flex items-center gap-2 px-3 py-2 pr-7 ...">
  {#if thumbnailSrc(recipe)}
    <img
      src={thumbnailSrc(recipe)}
      alt=""
      class="w-8 h-8 rounded flex-shrink-0 object-cover"
    />
  {:else}
    <div
      class="w-8 h-8 rounded flex-shrink-0"
      style="background: linear-gradient(135deg, {srmToHex(4)}, {srmToHex(16)});"
    ></div>
  {/if}
  <div class="flex flex-col min-w-0 flex-1">
    <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{recipe.name}</span>
    <span class="text-xs truncate mt-0.5" style="color: var(--color-text-secondary);">
      {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
    </span>
  </div>
</a>
```

Note: `RecipeSummary` doesn't carry `srm`, so the placeholder uses a fixed warm amber gradient (`srmToHex(4)` → `srmToHex(16)`) matching an average beer color.

- [ ] **Step 2: Update `mobile/RecipesHome.svelte`**

Add to the `<script>` block:

```typescript
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
import { srmToHex } from "$lib/utils/srm";
import type { RecipeSummary } from "$lib/api";

let appDataDir = $state("");

onMount(async () => {
  appDataDir = await getAppDataDir();
  // ... existing onMount body continues
});

function thumbnailSrc(recipe: RecipeSummary): string | null {
  return recipe.image_path
    ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
    : null;
}
```

In the template, find the recipe rows (the `<a>` tags inside `{#each $recipeList as recipe}`). Currently:

```svelte
<a href="/recipe/{recipe.id}" class="flex items-center justify-between px-4 py-3 border-b text-sm" ...>
  <span class="truncate">{recipe.name}</span>
  <span style="color: var(--color-text-muted);">›</span>
</a>
```

Change to:

```svelte
<a href="/recipe/{recipe.id}" class="flex items-center gap-3 px-4 py-3 border-b text-sm" style="border-color: var(--color-border); color: var(--color-text-primary);">
  {#if thumbnailSrc(recipe)}
    <img src={thumbnailSrc(recipe)} alt="" class="w-8 h-8 rounded flex-shrink-0 object-cover" />
  {:else}
    <div class="w-8 h-8 rounded flex-shrink-0" style="background: linear-gradient(135deg, {srmToHex(4)}, {srmToHex(16)});"></div>
  {/if}
  <span class="truncate flex-1">{recipe.name}</span>
  <span style="color: var(--color-text-muted);">›</span>
</a>
```

- [ ] **Step 3: Verify TypeScript compiles**

```bash
just check
```

Expected: No errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/RecipeList.svelte src/lib/mobile/RecipesHome.svelte
git commit -m "feat(frontend): add recipe image thumbnails to list views"
```

---

## Task 10: End-to-end smoke test

Manual verification that the full flow works on desktop.

- [ ] **Step 1: Start the desktop dev server**

```bash
just dev
```

- [ ] **Step 2: Upload an image**

1. Open or create a recipe
2. Click the camera icon on the hero area
3. Select a photo from disk (any JPEG, PNG, or HEIC)
4. Confirm the hero updates to show the photo

- [ ] **Step 3: Verify list thumbnail**

Navigate back to the recipe list. Confirm the recipe now shows the uploaded photo as a 32×32 thumbnail.

- [ ] **Step 4: Verify SRM placeholder**

Navigate to a recipe that has no image. Confirm the hero shows a warm color gradient (not a broken image).

- [ ] **Step 5: Remove the image**

Click the ✕ button on the hero. Confirm the hero reverts to the SRM gradient and the list thumbnail reverts to the color block.

- [ ] **Step 6: Delete the recipe**

Delete a recipe that has an image. Check `{appDataDir}/images/` — the file should be gone.

On macOS, `appDataDir` is typically:
```
~/Library/Application Support/com.shanehead.brewski/images/
```

- [ ] **Step 7: Final commit (if any fixups needed)**

```bash
git add -p
git commit -m "fix: address issues found in smoke test"
```
