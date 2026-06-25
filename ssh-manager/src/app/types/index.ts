export type AuthType = 'key' | 'password'

export interface AppProfile {
  id: string
  name: string
  host: string
  port: number
  username: string
  auth_type: AuthType
  identity_file: string | null
  use_ssh_config: boolean
  ssh_config_host: string | null
  created_at: string
  last_used: string | null
}

export interface SshConfigEntry {
  host: string
  host_name: string | null
  user: string | null
  port: number | null
  identity_file: string | null
  proxy_jump: string | null
  proxy_command: string | null
  local_forward: string[]
  remote_forward: string[]
}

export interface SshKeyInfo {
  filename: string
  path: string
  key_type: string | null
  fingerprint: string | null
  is_public: boolean
  modified_at: string
}

export interface FileEntry {
  name: string
  path: string
  is_directory: boolean
  is_symlink: boolean
  size: number | null
  permissions: string | null
  modified_at: string | null
}

export interface ConnectionParams {
  host: string
  port: number
  username: string
  auth_type: AuthType
  password: string | null
  identity_file: string | null
}

export interface SessionInfo {
  id: string
  host: string
  username: string
  connected_at: string
}

export interface TransferJob {
  jobId: string
  sessionId: string
  localPath: string
  remotePath: string
  bytesTransferred: number
  totalBytes: number | null
  status: 'in_progress' | 'completed' | 'failed' | 'cancelled'
  direction: 'upload' | 'download'
}

export interface TerminalTabState {
  id: string
  sessionId: string
  label: string
}
