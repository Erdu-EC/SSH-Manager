import type { FileEntry } from '~/types'

interface ExplorerSession {
  currentPath: string
  entries: FileEntry[]
  loading: boolean
  breadcrumbs: { name: string, path: string }[]
}

export const useExplorerStore = defineStore('explorer', () => {
  const perSession = ref<Record<string, ExplorerSession>>({})
  const selectedPath = ref<string | null>(null)

  async function navigateTo(sessionId: string, path: string) {
    if (!perSession.value[sessionId]) {
      perSession.value[sessionId] = {
        currentPath: '/',
        entries: [],
        loading: false,
        breadcrumbs: [{ name: '/', path: '/' }]
      }
    }
    const session = perSession.value[sessionId]
    session.currentPath = path
    session.loading = true

    try {
      const tauri = useTauri()
      session.entries = await tauri.commands.explorer.listDirectory(sessionId, path)
      session.breadcrumbs = buildBreadcrumbs(path)
    } catch (e) {
      console.error('Failed to list directory:', e)
    } finally {
      session.loading = false
    }
  }

  async function refresh(sessionId: string) {
    const session = perSession.value[sessionId]
    if (session) {
      await navigateTo(sessionId, session.currentPath)
    }
  }

  async function createDirectory(sessionId: string, path: string) {
    const tauri = useTauri()
    await tauri.commands.explorer.createDirectory(sessionId, path)
    await refresh(sessionId)
  }

  async function removeEntry(sessionId: string, path: string, recursive = false) {
    const tauri = useTauri()
    await tauri.commands.explorer.removeFile(sessionId, path, recursive)
    await refresh(sessionId)
  }

  async function rename(sessionId: string, oldPath: string, newPath: string) {
    const tauri = useTauri()
    await tauri.commands.explorer.rename(sessionId, oldPath, newPath)
    await refresh(sessionId)
  }

  function selectFile(path: string | null) {
    selectedPath.value = path
  }

  function cleanupSession(sessionId: string) {
    // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
    delete perSession.value[sessionId]
  }

  async function loadChildren(sessionId: string, path: string): Promise<FileEntry[]> {
    try {
      const tauri = useTauri()
      return await tauri.commands.explorer.listDirectory(sessionId, path)
    } catch {
      return []
    }
  }

  return {
    perSession,
    selectedPath,
    navigateTo,
    refresh,
    createDirectory,
    removeEntry,
    rename,
    selectFile,
    cleanupSession,
    loadChildren
  }
})

function buildBreadcrumbs(path: string): { name: string, path: string }[] {
  const parts = path.split('/').filter(Boolean)
  const crumbs = [{ name: '/', path: '/' }]
  let current = ''
  for (const part of parts) {
    current += `/${part}`
    crumbs.push({ name: part, path: current })
  }
  return crumbs
}
