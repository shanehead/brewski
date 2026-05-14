# sqlx Migrations Design

**Date:** 2026-05-13  
**Status:** Approved

## Problem

The `sea-orm-migration` layer is cumbersome. Each migration requires a boilerplate Rust wrapper file (`mXXX_name.rs`) that does nothing except call `execute_unprepared(include_str!("sql/..."))`. The actual work lives entirely in the `.sql` files. The migration runner (`Migrator`) and its trait implementations add code with no value.

## Goal

Replace `sea-orm-migration` with `sqlx`'s built-in migration runner. Keep SeaORM for all query/entity work — only the migration layer changes.

## Design

### Dependencies

- Remove `sea-orm-migration` from `src-tauri/Cargo.toml`.
- Add `sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls", "migrate"] }`. Version 0.8 matches what sea-orm 1.x uses internally, avoiding duplicate sqlx versions in the build.

### Migration files

Move `src-tauri/src/migration/sql/*.sql` → `src-tauri/migrations/*.sql` (conventional sqlx location, resolved relative to `CARGO_MANIFEST_DIR` by the `sqlx::migrate!()` macro). No changes to SQL content or filenames — sqlx accepts the existing `001_name.sql` naming convention. Delete the entire `src-tauri/src/migration/` directory.

### Startup (`lib.rs`)

The current flow:
```
Database::connect(url) → Migrator::up(&db) → app.manage(AppState { db })
```

New flow:
```
SqlitePool::connect_with(options) → sqlx::migrate!().run(&pool) → SqlxSqliteConnector::from_sqlx_sqlite_pool(pool) → app.manage(AppState { db })
```

`SqliteConnectOptions` replaces the `?mode=rwc` URL suffix for create-if-missing behavior. `SqlxSqliteConnector::from_sqlx_sqlite_pool` wraps the pool into a `sea_orm::DatabaseConnection` — SeaORM is designed to accept an external pool for exactly this pattern. Remove the `pub mod migration` declaration and the `sea_orm_migration` import.

### Test helpers (`test_helpers.rs`)

Same pattern using `SqlitePool::connect("sqlite::memory:")`. The in-memory URL works directly with sqlx. Remove the `Migrator` import.

### Migration binary (`bin/migrate.rs`)

Same pool + `sqlx::migrate!()` pattern. Becomes ~6 lines.

## Files Changed

| Action | Path |
|--------|------|
| Delete | `src-tauri/src/migration/m001_initial.rs` |
| Delete | `src-tauri/src/migration/m002_water_chemistry.rs` |
| Delete | `src-tauri/src/migration/m003_whirlpool_temp.rs` |
| Delete | `src-tauri/src/migration/m004_hopstand_temp_rename.rs` |
| Delete | `src-tauri/src/migration/mod.rs` |
| Move   | `src-tauri/src/migration/sql/*.sql` → `src-tauri/migrations/*.sql` |
| Edit   | `src-tauri/Cargo.toml` |
| Edit   | `src-tauri/src/lib.rs` |
| Edit   | `src-tauri/src/test_helpers.rs` |
| Edit   | `src-tauri/src/bin/migrate.rs` |

## Out of Scope

- No changes to SeaORM entities, repositories, or commands.
- No changes to SQL content.
- No `down` migration support (sea-orm-migration's `down()` implementations are not replicated — sqlx migrations are up-only by default, which is appropriate for a Tauri app with an embedded SQLite database).
