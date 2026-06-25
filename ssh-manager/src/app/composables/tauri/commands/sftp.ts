import { invoke } from '@tauri-apps/api/core'

export async function uploadFile(sessionId: string, jobId: string, localPath: string, remotePath: string): Promise<void> {
  return invoke('upload_file', { sessionId, jobId, localPath, remotePath })
}

export async function cancelTransfer(jobId: string): Promise<void> {
  return invoke('cancel_transfer', { jobId })
}
