import { invoke } from '@tauri-apps/api/core'

export async function createTerminal(sessionId: string, terminalId: string, rows: number, cols: number): Promise<string> {
  return invoke<string>('create_terminal', { sessionId, terminalId, rows, cols })
}

export async function closeTerminal(terminalId: string): Promise<void> {
  return invoke('close_terminal', { terminalId })
}

export async function writeStdin(terminalId: string, data: string): Promise<void> {
  return invoke('write_stdin', { terminalId, data })
}

export async function resizePty(terminalId: string, rows: number, cols: number): Promise<void> {
  return invoke('resize_pty', { terminalId, rows, cols })
}
