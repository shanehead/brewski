use tauri::State;
use crate::AppState;
use crate::models::*;
use crate::repositories::addition::AdditionRepository;

#[tauri::command]
pub async fn create_recipe_fermentable(state: State<'_, AppState>, recipe_id: String, input: CreateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    AdditionRepository::new(&state.db).create_fermentable(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_fermentable(state: State<'_, AppState>, id: String, input: UpdateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    AdditionRepository::new(&state.db).update_fermentable(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_fermentable(state: State<'_, AppState>, id: String) -> Result<(), String> {
    AdditionRepository::new(&state.db).delete_fermentable(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_hop(state: State<'_, AppState>, recipe_id: String, input: CreateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    AdditionRepository::new(&state.db).create_hop(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_hop(state: State<'_, AppState>, id: String, input: UpdateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    AdditionRepository::new(&state.db).update_hop(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), String> {
    AdditionRepository::new(&state.db).delete_hop(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_yeast(state: State<'_, AppState>, recipe_id: String, input: CreateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    AdditionRepository::new(&state.db).create_yeast(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_yeast(state: State<'_, AppState>, id: String, input: UpdateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    AdditionRepository::new(&state.db).update_yeast(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), String> {
    AdditionRepository::new(&state.db).delete_yeast(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_misc(state: State<'_, AppState>, recipe_id: String, input: CreateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    AdditionRepository::new(&state.db).create_misc(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_misc(state: State<'_, AppState>, id: String, input: UpdateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    AdditionRepository::new(&state.db).update_misc(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), String> {
    AdditionRepository::new(&state.db).delete_misc(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_water(state: State<'_, AppState>, recipe_id: String, input: CreateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    AdditionRepository::new(&state.db).create_water(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_water(state: State<'_, AppState>, id: String, input: UpdateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    AdditionRepository::new(&state.db).update_water(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), String> {
    AdditionRepository::new(&state.db).delete_water(&id).await.map_err(|e| e.to_string())
}
