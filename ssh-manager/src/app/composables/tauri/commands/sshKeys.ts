import { invoke } from '@tauri-apps/api/core'
import type { SshKeyInfo } from '~/types'

export async function listSshKeys(): Promise<SshKeyInfo[]> {
  return invoke<SshKeyInfo[]>('list_ssh_keys')
}
