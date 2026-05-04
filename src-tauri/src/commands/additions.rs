use tauri::State;
use crate::AppState;
use crate::models::*;
use crate::db;

#[tauri::command]
pub async fn create_recipe_fermentable(state: State<'_, AppState>, recipe_id: String, input: CreateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    db::additions::create_fermentable(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_fermentable(state: State<'_, AppState>, id: String, input: UpdateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    db::additions::update_fermentable(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_fermentable(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_fermentable(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_hop(state: State<'_, AppState>, recipe_id: String, input: CreateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    db::additions::create_hop(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_hop(state: State<'_, AppState>, id: String, input: UpdateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    db::additions::update_hop(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_hop(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_yeast(state: State<'_, AppState>, recipe_id: String, input: CreateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    db::additions::create_yeast(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_yeast(state: State<'_, AppState>, id: String, input: UpdateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    db::additions::update_yeast(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_yeast(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_misc(state: State<'_, AppState>, recipe_id: String, input: CreateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    db::additions::create_misc(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_misc(state: State<'_, AppState>, id: String, input: UpdateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    db::additions::update_misc(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_misc(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe_water(state: State<'_, AppState>, recipe_id: String, input: CreateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    db::additions::create_water(&state.db, &recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_water(state: State<'_, AppState>, id: String, input: UpdateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    db::additions::update_water(&state.db, &id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::additions::delete_water(&state.db, &id).await.map_err(|e| e.to_string())
}
