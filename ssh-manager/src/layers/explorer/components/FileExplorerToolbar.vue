<script setup lang="ts">
const props = defineProps<{
  currentPath: string
  loading: boolean
  canGoBack: boolean
  selectedPath: string | null
}>()

const emit = defineEmits<{
  navigate: [path: string]
  refresh: []
  newFolder: []
  parentDir: []
  delete: []
}>()

const inputPath = ref('')
const editingPath = ref(false)
const inputContainerRef = ref<HTMLElement>()

function startEdit() {
  inputPath.value = props.currentPath
  editingPath.value = true
  nextTick(() => {
    const input = inputContainerRef.value?.querySelector('input')
    input?.focus()
    input?.select()
  })
}

function commitPath() {
  editingPath.value = false
  if (inputPath.value.trim()) {
    emit('navigate', inputPath.value)
  }
}
</script>

<template>
  <div class="flex items-center gap-1 px-3 py-1.5 shrink-0 bg-elevated/50 border-b border-(--ui-border)/75">
    <UButton
      icon="i-lucide-arrow-left"
      size="sm"
      color="neutral"
      variant="ghost"
      :disabled="!canGoBack"
      :ui="{
        base: 'disabled:opacity-40 disabled:cursor-not-allowed'
      }"
      title="Parent directory"
      @click="emit('parentDir')"
    />
    <div class="w-px h-5 bg-(--ui-border)/40 mx-1" />
    <UButton
      icon="i-lucide-refresh-cw"
      size="sm"
      color="neutral"
      variant="ghost"
      :loading="loading"
      title="Refresh (F5)"
      @click="emit('refresh')"
    />
    <UButton
      icon="i-lucide-folder-plus"
      size="sm"
      color="neutral"
      variant="ghost"
      title="New Folder"
      @click="emit('newFolder')"
    />
    <UButton
      icon="i-lucide-trash-2"
      size="sm"
      color="error"
      variant="ghost"
      :disabled="!selectedPath"
      title="Delete (Del)"
      @click="emit('delete')"
    />
    <div class="w-px h-5 bg-(--ui-border)/40 mx-1.5" />

    <div
      ref="inputContainerRef"
      class="flex-1 min-w-0"
    >
      <UInput
        v-if="editingPath"
        v-model="inputPath"
        size="sm"
        variant="subtle"
        class="w-full"
        @keyup.enter="commitPath"
        @blur="commitPath"
      />
      <div
        v-else
        class="truncate text-sm font-mono text-foreground/80 px-2.5 py-1 rounded-md cursor-pointer bg-transparent hover:bg-elevated transition-colors"
        @click="startEdit"
      >
        {{ currentPath }}
      </div>
    </div>
  </div>
</template>
