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
  await terminalStore.createTab(props.sessionId)
}

function closeTab(tabId: string) {
  terminalStore.closeTab(tabId)
}
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center shrink-0 overflow-x-auto">
      <UTabs
        v-if="tabs.length"
        v-model="activeModel"
        :items="tabs.map(t => ({ label: t.label, id: t.id }))"
        size="xs"
        class="flex-1 min-w-0"
        value-key="id"
      >
        <template #trailing>
          <UButton
            v-if="activeModel"
            icon="i-lucide-x"
            size="xs"
            color="neutral"
            variant="ghost"
            class="ml-auto"
            @click.stop="closeTab(activeModel)"
          />
        </template>
      </UTabs>
      <div class="px-1 shrink-0">
        <UButton
          icon="i-lucide-plus"
          size="xs"
          color="neutral"
          variant="ghost"
          title="New Terminal (Ctrl+Shift+T)"
          @click="addTerminal"
        />
      </div>
    </div>

    <template v-if="tabs.length === 0">
      <div class="flex-1 flex items-center justify-center">
        <UButton
          variant="outline"
          color="neutral"
          @click="addTerminal"
        >
          Open Terminal
        </UButton>
      </div>
    </template>
    <template v-else>
      <div class="flex-1 min-h-0 relative">
        <TerminalTab
          v-for="tab in tabs"
          :key="tab.id"
          :terminal-id="tab.id"
          :session-id="props.sessionId"
          :class="tab.id === terminalStore.activeTabId ? 'z-10' : 'opacity-0 pointer-events-none z-0'"
          class="absolute inset-0"
          @close="closeTab"
        />
      </div>
    </template>
  </div>
</template>
