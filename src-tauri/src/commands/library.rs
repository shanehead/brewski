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
