<script setup lang="ts">
import type { Command } from '~/components/common/CommandPalette.vue'
import { LazyCommonCommandPalette } from '#components'

definePageMeta({ layout: false })

const route = useRoute()

const colorMode = useColorMode()
const overlay = useOverlay()

const sessionId = computed(() => route.params.id as string)
const connectionStore = useConnectionStore()
const terminalStore = useTerminalStore()
const explorerStore = useExplorerStore()
const profileStore = useProfileStore()
const transferStore = useTransferStore()

const showTransfers = ref(false)

const activeTransfers = computed(() => transferStore.activeTransfers)
const allTransfers = computed(() => transferStore.transfers)

const transferProgress = computed(() => {
  const active = activeTransfers.value
  if (active.length === 0) return 0
  const total = active.reduce((sum, t) => sum + (t.totalBytes ?? 0), 0)
  const transferred = active.reduce((sum, t) => sum + t.bytesTransferred, 0)
  return total > 0 ? Math.round((transferred / total) * 100) : 0
})

const profileName = computed(() => {
  const s = session.value
  if (!s) return ''
  const match = profileStore.combinedProfiles.find(
    p => p.host === s.host && p.username === s.username
  )
  return match?.name ?? ''
})

const currentExplorerPath = computed(() =>
  explorerStore.perSession[sessionId.value]?.currentPath ?? '/'
)

const session = computed(() => connectionStore.sessions.get(sessionId.value))
const showTerminal = ref(false)
const terminalHeight = ref(450)
const isDraggingTerminal = ref(false)

function onTerminalDragStart(e: MouseEvent) {
  isDraggingTerminal.value = true
  document.addEventListener('mousemove', onTerminalDrag)
  document.addEventListener('mouseup', onTerminalDragEnd)
  document.body.style.userSelect = 'none'
  document.body.style.cursor = 'row-resize'
}

function onTerminalDrag(e: MouseEvent) {
  if (!isDraggingTerminal.value) return
  const newHeight = window.innerHeight - e.clientY
  terminalHeight.value = Math.max(150, Math.min(newHeight, window.innerHeight - 150))
}

function onTerminalDragEnd() {
  isDraggingTerminal.value = false
  document.removeEventListener('mousemove', onTerminalDrag)
  document.removeEventListener('mouseup', onTerminalDragEnd)
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
}

const { listenForFileDrop } = useDragUpload(sessionId)

const connectionDuration = ref('')
useIntervalFn(() => {
  if (!session.value?.connected_at) {
    connectionDuration.value = ''
    return
  }
  const start = new Date(session.value.connected_at).getTime()
  const elapsed = Date.now() - start
  const totalSec = Math.floor(elapsed / 1000)
  const hours = Math.floor(totalSec / 3600)
  const minutes = Math.floor((totalSec % 3600) / 60)
  const secs = totalSec % 60
  if (hours > 0) connectionDuration.value = `${hours}h ${minutes}m ${secs}s`
  else if (minutes > 0) connectionDuration.value = `${minutes}m ${secs}s`
  else connectionDuration.value = `${secs}s`
}, 1000, { immediate: true })
const palette = overlay.create(LazyCommonCommandPalette)

const dropCleanup = ref<(() => void) | null>(null)

onMounted(async () => {
  dropCleanup.value = await listenForFileDrop()
})

onUnmounted(() => {
  dropCleanup.value?.()
})

const commands: Command[] = [
  {
    id: 'toggle-terminal',
    label: 'Toggle Terminal',
    icon: 'i-lucide-terminal',
    shortcut: 'Ctrl+`',
    action: toggleTerminal
  },
  {
    id: 'new-terminal',
    label: 'New Terminal Tab',
    icon: 'i-lucide-plus',
    shortcut: 'Ctrl+Shift+T',
    action: () => terminalStore.createTab(sessionId.value, 40, 120, currentExplorerPath.value)
  },
  {
    id: 'disconnect',
    label: 'Disconnect Session',
    icon: 'i-lucide-log-out',
    shortcut: 'Ctrl+D',
    action: handleDisconnect
  }
]

