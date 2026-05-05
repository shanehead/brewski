# Repository Abstraction Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `RecipeRepository` delegate all persistence to typed owning repositories, and split `AdditionRepository` into five focused repos — one per addition type.

**Architecture:** Five new typed addition repositories (`FermentableRepository`, `HopRepository`, `YeastRepository`, `MiscRepository`, `WaterRepository`) each expose `list(recipe_id)`, `create`, `update`, `delete`. `RecipeRepository::get` constructs each repo inline from `self.db` and delegates all entity access. `copy_additions` reads via `list` and writes via `create` on the same typed repos. `AdditionRepository` is deleted.

**Tech Stack:** Rust, SeaORM, SQLite, Tokio (async tests)

---

## File Map

| File | Action | Notes |
|---|---|---|
| `src-tauri/src/repositories/mod.rs` | Modify | Add `from_dec`/`from_dec_opt`, add 5 new mods, remove `addition` |
| `src-tauri/src/repositories/addition.rs` | Delete | Replaced by 5 typed repos |
| `src-tauri/src/repositories/fermentable.rs` | Create | CRUD for `recipe_addition_fermentables` |
| `src-tauri/src/repositories/hop.rs` | Create | CRUD for `recipe_addition_hops` |
| `src-tauri/src/repositories/yeast.rs` | Create | CRUD for `recipe_addition_yeasts` |
| `src-tauri/src/repositories/misc.rs` | Create | CRUD for `recipe_addition_miscs` |
| `src-tauri/src/repositories/water.rs` | Create | CRUD for `recipe_addition_waters` |
| `src-tauri/src/repositories/recipe.rs` | Modify | Delegate to typed repos; remove entity imports except `recipes` |
| `src-tauri/src/repositories/equipment.rs` | Modify | `find_by_id` → `pub get` |
| `src-tauri/src/repositories/library.rs` | Modify | Add `get_style(id)` |
| `src-tauri/src/repositories/mash.rs` | Modify | Use `super::from_dec`/`super::from_dec_opt` |
| `src-tauri/src/models.rs` | Modify | Add `add_to_secondary`/`times_cultured` to `CreateYeastAdditionInput` |
| `src-tauri/src/commands/additions.rs` | Modify | Import 5 typed repos instead of `AdditionRepository` |
| `src-tauri/src/commands/import_export.rs` | Modify | Import `FermentableRepository`/`HopRepository` instead of `AdditionRepository` |

---

### Task 1: Consolidate `from_dec`/`from_dec_opt` in `repositories/mod.rs`

**Files:**
- Modify: `src-tauri/src/repositories/mod.rs`
- Modify: `src-tauri/src/repositories/mash.rs`
- Modify: `src-tauri/src/repositories/recipe.rs`

Both `mash.rs` and `recipe.rs` define identical private `from_dec`/`from_dec_opt` helpers. Move them to `mod.rs` as `pub(crate)` so all repos share one definition.

- [ ] **Step 1: Add helpers to `repositories/mod.rs`**

Add to the bottom of `src-tauri/src/repositories/mod.rs`:

```rust
pub(crate) fn from_dec(v: rust_decimal::Decimal) -> Result<f64, crate::error::AppError> {
    use rust_decimal::prelude::ToPrimitive;
    v.to_f64()
        .ok_or_else(|| crate::error::AppError::Conversion(format!("cannot convert {} to f64", v)))
}

pub(crate) fn from_dec_opt(v: Option<rust_decimal::Decimal>) -> Result<Option<f64>, crate::error::AppError> {
    match v {
        Some(dec) => Ok(Some(from_dec(dec)?)),
        None => Ok(None),
    }
}
```

- [ ] **Step 2: Remove the local helpers from `repositories/mash.rs` and update call sites**

Delete the two private `fn from_dec` and `fn from_dec_opt` at lines 12–23 of `mash.rs`. Replace all `from_dec(` calls with `super::from_dec(` and all `from_dec_opt(` calls with `super::from_dec_opt(`. There are six call sites in `fetch_mash`.

- [ ] **Step 3: Remove the local helper from `repositories/recipe.rs` and update call sites**

