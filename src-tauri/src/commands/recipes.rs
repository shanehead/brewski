use tauri::State;
use crate::AppState;
use crate::models::{Recipe, RecipeSummary, RecipeStats, CreateRecipeInput, UpdateRecipeInput};
use crate::{db, brewing};

#[tauri::command]
pub async fn list_recipes(state: State<'_, AppState>) -> Result<Vec<RecipeSummary>, String> {
    db::recipes::list(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe(state: State<'_, AppState>, id: String) -> Result<Recipe, String> {
    db::recipes::get(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_recipe(
    state: State<'_, AppState>,
    input: CreateRecipeInput,
) -> Result<Recipe, String> {
    db::recipes::create(&state.db, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_recipe(
    state: State<'_, AppState>,
    id: String,
    input: UpdateRecipeInput,
) -> Result<Recipe, String> {
    db::recipes::update(&state.db, &id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_recipe(state: State<'_, AppState>, id: String) -> Result<(), String> {
    db::recipes::delete(&state.db, &id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recipe_stats(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<RecipeStats, String> {
    let recipe = db::recipes::get(&state.db, &recipe_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(brewing::calculate_stats(&recipe))
}
