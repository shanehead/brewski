# sqlx Migrations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the `sea-orm-migration` layer with `sqlx`'s built-in migration runner, eliminating all Rust boilerplate wrapper files while keeping SeaORM for queries.

**Architecture:** Create a `sqlx::SqlitePool`, run `sqlx::migrate!("migrations")` on it, then hand it to SeaORM via `SqlxSqliteConnector::from_sqlx_sqlite_pool`. The SQL files move to `src-tauri/migrations/` (conventional sqlx location) unchanged.

**Tech Stack:** Rust, sqlx 0.8 (`migrate` feature), sea-orm 1.x (`SqlxSqliteConnector`), SQLite

---

### Task 1: Update dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: Add sqlx and remove sea-orm-migration**

In `src-tauri/Cargo.toml`, replace:
```toml
sea-orm-migration = { version = "1", features = ["sqlx-sqlite", "runtime-tokio-native-tls"] }
```
with:
```toml
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls", "migrate"] }
```

The `[dependencies]` block should now read:
```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sea-orm = { version = "1", features = ["sqlx-sqlite", "runtime-tokio-native-tls", "macros", "with-uuid"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls", "migrate"] }
libsqlite3-sys = { version = "0.30", features = ["bundled"] }
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
thiserror = "2"
quick-xml = { version = "0.36", features = ["serialize"] }
```

- [ ] **Step 2: Verify it compiles (migration code will be broken — that's expected)**

```bash
cd src-tauri && cargo check 2>&1 | grep "^error" | head -20
```

Expected: errors referencing `sea_orm_migration` and `crate::migration` — those are the files we're about to delete. No other unexpected errors.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore(deps): replace sea-orm-migration with sqlx migrate"
```

---

### Task 2: Move SQL files to migrations/

**Files:**
- Create: `src-tauri/migrations/001_initial.sql` (move)
- Create: `src-tauri/migrations/002_water_chemistry.sql` (move)
- Create: `src-tauri/migrations/003_whirlpool_temp.sql` (move)
- Create: `src-tauri/migrations/004_hopstand_temp_rename.sql` (move)
- Delete: `src-tauri/src/migration/sql/` directory

sqlx resolves `sqlx::migrate!("migrations")` relative to `CARGO_MANIFEST_DIR`, which is `src-tauri/`. So the target is `src-tauri/migrations/`.

- [ ] **Step 1: Move the SQL files**

```bash
mkdir -p src-tauri/migrations
mv src-tauri/src/migration/sql/001_initial.sql src-tauri/migrations/
mv src-tauri/src/migration/sql/002_water_chemistry.sql src-tauri/migrations/
mv src-tauri/src/migration/sql/003_whirlpool_temp.sql src-tauri/migrations/
mv src-tauri/src/migration/sql/004_hopstand_temp_rename.sql src-tauri/migrations/
rmdir src-tauri/src/migration/sql
```

- [ ] **Step 2: Verify files are in place**

```bash
ls src-tauri/migrations/
```

Expected:
```
001_initial.sql
002_water_chemistry.sql
003_whirlpool_temp.sql
004_hopstand_temp_rename.sql
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/migrations/ src-tauri/src/migration/sql/
git commit -m "chore(migrations): move SQL files to src-tauri/migrations/"
```

---

### Task 3: Delete Rust migration wrapper files

**Files:**
- Delete: `src-tauri/src/migration/m001_initial.rs`
- Delete: `src-tauri/src/migration/m002_water_chemistry.rs`
- Delete: `src-tauri/src/migration/m003_whirlpool_temp.rs`
- Delete: `src-tauri/src/migration/m004_hopstand_temp_rename.rs`
- Delete: `src-tauri/src/migration/mod.rs`

- [ ] **Step 1: Delete the files**

```bash
rm src-tauri/src/migration/m001_initial.rs
rm src-tauri/src/migration/m002_water_chemistry.rs
rm src-tauri/src/migration/m003_whirlpool_temp.rs
rm src-tauri/src/migration/m004_hopstand_temp_rename.rs
rm src-tauri/src/migration/mod.rs
rmdir src-tauri/src/migration
```

- [ ] **Step 2: Commit**

```bash
git add -u src-tauri/src/migration/
git commit -m "chore(migrations): delete sea-orm-migration wrapper files"
```

---

### Task 4: Update lib.rs

**Files:**
- Modify: `src-tauri/src/lib.rs`

Replace the SeaORM migration setup with sqlx pool creation, migration run, then SeaORM pool wrapping.

- [ ] **Step 1: Update lib.rs**

Replace the top of `src-tauri/src/lib.rs` (the imports and `run()` function) with:

```rust
pub mod brewing;
mod commands;
pub mod entities;
mod error;
pub mod models;
#[path = "models.gen.rs"]
pub mod models_gen;
pub mod repositories;

#[cfg(test)]
mod test_helpers;

use sea_orm::SqlxSqliteConnector;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("brewski.db");
            let opts = SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true);
            let pool = tauri::async_runtime::block_on(SqlitePool::connect_with(opts))?;
            tauri::async_runtime::block_on(sqlx::migrate!("migrations").run(&pool))?;
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::equipment::list_equipment_profiles,
            commands::equipment::create_equipment_profile,
            commands::equipment::update_equipment_profile,
            commands::equipment::delete_equipment_profile,
            commands::library::list_styles,
            commands::library::list_fermentable_library,
            commands::library::list_hop_library,
            commands::library::list_yeast_library,
            commands::library::list_misc_library,
            commands::library::list_water_library,
            commands::recipes::list_recipes,
            commands::recipes::get_recipe,
            commands::recipes::create_recipe,
            commands::recipes::update_recipe,
            commands::recipes::delete_recipe,
            commands::recipes::get_recipe_stats,
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
            commands::water_chemistry::set_recipe_water_sources,
            commands::water_chemistry::calculate_water_profile,
            commands::water_chemistry::create_water_adjustment,
            commands::water_chemistry::update_water_adjustment,
            commands::water_chemistry::delete_water_adjustment,
            commands::mash::get_mash,
            commands::mash::update_mash,
            commands::mash::create_mash_step,
            commands::mash::update_mash_step,
            commands::mash::delete_mash_step,
            commands::mash::update_mash_step_order,
            commands::settings::get_settings,
            commands::settings::update_setting,
            commands::import_export::get_recipe_beerxml,
            commands::import_export::create_recipes_from_beerxml,
            commands::tools::calculate_abv_calories,
            commands::tools::correct_hydrometer_temp,
            commands::tools::calculate_refractometer,
            commands::tools::correct_refractometer_fg,
            commands::tools::calculate_priming_sugar,
            commands::tools::calculate_co2_pressure,
            commands::tools::convert_gravity,
            commands::tools::calculate_pitch_rate,
            commands::tools::convert_color,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: Check it compiles**

