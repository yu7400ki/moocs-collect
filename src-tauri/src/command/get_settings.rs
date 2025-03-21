use crate::store::Settings;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Settings {
    Settings::from(&app)
}
