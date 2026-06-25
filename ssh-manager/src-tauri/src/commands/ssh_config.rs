use crate::models::SshConfigEntry;
use crate::ssh_config_parser;

#[tauri::command]
pub fn get_ssh_config() -> Result<Vec<SshConfigEntry>, String> {
    ssh_config_parser::parse_ssh_config()
}