Delete the private `fn from_dec` at lines 18–22 of `recipe.rs`. The local `from_dec_opt` is never called in `recipe.rs` — delete it too if present. Replace the three `from_dec(` calls in `get` (for `batch_size_l`, `boil_size_l`, `boil_time_min`) with `super::from_dec(`.

- [ ] **Step 4: Verify it compiles**

```bash
cd src-tauri && cargo build 2>&1 | grep -E "^error"
```

Expected: no output (no errors).

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/repositories/mod.rs \
        src-tauri/src/repositories/mash.rs \
        src-tauri/src/repositories/recipe.rs
git commit -m "refactor: consolidate from_dec helpers in repositories/mod.rs"
```

---

### Task 2: Create `FermentableRepository`

**Files:**
- Create: `src-tauri/src/repositories/fermentable.rs`
- Modify: `src-tauri/src/repositories/mod.rs`

- [ ] **Step 1: Register the new module in `mod.rs`**

Add `pub mod fermentable;` to `src-tauri/src/repositories/mod.rs` (alongside the existing `pub mod` lines).

- [ ] **Step 2: Create `src-tauri/src/repositories/fermentable.rs`**

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_fermentables;
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, RecipeAdditionFermentable, UpdateFermentableAdditionInput,
};

use super::{new_id, to_dec};

pub struct FermentableRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FermentableRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionFermentable>, AppError> {
        recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionFermentable::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateFermentableAdditionInput,
    ) -> Result<RecipeAdditionFermentable, AppError> {
        let order = recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_fermentables::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            fermentable_id: Set(input.fermentable_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            yield_pct: Set(to_dec(input.yield_pct)),
            color_lovibond: Set(to_dec(input.color_lovibond)),
            amount_kg: Set(to_dec(input.amount_kg)),
            add_after_boil: Set(input.add_after_boil.map(|v| v as i32)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_fermentables::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionFermentable::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateFermentableAdditionInput,
    ) -> Result<RecipeAdditionFermentable, AppError> {
        let row = recipe_addition_fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_fermentables::ActiveModel = row.into();

        if let Some(v) = input.amount_kg {
            active.amount_kg = Set(to_dec(v));
        }
        if let Some(v) = input.add_after_boil {
            active.add_after_boil = Set(Some(v as i32));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        recipe_addition_fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionFermentable::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_fermentables::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateFermentableAdditionInput {
        CreateFermentableAdditionInput {
            fermentable_id: None,
            name: "Pale Malt".into(),
            type_: "grain".into(),
            yield_pct: 78.0,
            color_lovibond: 1.8,
            amount_kg: 4.5,
            add_after_boil: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Pale Malt");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_list_order() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let mut second = input();
        second.name = "Crystal 60".into();
        repo.create(&recipe_id, second).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items[0].addition_order, 0);
        assert_eq!(items[1].addition_order, 1);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateFermentableAdditionInput {
                amount_kg: Some(5.0),
                add_after_boil: None,
                addition_order: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.amount_kg, 5.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cd src-tauri && cargo test fermentable 2>&1 | tail -20
```

Expected: 4 tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/fermentable.rs \
        src-tauri/src/repositories/mod.rs
