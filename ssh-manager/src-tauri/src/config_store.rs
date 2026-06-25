use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{AppProfile, AuthType};

pub struct ConfigStore {
    conn: Mutex<Connection>,
}

impl ConfigStore {
    pub fn new(app_dir: PathBuf) -> Result<Self, String> {
        let db_path = app_dir.join("profiles.db");
        let conn = Connection::open(&db_path).map_err(|e| format!("Failed to open DB: {e}"))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                port INTEGER DEFAULT 22,
                username TEXT NOT NULL,
                auth_type TEXT DEFAULT 'key',
                identity_file TEXT,
                use_ssh_config INTEGER DEFAULT 0,
                ssh_config_host TEXT,
                created_at TEXT NOT NULL,
                last_used TEXT
            );"
        ).map_err(|e| format!("Failed to create table: {e}"))?;

        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn get_all_profiles(&self) -> Result<Vec<AppProfile>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(
            "SELECT id, name, host, port, username, auth_type, identity_file, use_ssh_config, ssh_config_host, created_at, last_used FROM profiles ORDER BY last_used DESC"
        ).map_err(|e| e.to_string())?;

        let profiles = stmt.query_map([], |row| {
            let auth_type_str: String = row.get(5)?;
            let auth_type = match auth_type_str.as_str() {
                "password" => AuthType::Password,
                _ => AuthType::Key,
            };

            Ok(AppProfile {
                id: row.get(0)?,
                name: row.get(1)?,
                host: row.get(2)?,
                port: row.get::<_, u16>(3)?,
                username: row.get(4)?,
                auth_type,
                identity_file: row.get(6)?,
                use_ssh_config: row.get::<_, i32>(7)? != 0,
                ssh_config_host: row.get(8)?,
                created_at: row.get(9)?,
                last_used: row.get(10)?,
            })
        }).map_err(|e| e.to_string())?;

        profiles.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn save_profile(&self, profile: &AppProfile) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let auth_type_str = match profile.auth_type {
            AuthType::Key => "key",
            AuthType::Password => "password",
        };

        conn.execute(
            "INSERT OR REPLACE INTO profiles (id, name, host, port, username, auth_type, identity_file, use_ssh_config, ssh_config_host, created_at, last_used)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                profile.id,
                profile.name,
                profile.host,
                profile.port,
                profile.username,
                auth_type_str,
                profile.identity_file,
                profile.use_ssh_config as i32,
                profile.ssh_config_host,
                profile.created_at,
                profile.last_used,
            ],
        ).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn delete_profile(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM profiles WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_last_used(&self, id: &str, timestamp: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE profiles SET last_used = ?1 WHERE id = ?2",
            params![timestamp, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
}
