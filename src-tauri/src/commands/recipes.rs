use tauri::State;
use crate::AppState;
use crate::error::AppError;
use crate::models::{CreateRecipeInput, Recipe, RecipeStats, RecipeSummary, UpdateRecipeInput};
use crate::repositories::recipe::RecipeRepository;
use crate::brewing;

#[tauri::command]
pub async fn list_recipes(state: State<'_, AppState>) -> Result<Vec<RecipeSummary>, AppError> {
    RecipeRepository::new(&state.db).list().await
}

#[tauri::command]
pub async fn get_recipe(state: State<'_, AppState>, id: String) -> Result<Recipe, AppError> {
    RecipeRepository::new(&state.db).get(&id).await
}

#[tauri::command]
pub async fn create_recipe(state: State<'_, AppState>, input: CreateRecipeInput) -> Result<Recipe, AppError> {
    RecipeRepository::new(&state.db).create(input).await
}

#[tauri::command]
pub async fn update_recipe(state: State<'_, AppState>, id: String, input: UpdateRecipeInput) -> Result<Recipe, AppError> {
    RecipeRepository::new(&state.db).update(&id, input).await
}

#[tauri::command]
pub async fn delete_recipe(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    RecipeRepository::new(&state.db).delete(&id).await
}

#[tauri::command]
pub async fn get_recipe_stats(state: State<'_, AppState>, recipe_id: String) -> Result<RecipeStats, AppError> {
    let recipe = RecipeRepository::new(&state.db).get(&recipe_id).await?;
    Ok(brewing::calculate_stats(&recipe))
}
