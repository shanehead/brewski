# BeerXML Export Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an "Export BeerXML" button to the recipe header on desktop (native Save As dialog) and mobile (blob download).

**Architecture:** The Rust backend already generates BeerXML via `get_recipe_beerxml`. This plan extracts that logic into a private `build_recipe_beerxml` helper, adds a new `write_recipe_beerxml` Tauri command that writes to a user-chosen path, wires it up through `tauri-plugin-dialog` on desktop, and uses a blob-download fallback on mobile.

**Tech Stack:** Rust/Tauri 2, `tauri-plugin-dialog`, SvelteKit + Svelte 5, TypeScript

---

## File Map

| File | Change |
|------|--------|
| `src-tauri/Cargo.toml` | Add `tauri-plugin-dialog = "2"` |
| `package.json` | Add `@tauri-apps/plugin-dialog` |
| `src-tauri/src/commands/import_export.rs` | Extract `build_recipe_beerxml` helper; add `write_recipe_beerxml` command |
| `src-tauri/src/lib.rs` | Register `tauri_plugin_dialog::init()` and `write_recipe_beerxml` handler |
| `src-tauri/capabilities/default.json` | Add `"dialog:allow-save"` permission |
| `src/lib/api.ts` | Add `writeRecipeBeerxml(recipeId, path)` wrapper |
| `src/lib/desktop/RecipeView.svelte` | Add export button + `handleExport` function |
| `src/lib/mobile/RecipeView.svelte` | Add export icon button + `handleExport` function |

---

### Task 1: Add `tauri-plugin-dialog` to Cargo and npm

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `package.json`

- [ ] **Step 1: Add Rust crate to Cargo.toml**

  In `src-tauri/Cargo.toml`, add after the `tauri-plugin-process` line:

  ```toml
  tauri-plugin-dialog = "2"
  ```

  The `[dependencies]` block should now include:
  ```toml
  tauri-plugin-opener = "2"
  tauri-plugin-process = "2"
  tauri-plugin-dialog = "2"
  ```

- [ ] **Step 2: Install the JS package**

  ```bash
  bun add @tauri-apps/plugin-dialog
  ```

  Expected: `bun.lock` updated, `@tauri-apps/plugin-dialog` appears in `package.json` dependencies.

- [ ] **Step 3: Verify Cargo resolves**

  ```bash
  cd src-tauri && cargo fetch
  ```

  Expected: exits 0, `tauri-plugin-dialog` downloaded.

- [ ] **Step 4: Commit**

  ```bash
  git add src-tauri/Cargo.toml src-tauri/Cargo.lock package.json bun.lock
  git commit -m "chore: add tauri-plugin-dialog dependency"
  ```

---

### Task 2: Refactor Rust — extract helper, add write command, add test

**Files:**
- Modify: `src-tauri/src/commands/import_export.rs`

- [ ] **Step 1: Write the failing test**

  Add to the `#[cfg(test)]` block at the bottom of `src-tauri/src/commands/import_export.rs`:

  ```rust
  #[test]
  fn test_build_recipe_beerxml_contains_recipe_fields() {
      use crate::models::{Recipe, RecipeSource};

      let recipe = Recipe {
          id: "r1".to_string(),
          name: "Pale Ale".to_string(),
          type_: "all_grain".to_string(),
          batch_size_l: 23.0,
          boil_size_l: 27.0,
          boil_time_min: 60.0,
          brewer: Some("Test Brewer".to_string()),
          efficiency_pct: Some(75.0),
          source: RecipeSource::User,
          fermentation_stages: 1,
          forced_carbonation: false,
          created_at: 0,
          updated_at: 0,
          fermentables: vec![],
          hops: vec![],
          yeasts: vec![],
          miscs: vec![],
          waters: vec![],
          water_adjustments: vec![],
          // All Option fields:
          age_days: None,
          age_temp_c: None,
          asst_brewer: None,
          carbonation_temp_c: None,
          carbonation_vols: None,
          date: None,
          equipment_profile: None,
          equipment_profile_id: None,
          fg: None,
          hopstand_temp_c: None,
          keg_priming_factor: None,
          mash: None,
          mash_water_id: None,
          notes: None,
          og: None,
          primary_age_days: None,
          primary_temp_c: None,
          priming_sugar_equiv: None,
          priming_sugar_name: None,
          secondary_age_days: None,
          secondary_temp_c: None,
          sparge_water_id: None,
          style: None,
          style_id: None,
          taste_notes: None,
          taste_rating: None,
          tertiary_age_days: None,
          tertiary_temp_c: None,
      };

      let xml = build_recipe_beerxml(&recipe);
      assert!(xml.starts_with("<?xml version=\"1.0\""));
      assert!(xml.contains("<NAME>Pale Ale</NAME>"));
      assert!(xml.contains("<BATCH_SIZE>23.0</BATCH_SIZE>"));
      assert!(xml.contains("<BOIL_TIME>60</BOIL_TIME>"));
      assert!(xml.contains("<BREWER>Test Brewer</BREWER>"));
      assert!(xml.contains("<EFFICIENCY>75.0</EFFICIENCY>"));
  }
  ```

