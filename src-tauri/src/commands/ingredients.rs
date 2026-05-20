use crate::error::AppError;
use crate::models::{
    CreateFermentableInput, CreateHopInput, CreateMiscInput, CreateWaterInput, CreateYeastInput,
    Fermentable, Hop, Misc, UpdateFermentableInput, UpdateHopInput, UpdateMiscInput,
    UpdateWaterInput, UpdateYeastInput, Water, Yeast,
};
use crate::repositories::ingredient::IngredientRepository;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_hop(
    state: State<'_, AppState>,
    input: CreateHopInput,
) -> Result<Hop, AppError> {
    IngredientRepository::new(&state.db).create_hop(input).await
}

#[tauri::command]
pub async fn update_hop(
    state: State<'_, AppState>,
    id: String,
    input: UpdateHopInput,
) -> Result<Hop, AppError> {
    IngredientRepository::new(&state.db)
        .update_hop(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_hop(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    IngredientRepository::new(&state.db).delete_hop(&id).await
}

#[tauri::command]
pub async fn create_fermentable(
    state: State<'_, AppState>,
    input: CreateFermentableInput,
) -> Result<Fermentable, AppError> {
    IngredientRepository::new(&state.db)
        .create_fermentable(input)
        .await
}

#[tauri::command]
pub async fn update_fermentable(
    state: State<'_, AppState>,
    id: String,
    input: UpdateFermentableInput,
) -> Result<Fermentable, AppError> {
    IngredientRepository::new(&state.db)
        .update_fermentable(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_fermentable(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    IngredientRepository::new(&state.db)
        .delete_fermentable(&id)
        .await
}

#[tauri::command]
pub async fn create_yeast(
    state: State<'_, AppState>,
    input: CreateYeastInput,
) -> Result<Yeast, AppError> {
    IngredientRepository::new(&state.db)
        .create_yeast(input)
        .await
}

#[tauri::command]
pub async fn update_yeast(
    state: State<'_, AppState>,
    id: String,
    input: UpdateYeastInput,
) -> Result<Yeast, AppError> {
    IngredientRepository::new(&state.db)
        .update_yeast(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_yeast(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    IngredientRepository::new(&state.db).delete_yeast(&id).await
}

#[tauri::command]
pub async fn create_misc(
    state: State<'_, AppState>,
    input: CreateMiscInput,
) -> Result<Misc, AppError> {
    IngredientRepository::new(&state.db)
        .create_misc(input)
        .await
}

#[tauri::command]
pub async fn update_misc(
    state: State<'_, AppState>,
    id: String,
    input: UpdateMiscInput,
) -> Result<Misc, AppError> {
    IngredientRepository::new(&state.db)
        .update_misc(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_misc(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    IngredientRepository::new(&state.db).delete_misc(&id).await
}

#[tauri::command]
pub async fn create_water(
    state: State<'_, AppState>,
    input: CreateWaterInput,
) -> Result<Water, AppError> {
    IngredientRepository::new(&state.db)
        .create_water(input)
        .await
}

#[tauri::command]
pub async fn update_water(
    state: State<'_, AppState>,
    id: String,
    input: UpdateWaterInput,
) -> Result<Water, AppError> {
    IngredientRepository::new(&state.db)
        .update_water(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_water(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    IngredientRepository::new(&state.db).delete_water(&id).await
}
