mod commands;
mod db;
mod error;
pub mod models;
pub mod brewing;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
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

            let opts = SqliteConnectOptions::from_str(&db_url)?
                .create_if_missing(true);
            let pool = tauri::async_runtime::block_on(
                SqlitePool::connect_with(opts)
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
            commands::library::list_styles,
            commands::library::list_fermentable_library,
            commands::library::list_hop_library,
            commands::library::list_yeast_library,
            commands::library::list_misc_library,
            commands::library::list_water_library,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