git commit -m "feat: add FermentableRepository"
```

---

### Task 3: Create `HopRepository`

**Files:**
- Create: `src-tauri/src/repositories/hop.rs`
- Modify: `src-tauri/src/repositories/mod.rs`

- [ ] **Step 1: Register the new module in `mod.rs`**

Add `pub mod hop;` to `src-tauri/src/repositories/mod.rs`.

- [ ] **Step 2: Create `src-tauri/src/repositories/hop.rs`**

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_hops;
use crate::error::AppError;
use crate::models::{CreateHopAdditionInput, RecipeAdditionHop, UpdateHopAdditionInput};

use super::{new_id, to_dec};

pub struct HopRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> HopRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionHop>, AppError> {
        recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_hops::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionHop::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateHopAdditionInput,
    ) -> Result<RecipeAdditionHop, AppError> {
        let order = recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_hops::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            hop_id: Set(input.hop_id),
            name: Set(input.name),
            alpha_pct: Set(to_dec(input.alpha_pct)),
            form: Set(input.form.unwrap_or_else(|| "Pellet".to_string())),
            amount_kg: Set(to_dec(input.amount_kg)),
            r#use: Set(input.use_),
            time_min: Set(to_dec(input.time_min)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_hops::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionHop::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateHopAdditionInput,
    ) -> Result<RecipeAdditionHop, AppError> {
        let row = recipe_addition_hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_hops::ActiveModel = row.into();

        if let Some(v) = input.amount_kg {
            active.amount_kg = Set(to_dec(v));
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(to_dec(v));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        recipe_addition_hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionHop::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_hops::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateHopAdditionInput {
        CreateHopAdditionInput {
            hop_id: None,
            name: "Cascade".into(),
            alpha_pct: 5.5,
            form: None,
            amount_kg: 0.05,
            use_: "Boil".into(),
            time_min: 60.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Cascade");
        assert_eq!(items[0].form, "Pellet");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateHopAdditionInput {
                amount_kg: Some(0.1),
                use_: None,
                time_min: None,
                addition_order: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.amount_kg, 0.1);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cd src-tauri && cargo test hop:: 2>&1 | tail -20
```

Expected: 3 tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/hop.rs \
        src-tauri/src/repositories/mod.rs
git commit -m "feat: add HopRepository"
```

---

### Task 4: Create `YeastRepository`

**Files:**
- Create: `src-tauri/src/repositories/yeast.rs`
- Modify: `src-tauri/src/repositories/mod.rs`
- Modify: `src-tauri/src/models.rs`

`CreateYeastAdditionInput` is missing `add_to_secondary` and `times_cultured`. The current `copy_additions` preserves these fields, but routing through `create` would silently drop them. Extend the input type first.

- [ ] **Step 1: Extend `CreateYeastAdditionInput` in `models.rs`**

Find `CreateYeastAdditionInput` (around line 682) and add two fields:

```rust
pub struct CreateYeastAdditionInput {
    pub yeast_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub add_to_secondary: Option<bool>,   // ← new
    pub times_cultured: Option<i64>,      // ← new
}
```

- [ ] **Step 2: Register the new module in `mod.rs`**

Add `pub mod yeast;` to `src-tauri/src/repositories/mod.rs`.

- [ ] **Step 3: Create `src-tauri/src/repositories/yeast.rs`**

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::entities::recipe_addition_yeasts;
use crate::error::AppError;
use crate::models::{CreateYeastAdditionInput, RecipeAdditionYeast, UpdateYeastAdditionInput};

use super::{new_id, to_dec, to_dec_opt};

pub struct YeastRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> YeastRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionYeast>, AppError> {
        recipe_addition_yeasts::Entity::find()
            .filter(recipe_addition_yeasts::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionYeast::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateYeastAdditionInput,
    ) -> Result<RecipeAdditionYeast, AppError> {
        let id = new_id();
        recipe_addition_yeasts::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            yeast_id: Set(input.yeast_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            form: Set(input.form),
            laboratory: Set(input.laboratory),
            product_id: Set(input.product_id),
            attenuation_pct: Set(to_dec_opt(input.attenuation_pct)),
            amount: Set(to_dec_opt(input.amount)),
            amount_is_weight: Set(input.amount_is_weight.map(|v| v as i32)),
            add_to_secondary: Set(input.add_to_secondary.map(|v| v as i32)),
            times_cultured: Set(input.times_cultured.map(|v| v as i32)),
        }
        .insert(self.db)
        .await?;

        recipe_addition_yeasts::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionYeast::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateYeastAdditionInput,
    ) -> Result<RecipeAdditionYeast, AppError> {
        let row = recipe_addition_yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_yeasts::ActiveModel = row.into();

        if let Some(v) = input.attenuation_pct {
            active.attenuation_pct = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.amount {
            active.amount = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.amount_is_weight {
            active.amount_is_weight = Set(Some(v as i32));
        }
        if let Some(v) = input.add_to_secondary {
            active.add_to_secondary = Set(Some(v as i32));
        }
        if let Some(v) = input.times_cultured {
            active.times_cultured = Set(Some(v as i32));
        }

        active.update(self.db).await?;

        recipe_addition_yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionYeast::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_yeasts::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateYeastAdditionInput {
        CreateYeastAdditionInput {
            yeast_id: None,
            name: "US-05".into(),
            type_: "ale".into(),
            form: "dry".into(),
            laboratory: Some("Fermentis".into()),
            product_id: None,
            attenuation_pct: Some(77.0),
            amount: None,
            amount_is_weight: None,
            add_to_secondary: None,
            times_cultured: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "US-05");
        assert_eq!(items[0].attenuation_pct, Some(77.0));
    }

    #[tokio::test]
    async fn test_create_preserves_add_to_secondary() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let mut i = input();
        i.add_to_secondary = Some(true);
        i.times_cultured = Some(2);
        let created = repo.create(&recipe_id, i).await.unwrap();
        assert!(created.add_to_secondary);
        assert_eq!(created.times_cultured, 2);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateYeastAdditionInput {
                attenuation_pct: Some(80.0),
                amount: None,
                amount_is_weight: None,
                add_to_secondary: None,
                times_cultured: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.attenuation_pct, Some(80.0));
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
```

