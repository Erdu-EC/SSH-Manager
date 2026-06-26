<script setup lang="ts">
const props = defineProps<{
  sessionId: string
}>()

const terminalStore = useTerminalStore()

const tabs = computed(() => terminalStore.tabs.filter(t => t.sessionId === props.sessionId))
const activeModel = computed({
  get: () => terminalStore.activeTabId ?? undefined,
  set: (val: string | undefined) => {
    if (val) terminalStore.setActiveTab(val)
  }
})

async function addTerminal() {
  terminalStore.createTab(props.sessionId)
}

function closeTab(tabId: string) {
  terminalStore.closeTab(tabId)
}
</script>

<template>
  <div class="flex flex-col h-full bg-default">
    <!-- Tab Bar -->
    <div
      v-if="tabs.length > 0"
      class="flex items-center shrink-0 overflow-x-auto bg-elevated border-b border-default gap-2"
    >
      <UTabs
        v-if="tabs.length"
        v-model="activeModel"
        :items="tabs.map(t => ({ label: t.label, id: t.id, icon: 'i-lucide-terminal' }))"
        size="sm"
        color="neutral"
        variant="link"
        class="min-w-0 gap-0.5 flex-none"
        value-key="id"
      >
        <template #trailing>
          <UButton
            v-if="activeModel"
            icon="i-lucide-x"
            size="xs"
            color="neutral"
            variant="ghost"
            class="ml-auto opacity-60 hover:opacity-100 hover:text-red-500 transition-colors"
            title="Close active tab"
            @click.stop="closeTab(activeModel)"
          />
        </template>
      </UTabs>

      <div class="shrink-0 flex items-center border-l border-default pl-2 ml-1">
        <UButton
          icon="i-lucide-plus"
          size="xs"
          color="primary"
          variant="soft"
          class="rounded-md w-7 h-7 flex items-center justify-center p-0 transition-all hover:scale-105 active:scale-95 shadow-sm"
          title="New Terminal"
          @click="addTerminal"
        />
      </div>
    </div>

    <!-- Empty State -->
    <template v-if="tabs.length === 0">
      <div class="flex-1 flex flex-col items-center justify-center gap-4 bg-muted/10">
        <div
          class="p-4 rounded-full bg-[var(--ui-bg-elevated)] shadow-sm border border-[var(--ui-border)] flex items-center justify-center"
        >
          <UIcon
            name="i-lucide-terminal"
            class="w-10 h-10 text-muted opacity-80"
          />
        </div>
        <p class="text-sm font-medium text-muted">
          No active terminal sessions
        </p>
        <UButton
          variant="solid"
          color="primary"
          icon="i-lucide-plus"
          class="transition-all hover:scale-105 shadow-sm"
          @click="addTerminal"
        >
          Open Terminal
        </UButton>
      </div>
    </template>

    <!-- Terminal Area -->
    <template v-else>
      <div class="flex-1 min-h-0 relative bg-[#0d1117] dark:bg-black/60 shadow-inner">
        <!-- Subtle gradient overlay to make it look premium -->
        <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-black/10 to-transparent z-20 h-4" />

        <TerminalTab
          v-for="tab in tabs"
          :key="tab.id"
          :terminal-id="tab.id"
          :session-id="props.sessionId"
          :class="tab.id === terminalStore.activeTabId ? 'z-10 opacity-100' : 'opacity-0 pointer-events-none z-0'"
          class="absolute inset-0 p-3 transition-opacity duration-200 ease-in-out"
          @close="closeTab"
        />
      </div>
    </template>
  </div>
</template>
