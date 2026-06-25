import type { TerminalTabState } from '~/types'

export const useTerminalStore = defineStore('terminal', () => {
  const tabs = ref<TerminalTabState[]>([])
  const activeTabId = ref<string | null>(null)

  const activeTab = computed(() => {
    if (!activeTabId.value) return null
    return tabs.value.find(t => t.id === activeTabId.value) || null
  })

  const tabsForSession = computed(() => {
    return (sessionId: string) => tabs.value.filter(t => t.sessionId === sessionId)
  })

  async function createTab(sessionId: string, rows = 40, cols = 120, cwd?: string) {
    try {
      const tauri = useTauri()
      const terminalId = await tauri.commands.terminal.createTerminal(
        sessionId,
        rows,
        cols,
        cwd ?? null
      )
      const tab: TerminalTabState = {
        id: terminalId,
        sessionId,
        label: `Term ${tabs.value.length + 1}`
      }
      tabs.value.push(tab)
      activeTabId.value = terminalId
      return terminalId
    } catch (e) {
      console.error('Failed to create terminal:', e)
      return null
    }
  }

  async function closeTab(tabId: string) {
    try {
      const tauri = useTauri()
      await tauri.commands.terminal.closeTerminal(tabId)
    } catch (e) {
      console.error('Failed to close terminal:', e)
    }
    const idx = tabs.value.findIndex(t => t.id === tabId)
    if (idx !== -1) {
      tabs.value.splice(idx, 1)
    }
    if (activeTabId.value === tabId) {
      activeTabId.value = tabs.value.length > 0
        ? (tabs.value[Math.min(idx, tabs.value.length - 1)]?.id ?? null)
        : null
    }
  }

  function setActiveTab(id: string) {
    activeTabId.value = id
  }

  async function writeToTab(tabId: string, data: string) {
    try {
      const tauri = useTauri()
      await tauri.commands.terminal.writeStdin(tabId, data)
    } catch (e) {
      console.error('Failed to write to terminal:', e)
    }
  }

  async function resizeTab(tabId: string, rows: number, cols: number) {
    try {
      const tauri = useTauri()
      await tauri.commands.terminal.resizePty(tabId, rows, cols)
    } catch (e) {
      console.error('Failed to resize terminal:', e)
    }
  }

  function cleanupSession(sessionId: string) {
    const tabsToRemove = tabs.value.filter(t => t.sessionId === sessionId)
    for (const tab of tabsToRemove) {
      const idx = tabs.value.indexOf(tab)
      tabs.value.splice(idx, 1)
    }
    if (activeTabId.value && !tabs.value.find(t => t.id === activeTabId.value)) {
      activeTabId.value = tabs.value.length > 0 ? (tabs.value[0]?.id ?? null) : null
    }
  }

  return {
    tabs,
    activeTabId,
    activeTab,
    tabsForSession,
    createTab,
    closeTab,
    setActiveTab,
    writeToTab,
    resizeTab,
    cleanupSession
  }
})