- [ ] **Step 2: Run the test to verify it fails**

  ```bash
  cd src-tauri && cargo test test_build_recipe_beerxml_contains_recipe_fields -- --nocapture
  ```

  Expected: compile error — `build_recipe_beerxml` not found.

- [ ] **Step 3: Extract `build_recipe_beerxml` helper and update `get_recipe_beerxml`**

  In `src-tauri/src/commands/import_export.rs`, add the `Recipe` import and replace the current `get_recipe_beerxml` function body (lines 16–87) with:

  ```rust
  use crate::models::{
      CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
      CreateRecipeInput, CreateYeastAdditionInput, Recipe, RecipeSummary,
  };
  ```

  Then add the new private helper function before `get_recipe_beerxml`:

  ```rust
  fn build_recipe_beerxml(recipe: &Recipe) -> String {
      let style_block = recipe
          .style
          .as_ref()
          .map(|s| {
              format!(
                  "    <STYLE>\n      <NAME>{}</NAME>\n      <CATEGORY>{}</CATEGORY>\n      <STYLE_GUIDE>{}</STYLE_GUIDE>\n    </STYLE>",
                  s.name, s.category, s.style_guide
              )
          })
          .unwrap_or_default();

      let fermentables: String = recipe
          .fermentables
          .iter()
          .map(|f| {
              format!(
                  "      <FERMENTABLE>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.4}</AMOUNT>\n        <TYPE>{}</TYPE>\n        <YIELD>{:.1}</YIELD>\n        <COLOR>{:.1}</COLOR>\n      </FERMENTABLE>",
                  f.name, f.amount_kg, f.type_, f.yield_pct, f.color_lovibond
              )
          })
          .collect::<Vec<_>>()
          .join("\n");

      let hops: String = recipe
          .hops
          .iter()
          .map(|h| {
              format!(
                  "      <HOP>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.5}</AMOUNT>\n        <ALPHA>{:.1}</ALPHA>\n        <USE>{}</USE>\n        <TIME>{:.0}</TIME>\n        <FORM>{}</FORM>\n      </HOP>",
                  h.name, h.amount_kg, h.alpha_pct, h.use_, h.time_min, h.form
              )
          })
          .collect::<Vec<_>>()
          .join("\n");

      let yeasts: String = recipe
          .yeasts
          .iter()
          .map(|y| {
              format!(
                  "      <YEAST>\n        <NAME>{}</NAME>\n        <TYPE>{}</TYPE>\n        <FORM>{}</FORM>\n        <AMOUNT>{:.4}</AMOUNT>\n      </YEAST>",
                  y.name, y.type_, y.form, y.amount.unwrap_or(0.0)
              )
          })
          .collect::<Vec<_>>()
          .join("\n");

      format!(
          "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<RECIPES>\n  <RECIPE>\n    <NAME>{name}</NAME>\n    <VERSION>1</VERSION>\n    <TYPE>{type_}</TYPE>\n    <BREWER>{brewer}</BREWER>\n    <BATCH_SIZE>{batch_size:.1}</BATCH_SIZE>\n    <BOIL_SIZE>{boil_size:.1}</BOIL_SIZE>\n    <BOIL_TIME>{boil_time:.0}</BOIL_TIME>\n    <EFFICIENCY>{efficiency:.1}</EFFICIENCY>\n{style}\n    <FERMENTABLES>\n{fermentables}\n    </FERMENTABLES>\n    <HOPS>\n{hops}\n    </HOPS>\n    <YEASTS>\n{yeasts}\n    </YEASTS>\n  </RECIPE>\n</RECIPES>",
          name = recipe.name,
          type_ = recipe.type_,
          brewer = recipe.brewer.as_deref().unwrap_or(""),
          batch_size = recipe.batch_size_l,
          boil_size = recipe.boil_size_l,
          boil_time = recipe.boil_time_min,
          efficiency = recipe.efficiency_pct.unwrap_or(72.0),
          style = style_block,
          fermentables = fermentables,
          hops = hops,
          yeasts = yeasts,
      )
  }
  ```

  Then update `get_recipe_beerxml` to delegate to the helper:

  ```rust
  #[tauri::command]
  pub async fn get_recipe_beerxml(
      state: State<'_, AppState>,
      recipe_id: String,
  ) -> Result<String, String> {
      let recipe = RecipeRepository::new(&state.db)
          .get(&recipe_id)
          .await
          .map_err(|e| e.to_string())?;
      Ok(build_recipe_beerxml(&recipe))
  }
  ```

  Then add the new command after `get_recipe_beerxml`:

  ```rust
  #[tauri::command]
  pub async fn write_recipe_beerxml(
      state: State<'_, AppState>,
      recipe_id: String,
      path: String,
  ) -> Result<(), String> {
      let recipe = RecipeRepository::new(&state.db)
          .get(&recipe_id)
          .await
          .map_err(|e| e.to_string())?;
      let xml = build_recipe_beerxml(&recipe);
      std::fs::write(&path, xml).map_err(|e| e.to_string())
  }
  ```

