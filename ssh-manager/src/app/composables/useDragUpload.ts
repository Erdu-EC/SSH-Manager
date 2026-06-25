import { listen } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { LazyConfirmDialog } from '#components'

export function useDragUpload(sessionId: Ref<string>) {
  const transferStore = useTransferStore()
  const explorerStore = useExplorerStore()
  const toast = useToast()
  const overlay = useOverlay()
  const confirmDialog = overlay.create(LazyConfirmDialog)
  const targetPath = ref('')

  watch(() => explorerStore.perSession[sessionId.value]?.currentPath, (path) => {
    if (path) targetPath.value = path
  }, { immediate: true })

  async function onFileDrop(paths: string[]) {
    const uploadTarget = targetPath.value
    for (const localPath of paths) {
      const jobId = crypto.randomUUID()
      const fileName = localPath.split(/[/\\]/).pop() || 'unknown'
      const remotePath = `${uploadTarget}/${fileName}`

      try {
        const tauri = useTauri()
        await tauri.commands.explorer.getFileInfo(sessionId.value, remotePath)
        const overwrite = await confirmDialog.open({
          title: 'Overwrite file',
          description: `"${fileName}" already exists. Overwrite?`,
          confirmLabel: 'Overwrite',
          confirmColor: 'error'
        })
        if (!overwrite) continue
      } catch {
        // File doesn't exist, proceed
      }

      transferStore.addJob({
        jobId,
        sessionId: sessionId.value,
        localPath,
        remotePath,
        bytesTransferred: 0,
        totalBytes: null,
        status: 'in_progress',
        direction: 'upload'
      })

      const tauri = useTauri()
      tauri.commands.sftp.uploadFile(
        sessionId.value,
        jobId,
        localPath,
        remotePath
      )
        .then(() => {
          transferStore.completeJob(jobId)
          if (explorerStore.perSession[sessionId.value]?.currentPath === uploadTarget) {
            explorerStore.refresh(sessionId.value)
          }
        }).catch((err) => {
          transferStore.failJob(jobId)
          toast.add({
            title: 'Upload failed',
            description: String(err).slice(0, 200),
            color: 'error',
            icon: 'i-lucide-upload',
            duration: 6000
          })
        })
    }
  }

  function setTargetPath(path: string) {
    targetPath.value = path
  }

  async function listenForFileDrop(): Promise<() => void> {
    const webview = getCurrentWebview()
    const unlistenDrop = await webview.onDragDropEvent((event) => {
      if (event.payload.type === 'drop') {
        onFileDrop(event.payload.paths)
      }
    })

    const unlistenProgress = await listen<{
      jobId: string
      bytesTransferred: number
      totalBytes: number
    }>('upload-progress', (event) => {
      transferStore.updateProgress(
        event.payload.jobId,
        event.payload.bytesTransferred,
        event.payload.totalBytes
      )
    })

    return () => {
      unlistenDrop()
      unlistenProgress()
    }
  }

  return { onFileDrop, setTargetPath, listenForFileDrop }
}
