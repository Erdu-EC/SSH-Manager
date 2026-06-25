use tauri::State;

use crate::config_store::ConfigStore;
use crate::models::AppProfile;

#[tauri::command]
pub fn get_app_profiles(store: State<'_, ConfigStore>) -> Result<Vec<AppProfile>, String> {
    store.get_all_profiles()
}

#[tauri::command]
pub fn save_app_profile(store: State<'_, ConfigStore>, profile: AppProfile) -> Result<(), String> {
    store.save_profile(&profile)
}

#[tauri::command]
pub fn delete_app_profile(store: State<'_, ConfigStore>, id: String) -> Result<(), String> {
    store.delete_profile(&id)
}
