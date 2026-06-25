use std::sync::Arc;

use russh::client;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{AuthType, ConnectionParams, SessionInfo};
use crate::session_manager::{HessHandler, SessionManager, SshConnection};

#[tauri::command]
pub async fn connect(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    params: ConnectionParams,
) -> Result<String, String> {
    let config_raw = client::Config {
        keepalive_interval: Some(std::time::Duration::from_secs(15)),
        keepalive_max: 3,
        ..Default::default()
    };
    let config = Arc::new(config_raw);
    let host = params.host.clone();
    let port = params.port;
    let username = params.username.clone();

    let connect_future = client::connect(config, (host.as_str(), port), HessHandler);
    let mut handle = tokio::time::timeout(std::time::Duration::from_secs(15), connect_future)
        .await
        .map_err(|_| "Connection timed out".to_string())?
        .map_err(|e| format!("Connection failed: {e}"))?;

    let auth_success = match params.auth_type {
        AuthType::Password => {
            let password = params.password.clone().ok_or("Password required")?;
            let auth_future = handle.authenticate_password(username.as_str(), password.as_str());
            let result = tokio::time::timeout(std::time::Duration::from_secs(15), auth_future)
                .await
                .map_err(|_| "Authentication timed out".to_string())?
                .map_err(|e| format!("Password auth failed: {e}"))?;
            result.success()
        }
        AuthType::Key => {
            let key_path = params.identity_file.clone().ok_or("Identity file required")?;
            let private_key = russh::keys::load_secret_key(&key_path, None)
                .map_err(|e| format!("Failed to load key {key_path}: {e}"))?;
            let key = russh::keys::PrivateKeyWithHashAlg::new(Arc::new(private_key), None);
            let auth_future = handle.authenticate_publickey(username.as_str(), key);
            let result = tokio::time::timeout(std::time::Duration::from_secs(15), auth_future)
                .await
                .map_err(|_| "Authentication timed out".to_string())?
                .map_err(|e| format!("Key auth failed: {e}"))?;
            result.success()
        }
    };

    if !auth_success {
        return Err("Authentication rejected by server".to_string());
    }

    // Establish SFTP session
    let channel = handle
        .channel_open_session()
        .await
        .map_err(|e| format!("Failed to open session channel: {e}"))?;

    channel
        .request_subsystem(true, "sftp")
        .await
        .map_err(|e| format!("Failed to start SFTP subsystem: {e}"))?;

    let sftp = russh_sftp::client::SftpSession::new(channel.into_stream())
        .await
        .map_err(|e| format!("SFTP init failed: {e}"))?;

    let session_id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();

    let conn = SshConnection {
        handle,
        sftp: Some(sftp),
        terminals: std::collections::HashMap::new(),
        params,
        connected_at: now,
    };

    let mut sm = session_manager.lock().await;
    sm.add_session(session_id, conn);

    Ok(session_id.to_string())
}



#[tauri::command]
pub async fn disconnect(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    if let Some(conn) = sm.remove_session(&id) {
        for (_, tx) in conn.terminals {
            let _ = tx.send(crate::session_manager::TerminalCmd::Close).await;
        }
        let _ = conn
            .handle
            .disconnect(russh::Disconnect::ByApplication, "", "")
            .await;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_active_sessions(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
) -> Result<Vec<SessionInfo>, String> {
    let sm = session_manager.lock().await;
    let sessions = sm.all_sessions_info();
    Ok(sessions)
}
