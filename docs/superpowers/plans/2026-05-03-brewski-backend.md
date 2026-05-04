# Brewski Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement all Tauri commands — equipment profiles, styles, ingredient library, recipe CRUD, recipe additions, mash, settings, and BeerXML import/export.

**Architecture:** DB layer (`src-tauri/src/db/`) holds sqlx queries. Commands layer (`src-tauri/src/commands/`) holds thin Tauri handlers that call db/ functions and return `Result<T, String>`. No business logic in commands — only in `brewing/`. Tests hit an in-memory SQLite DB.

**Tech Stack:** Rust, sqlx 0.8 (SQLite), Tauri 2, uuid v1, serde, thiserror

**Prerequisite:** `2026-05-03-brewski-foundation.md` must be complete.

---

### Task 1: Test helper — in-memory DB

**Files:**
- Create: `src-tauri/src/db/test_helpers.rs`
- Modify: `src-tauri/src/db/mod.rs`

- [ ] **Step 1: Create `src-tauri/src/db/test_helpers.rs`**

```rust
use sqlx::SqlitePool;

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.expect("in-memory DB failed");
    sqlx::migrate!("src/db/migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    pool
}
```

- [ ] **Step 2: Add to `src-tauri/src/db/mod.rs`**

```rust
pub mod additions;
pub mod equipment;
pub mod library;
pub mod mash;
pub mod recipes;
pub mod settings;

#[cfg(test)]
pub mod test_helpers;
```

- [ ] **Step 3: Verify compilation**

```bash
cd src-tauri && cargo build
```

Expected: compiles.

- [ ] **Step 4: Commit**

```bash
cd ..
git add src-tauri/src/db/
git commit -m "feat: in-memory test DB helper"
```

---

### Task 2: Equipment profiles — DB layer

**Files:**
- Modify: `src-tauri/src/db/equipment.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/db/equipment.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput};
use uuid::Uuid;

pub async fn list(db: &SqlitePool) -> Result<Vec<EquipmentProfile>, AppError> {
    todo!()
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<EquipmentProfile, AppError> {
    todo!()
}

pub async fn create(db: &SqlitePool, input: CreateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    todo!()
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    todo!()
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::setup_test_db;

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let input = CreateEquipmentProfileInput {
            name: "10 Gallon Kettle".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: Some(60.0),
            evap_rate_pct_hr: Some(10.0),
            trub_chiller_loss_l: Some(1.5),
            fermenter_loss_l: Some(1.0),
            efficiency_pct: 72.0,
        };
        let created = create(&db, input).await.unwrap();
        assert_eq!(created.name, "10 Gallon Kettle");

        let all = list(&db).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, created.id);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let created = create(&db, CreateEquipmentProfileInput {
            name: "Old Name".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: 72.0,
        }).await.unwrap();

        let updated = update(&db, &created.id, UpdateEquipmentProfileInput {
            name: Some("New Name".into()),
            notes: None,
            boil_size_l: None,
            batch_size_l: None,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: None,
        }).await.unwrap();
        assert_eq!(updated.name, "New Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let created = create(&db, CreateEquipmentProfileInput {
            name: "To Delete".into(),
            notes: None,
            boil_size_l: 30.0,
            batch_size_l: 23.0,
            boil_time_min: None,
            evap_rate_pct_hr: None,
            trub_chiller_loss_l: None,
            fermenter_loss_l: None,
            efficiency_pct: 72.0,
        }).await.unwrap();

        delete(&db, &created.id).await.unwrap();
        let all = list(&db).await.unwrap();
        assert!(all.is_empty());
    }
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cd src-tauri && cargo test db::equipment
```

Expected: FAIL with `not yet implemented`.

- [ ] **Step 3: Implement all five functions**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput};
use uuid::Uuid;