```bash
cd src-tauri && cargo check 2>&1 | grep "^error"
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "refactor(migrations): use sqlx migrate in lib.rs"
```

---

### Task 5: Update test_helpers.rs

**Files:**
- Modify: `src-tauri/src/test_helpers.rs`

- [ ] **Step 1: Update test_helpers.rs**

Replace the entire file content with:

```rust
use sea_orm::SqlxSqliteConnector;
use sea_orm::DatabaseConnection;
use sqlx::SqlitePool;

pub async fn setup_test_db() -> DatabaseConnection {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("in-memory DB failed");
    sqlx::migrate!("migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
}
```

- [ ] **Step 2: Run the tests**

```bash
cd src-tauri && cargo test 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/test_helpers.rs
git commit -m "refactor(migrations): use sqlx migrate in test_helpers"
```

---

### Task 6: Update bin/migrate.rs

**Files:**
- Modify: `src-tauri/src/bin/migrate.rs`

- [ ] **Step 1: Update migrate.rs**

Replace the entire file content with:

```rust
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "sqlite://./dev.db".to_string());
    let opts = SqliteConnectOptions::from_str(&url)
        .expect("invalid DB URL")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("DB connect failed");
    sqlx::migrate!("migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    println!("Migrations applied to {url}");
}
```

- [ ] **Step 2: Check it compiles**

```bash
cd src-tauri && cargo check --bin migrate 2>&1 | grep "^error"
```

Expected: no errors.

- [ ] **Step 3: Run the full test suite one final time**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass, `0 failed`.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/bin/migrate.rs
git commit -m "refactor(migrations): use sqlx migrate in bin/migrate"
```
