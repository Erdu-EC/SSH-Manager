use std::collections::HashMap;
use std::sync::Arc;

use tauri::Emitter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, Notify};
use uuid::Uuid;

#[derive(Default)]
pub struct TransferManager {
    pub cancellations: Mutex<HashMap<String, Arc<Notify>>>,
}
use crate::models::FileEntry;
use crate::session_manager::SessionManager;

fn get_session<'a>(
    sm: &'a mut SessionManager,
    id: &Uuid,
) -> Result<&'a mut crate::session_manager::SshConnection, String> {
    sm.get_session_mut(id).ok_or_else(|| "Session not found".to_string())
}

#[tauri::command]
pub async fn list_directory(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    path: String,
) -> Result<Vec<FileEntry>, String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;

    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;
    let entries = sftp
        .read_dir(path.as_str())
        .await
        .map_err(|e| format!("Failed to list directory: {e}"))?;

    let mut result = Vec::new();
    for entry in entries {
        let name = entry.file_name();
        let metadata = entry.metadata();

        if name == "." || name == ".." {
            continue;
        }

        let is_dir = metadata.is_dir();
        let is_symlink = metadata.is_symlink();
        let child_path = format!("{}/{}", path.trim_end_matches('/'), name);

        let permissions = metadata.permissions.map(|p| format!("{:o}", p));

        let modified_at = metadata.mtime.map(|m| m.to_string());

        result.push(FileEntry {
            name: name.to_string(),
            path: child_path,
            is_directory: is_dir,
            is_symlink,
            size: metadata.size,
            permissions,
            modified_at,
        });
    }

    // Sort: directories first, then by name
    result.sort_by(|a, b| {
        b.is_directory
            .cmp(&a.is_directory)
            .then(a.name.cmp(&b.name))
    });

    Ok(result)
}

#[tauri::command]
pub async fn create_directory(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    path: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;
    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;
    sftp
        .create_dir(path.as_str())
        .await
        .map_err(|e| format!("Failed to create directory: {e}"))
}

#[tauri::command]
pub async fn remove_file(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    path: String,
    recursive: bool,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;
    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;

    if recursive {
        remove_recursive(sftp, &path).await
    } else {
        sftp
            .remove_file(path.as_str())
            .await
            .map_err(|e| format!("Failed to remove file: {e}"))
    }
}

fn remove_recursive<'a>(
    sftp: &'a russh_sftp::client::SftpSession,
    path: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + 'a>> {
    Box::pin(async move {
        match sftp.read_dir(path).await {
            Ok(entries) => {
                for entry in entries {
                    let name = entry.file_name();
                    if name == "." || name == ".." {
                        continue;
                    }
                    let child_path = format!("{}/{}", path.trim_end_matches('/'), name);
                    let metadata = entry.metadata();
                    if metadata.is_dir() {
                        remove_recursive(sftp, &child_path).await?;
                    } else {
                        sftp.remove_file(&child_path)
                            .await
                            .map_err(|e| format!("Failed to remove {child_path}: {e}"))?;
                    }
                }
                sftp.remove_dir(path)
                    .await
                    .map_err(|e| format!("Failed to remove directory {path}: {e}"))
            }
            Err(_) => {
                sftp.remove_file(path)
                    .await
                    .map_err(|e| format!("Failed to remove {path}: {e}"))
            }
        }
    })
}

#[tauri::command]
pub async fn rename(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;
    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;
    sftp
        .rename(old_path.as_str(), new_path.as_str())
        .await
        .map_err(|e| format!("Failed to rename: {e}"))
}

#[tauri::command]
pub async fn get_file_info(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    path: String,
) -> Result<FileEntry, String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;
    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;

    let metadata = sftp
        .metadata(path.as_str())
        .await
        .map_err(|e| format!("Failed to stat file: {e}"))?;

    let name = path.split('/').next_back().unwrap_or(&path).to_string();
    let is_dir = metadata.is_dir();
    let is_symlink = metadata.is_symlink();

    let permissions = metadata.permissions.map(|p| format!("{:o}", p));
    let modified_at = metadata.mtime.map(|m| m.to_string());

    Ok(FileEntry {
        name,
        path,
        is_directory: is_dir,
        is_symlink,
        size: metadata.size,
        permissions,
        modified_at,
    })
}

#[tauri::command]
pub async fn cancel_transfer(
    transfer_manager: tauri::State<'_, TransferManager>,
    job_id: String,
) -> Result<(), String> {
    let mut cancellations = transfer_manager.cancellations.lock().await;
    if let Some(notifier) = cancellations.remove(&job_id) {
        notifier.notify_one();
    }
    Ok(())
}

#[tauri::command]
pub async fn upload_file(
    app: tauri::AppHandle,
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>, 
    transfer_manager: tauri::State<'_, TransferManager>,
    session_id: String,
    job_id: String,
    local_path: String,
    remote_path: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = get_session(&mut sm, &id)?;
    let sftp = conn.sftp.as_mut().ok_or("SFTP not initialized")?;

    let metadata = std::fs::metadata(&local_path)
        .map_err(|e| format!("Failed to stat local file: {e}"))?;
    let total_size = metadata.len();

    let notifier = Arc::new(Notify::new());
    {
        let mut cancellations = transfer_manager.cancellations.lock().await;
        cancellations.insert(job_id.clone(), notifier.clone());
    }

    let mut remote = sftp
        .create(&remote_path)
        .await
        .map_err(|e| format!("Failed to create remote file: {e}"))?;

    let mut local = tokio::fs::File::open(&local_path)
        .await
        .map_err(|e| format!("Failed to open local file: {e}"))?;

    let mut buffer = vec![0u8; 65536];
    let mut transferred: u64 = 0;

    let transfer_result: Result<(), String> = async {
        loop {
            tokio::select! {
                _ = notifier.notified() => {
                    // Clean up partial file
                    let _ = sftp.remove_file(&remote_path).await;
                    return Err("Transfer cancelled".into());
                }
                n = local.read(&mut buffer) => {
                    let n = n.map_err(|e| format!("Failed to read local file: {e}"))?;
                    if n == 0 {
                        break;
                    }
                    remote
                        .write_all(&buffer[..n])
                        .await
                        .map_err(|e| format!("Failed to write remote file: {e}"))?;
                    transferred += n as u64;

                    let _ = app.emit(
                        "upload-progress",
                        serde_json::json!({
                            "jobId": job_id,
                            "bytesTransferred": transferred,
                            "totalBytes": total_size
                        }),
                    );
                }
            }
        }
        Ok(())
    }.await;

    // Clean up cancellation token
    {
        let mut cancellations = transfer_manager.cancellations.lock().await;
        cancellations.remove(&job_id);
    }

    transfer_result
}
