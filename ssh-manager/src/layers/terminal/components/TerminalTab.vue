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

  setup(props.terminalId, terminal)

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
