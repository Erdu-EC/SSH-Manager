import type { ConnectionParams, SessionInfo } from '~/types'

export const useConnectionStore = defineStore('connection', () => {
  const sessions = ref(new Map<string, SessionInfo>())
  const activeSessionId = ref<string | null>(null)
  const connecting = ref(false)
  const error = ref<string | null>(null)

  const activeSession = computed(() => {
    if (!activeSessionId.value) return null
    return sessions.value.get(activeSessionId.value) || null
  })

  const isConnected = computed(() => activeSessionId.value !== null)

  async function connect(params: ConnectionParams) {
    connecting.value = true
    error.value = null
    try {
      const tauri = useTauri()
      const sessionId = await tauri.commands.connection.connect(params)
      const session: SessionInfo = {
        id: sessionId,
        host: params.host,
        username: params.username,
        connected_at: new Date().toISOString()
      }
      sessions.value.set(sessionId, session)
      activeSessionId.value = sessionId
      return sessionId
    } catch (e) {
      error.value = String(e)
      throw e
    } finally {
      connecting.value = false
    }
  }

  async function disconnect(sessionId?: string) {
    const id = sessionId || activeSessionId.value
    if (!id) return
    try {
      const tauri = useTauri()
      await tauri.commands.connection.disconnect(id)
      sessions.value.delete(id)
      if (activeSessionId.value === id) {
        activeSessionId.value = sessions.value.size > 0
          ? sessions.value.keys().next().value || null
          : null
      }
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  function setActiveSession(id: string | null) {
    activeSessionId.value = id
  }

  async function refreshSessions() {
    try {
      const tauri = useTauri()
      const sessionList = await tauri.commands.connection.getActiveSessions()
      sessions.value.clear()
      for (const s of sessionList) {
        sessions.value.set(s.id, s)
      }
      if (activeSessionId.value && !sessions.value.has(activeSessionId.value)) {
        activeSessionId.value = sessions.value.size > 0
          ? sessions.value.keys().next().value || null
          : null
      }
    } catch (e) {
      error.value = String(e)
    }
  }

  return {
    sessions,
    activeSessionId,
    connecting,
    error,
    activeSession,
    isConnected,
    connect,
    disconnect,
    setActiveSession,
    refreshSessions
  }
})