- [ ] **Step 4: Run the tests**

```bash
cd src-tauri && cargo test yeast:: 2>&1 | tail -20
```

Expected: 4 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/repositories/yeast.rs \
        src-tauri/src/repositories/mod.rs \
        src-tauri/src/models.rs
git commit -m "feat: add YeastRepository; extend CreateYeastAdditionInput with add_to_secondary and times_cultured"
```

---

### Task 5: Create `MiscRepository`

**Files:**
- Create: `src-tauri/src/repositories/misc.rs`
- Modify: `src-tauri/src/repositories/mod.rs`

- [ ] **Step 1: Register the new module in `mod.rs`**

Add `pub mod misc;` to `src-tauri/src/repositories/mod.rs`.

- [ ] **Step 2: Create `src-tauri/src/repositories/misc.rs`**

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_miscs;
use crate::error::AppError;
use crate::models::{CreateMiscAdditionInput, RecipeAdditionMisc, UpdateMiscAdditionInput};

use super::{new_id, to_dec};

pub struct MiscRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> MiscRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionMisc>, AppError> {
        recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionMisc::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateMiscAdditionInput,
    ) -> Result<RecipeAdditionMisc, AppError> {
        let order = recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_miscs::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            misc_id: Set(input.misc_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            r#use: Set(input.use_),
            amount: Set(to_dec(input.amount)),
            amount_is_weight: Set(input.amount_is_weight.map(|v| v as i32)),
            time_min: Set(to_dec(input.time_min)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_miscs::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionMisc::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateMiscAdditionInput,
    ) -> Result<RecipeAdditionMisc, AppError> {
        let row = recipe_addition_miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_miscs::ActiveModel = row.into();

        if let Some(v) = input.amount {
            active.amount = Set(to_dec(v));
        }
        if let Some(v) = input.amount_is_weight {
            active.amount_is_weight = Set(Some(v as i32));
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(to_dec(v));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        recipe_addition_miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionMisc::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_miscs::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateMiscAdditionInput {
        CreateMiscAdditionInput {
            misc_id: None,
            name: "Irish Moss".into(),
            type_: "fining".into(),
            use_: "Boil".into(),
            amount: 1.0,
            amount_is_weight: Some(true),
            time_min: 15.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Irish Moss");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateMiscAdditionInput {
                amount: Some(2.0),
                amount_is_weight: None,
                use_: None,
                time_min: None,
                addition_order: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.amount, 2.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cd src-tauri && cargo test misc:: 2>&1 | tail -20
```

Expected: 3 tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/misc.rs \
        src-tauri/src/repositories/mod.rs
