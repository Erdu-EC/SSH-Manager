import { invoke } from '@tauri-apps/api/core'

export async function showInFolder(path: string): Promise<void> {
  return invoke('show_in_folder', { path })
}
