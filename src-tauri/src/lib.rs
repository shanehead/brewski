pub mod brewing;
mod commands;
pub mod entities;
mod error;
pub mod migration;
pub mod models;
#[path = "models.gen.rs"]
pub mod models_gen;
pub mod repositories;

#[cfg(test)]
mod test_helpers;

use crate::migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
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
            let db_url = format!("sqlite://{}?mode=rwc", app_dir.join("brewski.db").display());
            let db = tauri::async_runtime::block_on(Database::connect(&db_url))?;
            tauri::async_runtime::block_on(Migrator::up(&db, None))?;
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
