use std::path::PathBuf;

use crate::models::SshConfigEntry;

pub fn parse_ssh_config() -> Result<Vec<SshConfigEntry>, String> {
    let ssh_dir = get_ssh_dir()?;
    let config_path = ssh_dir.join("config");

    if !config_path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read ~/.ssh/config: {e}"))?;

    let entries = parse_config_content(&content);
    Ok(entries)
}

fn parse_config_content(content: &str) -> Vec<SshConfigEntry> {
    let mut entries = Vec::new();
    let mut current_hosts: Vec<String> = Vec::new();
    let mut current: Option<SshConfigEntryBuilder> = None;
    let mut in_host_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once([' ', '\t']) {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            match key.as_str() {
                "host" => {
                    if in_host_block {
                        if let Some(ref builder) = current {
                            for host in &current_hosts {
                                entries.push(builder.build(host));
                            }
                        }
                        current_hosts.clear();
                    } else {
                        in_host_block = true;
                    }
                    current_hosts = value.split_whitespace().map(|s| s.to_string()).collect();
                    current = Some(SshConfigEntryBuilder::new());
                }
                "hostname" => {
                    if let Some(ref mut b) = current {
                        b.host_name = Some(value.to_string());
                    }
                }
                "user" => {
                    if let Some(ref mut b) = current {
                        b.user = Some(value.to_string());
                    }
                }
                "port" => {
                    if let Some(ref mut b) = current {
                        b.port = value.parse::<u16>().ok();
                    }
                }
                "identityfile" => {
                    if let Some(ref mut b) = current {
                        let expanded = expand_path(value);
                        b.identity_file = Some(expanded);
                    }
                }
                "proxyjump" => {
                    if let Some(ref mut b) = current {
                        b.proxy_jump = Some(value.to_string());
                    }
                }
                "proxycommand" => {
                    if let Some(ref mut b) = current {
                        b.proxy_command = Some(value.to_string());
                    }
                }
                "localforward" => {
                    if let Some(ref mut b) = current {
                        b.local_forward.push(value.to_string());
                    }
                }
                "remoteforward" => {
                    if let Some(ref mut b) = current {
                        b.remote_forward.push(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    if in_host_block {
        if let Some(ref builder) = current {
            for host in &current_hosts {
                entries.push(builder.build(host));
            }
        }
    }

    entries
}

#[derive(Clone)]
struct SshConfigEntryBuilder {
    host_name: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<String>,
    proxy_jump: Option<String>,
    proxy_command: Option<String>,
    local_forward: Vec<String>,
    remote_forward: Vec<String>,
}

impl SshConfigEntryBuilder {
    fn new() -> Self {
        Self {
            host_name: None,
            user: None,
            port: None,
            identity_file: None,
            proxy_jump: None,
            proxy_command: None,
            local_forward: Vec::new(),
            remote_forward: Vec::new(),
        }
    }

    fn build(&self, host: &str) -> SshConfigEntry {
        SshConfigEntry {
            host: host.to_string(),
            host_name: self.host_name.clone(),
            user: self.user.clone(),
            port: self.port,
            identity_file: self.identity_file.clone(),
            proxy_jump: self.proxy_jump.clone(),
            proxy_command: self.proxy_command.clone(),
            local_forward: self.local_forward.clone(),
            remote_forward: self.remote_forward.clone(),
        }
    }
}

fn expand_path(path: &str) -> String {
    if path.starts_with("~/") || path == "~" {
        if let Some(home) = dirs_next_home() {
            return path.replacen('~', &home, 1);
        }
    }
    path.to_string()
}

fn dirs_next_home() -> Option<String> {
    std::env::var("HOME").ok().or_else(|| {
        std::env::var("USERPROFILE").ok()
    })
}

pub fn get_ssh_dir() -> Result<PathBuf, String> {
    let home = dirs_next_home().ok_or("Could not determine home directory")?;
    Ok(PathBuf::from(home).join(".ssh"))
}
