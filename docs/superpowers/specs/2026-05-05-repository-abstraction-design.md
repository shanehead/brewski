# Repository Abstraction Refactor

**Date:** 2026-05-05
**Status:** Approved

## Problem

`RecipeRepository` bypasses the repository abstraction in two ways:

1. It queries SeaORM entities directly for additions, mash, equipment profile, and style instead of delegating to their owning repositories.
2. `copy_additions` bypasses the typed addition repositories for both reads and writes, going directly to entities instead.

Additionally, `AdditionRepository` groups five unrelated addition types (`fermentable`, `hop`, `yeast`, `misc`, `water`) into a single struct — violating single responsibility and making each type's interface harder to discover and reason about. `from_dec`/`from_dec_opt` helpers are duplicated across `recipe.rs` and `mash.rs`.

## Goals

- `RecipeRepository` touches only the `recipes` entity directly; all other persistence goes through the owning repository.
- Each addition type has its own focused repository with standard CRUD.
- No entity imports leak across repository boundaries.
- Shared helpers are defined once.

## Non-Goals

- Changing the public Tauri command API or model types.
- Adding new features or query capabilities beyond what's needed for the refactor.

## Design

### 1. Split `AdditionRepository` into five typed repositories

Delete `repositories/addition.rs`. Create five new files:

| File | Struct | Owns |
|---|---|---|
| `repositories/fermentable.rs` | `FermentableRepository` | `recipe_addition_fermentables` |
| `repositories/hop.rs` | `HopRepository` | `recipe_addition_hops` |
| `repositories/yeast.rs` | `YeastRepository` | `recipe_addition_yeasts` |
| `repositories/misc.rs` | `MiscRepository` | `recipe_addition_miscs` |
| `repositories/water.rs` | `WaterRepository` | `recipe_addition_waters` |

Each repository is constructed with `new(db: &DatabaseConnection)` and exposes exactly four methods:

```
list(recipe_id: &str)          -> Result<Vec<RecipeAdditionX>, AppError>
create(recipe_id: &str, input) -> Result<RecipeAdditionX, AppError>
update(id: &str, input)        -> Result<RecipeAdditionX, AppError>
delete(id: &str)               -> Result<(), AppError>
```

`list` returns rows ordered by `addition_order` ascending. `create` assigns `addition_order` by counting existing rows for the recipe (same logic as current `AdditionRepository`).

### 2. Consolidate shared helpers in `repositories/mod.rs`

Move `from_dec` and `from_dec_opt` from `recipe.rs` and `mash.rs` into `mod.rs` as `pub(crate)` functions, eliminating the duplication. Update all call sites.

### 3. Refactor `RecipeRepository::get`

Remove all entity imports except `recipes`. Replace inline entity queries with repository calls:

| Currently queries entity directly | Replace with |
|---|---|
| `recipe_addition_fermentables::Entity` | `FermentableRepository::list(id)` |
| `recipe_addition_hops::Entity` | `HopRepository::list(id)` |
| `recipe_addition_yeasts::Entity` | `YeastRepository::list(id)` |
| `recipe_addition_miscs::Entity` | `MiscRepository::list(id)` |
| `recipe_addition_waters::Entity` | `WaterRepository::list(id)` |
| `mashes::Entity` + `mash_steps::Entity` | `MashRepository::get_for_recipe(id)` — returns `Err(NotFound)` when no mash; handle as `None` in `get` |
| `equipment_profiles::Entity` | `EquipmentRepository::get(id)` (see §4) |
| `styles::Entity` | `LibraryRepository::get_style(id)` (see §5) |

Each typed repo is constructed inline from `self.db` — they hold no state beyond the connection reference, so this is cheap.

### 4. Make `EquipmentRepository::find_by_id` public

Rename `find_by_id` → `get` and make it `pub`. No other changes to `EquipmentRepository`.

### 5. Add `LibraryRepository::get_style`

Add a single new method:

```rust
pub async fn get_style(&self, id: &str) -> Result<Style, AppError>
```

Queries `styles::Entity::find_by_id(id)`, returns `AppError::NotFound` if missing.

### 6. Refactor `RecipeRepository::copy_additions`

Replace the current direct entity reads and writes with typed repository calls:

1. Construct each typed repo from `self.db`.
2. Call `list(src_id)` on each to read the source additions into memory.
3. Map each item to its `Create*Input` equivalent.
4. Call `create(dst_id, input)` on each typed repo for the destination recipe.

Order is preserved: `list` returns rows ordered by `addition_order`, and `create` assigns order by count — so the first inserted gets 0, second gets 1, etc.

Add this comment to the method:

```rust
// Additions are copied rather than referenced so that edits to the
// source recipe don't affect the duplicate.
```

### 7. Update `commands/additions.rs`

Update imports from `addition::AdditionRepository` to the five typed repositories. Command handler logic is unchanged.

## Files Changed

| File | Action |
|---|---|
| `repositories/mod.rs` | Add 5 new `pub mod` declarations, remove `pub mod addition`, add `from_dec`/`from_dec_opt` |
| `repositories/addition.rs` | Deleted |
| `repositories/fermentable.rs` | New |
| `repositories/hop.rs` | New |
| `repositories/yeast.rs` | New |
| `repositories/misc.rs` | New |
| `repositories/water.rs` | New |
| `repositories/recipe.rs` | Remove entity imports (except `recipes`), delegate to typed repos |
| `repositories/equipment.rs` | `find_by_id` → `pub get` |
| `repositories/library.rs` | Add `get_style` |
| `repositories/mash.rs` | Remove local `from_dec`/`from_dec_opt`, use `super::` versions |
| `commands/additions.rs` | Update imports to typed repositories |

## Testing

Existing tests in `recipe.rs` cover the full `get`/`create`/`update`/`delete`/`duplicate` paths and should continue to pass unchanged — they test observable behavior, not which repositories are called internally. Each new typed repository should have its own unit tests covering `list`, `create`, `update`, and `delete`.
