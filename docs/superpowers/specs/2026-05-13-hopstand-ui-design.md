# Hopstand UI & `hopstand_temp_c` Rename Design

**Date:** 2026-05-13
**Status:** Approved

## Problem

Two gaps remain after the hopstand IBU calculation work:

1. The hop addition picker lists `"whirlpool"` as a use type but the backend now uses `"Hopstand"` — users cannot select the correct use type.
2. The per-hop `whirlpool_temp_c` field has no UI surface and its name is inconsistent with the `"Hopstand"` use type.
3. The database column and all code references use `whirlpool_temp_c`; the name should be `hopstand_temp_c` for consistency.

## Decisions

| Topic | Decision |
|---|---|
| Use type in picker | Replace `"whirlpool"` with `"hopstand"` in `HOP_USES` |
| Temperature UI | Conditional "Temp (°C)" input in the add controls bar, shown only when use is `"hopstand"`, defaulting to 80 |
| Recipe-level temp | Not exposed in UI — per-hop field is sufficient |
| Field rename | `whirlpool_temp_c` → `hopstand_temp_c` everywhere (DB, entities, OpenAPI, generated types, all code) |

## Scope

### Part 1 — Database & Backend Rename

**Migration (new file `m004_hopstand_temp_rename`):**

```sql
ALTER TABLE recipes RENAME COLUMN whirlpool_temp_c TO hopstand_temp_c;
ALTER TABLE recipe_addition_hops RENAME COLUMN whirlpool_temp_c TO hopstand_temp_c;
```

`ALTER TABLE ... RENAME COLUMN` is supported since SQLite 3.25.0 — well within what Tauri bundles.

**Entity files (manual edit — do NOT run `just gen-entities`):**

- `src-tauri/src/entities/recipes.rs`: rename `pub whirlpool_temp_c: Option<f64>` → `pub hopstand_temp_c: Option<f64>`
- `src-tauri/src/entities/recipe_addition_hops.rs`: same rename

**OpenAPI schemas (rename the field in each):**

- `docs/openapi/components/schemas/Recipe.yaml`
- `docs/openapi/components/schemas/RecipeAdditionHop.yaml`
- `docs/openapi/components/schemas/CreateRecipeInput.yaml`
- `docs/openapi/components/schemas/UpdateRecipeInput.yaml`
- `docs/openapi/components/schemas/CreateHopAdditionInput.yaml`
- `docs/openapi/components/schemas/UpdateHopAdditionInput.yaml`

**Regenerate types:** run `just gen` (runs `just gen-ts` and `just gen-rust`) to update `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs`.

**Code references to update (rename field everywhere):**

- `src-tauri/src/models.rs` — `TryFrom` impl for `recipe_addition_hops::Model`
- `src-tauri/src/repositories/recipe.rs` — `get()`, `create()`, `update()`, `copy_additions()`
- `src-tauri/src/repositories/hop.rs` — `create()`, `update()`, test `input()` helper
- `src-tauri/src/commands/import_export.rs` — `parse_hop()` literal
- `src-tauri/src/brewing/beerxml_fixture.rs` — `RecipeAdditionHop` literal
- `src-tauri/src/brewing/mod.rs` — `calculate_stats()` hop input construction, test helpers
- `src-tauri/src/brewing/ibu.rs` — `HopIbuInput` field and any references

**Migration module:**

- `src-tauri/src/migration/m004_hopstand_temp_rename.rs` — new SeaORM migration module
- `src-tauri/src/migration/sql/004_hopstand_temp_rename.sql` — SQL file
- `src-tauri/src/migration/mod.rs` — register new migration

### Part 2 — UI Changes

**`src/lib/components/ingredients/IngredientPicker.svelte`:**

- `HOP_USES` (line 18): replace `'whirlpool'` with `'hopstand'`
- Add state: `let hopstand_temp_c = $state(80);`
- Reset on hop selection (in the `$effect` at line 66): add `hopstand_temp_c = 80;`
- Update `AddPayload` hop type: add `hopstand_temp_c: number | null`
- In the hop add controls bar (after the Time input): conditionally render a "Temp (°C)" number input (`min=0`, `step=1`) when `use_ === 'hopstand'`
- In `handleAdd`: pass `hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null`

**`src/lib/components/ingredients/HopsTable.svelte`:**

- `handlePickerAdd`: pass `hopstand_temp_c: payload.hopstand_temp_c` into `createRecipeHop`

## Testing

All 103 existing tests must continue to pass after the rename. No new tests are required — the IBU calculation behaviour is unchanged; this is purely a rename + UI addition.

Manual verification: open the hop picker, select a hop, choose "hopstand" — the Temp field should appear. Choose any other use — the Temp field should disappear.