- [ ] **Step 4: Run all tests to verify they pass**

  ```bash
  cd src-tauri && cargo test
  ```

  Expected: all tests pass including `test_build_recipe_beerxml_contains_recipe_fields`, `test_parse_single_recipe`, `test_parse_multiple_recipes`, `test_parse_empty_returns_empty`, `test_parse_malformed_xml_returns_error`.

- [ ] **Step 6: Commit**

  ```bash
  git add src-tauri/src/commands/import_export.rs
  git commit -m "refactor(import_export): extract build_recipe_beerxml helper, add write_recipe_beerxml command"
  ```

---

### Task 3: Register plugin and command, add capability permission

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/capabilities/default.json`

- [ ] **Step 1: Register the plugin in `lib.rs`**

  In `src-tauri/src/lib.rs`, add `.plugin(tauri_plugin_dialog::init())` after the existing plugin registrations (currently at lines 27–28):

  ```rust
  .plugin(tauri_plugin_opener::init())
  .plugin(tauri_plugin_process::init())
  .plugin(tauri_plugin_dialog::init())
  ```

- [ ] **Step 2: Register the new command in the invoke handler**

  In `src-tauri/src/lib.rs`, add `write_recipe_beerxml` to the `generate_handler!` list after `get_recipe_beerxml` (currently at line 108):

  ```rust
  commands::import_export::get_recipe_beerxml,
  commands::import_export::create_recipes_from_beerxml,
  commands::import_export::write_recipe_beerxml,
  ```

- [ ] **Step 3: Add the dialog save permission to capabilities**

  In `src-tauri/capabilities/default.json`, add `"dialog:allow-save"` to the permissions array:

  ```json
  {
    "$schema": "../gen/schemas/desktop-schema.json",
    "identifier": "default",
    "description": "Capability for the main window",
    "windows": ["main"],
    "permissions": [
      "core:default",
      "core:window:allow-show",
      "opener:default",
      "dialog:allow-save"
    ]
  }
  ```

- [ ] **Step 4: Verify it compiles**

  ```bash
  cd src-tauri && cargo build
  ```

  Expected: exits 0, no errors.

- [ ] **Step 5: Commit**

  ```bash
  git add src-tauri/src/lib.rs src-tauri/capabilities/default.json
  git commit -m "feat: register tauri-plugin-dialog and write_recipe_beerxml command"
  ```

---

### Task 4: Frontend API wrapper + desktop export button

**Files:**
- Modify: `src/lib/api.ts`
- Modify: `src/lib/desktop/RecipeView.svelte`

- [ ] **Step 1: Add `writeRecipeBeerxml` to `api.ts`**

  In `src/lib/api.ts`, add after the existing `getRecipeBeerxml` line (currently lines 224–225):

  ```typescript
  export const writeRecipeBeerxml = (recipeId: string, path: string) =>
    invoke<void>("write_recipe_beerxml", { recipeId, path });
  ```

  The import/export section should now look like:

  ```typescript
  // --- Import / export ---
  export const getRecipeBeerxml = (recipeId: string) =>
    invoke<string>("get_recipe_beerxml", { recipeId });
  export const createRecipesFromBeerxml = (xml: string) =>
    invoke<RecipeSummary[]>("create_recipes_from_beerxml", { xml });
  export const writeRecipeBeerxml = (recipeId: string, path: string) =>
    invoke<void>("write_recipe_beerxml", { recipeId, path });
  ```

- [ ] **Step 2: Add the import and handler to `RecipeView.svelte` (desktop)**

  At the top of the `<script lang="ts">` block in `src/lib/desktop/RecipeView.svelte`, add the new import alongside the existing api imports:

  ```typescript
  import {
    getRecipe,
    getRecipeStats,
    updateRecipe,
    listRecipeVersions,
    getRecipeVersion,
    saveRecipeVersion,
    branchFromVersion,
    deleteRecipeVersion,
    writeRecipeBeerxml,
  } from "$lib/api";
  ```

  Then add the `save` import from the dialog plugin (after the existing imports):

  ```typescript
  import { save } from "@tauri-apps/plugin-dialog";
  ```

  Then add the `handleExport` function to the script block (near the other handler functions):

  ```typescript
  async function handleExport() {
    if (!recipe) return;
    const path = await save({
      defaultPath: `${recipe.name}.xml`,
      filters: [{ name: "BeerXML", extensions: ["xml"] }],
    });
    if (!path) return;
    await ipc(writeRecipeBeerxml(recipe.id, path));
  }
  ```

- [ ] **Step 3: Add the export button to the desktop header**

  In `src/lib/desktop/RecipeView.svelte`, add the export button inside the `<header>` element, between the "Save Version" popover `</div>` and the "History" button. The exact insertion point is after the closing `</div>` of the Save Version popover (after line 236) and before the History toggle button:

  ```svelte
  <!-- Export BeerXML button -->
  <button
    onclick={handleExport}
    class="flex items-center gap-1 text-xs px-2 py-1 rounded transition-colors"
    style="color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
  >
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </svg>
    Export BeerXML
  </button>
  ```

- [ ] **Step 4: Verify TypeScript compiles**

  ```bash
  bun run build
  ```

  Expected: exits 0, no type errors.

- [ ] **Step 5: Commit**

  ```bash
  git add src/lib/api.ts src/lib/desktop/RecipeView.svelte
  git commit -m "feat(desktop): add Export BeerXML button with native save dialog"
  ```

---

### Task 5: Mobile export icon button

**Files:**
- Modify: `src/lib/mobile/RecipeView.svelte`

- [ ] **Step 1: Add imports to the mobile RecipeView script block**

  In `src/lib/mobile/RecipeView.svelte`, add `getRecipeBeerxml` to the existing api import:

  ```typescript
  import { getRecipe, getRecipeStats, getRecipeBeerxml } from "$lib/api";
  ```

- [ ] **Step 2: Add `handleExport` to the mobile script block**

  Add after the existing `load` function:

  ```typescript
  async function handleExport() {
    if (!recipe) return;
    const xml = await ipc(getRecipeBeerxml(recipe.id));
    if (!xml) return;
    const blob = new Blob([xml], { type: "application/xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${recipe.name}.xml`;
    a.click();
    URL.revokeObjectURL(url);
  }
  ```

- [ ] **Step 3: Add the icon button to the mobile header**

  In `src/lib/mobile/RecipeView.svelte`, the header div (lines 37–46) currently looks like:

  ```svelte
  <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
    <button
      onclick={() => goto("/")}
      class="text-sm"
      style="color: var(--color-accent);"
    >‹ Recipes</button>
    <span class="flex-1 font-semibold text-base truncate"
          style="color: var(--color-text-primary);">{recipe.name}</span>
  </div>
  ```

  Add the export button after the recipe name `<span>`, before the closing `</div>`:

  ```svelte
  <button
    onclick={handleExport}
    aria-label="Export BeerXML"
    class="flex items-center justify-center rounded flex-shrink-0"
    style="width: 28px; height: 28px; color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: var(--radius-md);"
  >
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </svg>
  </button>
  ```

- [ ] **Step 4: Verify TypeScript compiles**

  ```bash
  bun run build
  ```

  Expected: exits 0, no type errors.

- [ ] **Step 5: Commit**

  ```bash
  git add src/lib/mobile/RecipeView.svelte
  git commit -m "feat(mobile): add Export BeerXML icon button with blob download"
  ```