git commit -m "feat: add MiscRepository"
```

---

### Task 6: Create `WaterRepository`

**Files:**
- Create: `src-tauri/src/repositories/water.rs`
- Modify: `src-tauri/src/repositories/mod.rs`

- [ ] **Step 1: Register the new module in `mod.rs`**

Add `pub mod water;` to `src-tauri/src/repositories/mod.rs`.

- [ ] **Step 2: Create `src-tauri/src/repositories/water.rs`**

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::entities::recipe_addition_waters;
use crate::error::AppError;
use crate::models::{CreateWaterAdditionInput, RecipeAdditionWater, UpdateWaterAdditionInput};

use super::{new_id, to_dec};

pub struct WaterRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> WaterRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionWater>, AppError> {
        recipe_addition_waters::Entity::find()
            .filter(recipe_addition_waters::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionWater::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateWaterAdditionInput,
    ) -> Result<RecipeAdditionWater, AppError> {
        let id = new_id();
        recipe_addition_waters::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            water_id: Set(input.water_id),
            name: Set(input.name),
            amount_l: Set(to_dec(input.amount_l)),
        }
        .insert(self.db)
        .await?;

        recipe_addition_waters::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionWater::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateWaterAdditionInput,
    ) -> Result<RecipeAdditionWater, AppError> {
        let row = recipe_addition_waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_waters::ActiveModel = row.into();

        if let Some(v) = input.amount_l {
            active.amount_l = Set(to_dec(v));
        }

        active.update(self.db).await?;

        recipe_addition_waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionWater::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_waters::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateWaterAdditionInput {
        CreateWaterAdditionInput {
            water_id: None,
            name: "RO Water".into(),
            amount_l: 25.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "RO Water");
        assert_eq!(items[0].amount_l, 25.0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateWaterAdditionInput { amount_l: Some(20.0) })
            .await
            .unwrap();
        assert_eq!(updated.amount_l, 20.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cd src-tauri && cargo test water:: 2>&1 | tail -20
```

Expected: 3 tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/water.rs \
        src-tauri/src/repositories/mod.rs
