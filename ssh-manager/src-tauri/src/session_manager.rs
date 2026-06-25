use std::collections::HashMap;

use russh_sftp::client::SftpSession;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::models::{ConnectionParams, SessionInfo};

#[derive(Debug)]
pub enum TerminalCmd {
    Write(bytes::Bytes),
    Resize(u32, u32, u32, u32),
    Close,
}

pub struct SshConnection {
    pub handle: russh::client::Handle<HessHandler>,
    pub sftp: Option<SftpSession>,
    pub terminals: HashMap<Uuid, mpsc::Sender<TerminalCmd>>,
    pub params: ConnectionParams,
    pub connected_at: String,
}

pub struct HessHandler;

impl russh::client::Handler for HessHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SessionManager {
    pub sessions: HashMap<Uuid, SshConnection>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn add_session(&mut self, id: Uuid, conn: SshConnection) {
        self.sessions.insert(id, conn);
    }

    pub fn remove_session(&mut self, id: &Uuid) -> Option<SshConnection> {
        self.sessions.remove(id)
    }

    pub fn get_session_mut(&mut self, id: &Uuid) -> Option<&mut SshConnection> {
        self.sessions.get_mut(id)
    }

    #[allow(dead_code)]
    pub fn session_exists(&self, id: &Uuid) -> bool {
        self.sessions.contains_key(id)
    }

    pub fn all_sessions_info(&self) -> Vec<SessionInfo> {
        self.sessions
            .iter()
            .map(|(id, conn)| SessionInfo {
                id: id.to_string(),
                host: conn.params.host.clone(),
                username: conn.params.username.clone(),
                connected_at: conn.connected_at.clone(),
            })
            .collect()
    }
}