onMounted(() => {
  if (!session.value) {
    navigateTo('/')
  }
})

function openPalette() {
  palette.open({ commands })
}

const hostname = computed(() => {
  const s = session.value
  return s ? `${s.username}@${s.host}` : 'Disconnected'
})

function toggleColorMode() {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

function toggleTerminal() {
  if (!showTerminal.value) {
    const existingTabs = terminalStore.tabsForSession(sessionId.value)
    if (existingTabs.length === 0) {
      terminalStore.createTab(sessionId.value, 40, 120, currentExplorerPath.value)
    }
  }
  showTerminal.value = !showTerminal.value
}

async function handleDisconnect() {
  await connectionStore.disconnect(sessionId.value)
  navigateTo('/')
}

useEventListener(window, 'keydown', (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    openPalette()
  }
})

useEventListener(window, 'keydown', (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === '`') {
    e.preventDefault()
    toggleTerminal()
  }
})

useEventListener(window, 'keydown', (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'T') {
    e.preventDefault()
    terminalStore.createTab(sessionId.value, 40, 120, currentExplorerPath.value)
  }
})

useEventListener(window, 'keydown', (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'd') {
    e.preventDefault()
    handleDisconnect()
  }
})
</script>

<template>
  <div
    class="flex flex-col h-screen overflow-hidden"
    @dragover.prevent
  >
    <header class="h-11 flex items-center px-3 gap-2 shrink-0 border-b border-default">
      <div class="flex items-center gap-2 text-sm flex-1 min-w-0">
        <span class="i-lucide-terminal text-primary shrink-0" />
        <span
          v-if="profileName"
          class="font-semibold truncate max-w-50 text-foreground"
        >{{ profileName }}</span>
        <span
          :class="profileName ? 'text-foreground/70' : 'font-semibold text-foreground'"
          class="truncate"
        >{{ hostname }}</span>
        <span class="text-xs text-dimmed/60 tabular-nums shrink-0 before:content-['·'] before:mr-2 before:text-dimmed/25">{{ connectionDuration }}</span>
      </div>
      <div class="flex items-center gap-0.5">
        <button
          class="relative w-8 h-8 flex items-center justify-center rounded-md transition-colors"
          :class="activeTransfers.length > 0 ? 'text-primary bg-primary/10 hover:bg-primary/15' : 'text-muted hover:bg-elevated'"
          :title="`Transfers (${activeTransfers.length} active)`"
          @click="showTransfers = true"
        >
          <svg
            class="w-5 h-5"
            viewBox="0 0 36 36"
          >
            <path
              d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              fill="none"
              stroke="currentColor"
              stroke-width="3"
              opacity="0.25"
            />
            <path
              v-if="activeTransfers.length > 0"
              d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              fill="none"
              stroke="currentColor"
              stroke-width="3"
              :stroke-dasharray="`${transferProgress}, 100`"
              stroke-linecap="round"
              class="transition-all duration-500"
            />
          </svg>
          <span
            v-if="activeTransfers.length"
            class="absolute -top-1 -right-1 w-4.5 h-4.5 flex items-center justify-center bg-primary text-[10px] font-bold text-white rounded-full ring-2 ring-background"
          >{{ activeTransfers.length }}</span>
        </button>
        <UTooltip :text="colorMode.value === 'dark' ? 'Light Mode' : 'Dark Mode'">
          <UButton
            :icon="colorMode.value === 'dark' ? 'i-lucide-moon' : 'i-lucide-sun'"
            size="xs"
            color="neutral"
            variant="ghost"
            @click="toggleColorMode"
          />
        </UTooltip>
        <UTooltip text="Command Palette (Ctrl+K)">
          <UButton
            icon="i-lucide-command"
            size="xs"
            color="neutral"
            variant="ghost"
            @click="openPalette"
          />
        </UTooltip>
        <UTooltip text="Toggle Terminal (Ctrl+`)">
          <UButton
            icon="i-lucide-terminal"
            size="xs"
            :color="showTerminal ? 'primary' : 'neutral'"
            variant="ghost"
            @click="toggleTerminal"
          />
        </UTooltip>
        <UTooltip text="Disconnect (Ctrl+D)">
          <UButton
            icon="i-lucide-log-out"
            size="xs"
            color="error"
            variant="ghost"
            @click="handleDisconnect"
          />
        </UTooltip>
      </div>
    </header>

    <div class="flex-1 flex flex-col overflow-hidden">
      <div class="flex-1 overflow-hidden">
        <FileExplorer :session-id="sessionId" />
      </div>

      <div
        v-if="showTerminal"
        class="h-1.5 z-10 -mt-1.5 cursor-row-resize bg-transparent hover:bg-primary/50 transition-colors"
        :class="{ 'bg-primary/50': isDraggingTerminal }"
        @mousedown.prevent="onTerminalDragStart"
      />

      <div
        class="shrink-0 overflow-hidden"
        :class="[
          showTerminal ? 'border-t border-[var(--ui-border)]' : '',
          isDraggingTerminal ? '' : 'transition-all duration-300 ease-in-out'
        ]"
        :style="{ height: showTerminal ? `${terminalHeight}px` : '0px' }"
      >
        <div class="h-full flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 py-1.5 border-b border-[var(--ui-border)] shrink-0">
            <span class="text-xs font-medium text-muted">Terminal</span>
            <UButton
              icon="i-lucide-x"
              size="xs"
              color="neutral"
              variant="ghost"
              @click="showTerminal = false"
            />
          </div>
          <div class="flex-1 min-h-0">
            <TerminalPanel :session-id="sessionId" />
          </div>
        </div>
      </div>
    </div>

    <UDrawer
      v-model:open="showTransfers"
      direction="right"
      :modal="true"
      :title="`Transfers ${activeTransfers.length > 0 ? `(${activeTransfers.length})` : ''}`"
      :description="allTransfers.length > 0 ? `${allTransfers.length} total transfers` : 'No transfers yet'"
      class="max-w-96"
    >
      <template #body>
        <div
          v-if="allTransfers.length === 0"
          class="flex items-center justify-center h-32 text-xs text-muted"
        >
          No transfers yet
        </div>
        <div
          v-else
          class="space-y-3"
        >
          <div
            v-for="job in allTransfers"
            :key="job.jobId"
            class="text-xs p-2 rounded-md bg-elevated/30 border border-(--ui-border)/50"
          >
            <div class="flex items-center justify-between mb-1">
              <span class="truncate text-muted flex items-center gap-1.5">
                <span
                  :class="job.direction === 'upload' ? 'i-lucide-upload' : 'i-lucide-download'"
                  class="text-sm shrink-0 text-foreground/70"
                />
                <span class="font-medium text-foreground truncate">{{ job.localPath.split(/[/\\]/).pop() }}</span>
              </span>
              <span class="text-foreground/60 shrink-0 ml-2 tabular-nums font-medium">
                <template v-if="job.status === 'in_progress' && job.totalBytes">
                  {{ Math.round(job.bytesTransferred / job.totalBytes * 100) }}%
                </template>
                <template v-else-if="job.status === 'completed'">
                  <span class="text-success">Done</span>
                </template>
                <template v-else-if="job.status === 'failed'">
                  <span class="text-error">Failed</span>
                </template>
                <template v-else>{{ job.status }}</template>
              </span>
              <UButton
                icon="i-lucide-x"
                size="xs"
                color="neutral"
                variant="ghost"
                class="-mr-1"
                @click="transferStore.removeJob(job.jobId)"
              />
            </div>
            <div
              v-if="job.status === 'in_progress'"
              class="w-full h-1.5 bg-muted/50 rounded-full overflow-hidden"
            >
              <div
                class="h-full bg-primary rounded-full transition-all duration-500"
                :style="{ width: job.totalBytes ? `${(job.bytesTransferred / job.totalBytes) * 100}%` : '1%' }"
              />
            </div>
          </div>
        </div>
      </template>
    </UDrawer>
  </div>
</template>
