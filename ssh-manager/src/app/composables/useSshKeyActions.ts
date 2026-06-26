export function useSshKeyActions() {
  async function showInFolder(path: string) {
    try {
      const tauri = useTauri()
      await tauri.commands.shell.showInFolder(path)
    } catch (e) {
      console.error('Failed to open folder:', e)
    }
  }

  return {
    showInFolder
  }
}
