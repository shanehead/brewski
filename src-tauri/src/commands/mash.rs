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
