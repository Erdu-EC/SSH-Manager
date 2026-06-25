import type { TransferJob } from '~/types'

export const useTransferStore = defineStore('transfer', () => {
  const transfers = ref<TransferJob[]>([])

  const activeTransfers = computed(() => transfers.value.filter(t => t.status === 'in_progress'))
  const completedTransfers = computed(() => transfers.value.filter(t => t.status === 'completed'))

  function addJob(job: TransferJob) {
    transfers.value.push(job)
  }

  function updateProgress(jobId: string, bytesTransferred: number, totalBytes: number | null) {
    const job = transfers.value.find(t => t.jobId === jobId)
    if (job) {
      job.bytesTransferred = bytesTransferred
      if (totalBytes !== null) {
        job.totalBytes = totalBytes
      }
    }
  }

  function completeJob(jobId: string) {
    const job = transfers.value.find(t => t.jobId === jobId)
    if (job) {
      job.status = 'completed'
    }
  }

  function failJob(jobId: string) {
    const job = transfers.value.find(t => t.jobId === jobId)
    if (job) {
      job.status = 'failed'
    }
  }

  async function removeJob(jobId: string) {
    const idx = transfers.value.findIndex(t => t.jobId === jobId)
    if (idx !== -1) {
      const job = transfers.value[idx]
      if (job && job.status === 'in_progress') {
        try {
          const tauri = useTauri()
          await tauri.commands.sftp.cancelTransfer(jobId)

          if (job.direction === 'upload') {
            const explorerStore = useExplorerStore()
            const currentPath = explorerStore.perSession[job.sessionId]?.currentPath
            if (currentPath) {
              const lastSlashIdx = job.remotePath.lastIndexOf('/')
              const parentDir = lastSlashIdx > 0 ? job.remotePath.substring(0, lastSlashIdx) : '/'

              if (currentPath === parentDir) {
                explorerStore.refresh(job.sessionId)
              }
            }
          }
        } catch (e) {
          console.error('Failed to cancel transfer:', e)
        }
      }
      transfers.value.splice(idx, 1)
    }
  }

  return {
    transfers,
    activeTransfers,
    completedTransfers,
    addJob,
    updateProgress,
    completeJob,
    failJob,
    removeJob
  }
})
