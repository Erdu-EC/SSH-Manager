import { invoke } from '@tauri-apps/api/core'
import type { FileEntry } from '~/types'

export async function listDirectory(sessionId: string, path: string): Promise<FileEntry[]> {
  return invoke<FileEntry[]>('list_directory', { sessionId, path })
}

export async function createDirectory(sessionId: string, path: string): Promise<void> {
  return invoke('create_directory', { sessionId, path })
}

export async function removeFile(sessionId: string, path: string, recursive: boolean): Promise<void> {
  return invoke('remove_file', { sessionId, path, recursive })
}

export async function rename(sessionId: string, oldPath: string, newPath: string): Promise<void> {
  return invoke('rename', { sessionId, oldPath, newPath })
}

export async function getFileInfo(sessionId: string, path: string): Promise<FileEntry> {
  return invoke<FileEntry>('get_file_info', { sessionId, path })
}
