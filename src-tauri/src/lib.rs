#![warn(clippy::wildcard_imports)]

pub mod brewing;
mod commands;
pub mod entities;
mod error;
pub mod models;
#[path = "models.gen.rs"]
pub mod models_gen;
pub mod repositories;
pub mod sync_config;

#[cfg(test)]
mod test_helpers;

use sea_orm::SqlxSqliteConnector;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub db_path: std::path::PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;

            let config = crate::sync_config::SyncConfig::load(&app_dir);
            let db_path = config
                .database_path
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| app_dir.join("brewski.db"));

            let opts = SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true);
            let pool = tauri::async_runtime::block_on(SqlitePool::connect_with(opts))?;
            tauri::async_runtime::block_on(sqlx::migrate!("./migrations").run(&pool))?;
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
            app.manage(AppState { db, db_path });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::equipment::list_equipment_profiles,
            commands::equipment::create_equipment_profile,
            commands::equipment::update_equipment_profile,
            commands::equipment::delete_equipment_profile,
            commands::equipment::copy_equipment_profile,
            commands::library::list_styles,
            commands::library::list_fermentable_library,
            commands::library::list_hop_library,
            commands::library::list_yeast_library,
            commands::library::list_misc_library,
            commands::library::list_water_library,
            commands::recipes::list_recipes,
            commands::recipes::list_baseline_recipes,
            commands::recipes::get_recipe,
            commands::recipes::create_recipe,
            commands::recipes::update_recipe,
            commands::recipes::delete_recipe,
            commands::recipes::get_recipe_stats,
            commands::recipes::scale_recipe,
            commands::recipe_image::upload_recipe_image,
            commands::recipe_image::delete_recipe_image,
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
            commands::batches::create_batch,
            commands::batches::list_batches,
            commands::batches::list_batches_for_recipe,
            commands::batches::get_batch,
            commands::batches::update_batch,
            commands::batches::delete_batch,
            commands::batches::add_gravity_reading,
            commands::batches::delete_gravity_reading,
            commands::batches::list_recipe_versions,
            commands::batches::get_recipe_version,
            commands::batches::save_recipe_version,
            commands::batches::branch_from_version,
            commands::batches::delete_recipe_version,
            commands::settings::get_settings,
            commands::settings::update_setting,
            commands::import_export::get_recipe_beerxml,
            commands::import_export::create_recipes_from_beerxml,
            commands::import_export::write_recipe_beerxml,
            commands::tools::calculate_abv_calories,
            commands::tools::correct_hydrometer_temp,
            commands::tools::calculate_refractometer,
            commands::tools::correct_refractometer_fg,
            commands::tools::calculate_priming_sugar,
            commands::tools::calculate_co2_pressure,
            commands::tools::convert_gravity,
            commands::tools::calculate_pitch_rate,
            commands::tools::convert_color,
            commands::sync::detect_sync_folders,
            commands::sync::move_database,
            commands::sync::get_db_path,
            commands::ingredients::create_hop,
            commands::ingredients::update_hop,
            commands::ingredients::delete_hop,
            commands::ingredients::create_fermentable,
            commands::ingredients::update_fermentable,
            commands::ingredients::delete_fermentable,
            commands::ingredients::create_yeast,
            commands::ingredients::update_yeast,
            commands::ingredients::delete_yeast,
            commands::ingredients::create_misc,
            commands::ingredients::update_misc,
            commands::ingredients::delete_misc,
            commands::ingredients::create_water,
            commands::ingredients::update_water,
            commands::ingredients::delete_water,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
