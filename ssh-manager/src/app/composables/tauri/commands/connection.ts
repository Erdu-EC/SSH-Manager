import { invoke } from '@tauri-apps/api/core'
import type { ConnectionParams, SessionInfo } from '~/types'

export async function connect(params: ConnectionParams): Promise<string> {
  return invoke<string>('connect', { params })
}

export async function disconnect(sessionId: string): Promise<void> {
  return invoke('disconnect', { sessionId })
}

export async function getActiveSessions(): Promise<SessionInfo[]> {
  return invoke<SessionInfo[]>('get_active_sessions')
}