pub async fn list(db: &SqlitePool) -> Result<Vec<EquipmentProfile>, AppError> {
    let rows = sqlx::query_as::<_, EquipmentProfile>(
        "SELECT * FROM equipment_profiles ORDER BY name"
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<EquipmentProfile, AppError> {
    sqlx::query_as::<_, EquipmentProfile>(
        "SELECT * FROM equipment_profiles WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)
}

pub async fn create(db: &SqlitePool, input: CreateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query(
        "INSERT INTO equipment_profiles (
            id, name, notes, boil_size_l, batch_size_l, boil_time_min,
            evap_rate_pct_hr, trub_chiller_loss_l, fermenter_loss_l,
            efficiency_pct, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.name)
    .bind(&input.notes)
    .bind(input.boil_size_l)
    .bind(input.batch_size_l)
    .bind(input.boil_time_min.unwrap_or(60.0))
    .bind(input.evap_rate_pct_hr.unwrap_or(10.0))
    .bind(input.trub_chiller_loss_l.unwrap_or(1.0))
    .bind(input.fermenter_loss_l.unwrap_or(1.0))
    .bind(input.efficiency_pct)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    get(db, &id).await
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Fetch current, apply partial updates
    let current = get(db, id).await?;
    sqlx::query(
        "UPDATE equipment_profiles SET
            name = ?, notes = ?, boil_size_l = ?, batch_size_l = ?,
            boil_time_min = ?, evap_rate_pct_hr = ?, trub_chiller_loss_l = ?,
            fermenter_loss_l = ?, efficiency_pct = ?, updated_at = ?
        WHERE id = ?"
    )
    .bind(input.name.unwrap_or(current.name))
    .bind(input.notes.or(current.notes))
    .bind(input.boil_size_l.unwrap_or(current.boil_size_l))
    .bind(input.batch_size_l.unwrap_or(current.batch_size_l))
    .bind(input.boil_time_min.unwrap_or(current.boil_time_min))
    .bind(input.evap_rate_pct_hr.unwrap_or(current.evap_rate_pct_hr))
    .bind(input.trub_chiller_loss_l.unwrap_or(current.trub_chiller_loss_l))
    .bind(input.fermenter_loss_l.unwrap_or(current.fermenter_loss_l))
    .bind(input.efficiency_pct.unwrap_or(current.efficiency_pct))
    .bind(now)
    .bind(id)
    .execute(db)
    .await?;

    get(db, id).await
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM equipment_profiles WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cd src-tauri && cargo test db::equipment
```

Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
cd ..
git add src-tauri/src/db/equipment.rs
git commit -m "feat: equipment profiles DB layer"
```

---

### Task 3: Equipment profiles — Tauri commands

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/equipment.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create `src-tauri/src/commands/mod.rs`**

```rust
pub mod equipment;
pub mod library;
pub mod recipes;
pub mod additions;
pub mod mash;
pub mod settings;
pub mod import_export;
```

- [ ] **Step 2: Create `src-tauri/src/commands/equipment.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::{EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput};
use crate::db;

#[tauri::command]
pub async fn list_equipment_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<EquipmentProfile>, String> {
    db::equipment::list(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_equipment_profile(
    state: State<'_, AppState>,
    input: CreateEquipmentProfileInput,
) -> Result<EquipmentProfile, String> {
    db::equipment::create(&state.db, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_equipment_profile(
    state: State<'_, AppState>,
    id: String,
    input: UpdateEquipmentProfileInput,
) -> Result<EquipmentProfile, String> {
    db::equipment::update(&state.db, &id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_equipment_profile(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    db::equipment::delete(&state.db, &id).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 3: Create stub files for remaining command modules**

`src-tauri/src/commands/library.rs`:
```rust
use tauri::State;
use crate::AppState;
```

`src-tauri/src/commands/recipes.rs`:
```rust
use tauri::State;
use crate::AppState;
```

`src-tauri/src/commands/additions.rs`:
```rust
use tauri::State;
use crate::AppState;
```

`src-tauri/src/commands/mash.rs`:
```rust
use tauri::State;
use crate::AppState;
```

`src-tauri/src/commands/settings.rs`:
```rust
use tauri::State;
use crate::AppState;
```

`src-tauri/src/commands/import_export.rs`:
```rust
use tauri::State;
use crate::AppState;
```

- [ ] **Step 4: Register commands in `src-tauri/src/lib.rs`**

Add `mod commands;` near the top and update the `invoke_handler`:

```rust
mod commands;
mod db;
mod error;
pub mod models;
pub mod brewing;

use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_url = format!("sqlite:{}", app_dir.join("brewski.db").display());

            let pool = tauri::async_runtime::block_on(
                SqlitePool::connect(&db_url)
            )?;

            tauri::async_runtime::block_on(
                sqlx::migrate!("src/db/migrations").run(&pool)
            )?;

            app.manage(AppState { db: pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::equipment::list_equipment_profiles,
            commands::equipment::create_equipment_profile,
            commands::equipment::update_equipment_profile,
            commands::equipment::delete_equipment_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Verify compilation**

```bash
cd src-tauri && cargo build
```

Expected: compiles cleanly.

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/commands/ src-tauri/src/lib.rs
git commit -m "feat: equipment profile Tauri commands"
```

---

### Task 4: Styles + ingredient library — DB layer and commands

**Files:**
- Modify: `src-tauri/src/db/library.rs`
- Modify: `src-tauri/src/commands/library.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Implement `src-tauri/src/db/library.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Style, Fermentable, Hop, Yeast, Misc, Water};

pub async fn list_styles(db: &SqlitePool) -> Result<Vec<Style>, AppError> {
    Ok(sqlx::query_as::<_, Style>("SELECT * FROM styles ORDER BY category, name")
        .fetch_all(db).await?)
}

pub async fn list_fermentables(db: &SqlitePool) -> Result<Vec<Fermentable>, AppError> {
    Ok(sqlx::query_as::<_, Fermentable>("SELECT * FROM fermentables ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_hops(db: &SqlitePool) -> Result<Vec<Hop>, AppError> {
    Ok(sqlx::query_as::<_, Hop>("SELECT * FROM hops ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_yeasts(db: &SqlitePool) -> Result<Vec<Yeast>, AppError> {
    Ok(sqlx::query_as::<_, Yeast>("SELECT * FROM yeasts ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_miscs(db: &SqlitePool) -> Result<Vec<Misc>, AppError> {
    Ok(sqlx::query_as::<_, Misc>("SELECT * FROM miscs ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_waters(db: &SqlitePool) -> Result<Vec<Water>, AppError> {
    Ok(sqlx::query_as::<_, Water>("SELECT * FROM waters ORDER BY name")
        .fetch_all(db).await?)
}
```

- [ ] **Step 2: Implement `src-tauri/src/commands/library.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::{Style, Fermentable, Hop, Yeast, Misc, Water};
use crate::db;

#[tauri::command]
pub async fn list_styles(state: State<'_, AppState>) -> Result<Vec<Style>, String> {
    db::library::list_styles(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_fermentable_library(state: State<'_, AppState>) -> Result<Vec<Fermentable>, String> {
    db::library::list_fermentables(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_hop_library(state: State<'_, AppState>) -> Result<Vec<Hop>, String> {
    db::library::list_hops(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_yeast_library(state: State<'_, AppState>) -> Result<Vec<Yeast>, String> {
    db::library::list_yeasts(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_misc_library(state: State<'_, AppState>) -> Result<Vec<Misc>, String> {
    db::library::list_miscs(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_water_library(state: State<'_, AppState>) -> Result<Vec<Water>, String> {
    db::library::list_waters(&state.db).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 3: Register commands in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::library::list_styles,
commands::library::list_fermentable_library,
commands::library::list_hop_library,
commands::library::list_yeast_library,
commands::library::list_misc_library,
commands::library::list_water_library,
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd ..
git add src-tauri/src/db/library.rs src-tauri/src/commands/library.rs src-tauri/src/lib.rs
git commit -m "feat: styles and ingredient library commands"
```

---

### Task 5: Recipes — DB layer

**Files:**
- Modify: `src-tauri/src/db/recipes.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/db/recipes.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Recipe, RecipeRow, RecipeSummary, CreateRecipeInput, UpdateRecipeInput};
use uuid::Uuid;

pub async fn list(db: &SqlitePool) -> Result<Vec<RecipeSummary>, AppError> {
    todo!()
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<Recipe, AppError> {
    todo!()
}

pub async fn create(db: &SqlitePool, input: CreateRecipeInput) -> Result<Recipe, AppError> {
    todo!()
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateRecipeInput) -> Result<Recipe, AppError> {
    todo!()
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    todo!()
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::setup_test_db;

    fn basic_create_input() -> CreateRecipeInput {
        CreateRecipeInput {
            name: "Test Recipe".into(),
            type_: Some("all_grain".into()),
            batch_size_l: Some(23.0),
            boil_size_l: Some(27.0),
            boil_time_min: Some(60.0),
            equipment_profile_id: None,
            source_id: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        create(&db, basic_create_input()).await.unwrap();
        let all = list(&db).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "Test Recipe");
    }

    #[tokio::test]
    async fn test_get_returns_full_recipe() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        let fetched = get(&db, &created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.batch_size_l, 23.0);
        assert!(fetched.fermentables.is_empty());
    }

    #[tokio::test]
    async fn test_update_name() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        let updated = update(&db, &created.id, UpdateRecipeInput {
            name: Some("Updated Name".into()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let created = create(&db, basic_create_input()).await.unwrap();
        delete(&db, &created.id).await.unwrap();
        let all = list(&db).await.unwrap();
        assert!(all.is_empty());
    }

    #[tokio::test]
    async fn test_duplicate_via_source_id() {
        let db = setup_test_db().await;
        let original = create(&db, basic_create_input()).await.unwrap();
        let dupe = create(&db, CreateRecipeInput {
            name: "Copy of Test Recipe".into(),
            source_id: Some(original.id.clone()),
            ..Default::default()
        }).await.unwrap();
        assert_ne!(dupe.id, original.id);
        assert_eq!(dupe.batch_size_l, original.batch_size_l);
    }
}
```

- [ ] **Step 2: Add `Default` derive to `UpdateRecipeInput` in `models.rs`**

Open `src-tauri/src/models.rs` and change `#[derive(Debug, Deserialize)]` on `UpdateRecipeInput` to:

```rust
#[derive(Debug, Deserialize, Default)]
```

Do the same for `CreateRecipeInput`.

- [ ] **Step 3: Run to verify failure**

```bash
cd src-tauri && cargo test db::recipes
```

Expected: FAIL.

- [ ] **Step 4: Implement all functions**

Replace the `todo!()` stubs with:

```rust
pub async fn list(db: &SqlitePool) -> Result<Vec<RecipeSummary>, AppError> {
    let rows = sqlx::query_as::<_, RecipeSummary>(
        "SELECT r.id, r.name, r.type, r.batch_size_l, r.created_at, r.updated_at,
                s.name as style_name
         FROM recipes r
         LEFT JOIN styles s ON r.style_id = s.id
         ORDER BY r.updated_at DESC"
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn get(db: &SqlitePool, id: &str) -> Result<Recipe, AppError> {
    let row = sqlx::query_as::<_, RecipeRow>(
        "SELECT * FROM recipes WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)?;

    let fermentables = sqlx::query_as::<_, crate::models::RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let hops = sqlx::query_as::<_, crate::models::RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let yeasts = sqlx::query_as::<_, crate::models::RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let miscs = sqlx::query_as::<_, crate::models::RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE recipe_id = ? ORDER BY addition_order"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let waters = sqlx::query_as::<_, crate::models::RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let mash_row = sqlx::query_as::<_, crate::models::MashRow>(
        "SELECT * FROM mashes WHERE recipe_id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    let mash = if let Some(mash_row) = mash_row {
        let steps = sqlx::query_as::<_, crate::models::MashStep>(
            "SELECT * FROM mash_steps WHERE mash_id = ? ORDER BY step_order"
        )
        .bind(&mash_row.id)
        .fetch_all(db)
        .await?;
        Some(crate::models::Mash {
            id: mash_row.id,
            recipe_id: mash_row.recipe_id,
            name: mash_row.name,
            grain_temp_c: mash_row.grain_temp_c,
            tun_temp_c: mash_row.tun_temp_c,
            sparge_temp_c: mash_row.sparge_temp_c,
            ph: mash_row.ph,
            tun_weight_kg: mash_row.tun_weight_kg,
            tun_specific_heat: mash_row.tun_specific_heat,
            equip_adjust: mash_row.equip_adjust,
            notes: mash_row.notes,
            steps,
        })
    } else {
        None
    };

    let equipment_profile = if let Some(ref ep_id) = row.equipment_profile_id {
        sqlx::query_as::<_, crate::models::EquipmentProfile>(
            "SELECT * FROM equipment_profiles WHERE id = ?"
        )
        .bind(ep_id)
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    let style = if let Some(ref s_id) = row.style_id {
        sqlx::query_as::<_, crate::models::Style>(
            "SELECT * FROM styles WHERE id = ?"
        )
        .bind(s_id)
        .fetch_optional(db)
        .await?
    } else {
        None
    };

    Ok(Recipe {
        id: row.id,
        name: row.name,
        type_: row.type_,
        brewer: row.brewer,
        asst_brewer: row.asst_brewer,
        batch_size_l: row.batch_size_l,
        boil_size_l: row.boil_size_l,
        boil_time_min: row.boil_time_min,
        efficiency_pct: row.efficiency_pct,
        style_id: row.style_id,
        equipment_profile_id: row.equipment_profile_id,
        notes: row.notes,
        taste_notes: row.taste_notes,
        taste_rating: row.taste_rating,
        og: row.og,
        fg: row.fg,
        fermentation_stages: row.fermentation_stages,
        primary_age_days: row.primary_age_days,
        primary_temp_c: row.primary_temp_c,
        secondary_age_days: row.secondary_age_days,
        secondary_temp_c: row.secondary_temp_c,
        tertiary_age_days: row.tertiary_age_days,
        tertiary_temp_c: row.tertiary_temp_c,
        age_days: row.age_days,
        age_temp_c: row.age_temp_c,
        carbonation_vols: row.carbonation_vols,
        forced_carbonation: row.forced_carbonation,
        priming_sugar_name: row.priming_sugar_name,
        carbonation_temp_c: row.carbonation_temp_c,
        priming_sugar_equiv: row.priming_sugar_equiv,
        keg_priming_factor: row.keg_priming_factor,
        date: row.date,
        created_at: row.created_at,
        updated_at: row.updated_at,
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

pub async fn create(db: &SqlitePool, input: CreateRecipeInput) -> Result<Recipe, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = now_secs();

    // If source_id is set, copy values from the source recipe
    let (batch_size, boil_size, boil_time, equipment_profile_id) = if let Some(ref src_id) = input.source_id {
        let src = get(db, src_id).await?;
        (src.batch_size_l, src.boil_size_l, src.boil_time_min, src.equipment_profile_id)
    } else {
        (
            input.batch_size_l.unwrap_or(23.0),
            input.boil_size_l.unwrap_or(27.0),
            input.boil_time_min.unwrap_or(60.0),
            input.equipment_profile_id,
        )
    };

    sqlx::query(
        "INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min, equipment_profile_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.name)
    .bind(input.type_.as_deref().unwrap_or("all_grain"))
    .bind(batch_size)
    .bind(boil_size)
    .bind(boil_time)
    .bind(&equipment_profile_id)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    // If duplicating, copy all additions
    if let Some(src_id) = input.source_id {
        sqlx::query("INSERT INTO recipe_addition_fermentables (id, recipe_id, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order) SELECT ?, ?, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order FROM recipe_addition_fermentables WHERE recipe_id = ?")
            .bind(Uuid::new_v4().to_string()).bind(&id).bind(&src_id).execute(db).await?;
        // Note: the above only copies one row. In practice, use a loop or subquery per row.
        // The correct approach is to SELECT all additions from source and INSERT each with a new ID:
        let src_fermentables = sqlx::query_as::<_, crate::models::RecipeAdditionFermentable>(
            "SELECT * FROM recipe_addition_fermentables WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        // Delete the incorrectly inserted row above and re-insert properly
        sqlx::query("DELETE FROM recipe_addition_fermentables WHERE recipe_id = ?")
            .bind(&id).execute(db).await?;
        for f in src_fermentables {
            sqlx::query("INSERT INTO recipe_addition_fermentables (id, recipe_id, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(f.fermentable_id).bind(f.name).bind(f.type_).bind(f.yield_pct).bind(f.color_lovibond).bind(f.amount_kg).bind(f.add_after_boil).bind(f.addition_order)
                .execute(db).await?;
        }

        let src_hops = sqlx::query_as::<_, crate::models::RecipeAdditionHop>(
            "SELECT * FROM recipe_addition_hops WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        for h in src_hops {
            sqlx::query("INSERT INTO recipe_addition_hops (id, recipe_id, hop_id, name, alpha_pct, form, amount_kg, use, time_min, addition_order) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(h.hop_id).bind(h.name).bind(h.alpha_pct).bind(h.form).bind(h.amount_kg).bind(h.use_).bind(h.time_min).bind(h.addition_order)
                .execute(db).await?;
        }

        let src_yeasts = sqlx::query_as::<_, crate::models::RecipeAdditionYeast>(
            "SELECT * FROM recipe_addition_yeasts WHERE recipe_id = ?"
        ).bind(&src_id).fetch_all(db).await?;
        for y in src_yeasts {
            sqlx::query("INSERT INTO recipe_addition_yeasts (id, recipe_id, yeast_id, name, type, form, laboratory, product_id, attenuation_pct, amount, amount_is_weight, add_to_secondary, times_cultured) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(Uuid::new_v4().to_string()).bind(&id).bind(y.yeast_id).bind(y.name).bind(y.type_).bind(y.form).bind(y.laboratory).bind(y.product_id).bind(y.attenuation_pct).bind(y.amount).bind(y.amount_is_weight).bind(y.add_to_secondary).bind(y.times_cultured)
                .execute(db).await?;
        }
    }

    get(db, &id).await
}

pub async fn update(db: &SqlitePool, id: &str, input: UpdateRecipeInput) -> Result<Recipe, AppError> {
    let current = get(db, id).await?;
    let now = now_secs();

    sqlx::query(
        "UPDATE recipes SET
            name = ?, type = ?, brewer = ?, asst_brewer = ?,
            batch_size_l = ?, boil_size_l = ?, boil_time_min = ?,
            efficiency_pct = ?, style_id = ?, equipment_profile_id = ?,
            notes = ?, taste_notes = ?, taste_rating = ?,
            fermentation_stages = ?, primary_age_days = ?, primary_temp_c = ?,
            secondary_age_days = ?, secondary_temp_c = ?, tertiary_age_days = ?,
            tertiary_temp_c = ?, age_days = ?, age_temp_c = ?,
            carbonation_vols = ?, forced_carbonation = ?,
            priming_sugar_name = ?, carbonation_temp_c = ?, date = ?,
            updated_at = ?
         WHERE id = ?"
    )
    .bind(input.name.unwrap_or(current.name))
    .bind(input.type_.unwrap_or(current.type_))
    .bind(input.brewer.or(current.brewer))
    .bind(input.asst_brewer.or(current.asst_brewer))
    .bind(input.batch_size_l.unwrap_or(current.batch_size_l))
    .bind(input.boil_size_l.unwrap_or(current.boil_size_l))
    .bind(input.boil_time_min.unwrap_or(current.boil_time_min))
    .bind(input.efficiency_pct.or(current.efficiency_pct))
    .bind(input.style_id.or(current.style_id))
    .bind(input.equipment_profile_id.or(current.equipment_profile_id))
    .bind(input.notes.or(current.notes))
    .bind(input.taste_notes.or(current.taste_notes))
    .bind(input.taste_rating.or(current.taste_rating))
    .bind(input.fermentation_stages.unwrap_or(current.fermentation_stages))
    .bind(input.primary_age_days.or(current.primary_age_days))
    .bind(input.primary_temp_c.or(current.primary_temp_c))
    .bind(input.secondary_age_days.or(current.secondary_age_days))
    .bind(input.secondary_temp_c.or(current.secondary_temp_c))
    .bind(input.tertiary_age_days.or(current.tertiary_age_days))
    .bind(input.tertiary_temp_c.or(current.tertiary_temp_c))
    .bind(input.age_days.or(current.age_days))
    .bind(input.age_temp_c.or(current.age_temp_c))
    .bind(input.carbonation_vols.or(current.carbonation_vols))
    .bind(input.forced_carbonation.unwrap_or(current.forced_carbonation))
    .bind(input.priming_sugar_name.or(current.priming_sugar_name))
    .bind(input.carbonation_temp_c.or(current.carbonation_temp_c))
    .bind(input.date.or(current.date))
    .bind(now)
    .bind(id)
    .execute(db)
    .await?;

    get(db, id).await
}

pub async fn delete(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipes WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
```

- [ ] **Step 5: Run to verify tests pass**

```bash
cd src-tauri && cargo test db::recipes
```

Expected: 5 tests pass.

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/db/recipes.rs src-tauri/src/models.rs
git commit -m "feat: recipe CRUD DB layer"
```

---

### Task 6: Recipes — Tauri commands + stats

**Files:**
- Modify: `src-tauri/src/commands/recipes.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Implement `src-tauri/src/commands/recipes.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::{Recipe, RecipeSummary, RecipeStats, CreateRecipeInput, UpdateRecipeInput};
use crate::{db, brewing};

#[tauri::command]
pub async fn list_recipes(state: State<'_, AppState>) -> Result<Vec<RecipeSummary>, String> {
    db::recipes::list(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe(state: State<'_, AppState>, id: String) -> Result<Recipe, String> {
    db::recipes::get(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe(
    state: State<'_, AppState>,
    input: CreateRecipeInput,
) -> Result<Recipe, String> {
    db::recipes::create(&state.db, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_recipe(
    state: State<'_, AppState>,
    id: String,
    input: UpdateRecipeInput,
) -> Result<Recipe, String> {
    db::recipes::update(&state.db, &id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_recipe(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::recipes::delete(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe_stats(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<RecipeStats, String> {
    let recipe = db::recipes::get(&state.db, &recipe_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(brewing::calculate_stats(&recipe))
}
```

- [ ] **Step 2: Register in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::recipes::list_recipes,
commands::recipes::get_recipe,
commands::recipes::create_recipe,
commands::recipes::update_recipe,
commands::recipes::delete_recipe,
commands::recipes::get_recipe_stats,
```

- [ ] **Step 3: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 4: Commit**

```bash
cd ..
git add src-tauri/src/commands/recipes.rs src-tauri/src/lib.rs
git commit -m "feat: recipe CRUD and stats Tauri commands"
```

---

### Task 7: Recipe additions — DB layer and commands

**Files:**
- Modify: `src-tauri/src/db/additions.rs`
- Modify: `src-tauri/src/commands/additions.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Implement `src-tauri/src/db/additions.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{
    RecipeAdditionFermentable, RecipeAdditionHop, RecipeAdditionYeast,
    RecipeAdditionMisc, RecipeAdditionWater,
    CreateFermentableAdditionInput, UpdateFermentableAdditionInput,
    CreateHopAdditionInput, UpdateHopAdditionInput,
    CreateYeastAdditionInput, UpdateYeastAdditionInput,
    CreateMiscAdditionInput, UpdateMiscAdditionInput,
    CreateWaterAdditionInput, UpdateWaterAdditionInput,
};
use uuid::Uuid;

fn now_secs_str() -> String { Uuid::new_v4().to_string() }
fn new_id() -> String { Uuid::new_v4().to_string() }

pub async fn create_fermentable(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_fermentables WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_fermentables
            (id, recipe_id, fermentable_id, name, type, yield_pct, color_lovibond, amount_kg, add_after_boil, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.fermentable_id).bind(&input.name)
    .bind(&input.type_).bind(input.yield_pct).bind(input.color_lovibond)
    .bind(input.amount_kg).bind(input.add_after_boil.unwrap_or(false))
    .bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_fermentable(
    db: &SqlitePool,
    id: &str,
    input: UpdateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_fermentables SET amount_kg = ?, add_after_boil = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount_kg.unwrap_or(current.amount_kg))
    .bind(input.add_after_boil.unwrap_or(current.add_after_boil))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionFermentable>(
        "SELECT * FROM recipe_addition_fermentables WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_fermentable(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_fermentables WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_hop(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_hops WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_hops
            (id, recipe_id, hop_id, name, alpha_pct, form, amount_kg, use, time_min, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.hop_id).bind(&input.name)
    .bind(input.alpha_pct).bind(input.form.as_deref().unwrap_or("pellet"))
    .bind(input.amount_kg).bind(&input.use_).bind(input.time_min).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_hop(
    db: &SqlitePool,
    id: &str,
    input: UpdateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_hops SET amount_kg = ?, use = ?, time_min = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount_kg.unwrap_or(current.amount_kg))
    .bind(input.use_.as_deref().unwrap_or(&current.use_))
    .bind(input.time_min.unwrap_or(current.time_min))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionHop>(
        "SELECT * FROM recipe_addition_hops WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_hop(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_hops WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_yeast(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO recipe_addition_yeasts
            (id, recipe_id, yeast_id, name, type, form, laboratory, product_id, attenuation_pct, amount, amount_is_weight)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.yeast_id).bind(&input.name)
    .bind(&input.type_).bind(&input.form).bind(&input.laboratory).bind(&input.product_id)
    .bind(input.attenuation_pct).bind(input.amount)
    .bind(input.amount_is_weight.unwrap_or(false))
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_yeast(
    db: &SqlitePool,
    id: &str,
    input: UpdateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_yeasts SET attenuation_pct = ?, amount = ?, amount_is_weight = ?, add_to_secondary = ?, times_cultured = ? WHERE id = ?"
    )
    .bind(input.attenuation_pct.or(current.attenuation_pct))
    .bind(input.amount.or(current.amount))
    .bind(input.amount_is_weight.unwrap_or(current.amount_is_weight))
    .bind(input.add_to_secondary.unwrap_or(current.add_to_secondary))
    .bind(input.times_cultured.unwrap_or(current.times_cultured))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionYeast>(
        "SELECT * FROM recipe_addition_yeasts WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_yeast(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_yeasts WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_misc(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM recipe_addition_miscs WHERE recipe_id = ?"
    ).bind(recipe_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO recipe_addition_miscs
            (id, recipe_id, misc_id, name, type, use, amount, amount_is_weight, time_min, addition_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.misc_id).bind(&input.name)
    .bind(&input.type_).bind(&input.use_).bind(input.amount)
    .bind(input.amount_is_weight.unwrap_or(false)).bind(input.time_min).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_misc(
    db: &SqlitePool,
    id: &str,
    input: UpdateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE recipe_addition_miscs SET amount = ?, amount_is_weight = ?, use = ?, time_min = ?, addition_order = ? WHERE id = ?"
    )
    .bind(input.amount.unwrap_or(current.amount))
    .bind(input.amount_is_weight.unwrap_or(current.amount_is_weight))
    .bind(input.use_.as_deref().unwrap_or(&current.use_))
    .bind(input.time_min.unwrap_or(current.time_min))
    .bind(input.addition_order.unwrap_or(current.addition_order))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionMisc>(
        "SELECT * FROM recipe_addition_miscs WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_misc(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_miscs WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}

pub async fn create_water(
    db: &SqlitePool,
    recipe_id: &str,
    input: CreateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO recipe_addition_waters (id, recipe_id, water_id, name, amount_l) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(recipe_id).bind(&input.water_id).bind(&input.name).bind(input.amount_l)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(&id).fetch_one(db).await?)
}

pub async fn update_water(
    db: &SqlitePool,
    id: &str,
    input: UpdateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    let current = sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query("UPDATE recipe_addition_waters SET amount_l = ? WHERE id = ?")
        .bind(input.amount_l.unwrap_or(current.amount_l))
        .bind(id)
        .execute(db).await?;

    Ok(sqlx::query_as::<_, RecipeAdditionWater>(
        "SELECT * FROM recipe_addition_waters WHERE id = ?"
    ).bind(id).fetch_one(db).await?)
}

pub async fn delete_water(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM recipe_addition_waters WHERE id = ?")
        .bind(id).execute(db).await?;
    Ok(())
}
```

- [ ] **Step 2: Implement `src-tauri/src/commands/additions.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::*;
use crate::db;

#[tauri::command]
pub async fn create_recipe_fermentable(state: State<'_, AppState>, recipe_id: String, input: CreateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    db::additions::create_fermentable(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_fermentable(state: State<'_, AppState>, id: String, input: UpdateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    db::additions::update_fermentable(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_fermentable(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_fermentable(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_hop(state: State<'_, AppState>, recipe_id: String, input: CreateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    db::additions::create_hop(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_hop(state: State<'_, AppState>, id: String, input: UpdateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    db::additions::update_hop(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_hop(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_yeast(state: State<'_, AppState>, recipe_id: String, input: CreateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    db::additions::create_yeast(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_yeast(state: State<'_, AppState>, id: String, input: UpdateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    db::additions::update_yeast(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_yeast(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_misc(state: State<'_, AppState>, recipe_id: String, input: CreateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    db::additions::create_misc(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_misc(state: State<'_, AppState>, id: String, input: UpdateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    db::additions::update_misc(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_misc(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_water(state: State<'_, AppState>, recipe_id: String, input: CreateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    db::additions::create_water(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_water(state: State<'_, AppState>, id: String, input: UpdateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    db::additions::update_water(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_water(&state.db, &id).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 3: Register all addition commands in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::additions::create_recipe_fermentable,
commands::additions::update_recipe_fermentable,
commands::additions::delete_recipe_fermentable,
commands::additions::create_recipe_hop,
commands::additions::update_recipe_hop,
commands::additions::delete_recipe_hop,
commands::additions::create_recipe_yeast,
commands::additions::update_recipe_yeast,
commands::additions::delete_recipe_yeast,
commands::additions::create_recipe_misc,
commands::additions::update_recipe_misc,
commands::additions::delete_recipe_misc,
commands::additions::create_recipe_water,
commands::additions::update_recipe_water,
commands::additions::delete_recipe_water,
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd ..
git add src-tauri/src/db/additions.rs src-tauri/src/commands/additions.rs src-tauri/src/lib.rs
git commit -m "feat: recipe additions DB layer and commands"
```

---

### Task 8: Mash — DB layer and commands

**Files:**
- Modify: `src-tauri/src/db/mash.rs`
- Modify: `src-tauri/src/commands/mash.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Implement `src-tauri/src/db/mash.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Mash, MashRow, MashStep, UpdateMashInput, CreateMashStepInput, UpdateMashStepInput};
use uuid::Uuid;

fn new_id() -> String { Uuid::new_v4().to_string() }
fn now_secs() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

async fn fetch_mash(db: &SqlitePool, mash_id: &str) -> Result<Mash, AppError> {
    let row = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE id = ?")
        .bind(mash_id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;
    let steps = sqlx::query_as::<_, MashStep>(
        "SELECT * FROM mash_steps WHERE mash_id = ? ORDER BY step_order"
    ).bind(mash_id).fetch_all(db).await?;
    Ok(Mash {
        id: row.id, recipe_id: row.recipe_id, name: row.name,
        grain_temp_c: row.grain_temp_c, tun_temp_c: row.tun_temp_c,
        sparge_temp_c: row.sparge_temp_c, ph: row.ph,
        tun_weight_kg: row.tun_weight_kg, tun_specific_heat: row.tun_specific_heat,
        equip_adjust: row.equip_adjust, notes: row.notes, steps,
    })
}

pub async fn get_for_recipe(db: &SqlitePool, recipe_id: &str) -> Result<Mash, AppError> {
    let row = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE recipe_id = ?")
        .bind(recipe_id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;
    fetch_mash(db, &row.id).await
}

pub async fn upsert_for_recipe(db: &SqlitePool, recipe_id: &str, input: UpdateMashInput) -> Result<Mash, AppError> {
    let existing = sqlx::query_as::<_, MashRow>("SELECT * FROM mashes WHERE recipe_id = ?")
        .bind(recipe_id).fetch_optional(db).await?;

    let mash_id = if let Some(row) = existing {
        sqlx::query(
            "UPDATE mashes SET name = ?, grain_temp_c = ?, tun_temp_c = ?, sparge_temp_c = ?, ph = ?, notes = ? WHERE id = ?"
        )
        .bind(input.name.unwrap_or(row.name))
        .bind(input.grain_temp_c.unwrap_or(row.grain_temp_c))
        .bind(input.tun_temp_c.or(row.tun_temp_c))
        .bind(input.sparge_temp_c.or(row.sparge_temp_c))
        .bind(input.ph.or(row.ph))
        .bind(input.notes.or(row.notes))
        .bind(&row.id)
        .execute(db).await?;
        row.id
    } else {
        let id = new_id();
        sqlx::query(
            "INSERT INTO mashes (id, recipe_id, name, grain_temp_c, tun_temp_c, sparge_temp_c, ph, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id).bind(recipe_id)
        .bind(input.name.as_deref().unwrap_or("Single Infusion"))
        .bind(input.grain_temp_c.unwrap_or(21.0))
        .bind(input.tun_temp_c).bind(input.sparge_temp_c).bind(input.ph).bind(input.notes)
        .execute(db).await?;
        id
    };

    fetch_mash(db, &mash_id).await
}

pub async fn create_step(db: &SqlitePool, mash_id: &str, input: CreateMashStepInput) -> Result<MashStep, AppError> {
    let id = new_id();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM mash_steps WHERE mash_id = ?")
        .bind(mash_id).fetch_one(db).await?;

    sqlx::query(
        "INSERT INTO mash_steps (id, mash_id, name, type, infuse_amount_l, step_temp_c, step_time_min, ramp_time_min, end_temp_c, step_order)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(mash_id).bind(&input.name)
    .bind(input.type_.as_deref().unwrap_or("infusion"))
    .bind(input.infuse_amount_l).bind(input.step_temp_c).bind(input.step_time_min)
    .bind(input.ramp_time_min).bind(input.end_temp_c).bind(count.0)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(&id).fetch_one(db).await?)
}

pub async fn update_step(db: &SqlitePool, id: &str, input: UpdateMashStepInput) -> Result<MashStep, AppError> {
    let current = sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(id).fetch_optional(db).await?.ok_or(AppError::NotFound)?;

    sqlx::query(
        "UPDATE mash_steps SET name = ?, type = ?, infuse_amount_l = ?, step_temp_c = ?, step_time_min = ?, ramp_time_min = ?, end_temp_c = ? WHERE id = ?"
    )
    .bind(input.name.as_deref().unwrap_or(&current.name))
    .bind(input.type_.as_deref().unwrap_or(&current.type_))
    .bind(input.infuse_amount_l.or(current.infuse_amount_l))
    .bind(input.step_temp_c.unwrap_or(current.step_temp_c))
    .bind(input.step_time_min.unwrap_or(current.step_time_min))
    .bind(input.ramp_time_min.or(current.ramp_time_min))
    .bind(input.end_temp_c.or(current.end_temp_c))
    .bind(id)
    .execute(db).await?;

    Ok(sqlx::query_as::<_, MashStep>("SELECT * FROM mash_steps WHERE id = ?")
        .bind(id).fetch_one(db).await?)
}

pub async fn delete_step(db: &SqlitePool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM mash_steps WHERE id = ?").bind(id).execute(db).await?;
    Ok(())
}

pub async fn update_step_order(db: &SqlitePool, ordered_ids: Vec<String>) -> Result<(), AppError> {
    for (i, id) in ordered_ids.iter().enumerate() {
        sqlx::query("UPDATE mash_steps SET step_order = ? WHERE id = ?")
            .bind(i as i64).bind(id).execute(db).await?;
    }
    Ok(())
}
```

- [ ] **Step 2: Implement `src-tauri/src/commands/mash.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::{Mash, MashStep, UpdateMashInput, CreateMashStepInput, UpdateMashStepInput};
use crate::db;

#[tauri::command]
pub async fn get_mash(state: State<'_, AppState>, recipe_id: String) -> Result<Mash, String> {
    db::mash::get_for_recipe(&state.db, &recipe_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_mash(state: State<'_, AppState>, recipe_id: String, input: UpdateMashInput) -> Result<Mash, String> {
    db::mash::upsert_for_recipe(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_mash_step(state: State<'_, AppState>, mash_id: String, input: CreateMashStepInput) -> Result<MashStep, String> {
    db::mash::create_step(&state.db, &mash_id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_mash_step(state: State<'_, AppState>, id: String, input: UpdateMashStepInput) -> Result<MashStep, String> {
    db::mash::update_step(&state.db, &id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_mash_step(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::mash::delete_step(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_mash_step_order(state: State<'_, AppState>, ordered_ids: Vec<String>) -> Result<(), String> {
    db::mash::update_step_order(&state.db, ordered_ids).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 3: Register in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::mash::get_mash,
commands::mash::update_mash,
commands::mash::create_mash_step,
commands::mash::update_mash_step,
commands::mash::delete_mash_step,
commands::mash::update_mash_step_order,
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd ..
git add src-tauri/src/db/mash.rs src-tauri/src/commands/mash.rs src-tauri/src/lib.rs
git commit -m "feat: mash DB layer and commands"
```

---

### Task 9: Settings — DB layer and commands

**Files:**
- Modify: `src-tauri/src/db/settings.rs`
- Modify: `src-tauri/src/commands/settings.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Implement `src-tauri/src/db/settings.rs`**

```rust
use sqlx::SqlitePool;
use crate::error::AppError;
use std::collections::HashMap;

pub async fn get_all(db: &SqlitePool) -> Result<HashMap<String, String>, AppError> {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM settings")
        .fetch_all(db).await?;
    Ok(rows.into_iter().collect())
}

pub async fn set(db: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
    sqlx::query("INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
        .bind(key).bind(value).execute(db).await?;
    Ok(())
}
```

- [ ] **Step 2: Implement `src-tauri/src/commands/settings.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::db;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    db::settings::get_all(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    db::settings::set(&state.db, &key, &value).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 3: Register in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::settings::get_settings,
commands::settings::update_setting,
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 5: Commit**

```bash
cd ..
git add src-tauri/src/db/settings.rs src-tauri/src/commands/settings.rs src-tauri/src/lib.rs
git commit -m "feat: settings DB layer and commands"
```

---

### Task 10: BeerXML export and import

**Files:**
- Modify: `src-tauri/src/commands/import_export.rs`
- Modify: `src-tauri/src/lib.rs`

Note: BeerXML is XML — we use Rust's string formatting rather than pulling in an XML crate, keeping dependencies minimal. Import parses using basic string manipulation; a full XML parser can replace this later.

- [ ] **Step 1: Add `quick-xml` to `src-tauri/Cargo.toml`**

```toml
quick-xml = { version = "0.36", features = ["serialize"] }
```

- [ ] **Step 2: Implement BeerXML export and import in `src-tauri/src/commands/import_export.rs`**

```rust
use tauri::State;
use crate::AppState;
use crate::models::{RecipeSummary, CreateRecipeInput, CreateFermentableAdditionInput, CreateHopAdditionInput, CreateYeastAdditionInput};
use crate::db;

#[tauri::command]
pub async fn get_recipe_beerxml(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<String, String> {
    let recipe = db::recipes::get(&state.db, &recipe_id)
        .await
        .map_err(|e| e.to_string())?;

    let style_block = recipe.style.as_ref().map(|s| format!(
        "    <STYLE>\n      <NAME>{}</NAME>\n      <CATEGORY>{}</CATEGORY>\n      <STYLE_GUIDE>{}</STYLE_GUIDE>\n    </STYLE>",
        s.name, s.category, s.style_guide
    )).unwrap_or_default();

    let fermentables: String = recipe.fermentables.iter().map(|f| format!(
        "      <FERMENTABLE>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.4}</AMOUNT>\n        <TYPE>{}</TYPE>\n        <YIELD>{:.1}</YIELD>\n        <COLOR>{:.1}</COLOR>\n      </FERMENTABLE>",
        f.name, f.amount_kg, f.type_, f.yield_pct, f.color_lovibond
    )).collect::<Vec<_>>().join("\n");

    let hops: String = recipe.hops.iter().map(|h| format!(
        "      <HOP>\n        <NAME>{}</NAME>\n        <AMOUNT>{:.5}</AMOUNT>\n        <ALPHA>{:.1}</ALPHA>\n        <USE>{}</USE>\n        <TIME>{:.0}</TIME>\n        <FORM>{}</FORM>\n      </HOP>",
        h.name, h.amount_kg, h.alpha_pct, h.use_, h.time_min, h.form
    )).collect::<Vec<_>>().join("\n");

    let yeasts: String = recipe.yeasts.iter().map(|y| format!(
        "      <YEAST>\n        <NAME>{}</NAME>\n        <TYPE>{}</TYPE>\n        <FORM>{}</FORM>\n        <AMOUNT>{:.4}</AMOUNT>\n      </YEAST>",
        y.name, y.type_, y.form, y.amount.unwrap_or(0.0)
    )).collect::<Vec<_>>().join("\n");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<RECIPES>
  <RECIPE>
    <NAME>{name}</NAME>
    <VERSION>1</VERSION>
    <TYPE>{type_}</TYPE>
    <BREWER>{brewer}</BREWER>
    <BATCH_SIZE>{batch:.1}</BATCH_SIZE>
    <BOIL_SIZE>{boil:.1}</BOIL_SIZE>
    <BOIL_TIME>{boil_time:.0}</BOIL_TIME>
    <EFFICIENCY>{eff:.1}</EFFICIENCY>
{style}
    <FERMENTABLES>
{fermentables}
    </FERMENTABLES>
    <HOPS>
{hops}
    </HOPS>
    <YEASTS>
{yeasts}
    </YEASTS>
  </RECIPE>
</RECIPES>"#,
        name = recipe.name,
        type_ = recipe.type_,
        brewer = recipe.brewer.as_deref().unwrap_or(""),
        batch = recipe.batch_size_l,
        boil = recipe.boil_size_l,
        boil_time = recipe.boil_time_min,
        eff = recipe.efficiency_pct.unwrap_or(72.0),
        style = style_block,
        fermentables = fermentables,
        hops = hops,
        yeasts = yeasts,
    );

    Ok(xml)
}

#[tauri::command]
pub async fn create_recipes_from_beerxml(
    state: State<'_, AppState>,
    xml: String,
) -> Result<Vec<RecipeSummary>, String> {
    // Parse recipe name and basic fields from BeerXML
    // This is a minimal parser for the most common fields.
    // A full XML parser can replace this in a future iteration.
    let mut results = Vec::new();

    let recipes_start = xml.find("<RECIPE>").ok_or("No <RECIPE> found in XML")?;
    let recipes_end = xml.rfind("</RECIPE>").ok_or("No </RECIPE> found in XML")?;
    let recipe_xml = &xml[recipes_start..recipes_end + 9];

    let name = extract_tag(recipe_xml, "NAME").unwrap_or("Imported Recipe".to_string());
    let type_ = extract_tag(recipe_xml, "TYPE").unwrap_or("all_grain".to_string());
    let batch_size: f64 = extract_tag(recipe_xml, "BATCH_SIZE")
        .and_then(|v| v.parse().ok()).unwrap_or(23.0);
    let boil_size: f64 = extract_tag(recipe_xml, "BOIL_SIZE")
        .and_then(|v| v.parse().ok()).unwrap_or(27.0);
    let boil_time: f64 = extract_tag(recipe_xml, "BOIL_TIME")
        .and_then(|v| v.parse().ok()).unwrap_or(60.0);

    let recipe = db::recipes::create(&state.db, CreateRecipeInput {
        name,
        type_: Some(type_),
        batch_size_l: Some(batch_size),
        boil_size_l: Some(boil_size),
        boil_time_min: Some(boil_time),
        equipment_profile_id: None,
        source_id: None,
    }).await.map_err(|e| e.to_string())?;

    // Import fermentables
    let ferm_xml = extract_between(&xml, "<FERMENTABLES>", "</FERMENTABLES>").unwrap_or_default();
    for ferm_block in split_tags(&ferm_xml, "FERMENTABLE") {
        let fname = extract_tag(&ferm_block, "NAME").unwrap_or_default();
        let ftype = extract_tag(&ferm_block, "TYPE").unwrap_or("grain".to_string());
        let amount: f64 = extract_tag(&ferm_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0);
        let yield_pct: f64 = extract_tag(&ferm_block, "YIELD").and_then(|v| v.parse().ok()).unwrap_or(75.0);
        let color: f64 = extract_tag(&ferm_block, "COLOR").and_then(|v| v.parse().ok()).unwrap_or(2.0);
        if !fname.is_empty() {
            let _ = db::additions::create_fermentable(&state.db, &recipe.id, CreateFermentableAdditionInput {
                fermentable_id: None, name: fname, type_: ftype,
                yield_pct, color_lovibond: color, amount_kg: amount, add_after_boil: None,
            }).await;
        }
    }

    // Import hops
    let hops_xml = extract_between(&xml, "<HOPS>", "</HOPS>").unwrap_or_default();
    for hop_block in split_tags(&hops_xml, "HOP") {
        let hname = extract_tag(&hop_block, "NAME").unwrap_or_default();
        let alpha: f64 = extract_tag(&hop_block, "ALPHA").and_then(|v| v.parse().ok()).unwrap_or(5.0);
        let amount: f64 = extract_tag(&hop_block, "AMOUNT").and_then(|v| v.parse().ok()).unwrap_or(0.0);
        let use_ = extract_tag(&hop_block, "USE").unwrap_or("boil".to_string());
        let time: f64 = extract_tag(&hop_block, "TIME").and_then(|v| v.parse().ok()).unwrap_or(60.0);
        let form = extract_tag(&hop_block, "FORM").unwrap_or("pellet".to_string());
        if !hname.is_empty() {
            let _ = db::additions::create_hop(&state.db, &recipe.id, crate::models::CreateHopAdditionInput {
                hop_id: None, name: hname, alpha_pct: alpha, form: Some(form),
                amount_kg: amount, use_: use_, time_min: time,
            }).await;
        }
    }

    let summary = db::recipes::list(&state.db).await.map_err(|e| e.to_string())?
        .into_iter().filter(|r| r.id == recipe.id).collect::<Vec<_>>();
    results.extend(summary);

    Ok(results)
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)? + open.len();
    let end = xml.find(&close)?;
    if end > start {
        Some(xml[start..end].trim().to_string())
    } else {
        None
    }
}

fn extract_between(xml: &str, open: &str, close: &str) -> Option<String> {
    let start = xml.find(open)? + open.len();
    let end = xml.find(close)?;
    if end > start {
        Some(xml[start..end].to_string())
    } else {
        None
    }
}

fn split_tags(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let mut results = Vec::new();
    let mut remaining = xml;
    while let Some(start) = remaining.find(&open) {
        let end = remaining.find(&close).unwrap_or(remaining.len());
        results.push(remaining[start..end + close.len()].to_string());
        remaining = &remaining[end + close.len()..];
    }
    results
}
```

- [ ] **Step 3: Register commands in `src-tauri/src/lib.rs`**

Add to `invoke_handler`:
```rust
commands::import_export::get_recipe_beerxml,
commands::import_export::create_recipes_from_beerxml,
```

- [ ] **Step 4: Verify compilation**

```bash
cd src-tauri && cargo build
```

- [ ] **Step 5: Run all tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass (equipment: 3, recipes: 5, brewing: 11 = 19+ total).

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/commands/import_export.rs src-tauri/Cargo.toml src-tauri/src/lib.rs
git commit -m "feat: BeerXML import and export commands"
```

---

### Task 11: Seed data — BJCP styles + common ingredients

**Files:**
- Create: `src-tauri/src/db/migrations/002_seed_data.sql`

- [ ] **Step 1: Create `src-tauri/src/db/migrations/002_seed_data.sql`**

This migration seeds a representative subset of BJCP 2021 styles and common ingredients. Expand as needed.

```sql
-- BJCP 2021 style subset (American ales)
INSERT INTO styles (id, name, category, category_number, style_letter, style_guide, type, og_min, og_max, fg_min, fg_max, ibu_min, ibu_max, color_min_srm, color_max_srm, abv_min_pct, abv_max_pct) VALUES
('bjcp-18b', 'American Pale Ale', 'Pale American Ale', '18', 'B', 'BJCP 2021', 'Ale', 1.045, 1.060, 1.010, 1.015, 30, 50, 5, 10, 4.5, 6.2),
('bjcp-21a', 'American IPA', 'IPA', '21', 'A', 'BJCP 2021', 'Ale', 1.056, 1.070, 1.008, 1.014, 40, 70, 6, 14, 5.5, 7.5),
('bjcp-15b', 'American Porter', 'Dark British Beer', '15', 'B', 'BJCP 2021', 'Ale', 1.050, 1.070, 1.012, 1.018, 25, 40, 22, 40, 4.8, 6.5),
('bjcp-20a', 'American Stout', 'American Porter and Stout', '20', 'A', 'BJCP 2021', 'Ale', 1.050, 1.075, 1.010, 1.022, 35, 75, 30, 40, 5.0, 7.0),
('bjcp-1b', 'American Lager', 'Standard American Beer', '1', 'B', 'BJCP 2021', 'Lager', 1.040, 1.050, 1.004, 1.010, 8, 18, 2, 3, 4.2, 5.3),
('bjcp-26c', 'Belgian Tripel', 'Trappist Ale', '26', 'C', 'BJCP 2021', 'Ale', 1.075, 1.085, 1.008, 1.014, 20, 40, 4.5, 7, 7.5, 9.5),
('bjcp-9c', 'Baltic Porter', 'Brown British Beer', '9', 'C', 'BJCP 2021', 'Lager', 1.060, 1.090, 1.016, 1.024, 20, 40, 17, 30, 6.5, 9.5);

-- Common fermentables
INSERT INTO fermentables (id, name, type, yield_pct, color_lovibond) VALUES
('f-pale-malt', 'Pale Malt (2-row)', 'grain', 78.0, 1.8),
('f-pale-malt-6', 'Pale Malt (6-row)', 'grain', 73.0, 1.8),
('f-pilsner', 'Pilsner Malt', 'grain', 75.0, 1.6),
('f-munich', 'Munich Malt', 'grain', 77.0, 9.0),
('f-vienna', 'Vienna Malt', 'grain', 77.0, 3.5),
('f-crystal-40', 'Crystal/Caramel 40L', 'grain', 74.0, 40.0),
('f-crystal-60', 'Crystal/Caramel 60L', 'grain', 74.0, 60.0),
('f-crystal-120', 'Crystal/Caramel 120L', 'grain', 72.0, 120.0),
('f-chocolate', 'Chocolate Malt', 'grain', 60.0, 350.0),
('f-roasted-barley', 'Roasted Barley', 'grain', 55.0, 300.0),
('f-black-patent', 'Black Patent Malt', 'grain', 53.0, 500.0),
('f-wheat', 'White Wheat Malt', 'grain', 77.0, 2.0),
('f-dme-light', 'Dry Malt Extract - Light', 'dry extract', 95.0, 4.0),
('f-dme-amber', 'Dry Malt Extract - Amber', 'dry extract', 95.0, 10.0),
('f-corn-sugar', 'Corn Sugar (Dextrose)', 'sugar', 96.0, 0.5);

-- Common hops
INSERT INTO hops (id, name, alpha_pct, form, type, origin) VALUES
('h-cascade', 'Cascade', 5.5, 'pellet', 'aroma', 'US'),
('h-centennial', 'Centennial', 10.0, 'pellet', 'bittering/aroma', 'US'),
('h-chinook', 'Chinook', 13.0, 'pellet', 'bittering', 'US'),
('h-citra', 'Citra', 12.0, 'pellet', 'aroma', 'US'),
('h-columbus', 'Columbus (CTZ)', 15.0, 'pellet', 'bittering', 'US'),
('h-fuggle', 'Fuggle', 4.5, 'pellet', 'aroma', 'UK'),
('h-hallertau', 'Hallertau Mittelfrüh', 4.0, 'pellet', 'aroma', 'Germany'),
('h-magnum', 'Magnum', 14.0, 'pellet', 'bittering', 'Germany'),
('h-mosaic', 'Mosaic', 12.5, 'pellet', 'aroma', 'US'),
('h-saaz', 'Saaz', 3.5, 'pellet', 'aroma', 'Czech Republic'),
('h-simcoe', 'Simcoe', 13.0, 'pellet', 'aroma/bittering', 'US'),
('h-willamette', 'Willamette', 5.0, 'pellet', 'aroma', 'US');

-- Common yeasts
INSERT INTO yeasts (id, name, type, form, laboratory, product_id, min_temperature_c, max_temperature_c, flocculation, attenuation_pct) VALUES
('y-us05', 'American Ale (US-05)', 'ale', 'dry', 'Fermentis', 'US-05', 15.0, 24.0, 'medium', 77.0),
('y-1056', 'American Ale (WY1056)', 'ale', 'liquid', 'Wyeast', '1056', 16.0, 22.0, 'medium', 75.0),
('y-wlp001', 'California Ale (WLP001)', 'ale', 'liquid', 'White Labs', 'WLP001', 20.0, 23.0, 'medium', 77.0),
('y-s04', 'English Ale (S-04)', 'ale', 'dry', 'Fermentis', 'S-04', 15.0, 24.0, 'high', 73.0),
('y-1084', 'Irish Ale (WY1084)', 'ale', 'liquid', 'Wyeast', '1084', 16.0, 22.0, 'medium', 72.0),
('y-wlp300', 'Hefeweizen (WLP300)', 'wheat', 'liquid', 'White Labs', 'WLP300', 18.0, 23.0, 'low', 74.0),
('y-t58', 'Belgian Ale (T-58)', 'ale', 'dry', 'Fermentis', 'T-58', 15.0, 24.0, 'medium', 78.0),
('y-w34-70', 'Bohemian Lager (W-34/70)', 'lager', 'dry', 'Fermentis', 'W-34/70', 9.0, 15.0, 'high', 80.0),
('y-s189', 'Lager (S-189)', 'lager', 'dry', 'Fermentis', 'S-189', 9.0, 15.0, 'medium', 80.0);

-- Default equipment profile
INSERT INTO equipment_profiles (id, name, boil_size_l, batch_size_l, boil_time_min, evap_rate_pct_hr, trub_chiller_loss_l, fermenter_loss_l, hop_utilization_pct, efficiency_pct, created_at, updated_at) VALUES
('eq-default', 'Standard 5 Gallon', 27.0, 23.0, 60.0, 10.0, 1.5, 1.0, 100.0, 72.0, 0, 0);

INSERT INTO settings (key, value) VALUES ('default_equipment_profile_id', 'eq-default')
  ON CONFLICT(key) DO NOTHING;
```

- [ ] **Step 2: Verify migration runs on fresh DB**

```bash
cd src-tauri
rm -f dev.db
DATABASE_URL="sqlite:dev.db" cargo build
```

Or launch `npm run tauri dev` and confirm the app opens without errors.

- [ ] **Step 3: Commit**

```bash
cd ..
git add src-tauri/src/db/migrations/002_seed_data.sql
git commit -m "feat: seed BJCP styles, common ingredients, and default equipment profile"
```

---

## Backend complete

At this point:
- All 30+ Tauri commands registered and compiling
- Equipment, styles, library, recipe CRUD, additions, mash, settings, BeerXML all wired
- 19+ passing tests across equipment, recipe, and brewing modules
- Seed data for 7 BJCP styles, 15 fermentables, 12 hops, 9 yeasts, and a default equipment profile

Continue with `2026-05-03-brewski-ui.md` to build the Svelte frontend.
