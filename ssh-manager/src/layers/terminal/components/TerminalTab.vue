<script setup lang="ts">
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'

const props = defineProps<{
  terminalId: string
  sessionId: string
}>()

defineEmits<{
  close: [terminalId: string]
}>()

const termRef = ref<HTMLDivElement>()
let terminal: Terminal | null = null
let fitAddon: FitAddon | null = null
let resizeObserver: ResizeObserver | null = null

const { setup, cleanup } = useTerminalSync()

onMounted(async () => {
  if (!termRef.value) return

  // Wait for web fonts to load.
  await document.fonts.ready

  // Ensure the container is actually visible (not hidden or 0 height due to transitions)
  while (termRef.value && (termRef.value.offsetHeight === 0 || termRef.value.offsetWidth === 0)) {
    await new Promise(resolve => setTimeout(resolve, 50))
  }

  if (!termRef.value) return

  fitAddon = new FitAddon()
  terminal = new Terminal({
    cursorBlink: true,
    cursorStyle: 'block',
    fontSize: 14,
    fontFamily: '\'JetBrains Mono\', \'Fira Code\', \'Cascadia Code\', monospace',
    allowTransparency: true,
    theme: {
      background: 'transparent',
      foreground: '#d4d4d4',
      cursor: '#10b981',
      cursorAccent: '#10b981',
      selectionBackground: 'rgba(16, 185, 129, 0.3)',
      black: '#000000',
      red: '#ef4444',
      green: '#10b981',
      yellow: '#f59e0b',
      blue: '#3b82f6',
      magenta: '#8b5cf6',
      cyan: '#06b6d4',
      white: '#ffffff',
      brightBlack: '#666666',
      brightRed: '#f87171',
      brightGreen: '#34d399',
      brightYellow: '#fbbf24',
      brightBlue: '#60a5fa',
      brightMagenta: '#a78bfa',
      brightCyan: '#22d3ee',
      brightWhite: '#ffffff'
    }
  })

  terminal.loadAddon(fitAddon)

  // Only open once the container is ready
  terminal.open(termRef.value)

  // Try fitting immediately to get exact rows and columns based on the UI
  try {
    fitAddon.fit()
  } catch (_e) {
    // Ignore fit errors if container is still too small
  }

  // 1. Attach listeners FIRST so no data is dropped (terminal-output from backend to frontend)
  setup(props.terminalId, terminal)

  const terminalStore = useTerminalStore()

  // 2. NOW request the backend to create the PTY and start sending data.
  // Because listeners are attached and size is exact, the MOTD will format perfectly!
  await terminalStore.initTerminal(props.sessionId, props.terminalId, terminal.rows, terminal.cols)

  // 4. Now that backend has created the terminal, we can safely send resize and input events
  terminal.onResize((size) => {
    terminalStore.resizeTab(props.terminalId, size.rows, size.cols)
  })

  if (termRef.value) {
    resizeObserver = new ResizeObserver(() => {
      if (termRef.value && termRef.value.offsetHeight > 20 && termRef.value.offsetWidth > 20) {
        requestAnimationFrame(() => fitAddon?.fit())
      }
    })
    resizeObserver.observe(termRef.value)
  }
})

onUnmounted(() => {
  resizeObserver?.disconnect()
  terminal?.dispose()
  cleanup()
  terminal = null
  fitAddon = null
})
</script>

<template>
  <div
    ref="termRef"
    class="h-full w-full overflow-hidden"
  />
</template>

<style>
.xterm-helper-textarea {
  position: absolute !important;
  left: -9999px !important;
  top: 0 !important;
  opacity: 0 !important;
  width: 1px !important;
  height: 1px !important;
  z-index: -1 !important;
  pointer-events: none !important;
}
</style>
