interface Shortcut {
  key: string
  ctrl?: boolean
  meta?: boolean
  shift?: boolean
  handler: () => void
}

export function useKeyboardShortcuts() {
  const shortcuts = ref<Shortcut[]>([])

  function register(shortcut: Shortcut) {
    shortcuts.value.push(shortcut)
  }

  function registerMultiple(newShortcuts: Shortcut[]) {
    shortcuts.value.push(...newShortcuts)
  }

  function handleKeydown(e: KeyboardEvent) {
    for (const s of shortcuts.value) {
      const ctrlOrMeta = s.ctrl || s.meta
      const matchCtrl = ctrlOrMeta ? (e.ctrlKey || e.metaKey) : (!e.ctrlKey && !e.metaKey)
      if (
        e.key.toLowerCase() === s.key.toLowerCase()
        && matchCtrl
        && (s.shift ? e.shiftKey : !e.shiftKey)
      ) {
        e.preventDefault()
        s.handler()
        return
      }
    }
  }

  onMounted(() => {
    window.addEventListener('keydown', handleKeydown)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
  })

  return { register, registerMultiple }
}
