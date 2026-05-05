use tauri::State;
use crate::AppState;
use crate::models::*;
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::water::WaterRepository;
use crate::repositories::yeast::YeastRepository;

#[tauri::command]
pub async fn create_recipe_fermentable(state: State<'_, AppState>, recipe_id: String, input: CreateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    FermentableRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_fermentable(state: State<'_, AppState>, id: String, input: UpdateFermentableAdditionInput) -> Result<RecipeAdditionFermentable, String> {
    FermentableRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_fermentable(state: State<'_, AppState>, id: String) -> Result<(), String> {
    FermentableRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_hop(state: State<'_, AppState>, recipe_id: String, input: CreateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    HopRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_hop(state: State<'_, AppState>, id: String, input: UpdateHopAdditionInput) -> Result<RecipeAdditionHop, String> {
    HopRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), String> {
    HopRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_yeast(state: State<'_, AppState>, recipe_id: String, input: CreateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    YeastRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_yeast(state: State<'_, AppState>, id: String, input: UpdateYeastAdditionInput) -> Result<RecipeAdditionYeast, String> {
    YeastRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), String> {
    YeastRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_misc(state: State<'_, AppState>, recipe_id: String, input: CreateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    MiscRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_misc(state: State<'_, AppState>, id: String, input: UpdateMiscAdditionInput) -> Result<RecipeAdditionMisc, String> {
    MiscRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), String> {
    MiscRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_recipe_water(state: State<'_, AppState>, recipe_id: String, input: CreateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    WaterRepository::new(&state.db).create(&recipe_id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_recipe_water(state: State<'_, AppState>, id: String, input: UpdateWaterAdditionInput) -> Result<RecipeAdditionWater, String> {
    WaterRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), String> {
    WaterRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
