# BeerXML Export

**Date:** 2026-05-25  
**Status:** Approved

## Summary

Add an "Export BeerXML" button to the recipe view on both desktop and mobile. Desktop gets a native Save As dialog; mobile gets a direct blob download to the device's Downloads folder.

## Context

The Rust backend already has `get_recipe_beerxml(recipe_id)` in `src-tauri/src/commands/import_export.rs` that generates valid BeerXML from a recipe. The frontend already wraps it as `getRecipeBeerxml` in `src/lib/api.ts`. This feature is purely UI wiring + a thin new Rust write command.

## Design Decisions

- **Button placement:** Recipe header toolbar (desktop: text + icon button alongside Save Version and History; mobile: icon-only at the right of the header)
- **Desktop delivery:** Native Save As dialog via `tauri-plugin-dialog`, file written by a new Rust command
- **Mobile delivery:** Blob download (browser `<a download>`) — file dialog not available on iOS/Android
- **Default filename:** `<recipe name>.xml` (e.g. `Cascade Pale Ale.xml`)
- **File filter:** BeerXML / `.xml` extension only
- **Cancel handling:** If the user dismisses the Save As dialog, do nothing — no error, no feedback
- **Error handling:** Rust errors surface via the existing `ipc()` error store, same as all other commands

## Architecture

No new files. Seven touch points:

| File | Change |
|------|--------|
| `src-tauri/Cargo.toml` | Add `tauri-plugin-dialog = "2"` |
| `package.json` | Add `@tauri-apps/plugin-dialog` |
| `src-tauri/src/lib.rs` | Register `tauri_plugin_dialog::init()` + new command |
| `src-tauri/src/commands/import_export.rs` | Extract XML generation into `build_recipe_beerxml()` helper; new `write_recipe_beerxml(recipe_id, path)` command |
| `src-tauri/capabilities/default.json` | Add `"dialog:allow-save"` permission |
| `src/lib/desktop/RecipeView.svelte` | Import `save` from plugin, add button, wire click handler |
| `src/lib/mobile/RecipeView.svelte` | Add icon button, wire blob-download handler |

## Data Flow

### Desktop

1. User clicks "Export BeerXML"
2. Frontend calls `save({ defaultPath: '<recipe.name>.xml', filters: [{ name: 'BeerXML', extensions: ['xml'] }] })`
3. If `save()` returns `null` (user cancelled) → stop, do nothing
4. If path returned → `invoke('write_recipe_beerxml', { recipeId: recipe.id, path })`
5. Rust: calls `build_recipe_beerxml(&recipe)` (shared helper), writes result with `std::fs::write(path, xml)`
6. On error → propagates via `ipc()` to `lastError` store

### Mobile

1. User taps the download icon
2. Frontend calls `getRecipeBeerxml(recipe.id)` (existing API wrapper)
3. Creates `Blob` with `type: 'application/xml'`, builds object URL
4. Creates `<a download="<recipe.name>.xml">`, programmatically clicks it, revokes URL
5. On error → propagates via `ipc()` to `lastError` store

## Rust Changes

Extract the XML string-building logic from `get_recipe_beerxml` into a private `build_recipe_beerxml(recipe: &Recipe) -> String` function. Both `get_recipe_beerxml` and the new `write_recipe_beerxml` call it.

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

## Button Appearance

**Desktop** — ghost button (matches Save Version / History style):
```
background: var(--color-bg-elevated)
color: var(--color-text-secondary)
border: 1px solid var(--color-border)
border-radius: var(--radius-md)
font-size: 11px, padding: 3px 8px
```
Prefix: Lucide `download` icon (14×14, `stroke="currentColor"`, `stroke-width="2"`)  
Label: `Export BeerXML`

**Mobile** — icon-only button (28×28, same ghost style as desktop, `border-radius: var(--radius-md)`):
- Same Lucide download icon (13×13)
- `aria-label="Export BeerXML"`

## Out of Scope

- Exporting from the version history panel (exports current live recipe only)
- Batch export of multiple recipes
- Export from the recipe list sidebar
