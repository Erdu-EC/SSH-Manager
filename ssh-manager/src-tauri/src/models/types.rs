use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: AuthType,
    pub identity_file: Option<String>,
    pub use_ssh_config: bool,
    pub ssh_config_host: Option<String>,
    pub created_at: String,
    pub last_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "password")]
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfigEntry {
    pub host: String,
    pub host_name: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    pub proxy_jump: Option<String>,
    pub proxy_command: Option<String>,
    pub local_forward: Vec<String>,
    pub remote_forward: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyInfo {
    pub filename: String,
    pub path: String,
    pub key_type: Option<String>,
    pub fingerprint: Option<String>,
    pub is_public: bool,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub size: Option<u64>,
    pub permissions: Option<String>,
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionParams {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: AuthType,
    pub password: Option<String>,
    pub identity_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub host: String,
    pub username: String,
    pub connected_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    pub job_id: String,
    pub session_id: String,
    pub local_path: String,
    pub remote_path: String,
    pub bytes_transferred: u64,
    pub total_bytes: Option<u64>,
    pub status: TransferStatus,
    pub direction: TransferDirection,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferDirection {
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "download")]
    Download,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOutputPayload {
    pub terminal_id: String,
    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalErrorPayload {
    pub terminal_id: String,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalExitPayload {
    pub terminal_id: String,
    pub exit_code: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEventPayload {
    pub job_id: String,
    pub session_id: String,
    pub bytes_transferred: u64,
    pub total_bytes: Option<u64>,
    pub status: String,
    pub local_path: String,
    pub remote_path: String,
    pub direction: String,
}
