# Brewski Code Review — 2026-05-28

Reviewed: architecture, the `feat/recipe-scaling` branch, file-handling commands, and the conversion/stats layer.

**Quality gates (all green):** `cargo test`, `cargo clippy`, `svelte-check`, frontend `vitest`. Repo hygiene is clean (`dev.db`, `.env`, `notes.md` properly gitignored).

---

## High impact

### 1. Gravity conversion over IPC creates fragile async effects

`convert_gravity` is pure math (SG ↔ Plato ↔ Brix) but is called as an **async** Tauri command in 6+ components. Because results arrive asynchronously, every call site needs a staleness guard to avoid rendering a value computed from stale inputs. See `src/lib/components/StatsSidebar.svelte:20-43`:

```js
ipc(convertGravity(capturedVal, "sg")).then(r => {
  if (r && gravityUnit === unit && stats?.og === ogVal && stats?.fg === fgVal && stats?.pre_boil_gravity === preBoilVal) {
    set(formatGravity(r, unit));
  }
});
```

Same pattern duplicated in:
- `src/lib/mobile/RecipeView.svelte:48`
- `src/lib/mobile/BaselineRecipeView.svelte:43`
- `src/lib/components/batch/BatchGravityTab.svelte:25`
- `src/lib/components/batch/BatchOverviewTab.svelte:47`

Recent commits (`a2901f1` staleness guard, `1983c86` cancellation flag, `5c5c492`) were all patching races this pattern creates.

**Suggestion:** move the SG→Plato/Brix formula into a synchronous TS helper in `src/lib/units.ts` (alongside the existing pure `lToGal`/`cToF` conversions) and make `displayOg` etc. plain `$derived`. The async effects, staleness guards, and a whole class of race conditions disappear. Keep `convert_gravity` only for the standalone tool page if you want a single Rust source of truth, or drop it. Highest-leverage cleanup in the repo.

### 2. `scale()` duplicates `copy_additions()` (DRY)

`src-tauri/src/repositories/recipe.rs:351-554` reimplements every addition-copy loop from `copy_additions()` (`recipe.rs:231-349`), with the only difference being `* ratio`. ~120 lines of near-identical code that must be kept in sync — adding a new addition type requires updating both paths.

**Suggestion:** unify into one `copy_additions(src, dst, ratio)` and have the duplicate path pass `ratio = 1.0`. One code path, impossible to drift.

---

## Medium impact

### 3. BeerXML export is lossy relative to import

`build_recipe_beerxml` (`src-tauri/src/commands/import_export.rs:17`) writes only FERMENTABLES, HOPS, and YEASTS — but `parse_beerxml` (`import_export.rs:194`) *reads* MISCS too (and neither side handles MASH). Export → re-import silently drops misc additions. Close the asymmetry or document it.

### 4. `ScaleRecipeModal` edge cases (`src/lib/components/ScaleRecipeModal.svelte`)

- Lines 25-27 seed a `$state` from a `$derived` via `$effect`. If `$settings` re-emits while the modal is open, it clobbers whatever the user typed. Since `currentBatchSizeL` is a prop available immediately, initialize directly.
- Lines 49-51: `onkeydown={onClose}` on a `role="none"` backdrop is a no-op (the div isn't focusable), so there's no real Escape-to-close or focus trap. If the linter is the only reason it's there, a window-level `Escape` handler would actually work.

---

## Low impact / polish

- **Clippy:** one real warning in hand-written code — `assert_eq!(updated.forced_carbonation, true)` at `recipe.rs:817` should be `assert!(...)`. The other 17 are all in generated `models.gen.rs` and can be ignored or `#[allow]`'d at the module level.
- **Error-type inconsistency:** `import_export.rs` commands return `Result<_, String>` with manual `.map_err(|e| e.to_string())` everywhere, while the rest of the codebase uses the `AppError` enum. Switching to `AppError` removes the noise.
- **Minor inefficiency:** `create_recipes_from_beerxml` (`import_export.rs:519`) calls `recipe_repo.list()` (all user recipes) just to filter down to the few imported IDs.

---

## What's working well

- Clean `invoke()` boundary isolated in `src/lib/api.ts`; the `ipc()` error wrapper is a nice touch.
- Pure/testable functions well separated from Tauri glue (`execute_database_move`, `find_sync_folders_with_home`).
- XML escaping explicitly tested.
- Image upload resizes/re-encodes (caps memory).
- 95%-line coverage gate in the `Justfile:102` — strong discipline.
