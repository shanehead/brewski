use tauri::State;
use crate::AppState;
use crate::error::AppError;
use crate::models::{Fermentable, Hop, Misc, Style, Water, Yeast};
use crate::repositories::library::LibraryRepository;

#[tauri::command]
pub async fn list_styles(state: State<'_, AppState>) -> Result<Vec<Style>, AppError> {
    LibraryRepository::new(&state.db).list_styles().await
}
#[tauri::command]
pub async fn list_fermentable_library(state: State<'_, AppState>) -> Result<Vec<Fermentable>, AppError> {
    LibraryRepository::new(&state.db).list_fermentables().await
}
#[tauri::command]
pub async fn list_hop_library(state: State<'_, AppState>) -> Result<Vec<Hop>, AppError> {
    LibraryRepository::new(&state.db).list_hops().await
}
#[tauri::command]
pub async fn list_yeast_library(state: State<'_, AppState>) -> Result<Vec<Yeast>, AppError> {
    LibraryRepository::new(&state.db).list_yeasts().await
}
#[tauri::command]
pub async fn list_misc_library(state: State<'_, AppState>) -> Result<Vec<Misc>, AppError> {
    LibraryRepository::new(&state.db).list_miscs().await
}
#[tauri::command]
pub async fn list_water_library(state: State<'_, AppState>) -> Result<Vec<Water>, AppError> {
    LibraryRepository::new(&state.db).list_waters().await
}
