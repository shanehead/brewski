# Schema-First Development with Split OpenAPI Spec

**Date:** 2026-05-10
**Status:** Approved

## Overview

Replace the single monolithic `docs/openapi.yaml` with a Redocly-style split directory, then generate TypeScript types and Rust model structs from it via `just gen`. Editing the spec is the authoritative first step for any API change; codegen propagates those changes to both sides of the IPC boundary.

## 1. Split Spec Structure

`docs/openapi.yaml` → `docs/openapi/openapi.yaml` (root) + one file per path and schema.

```
docs/openapi/
  openapi.yaml                       ← root: info, servers, tags, $refs only
  paths/
    commands/
      list_recipes.yaml
      get_recipe.yaml
      create_recipe.yaml
      update_recipe.yaml
      delete_recipe.yaml
      get_recipe_stats.yaml
      create_recipe_fermentable.yaml
      update_recipe_fermentable.yaml
      delete_recipe_fermentable.yaml
      create_recipe_hop.yaml
      update_recipe_hop.yaml
      delete_recipe_hop.yaml
      create_recipe_yeast.yaml
      update_recipe_yeast.yaml
      delete_recipe_yeast.yaml
      create_recipe_misc.yaml
      update_recipe_misc.yaml
      delete_recipe_misc.yaml
      create_recipe_water.yaml
      update_recipe_water.yaml
      delete_recipe_water.yaml
      get_mash.yaml
      update_mash.yaml
      create_mash_step.yaml
      update_mash_step.yaml
      delete_mash_step.yaml
      update_mash_step_order.yaml
      list_equipment_profiles.yaml
      create_equipment_profile.yaml
      update_equipment_profile.yaml
      delete_equipment_profile.yaml
      list_styles.yaml
      list_fermentable_library.yaml
      list_hop_library.yaml
      list_yeast_library.yaml
      list_misc_library.yaml
      list_water_library.yaml
      get_settings.yaml
      update_setting.yaml
      get_recipe_beerxml.yaml
      create_recipes_from_beerxml.yaml
  components/
    schemas/
      RecipeSummary.yaml
      Recipe.yaml
      RecipeStats.yaml
      Style.yaml
      EquipmentProfile.yaml
      Fermentable.yaml
      Hop.yaml
      Yeast.yaml
      Misc.yaml
      Water.yaml
      RecipeAdditionFermentable.yaml
      RecipeAdditionHop.yaml
      RecipeAdditionYeast.yaml
      RecipeAdditionMisc.yaml
      RecipeAdditionWater.yaml
      Mash.yaml
      MashStep.yaml
      CreateRecipeInput.yaml
      UpdateRecipeInput.yaml
      CreateFermentableAdditionInput.yaml
      UpdateFermentableAdditionInput.yaml
      CreateHopAdditionInput.yaml
      UpdateHopAdditionInput.yaml
      CreateYeastAdditionInput.yaml
      UpdateYeastAdditionInput.yaml
      CreateMiscAdditionInput.yaml
      UpdateMiscAdditionInput.yaml
      CreateWaterAdditionInput.yaml
      UpdateWaterAdditionInput.yaml
      UpdateMashInput.yaml
      CreateMashStepInput.yaml
      UpdateMashStepInput.yaml
      CreateEquipmentProfileInput.yaml
      UpdateEquipmentProfileInput.yaml
    responses/
      Error.yaml
```

`redocly.yaml` root path changes from `docs/openapi.yaml` to `docs/openapi/openapi.yaml`. All existing Justfile recipes (`just lint-openapi`, `just preview-docs`, `just check`) continue to work without modification — Redocly follows `$ref`s natively.

The old `docs/openapi.yaml` is deleted.

## 2. TypeScript Generation

**Tool:** `openapi-typescript` (invoked via `bunx`)

**Input:** bundled spec (single resolved YAML produced by `redocly bundle`)
**Output:** `src/lib/api.gen.ts` — committed, do not edit by hand

`openapi-typescript` emits a namespaced shape:

```typescript
// src/lib/api.gen.ts — GENERATED, do not edit
export interface components {
  schemas: {
    Recipe: { id: string; name: string; ... };
    RecipeSummary: { id: string; name: string; ... };
    // ...
  }
}
```

