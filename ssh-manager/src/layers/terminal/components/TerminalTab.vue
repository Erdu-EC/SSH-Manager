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
const cleanupFns: (() => void)[] = []
const { call, onEvent } = useTauri()

onMounted(() => {
  if (!termRef.value) return

  fitAddon = new FitAddon()
  terminal = new Terminal({
    cursorBlink: true,
    cursorStyle: 'block',
    fontSize: 14,
    fontFamily: '\'JetBrains Mono\', \'Fira Code\', \'Cascadia Code\', monospace',
    allowTransparency: true,
    theme: {
      background: 'transparent'
    }
  })

  terminal.loadAddon(fitAddon)
  terminal.open(termRef.value)
  fitAddon.fit()

  terminal.onData((data: string) => {
    call('write_stdin', { terminalId: props.terminalId, data }).catch(console.error)
  })

  const unlistenOutput = onEvent<{ terminalId: string, data: string }>('terminal-output', (payload) => {
    if (payload.terminalId === props.terminalId && terminal) {
      const binary = atob(payload.data)
      const bytes = new Uint8Array(binary.length)
      for (let i = 0; i < binary.length; i++) {
        bytes[i] = binary.charCodeAt(i)
      }
      terminal.write(bytes)
    }
  })
  cleanupFns.push(unlistenOutput)

  const unlistenError = onEvent<{ terminalId: string, message: string }>('terminal-error', (payload) => {
    if (payload.terminalId === props.terminalId && terminal) {
      terminal.writeln(`\r\n\x1b[31mError: ${payload.message}\x1b[0m`)
    }
  })
  cleanupFns.push(unlistenError)

  const unlistenExit = onEvent<{ terminalId: string, exitCode: number }>('terminal-exit', (payload) => {
    if (payload.terminalId === props.terminalId && terminal) {
      terminal.writeln(`\r\n\x1b[33mProcess exited with code ${payload.exitCode}\x1b[0m`)
    }
  })
  cleanupFns.push(unlistenExit)

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
  cleanupFns.forEach(fn => fn())
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
