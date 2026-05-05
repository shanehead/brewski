use tauri::State;
use crate::AppState;
use crate::models::{Fermentable, Hop, Misc, Style, Water, Yeast};
use crate::repositories::library::LibraryRepository;

#[tauri::command]
pub async fn list_styles(state: State<'_, AppState>) -> Result<Vec<Style>, String> {
    LibraryRepository::new(&state.db).list_styles().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_fermentable_library(state: State<'_, AppState>) -> Result<Vec<Fermentable>, String> {
    LibraryRepository::new(&state.db).list_fermentables().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_hop_library(state: State<'_, AppState>) -> Result<Vec<Hop>, String> {
    LibraryRepository::new(&state.db).list_hops().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_yeast_library(state: State<'_, AppState>) -> Result<Vec<Yeast>, String> {
    LibraryRepository::new(&state.db).list_yeasts().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_misc_library(state: State<'_, AppState>) -> Result<Vec<Misc>, String> {
    LibraryRepository::new(&state.db).list_miscs().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_water_library(state: State<'_, AppState>) -> Result<Vec<Water>, String> {
    LibraryRepository::new(&state.db).list_waters().await.map_err(|e| e.to_string())
}
