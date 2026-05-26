use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateWaterAdditionInput, CreateYeastAdditionInput, RecipeAdditionFermentable,
    RecipeAdditionHop, RecipeAdditionMisc, RecipeAdditionWater, RecipeAdditionYeast,
    UpdateFermentableAdditionInput, UpdateHopAdditionInput, UpdateMiscAdditionInput,
    UpdateWaterAdditionInput, UpdateYeastAdditionInput,
};
use crate::repositories::fermentable::FermentableRepository;
use crate::repositories::hop::HopRepository;
use crate::repositories::misc::MiscRepository;
use crate::repositories::water::WaterRepository;
use crate::repositories::yeast::YeastRepository;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_recipe_fermentable(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    FermentableRepository::new(&state.db)
        .create(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn update_recipe_fermentable(
    state: State<'_, AppState>,
    id: String,
    input: UpdateFermentableAdditionInput,
) -> Result<RecipeAdditionFermentable, AppError> {
    FermentableRepository::new(&state.db)
        .update(&id, input)
        .await
}
#[tauri::command]
pub async fn delete_recipe_fermentable(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    FermentableRepository::new(&state.db).delete(&id).await
}
#[tauri::command]
pub async fn create_recipe_hop(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    HopRepository::new(&state.db)
        .create(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn update_recipe_hop(
    state: State<'_, AppState>,
    id: String,
    input: UpdateHopAdditionInput,
) -> Result<RecipeAdditionHop, AppError> {
    HopRepository::new(&state.db).update(&id, input).await
}
#[tauri::command]
pub async fn delete_recipe_hop(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    HopRepository::new(&state.db).delete(&id).await
}
#[tauri::command]
pub async fn create_recipe_yeast(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    YeastRepository::new(&state.db)
        .create(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn update_recipe_yeast(
    state: State<'_, AppState>,
    id: String,
    input: UpdateYeastAdditionInput,
) -> Result<RecipeAdditionYeast, AppError> {
    YeastRepository::new(&state.db).update(&id, input).await
}
#[tauri::command]
pub async fn delete_recipe_yeast(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    YeastRepository::new(&state.db).delete(&id).await
}
#[tauri::command]
pub async fn create_recipe_misc(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    MiscRepository::new(&state.db)
        .create(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn update_recipe_misc(
    state: State<'_, AppState>,
    id: String,
    input: UpdateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    MiscRepository::new(&state.db).update(&id, input).await
}
#[tauri::command]
pub async fn delete_recipe_misc(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    MiscRepository::new(&state.db).delete(&id).await
}
#[tauri::command]
pub async fn create_recipe_water(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    WaterRepository::new(&state.db)
        .create(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn update_recipe_water(
    state: State<'_, AppState>,
    id: String,
    input: UpdateWaterAdditionInput,
) -> Result<RecipeAdditionWater, AppError> {
    WaterRepository::new(&state.db).update(&id, input).await
}
#[tauri::command]
pub async fn delete_recipe_water(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    WaterRepository::new(&state.db).delete(&id).await
}
