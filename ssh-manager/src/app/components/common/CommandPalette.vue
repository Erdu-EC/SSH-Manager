<script setup lang="ts">
export interface Command {
  id: string
  label: string
  icon: string
  shortcut?: string
  action: () => void
}

const props = defineProps<{
  commands?: Command[]
}>()

const emit = defineEmits<{
  close: []
  execute: [id: string]
}>()

const search = ref('')
const inputRef = ref<HTMLInputElement>()

const filtered = computed(() => {
  if (!search.value) return props.commands ?? []
  const q = search.value.toLowerCase()
  return (props.commands ?? []).filter(c =>
    c.label.toLowerCase().includes(q)
  )
})

function execute(cmd: Command) {
  cmd.action()
  emit('execute', cmd.id)
}

onMounted(() => {
  nextTick(() => inputRef.value?.focus())
})
</script>

<template>
  <UModal @close:prevent="emit('close')">
    <div class="p-3 min-w-80">
      <div class="flex items-center gap-2 border-b border-[var(--ui-border)] pb-2 mb-2">
        <span class="i-lucide-search text-muted shrink-0" />
        <input
          ref="inputRef"
          v-model="search"
          type="text"
          placeholder="Type a command..."
          class="flex-1 bg-transparent outline-none text-sm"
        >
        <kbd class="text-xs text-muted bg-elevated px-1.5 py-0.5 rounded">ESC</kbd>
      </div>

      <div
        v-if="filtered.length === 0"
        class="text-sm text-muted py-4 text-center"
      >
        No commands found
      </div>

      <div
        v-else
        class="space-y-0.5 max-h-80 overflow-y-auto"
      >
        <div
          v-for="cmd in filtered"
          :key="cmd.id"
          class="flex items-center gap-2 px-2 py-1.5 rounded hover:bg-muted/10 cursor-pointer text-sm"
          @click="execute(cmd)"
        >
          <span :class="[cmd.icon, 'text-sm shrink-0']" />
          <span class="flex-1">{{ cmd.label }}</span>
          <kbd
            v-if="cmd.shortcut"
            class="text-xs text-muted"
          >{{ cmd.shortcut }}</kbd>
        </div>
      </div>
    </div>
  </UModal>
</template>
