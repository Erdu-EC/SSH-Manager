import { LazyNewFolderDialog, LazyConfirmDialog } from '#components'
import type { FileEntry } from '~/types'
import type { Ref } from 'vue'

export function useExplorerActions(
  sessionId: string,
  entries: Ref<FileEntry[]>,
  currentPath: Ref<string>,
  selectedPath: Ref<string | null>,
  rowSelection: Ref<Record<string, boolean>>,
  contextEntry: Ref<FileEntry | null>,
  closeContextMenu: () => void,
  formatSize: (bytes: number) => string
) {
  const explorerStore = useExplorerStore()
  const overlay = useOverlay()
  const toast = useToast()

  const newFolderDialog = overlay.create(LazyNewFolderDialog)
  const confirmDialog = overlay.create(LazyConfirmDialog)

  function joinPath(parent: string, child: string): string {
    if (parent.endsWith('/')) return parent + child
    return parent + '/' + child
  }

  async function deleteSelected() {
    if (!selectedPath.value) return

    const name = selectedPath.value.split('/').pop() || ''
    const entry = entries.value.find(e => e.path === selectedPath.value)
    const isDir = entry?.is_directory ?? false

    const confirmed = await confirmDialog.open({
      title: isDir ? 'Delete directory' : 'Delete file',
      description: isDir
        ? `Are you sure you want to delete "${name}" and all its contents?`
        : `Are you sure you want to delete "${name}"?`,
      confirmLabel: isDir ? 'Delete directory' : 'Delete file',
      confirmColor: 'error'
    })

    if (!confirmed) return

    try {
      await explorerStore.removeEntry(sessionId, selectedPath.value, isDir)
      selectedPath.value = null
      rowSelection.value = {}
    } catch (e) {
      toast.add({
        title: 'Failed to delete',
        description: String(e).slice(0, 200),
        color: 'error',
        icon: 'i-lucide-trash-2',
        duration: 5000
      })
    }
  }

  async function handleRename() {
    const entry = contextEntry.value
    if (!entry) return
    closeContextMenu()
    const oldName = entry.name
    const dir = entry.path.slice(0, entry.path.length - oldName.length)
    const newName = window.prompt('Rename to:', oldName)
    if (newName && newName !== oldName) {
      try {
        await explorerStore.rename(sessionId, entry.path, dir + newName)
      } catch (e) {
        toast.add({
          title: 'Failed to rename',
          description: String(e).slice(0, 200),
          color: 'error',
          icon: 'i-lucide-file-edit',
          duration: 5000
        })
      }
    }
  }

  async function handleDeleteAction() {
    const entry = contextEntry.value
    if (!entry) return
    selectedPath.value = entry.path
    closeContextMenu()

    const isDir = entry.is_directory
    const confirmed = await confirmDialog.open({
      title: isDir ? 'Delete directory' : 'Delete file',
      description: isDir
        ? `Are you sure you want to delete "${entry.name}" and all its contents?`
        : `Are you sure you want to delete "${entry.name}"?`,
      confirmLabel: isDir ? 'Delete directory' : 'Delete file',
      confirmColor: 'error'
    })

    if (!confirmed) return

    try {
      await explorerStore.removeEntry(sessionId, entry.path, isDir)
      selectedPath.value = null
      rowSelection.value = {}
    } catch (e) {
      toast.add({
        title: 'Failed to delete',
        description: String(e).slice(0, 200),
        color: 'error',
        icon: 'i-lucide-trash-2',
        duration: 5000
      })
    }
  }

  function handleCopyPath() {
    const path = contextEntry.value?.path
    if (!path) return
    navigator.clipboard.writeText(path)
    toast.add({ title: 'Path copied to clipboard', color: 'neutral', duration: 2000, icon: 'i-lucide-check' })
    closeContextMenu()
  }

  function handleProperties() {
    const entry = contextEntry.value
    if (!entry) return
    const date = entry.modified_at ? new Date(entry.modified_at).toLocaleString() : '-'
    const size = entry.is_directory ? '-' : formatSize(entry.size ?? 0)
    toast.add({
      title: entry.name,
      description: `Path: ${entry.path}\nSize: ${size}\nModified: ${date}\nPermissions: ${entry.permissions ?? '-'}`,
      color: 'neutral',
      duration: 8000,
      icon: 'i-lucide-info'
    })
    closeContextMenu()
  }

  async function openNewFolder() {
    const folderName = await newFolderDialog.open()
    if (!folderName) return
    try {
      const path = joinPath(currentPath.value, folderName)
      await explorerStore.createDirectory(sessionId, path)
    } catch (e) {
      const msg = String(e)
      toast.add({
        title: 'Failed to create folder',
        description: msg.length > 200 ? msg.slice(0, 200) + '\u2026' : msg,
        color: 'error',
        icon: 'i-lucide-folder-plus',
        duration: 5000
      })
    }
  }

  return {
    deleteSelected,
    handleRename,
    handleDeleteAction,
    handleCopyPath,
    handleProperties,
    openNewFolder
  }
}
