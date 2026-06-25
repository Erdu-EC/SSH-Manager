import { invoke } from '@tauri-apps/api/core'
import type { SshConfigEntry } from '~/types'

export async function getSshConfig(): Promise<SshConfigEntry[]> {
  return invoke<SshConfigEntry[]>('get_ssh_config')
}
