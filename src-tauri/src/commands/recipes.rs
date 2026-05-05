use tauri::State;
use crate::AppState;
use crate::models::{CreateRecipeInput, Recipe, RecipeStats, RecipeSummary, UpdateRecipeInput};
use crate::repositories::recipe::RecipeRepository;
use crate::brewing;

#[tauri::command]
pub async fn list_recipes(state: State<'_, AppState>) -> Result<Vec<RecipeSummary>, String> {
    RecipeRepository::new(&state.db).list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe(state: State<'_, AppState>, id: String) -> Result<Recipe, String> {
    RecipeRepository::new(&state.db).get(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe(state: State<'_, AppState>, input: CreateRecipeInput) -> Result<Recipe, String> {
    RecipeRepository::new(&state.db).create(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_recipe(state: State<'_, AppState>, id: String, input: UpdateRecipeInput) -> Result<Recipe, String> {
    RecipeRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_recipe(state: State<'_, AppState>, id: String) -> Result<(), String> {
    RecipeRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe_stats(state: State<'_, AppState>, recipe_id: String) -> Result<RecipeStats, String> {
    let recipe = RecipeRepository::new(&state.db).get(&recipe_id).await.map_err(|e| e.to_string())?;
    Ok(brewing::calculate_stats(&recipe))
}
