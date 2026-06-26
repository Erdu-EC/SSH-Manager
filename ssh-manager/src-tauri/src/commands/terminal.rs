use std::sync::Arc;

use base64::Engine;
use russh::ChannelMsg;
use tauri::Emitter;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

use crate::session_manager::{SessionManager, TerminalCmd};

#[tauri::command]
pub async fn create_terminal(
    app_handle: tauri::AppHandle,
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    session_id: String,
    terminal_id: String,
    rows: u32,
    cols: u32
) -> Result<String, String> {
    let sid = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {e}"))?;
    let tid = Uuid::parse_str(&terminal_id).map_err(|e| format!("Invalid terminal ID: {e}"))?;
    let mut sm = session_manager.lock().await;
    let conn = sm
        .get_session_mut(&sid)
        .ok_or("Session not found")?;

    let mut channel = conn
        .handle
        .channel_open_session()
        .await
        .map_err(|e| format!("Failed to open channel: {e}"))?;

    channel
        .request_pty(
            false,
            "xterm-256color",
            cols,
            rows,
            0,
            0,
            &[],
        )
        .await
        .map_err(|e| format!("PTY request failed: {e}"))?;

    channel
        .request_shell(false)
        .await
        .map_err(|e| format!("Shell request failed: {e}"))?;

    let (tx, mut rx) = mpsc::channel::<TerminalCmd>(64);

    let app = app_handle.clone();
    let tid_str = tid.to_string();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                msg = channel.wait() => {
                    match msg {
                        Some(ChannelMsg::Data { ref data }) => {
                            let encoded = base64::engine::general_purpose::STANDARD.encode(data);
                            let _ = app.emit("terminal-output", serde_json::json!({
                                "terminalId": tid_str,
                                "data": encoded
                            }));
                        }
                        Some(ChannelMsg::Eof) | Some(ChannelMsg::Close) | None => {
                            let _ = app.emit("terminal-exit", serde_json::json!({
                                "terminalId": tid_str,
                                "exitCode": 0
                            }));
                            break;
                        }
                        _ => {}
                    }
                }
                cmd = rx.recv() => {
                    match cmd {
                        Some(TerminalCmd::Write(data)) => {
                            let _ = channel.data_bytes(data).await;
                        }
                        Some(TerminalCmd::Resize(c, r, pw, ph)) => {
                            let _ = channel.window_change(c, r, pw, ph).await;
                        }
                        Some(TerminalCmd::Close) => {
                            let _ = channel.eof().await;
                            let _ = channel.close().await;
                            break;
                        }
                        None => break,
                    }
                }
            }
        }
    });

    conn.terminals.insert(tid, tx);

    Ok(tid.to_string())
}

#[tauri::command]
pub async fn write_stdin(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    terminal_id: String,
    data: String,
) -> Result<(), String> {
    let tid = Uuid::parse_str(&terminal_id).map_err(|e| format!("Invalid terminal ID: {e}"))?;

    let sm = session_manager.lock().await;
    for (_, conn) in sm.sessions.iter() {
        if let Some(tx) = conn.terminals.get(&tid) {
            let bytes: bytes::Bytes = data.into_bytes().into();
            tx.send(TerminalCmd::Write(bytes))
                .await
                .map_err(|_| "Terminal channel closed".to_string())?;
            return Ok(());
        }
    }

    Err("Terminal not found".to_string())
}

#[tauri::command]
pub async fn resize_pty(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    terminal_id: String,
    rows: u32,
    cols: u32,
) -> Result<(), String> {
    let tid = Uuid::parse_str(&terminal_id).map_err(|e| format!("Invalid terminal ID: {e}"))?;

    let sm = session_manager.lock().await;
    for (_, conn) in sm.sessions.iter() {
        if let Some(tx) = conn.terminals.get(&tid) {
            tx.send(TerminalCmd::Resize(cols, rows, 0, 0))
                .await
                .map_err(|_| "Terminal channel closed".to_string())?;
            return Ok(());
        }
    }

    Err("Terminal not found".to_string())
}

#[tauri::command]
pub async fn close_terminal(
    session_manager: tauri::State<'_, Arc<Mutex<SessionManager>>>,
    terminal_id: String,
) -> Result<(), String> {
    let tid = Uuid::parse_str(&terminal_id).map_err(|e| format!("Invalid terminal ID: {e}"))?;

    let mut sm = session_manager.lock().await;
    for (_, conn) in sm.sessions.iter_mut() {
        if let Some(tx) = conn.terminals.remove(&tid) {
            let _ = tx.send(TerminalCmd::Close).await;
            return Ok(());
        }
    }

    Err("Terminal not found".to_string())
}
