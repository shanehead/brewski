use tauri::State;
use crate::AppState;
use crate::error::AppError;
use crate::repositories::settings::SettingsRepository;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, AppError> {
    SettingsRepository::new(&state.db).get_all().await
}
#[tauri::command]
pub async fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), AppError> {
    SettingsRepository::new(&state.db).set(&key, &value).await
}
