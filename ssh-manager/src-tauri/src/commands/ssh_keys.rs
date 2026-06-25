use std::fs;
use std::path::PathBuf;

use crate::models::SshKeyInfo;
use crate::ssh_config_parser;

#[tauri::command]
pub fn list_ssh_keys() -> Result<Vec<SshKeyInfo>, String> {
    let ssh_dir = ssh_config_parser::get_ssh_dir()?;

    if !ssh_dir.exists() {
        return Ok(Vec::new());
    }

    let mut keys = Vec::new();
    let entries = fs::read_dir(&ssh_dir)
        .map_err(|e| format!("Failed to read ~/.ssh: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = entry.file_name().to_string_lossy().to_string();

        // Skip common non-key files
        if filename == "config" || filename == "known_hosts" || filename == "authorized_keys" || filename == "environment" {
            continue;
        }

        // Identify public keys
        let is_public = filename.ends_with(".pub");

        // Read key type from public key file or detect from private key
        let key_type = if is_public {
            detect_pubkey_type(&path)
        } else {
            detect_privkey_type(&path)
        };

        let metadata = fs::metadata(&path)
            .map_err(|e| format!("Failed to read metadata: {e}"))?;

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|t| {
                let duration = t.duration_since(std::time::UNIX_EPOCH).ok()?;
                Some(format!("{}", duration.as_secs()))
            })
            .unwrap_or_default();

        keys.push(SshKeyInfo {
            filename,
            path: path.to_string_lossy().to_string(),
            key_type,
            fingerprint: None,
            is_public,
            modified_at,
        });
    }

    // Sort: public keys first, then alphabetical
    keys.sort_by(|a, b| {
        b.is_public.cmp(&a.is_public).then(a.filename.cmp(&b.filename))
    });

    Ok(keys)
}

fn detect_pubkey_type(path: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    // The first field in a .pub file is the key type (e.g., "ssh-rsa", "ssh-ed25519")
    content.split_whitespace().next().map(|s| s.to_string())
}

fn detect_privkey_type(path: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    // Private key headers look like "-----BEGIN OPENSSH PRIVATE KEY-----"
    if let Some(line) = content.lines().next() {
        if line.contains("PRIVATE KEY") {
            let type_str = line
                .trim_start_matches("-----BEGIN ")
                .trim_end_matches(" PRIVATE KEY-----")
                .to_lowercase();
            return Some(match type_str.as_str() {
                "openssh" => "ssh-ed25519".to_string(),
                "rsa" => "ssh-rsa".to_string(),
                "ecdsa" => "ecdsa-sha2-nistp256".to_string(),
                "dsa" => "ssh-dss".to_string(),
                _ => type_str,
            });
        }
    }
    None
}
