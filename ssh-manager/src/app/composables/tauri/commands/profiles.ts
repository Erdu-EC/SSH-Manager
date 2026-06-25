import { invoke } from '@tauri-apps/api/core'
import type { AppProfile } from '~/types'

export async function getAppProfiles(): Promise<AppProfile[]> {
  return invoke<AppProfile[]>('get_app_profiles')
}

export async function saveAppProfile(profile: AppProfile): Promise<void> {
  return invoke('save_app_profile', { profile })
}

export async function deleteAppProfile(id: string): Promise<void> {
  return invoke('delete_app_profile', { id })
}