`src/lib/api.ts` is updated to remove all hand-written `export interface` declarations and instead import from `api.gen.ts`. The `invoke()` wrapper functions remain hand-written:

```typescript
import type { components } from './api.gen';

type Recipe = components['schemas']['Recipe'];
type RecipeSummary = components['schemas']['RecipeSummary'];

export async function getRecipe(id: string): Promise<Recipe> {
  return invoke('get_recipe', { id });
}
```

Generated files are committed to the repo so that diffs are reviewable, `grep` works on the generated types, and CI needs no codegen tooling installed.

## 3. Rust Generation

**Tool:** `cargo typify` (the `typify` CLI crate from Oxide Computer)

`typify` takes a JSON Schema `$defs` document and emits Rust structs with `#[derive(Debug, Serialize, Deserialize, Clone)]`. Since `typify` takes JSON Schema (not OpenAPI directly), a two-step pipeline is used:

1. **Bundle** — `redocly bundle` resolves all `$ref`s into a single YAML file
2. **Extract** — `scripts/extract-schemas.mjs` (Bun script, ~15 lines) reads `components.schemas` from the bundled YAML and writes a JSON Schema document: `{ "$defs": { "Recipe": {...}, ... } }`
3. **Generate** — `cargo typify` reads the JSON Schema and writes `src-tauri/src/models.gen.rs`
4. **Format** — `cargo fmt` normalises the generated output

`src-tauri/src/models.gen.rs` contains only struct and enum definitions with serde derives. Input types (`CreateRecipeInput`, etc.) are included in the spec and generated alongside response types.

`src-tauri/src/models.rs` is trimmed to contain only the `TryFrom<entities::X::Model>` implementations. These are hand-written and remain so — they encode the mapping between the DB schema and the public API, which the OpenAPI spec cannot describe. When a field is added to the spec and `just gen` is run, the compiler flags every `TryFrom` impl that needs updating.

`src-tauri/src/lib.rs` includes both `models.gen` and `models` modules.

## 4. `just gen` Command

```makefile
# Regenerate TypeScript and Rust types from the OpenAPI spec
gen: gen-ts gen-rust

gen-ts:
    bunx redocly bundle docs/openapi/openapi.yaml -o /tmp/brewski-bundled.yaml
    bunx openapi-typescript /tmp/brewski-bundled.yaml -o src/lib/api.gen.ts

gen-rust:
    bunx redocly bundle docs/openapi/openapi.yaml -o /tmp/brewski-bundled.yaml
    bun scripts/extract-schemas.mjs /tmp/brewski-bundled.yaml /tmp/brewski-schemas.json
    cargo typify /tmp/brewski-schemas.json
    cargo fmt --manifest-path src-tauri/Cargo.toml
```

`cargo typify` writes output adjacent to the input file by default; the script moves it into place. `typify` is installed as a Cargo binary: `cargo install cargo-typify`.

## 5. Development Workflow

The new workflow for any API change:

1. Edit the relevant schema file(s) in `docs/openapi/components/schemas/` or path file(s) in `docs/openapi/paths/`
2. Run `just lint-openapi` to validate the spec
3. Run `just gen` to regenerate `api.gen.ts` and `models.gen.rs`
4. Fix compiler errors in `models.rs` (`TryFrom` impls) and `api.ts` (`invoke()` wrappers)
5. Commit all changed files together (spec + generated + hand-written fixups)

## 6. What Is Not Generated

| File | Why hand-written |
|------|-----------------|
| `src-tauri/src/models.rs` | `TryFrom` impls encode DB↔API mapping not expressible in the spec |
| `src/lib/api.ts` | `invoke()` wrappers are Tauri-specific; no HTTP client is generated |
| `src-tauri/src/commands/` | Command handler logic is business logic, not a type mapping |
| `src-tauri/src/entities/` | Generated from DB schema by `sea-orm-cli`, separate pipeline |

## 7. New Dependencies

| Dependency | Purpose | Install |
|------------|---------|---------|
| `openapi-typescript` | TypeScript type generation | `bun add -d openapi-typescript` |
| `cargo-typify` | Rust struct generation | `cargo install cargo-typify` |
| `js-yaml` | YAML parsing in extract script | `bun add -d js-yaml` |
