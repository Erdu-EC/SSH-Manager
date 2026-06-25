mod commands;
mod config_store;
mod models;
mod session_manager;
mod ssh_config_parser;

use std::sync::Arc;

use directories::BaseDirs;
use tokio::sync::Mutex;

use config_store::ConfigStore;
use session_manager::SessionManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_store = {
        let base_dirs = BaseDirs::new().expect("Failed to determine app data directory");
        let app_dir = base_dirs.data_dir().join("ssh-manager");
        std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
        ConfigStore::new(app_dir).expect("Failed to initialize config store")
    };

    let session_manager = Arc::new(Mutex::new(SessionManager::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(config_store)
    .manage(session_manager)
    .manage(commands::sftp::TransferManager::default())
    .invoke_handler(tauri::generate_handler![
        commands::profiles::get_app_profiles,
        commands::profiles::save_app_profile,
        commands::profiles::delete_app_profile,
        commands::ssh_config::get_ssh_config,
        commands::ssh_keys::list_ssh_keys,
        commands::connection::connect,
        commands::connection::disconnect,
        commands::connection::get_active_sessions,
        commands::sftp::cancel_transfer,
        commands::sftp::list_directory,
        commands::sftp::create_directory,
        commands::sftp::remove_file,
        commands::sftp::rename,
        commands::sftp::get_file_info,
        commands::sftp::upload_file,
        commands::terminal::create_terminal,
        commands::terminal::close_terminal,
        commands::terminal::write_stdin,
        commands::terminal::resize_pty,
        commands::shell::show_in_folder,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
