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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