git commit -m "feat: add WaterRepository"
```

---

### Task 7: Expose `EquipmentRepository::get` and add `LibraryRepository::get_style`

**Files:**
- Modify: `src-tauri/src/repositories/equipment.rs`
- Modify: `src-tauri/src/repositories/library.rs`

- [ ] **Step 1: Make `EquipmentRepository::find_by_id` public and rename it to `get`**

In `src-tauri/src/repositories/equipment.rs`, change line 28 from:

```rust
    async fn find_by_id(&self, id: &str) -> Result<EquipmentProfile, AppError> {
```

to:

```rust
    pub async fn get(&self, id: &str) -> Result<EquipmentProfile, AppError> {
```

Update the two internal call sites within `equipment.rs` (`create` calls `self.find_by_id(&id)` and `update` calls `self.find_by_id(id)`) to use `self.get`:

```rust
        self.get(&id).await   // in create
        self.get(id).await    // in update
```

- [ ] **Step 2: Add `get_style` to `LibraryRepository`**

In `src-tauri/src/repositories/library.rs`, add after `list_styles`:

```rust
    pub async fn get_style(&self, id: &str) -> Result<Style, AppError> {
        styles::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Style::try_from)
    }
```

- [ ] **Step 3: Verify it compiles**

```bash
cd src-tauri && cargo build 2>&1 | grep -E "^error"
```

Expected: no output.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/equipment.rs \
        src-tauri/src/repositories/library.rs
git commit -m "refactor: expose EquipmentRepository::get; add LibraryRepository::get_style"
```

---

### Task 8: Update command handlers to use typed repositories

**Files:**
- Modify: `src-tauri/src/commands/additions.rs`
- Modify: `src-tauri/src/commands/import_export.rs`

- [ ] **Step 1: Rewrite `commands/additions.rs`**

Replace the entire file:

```rust
use tauri::State;
use crate::AppState;
use crate::models::*;
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::water::WaterRepository;
use crate::repositories::yeast::YeastRepository;

#[tauri::command]
pub async fn create_recipe_fermentable(state: State<'_, AppState>, recipe_id: String, input: CreateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    FermentableRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_fermentable(state: State<'_, AppState>, id: String, input: UpdateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    FermentableRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_fermentable(state: State<'_, AppState>, id: String) -> Result<(), String> {
    FermentableRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_hop(state: State<'_, AppState>, recipe_id: String, input: CreateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    HopRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_hop(state: State<'_, AppState>, id: String, input: UpdateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    HopRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), String> {
    HopRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_yeast(state: State<'_, AppState>, recipe_id: String, input: CreateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    YeastRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_yeast(state: State<'_, AppState>, id: String, input: UpdateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    YeastRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), String> {
    YeastRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_misc(state: State<'_, AppState>, recipe_id: String, input: CreateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    MiscRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_misc(state: State<'_, AppState>, id: String, input: UpdateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    MiscRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), String> {
    MiscRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_water(state: State<'_, AppState>, recipe_id: String, input: CreateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    WaterRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_water(state: State<'_, AppState>, id: String, input: UpdateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    WaterRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), String> {
    WaterRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 2: Update the import in `commands/import_export.rs`**

Replace line 4:

```rust
use crate::repositories::addition::AdditionRepository;
```

with:

```rust
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
```

Replace the `addition_repo` construction and usage in `create_recipes_from_beerxml`. The block at lines 53–88 becomes:

```rust
    let recipe_repo = RecipeRepository::new(&state.db);
    let fermentable_repo = FermentableRepository::new(&state.db);
    let hop_repo = HopRepository::new(&state.db);

    let recipe = recipe_repo.create(CreateRecipeInput {
        name, type_: Some(type_), batch_size_l: Some(batch_size),
        boil_size_l: Some(boil_size), boil_time_min: Some(boil_time),
        equipment_profile_id: None, source_id: None,
    }).await.map_err(|e| e.to_string())?;

    let ferm_xml = extract_between(&xml, "<FERMENTABLES>", "</FERMENTABLES>").unwrap_or_default();
    for ferm_block in split_tags(&ferm_xml, "FERMENTABLE") {
        let fermentable_name = extract_tag(&ferm_block, "NAME").unwrap_or_default();
        if fermentable_name.is_empty() { continue; }
        let _ = fermentable_repo.create(&recipe.id, CreateFermentableAdditionInput {
            fermentable_id: None, name: fermentable_name,
            type_: extract_tag(&ferm_block, "TYPE").unwrap_or("grain".to_string()),
            yield_pct: extract_tag(&ferm_block, "YIELD").and_then(|v| v.parse().ok()).unwrap_or(75.0),
            color_lovibond: extract_tag(&ferm_block, "COLOR").and_then(|v| v.parse().ok()).unwrap_or(2.0),
            amount_kg: extract_tag(&ferm_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0),
            add_after_boil: None,
        }).await;
    }

    let hops_xml = extract_between(&xml, "<HOPS>", "</HOPS>").unwrap_or_default();
    for hop_block in split_tags(&hops_xml, "HOP") {
        let hop_name = extract_tag(&hop_block, "NAME").unwrap_or_default();
        if hop_name.is_empty() { continue; }
        let _ = hop_repo.create(&recipe.id, CreateHopAdditionInput {
            hop_id: None, name: hop_name,
            alpha_pct: extract_tag(&hop_block, "ALPHA").and_then(|v| v.parse().ok()).unwrap_or(5.0),
            form: extract_tag(&hop_block, "FORM"),
            amount_kg: extract_tag(&hop_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0),
            use_: extract_tag(&hop_block, "USE").unwrap_or("boil".to_string()),
            time_min: extract_tag(&hop_block, "TIME").and_then(|v| v.parse().ok()).unwrap_or(60.0),
        }).await;
    }
```

- [ ] **Step 3: Verify it compiles**

```bash
cd src-tauri && cargo build 2>&1 | grep -E "^error"
```

Expected: no output.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/additions.rs \
        src-tauri/src/commands/import_export.rs
git commit -m "refactor: update command handlers to use typed addition repositories"
```

---

### Task 9: Refactor `RecipeRepository::get` to delegate to typed repos

**Files:**
- Modify: `src-tauri/src/repositories/recipe.rs`

- [ ] **Step 1: Replace the imports at the top of `recipe.rs`**

Replace the existing `use` block with:

```rust
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryOrder, Set,
};

use crate::entities::recipes;
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateRecipeInput, CreateWaterAdditionInput, CreateYeastAdditionInput,
    Recipe, RecipeSummary, UpdateRecipeInput,
};

use super::{from_dec, new_id, now_secs, to_dec};
use super::equipment::EquipmentRepository;
use super::fermentable::FermentableRepository;
use super::hop::HopRepository;
use super::library::LibraryRepository;
use super::mash::MashRepository;
use super::misc::MiscRepository;
use super::water::WaterRepository;
use super::yeast::YeastRepository;
```

- [ ] **Step 2: Replace the `get` method body**

Replace the entire `pub async fn get` method (lines 56–222 in the original) with:

```rust
    pub async fn get(&self, id: &str) -> Result<Recipe, AppError> {
        let recipe_row = recipes::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let fermentables = FermentableRepository::new(self.db).list(id).await?;
        let hops = HopRepository::new(self.db).list(id).await?;
        let yeasts = YeastRepository::new(self.db).list(id).await?;
        let miscs = MiscRepository::new(self.db).list(id).await?;
        let waters = WaterRepository::new(self.db).list(id).await?;

        let mash = match MashRepository::new(self.db).get_for_recipe(id).await {
            Ok(mash) => Some(mash),
            Err(AppError::NotFound) => None,
            Err(e) => return Err(e),
        };

        let equipment_profile = if let Some(ep_id) = &recipe_row.equipment_profile_id {
            Some(EquipmentRepository::new(self.db).get(ep_id).await?)
        } else {
            None
        };

        let style = if let Some(style_id) = &recipe_row.style_id {
            Some(LibraryRepository::new(self.db).get_style(style_id).await?)
        } else {
            None
        };

        Ok(Recipe {
            id: recipe_row.id,
            name: recipe_row.name,
            type_: recipe_row.r#type,
            brewer: recipe_row.brewer,
            asst_brewer: recipe_row.asst_brewer,
            batch_size_l: from_dec(recipe_row.batch_size_l)?,
            boil_size_l: from_dec(recipe_row.boil_size_l)?,
            boil_time_min: from_dec(recipe_row.boil_time_min)?,
            efficiency_pct: recipe_row.efficiency_pct.and_then(|v| v.to_f64()),
            style_id: recipe_row.style_id,
            equipment_profile_id: recipe_row.equipment_profile_id,
            notes: recipe_row.notes,
            taste_notes: recipe_row.taste_notes,
            taste_rating: recipe_row.taste_rating.and_then(|v| v.to_f64()),
            og: recipe_row.og.and_then(|v| v.to_f64()),
            fg: recipe_row.fg.and_then(|v| v.to_f64()),
            fermentation_stages: recipe_row.fermentation_stages.unwrap_or(1) as i64,
            primary_age_days: recipe_row.primary_age_days.and_then(|v| v.to_f64()),
            primary_temp_c: recipe_row.primary_temp_c.and_then(|v| v.to_f64()),
            secondary_age_days: recipe_row.secondary_age_days.and_then(|v| v.to_f64()),
            secondary_temp_c: recipe_row.secondary_temp_c.and_then(|v| v.to_f64()),
            tertiary_age_days: recipe_row.tertiary_age_days.and_then(|v| v.to_f64()),
            tertiary_temp_c: recipe_row.tertiary_temp_c.and_then(|v| v.to_f64()),
            age_days: recipe_row.age_days.and_then(|v| v.to_f64()),
            age_temp_c: recipe_row.age_temp_c.and_then(|v| v.to_f64()),
            carbonation_vols: recipe_row.carbonation_vols.and_then(|v| v.to_f64()),
            forced_carbonation: recipe_row.forced_carbonation.unwrap_or(0) != 0,
            priming_sugar_name: recipe_row.priming_sugar_name,
            carbonation_temp_c: recipe_row.carbonation_temp_c.and_then(|v| v.to_f64()),
            priming_sugar_equiv: recipe_row.priming_sugar_equiv.and_then(|v| v.to_f64()),
            keg_priming_factor: recipe_row.keg_priming_factor.and_then(|v| v.to_f64()),
            date: recipe_row.date,
            created_at: recipe_row.created_at as i64,
            updated_at: recipe_row.updated_at as i64,
            equipment_profile,
            style,
            fermentables,
            hops,
            yeasts,
            miscs,
            waters,
            mash,
        })
    }
```

- [ ] **Step 3: Run the existing recipe tests**

```bash
cd src-tauri && cargo test repositories::recipe 2>&1 | tail -20
```

Expected: all 5 recipe tests pass (`test_create_and_list`, `test_get_returns_full_recipe`, `test_update_name`, `test_delete`, `test_duplicate_via_source_id`).

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/recipe.rs
git commit -m "refactor: RecipeRepository::get delegates to typed repositories"
```

---

### Task 10: Refactor `RecipeRepository::copy_additions` to delegate to typed repos

**Files:**
- Modify: `src-tauri/src/repositories/recipe.rs`

- [ ] **Step 1: Replace `copy_additions`**

Replace the entire `async fn copy_additions` method with:

```rust
    async fn copy_additions(&self, src_id: &str, dst_id: &str) -> Result<(), AppError> {
        // Additions are copied rather than referenced so that edits to the
        // source recipe don't affect the duplicate.

        let fermentable_repo = FermentableRepository::new(self.db);
        for f in fermentable_repo.list(src_id).await? {
            fermentable_repo.create(dst_id, CreateFermentableAdditionInput {
                fermentable_id: f.fermentable_id,
                name: f.name,
                type_: f.type_,
                yield_pct: f.yield_pct,
                color_lovibond: f.color_lovibond,
                amount_kg: f.amount_kg,
                add_after_boil: Some(f.add_after_boil),
            }).await?;
        }

        let hop_repo = HopRepository::new(self.db);
        for h in hop_repo.list(src_id).await? {
            hop_repo.create(dst_id, CreateHopAdditionInput {
                hop_id: h.hop_id,
                name: h.name,
                alpha_pct: h.alpha_pct,
                form: Some(h.form),
                amount_kg: h.amount_kg,
                use_: h.use_,
                time_min: h.time_min,
            }).await?;
        }

        let yeast_repo = YeastRepository::new(self.db);
        for y in yeast_repo.list(src_id).await? {
            yeast_repo.create(dst_id, CreateYeastAdditionInput {
                yeast_id: y.yeast_id,
                name: y.name,
                type_: y.type_,
                form: y.form,
                laboratory: y.laboratory,
                product_id: y.product_id,
                attenuation_pct: y.attenuation_pct,
                amount: y.amount,
                amount_is_weight: Some(y.amount_is_weight),
                add_to_secondary: Some(y.add_to_secondary),
                times_cultured: Some(y.times_cultured),
            }).await?;
        }

        let misc_repo = MiscRepository::new(self.db);
        for m in misc_repo.list(src_id).await? {
            misc_repo.create(dst_id, CreateMiscAdditionInput {
                misc_id: m.misc_id,
                name: m.name,
                type_: m.type_,
                use_: m.use_,
                amount: m.amount,
                amount_is_weight: Some(m.amount_is_weight),
                time_min: m.time_min,
            }).await?;
        }

        let water_repo = WaterRepository::new(self.db);
        for w in water_repo.list(src_id).await? {
            water_repo.create(dst_id, CreateWaterAdditionInput {
                water_id: w.water_id,
                name: w.name,
                amount_l: w.amount_l,
            }).await?;
        }

        Ok(())
    }
```

- [ ] **Step 2: Run all repository tests**

```bash
cd src-tauri && cargo test 2>&1 | tail -30
```

Expected: all tests pass.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/repositories/recipe.rs
git commit -m "refactor: copy_additions delegates to typed repositories"
```

---

### Task 11: Delete `AdditionRepository` and clean up

**Files:**
- Modify: `src-tauri/src/repositories/mod.rs`
- Delete: `src-tauri/src/repositories/addition.rs`

- [ ] **Step 1: Remove `pub mod addition` from `mod.rs`**

Delete the line `pub mod addition;` from `src-tauri/src/repositories/mod.rs`.

- [ ] **Step 2: Delete `addition.rs`**

```bash
rm src-tauri/src/repositories/addition.rs
```

- [ ] **Step 3: Full compile and test run**

```bash
cd src-tauri && cargo test 2>&1 | tail -30
```

Expected: all tests pass, no compilation errors.

- [ ] **Step 4: Commit**

```bash
git add -u src-tauri/src/repositories/
git commit -m "refactor: delete AdditionRepository — replaced by typed addition repositories"
```
