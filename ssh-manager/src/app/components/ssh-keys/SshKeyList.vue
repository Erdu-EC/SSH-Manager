<script setup lang="ts">
const profileStore = useProfileStore()

const keys = computed(() => profileStore.sshKeys)
const loading = computed(() => profileStore.loading)

function formatTimestamp(ts: string): string {
  if (!ts) return ''
  const seconds = Number(ts)
  if (isNaN(seconds)) return ts
  return new Date(seconds * 1000).toLocaleDateString()
}

async function showInFolder(path: string) {
  try {
    const tauri = useTauri()
    await tauri.commands.shell.showInFolder(path)
  } catch (e) {
    console.error('Failed to open folder:', e)
  }
}
</script>

<template>
  <div>
    <div
      v-if="loading"
      class="grid grid-cols-1 md:grid-cols-2 gap-3"
    >
      <UCard
        v-for="i in 4"
        :key="i"
        variant="outline"
      >
        <div class="h-16 animate-pulse bg-muted/20 rounded" />
      </UCard>
    </div>

    <div v-else-if="keys.length === 0">
      <CommonEmptyState
        icon="i-lucide-key-round"
        title="No SSH Keys Found"
        description="No keys found in ~/.ssh/ directory."
      />
    </div>

    <div
      v-else
      class="grid grid-cols-1 md:grid-cols-2 gap-3"
    >
      <UCard
        v-for="key in keys"
        :key="key.filename"
        variant="outline"
        class="group cursor-pointer transition-all duration-200 hover:shadow-sm"
        @click="showInFolder(key.path)"
      >
        <div class="flex items-center gap-3">
          <div
            :class="[
              'text-xl shrink-0',
              key.is_public ? 'i-lucide-file-key text-primary' : 'i-lucide-key-round text-warning'
            ]"
          />
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <p class="text-sm font-semibold truncate">
                {{ key.filename }}
              </p>
              <UBadge
                size="xs"
                :color="key.is_public ? 'primary' : 'warning'"
                variant="subtle"
              >
                {{ key.is_public ? 'Public' : 'Private' }}
              </UBadge>
            </div>
            <div class="flex items-center gap-2 text-xs text-muted mt-0.5">
              <span v-if="key.key_type">{{ key.key_type }}</span>
              <span
                v-if="key.fingerprint"
                class="truncate font-mono"
              >{{ key.fingerprint }}</span>
              <span v-if="key.modified_at">{{ formatTimestamp(key.modified_at) }}</span>
            </div>
          </div>
          <div class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
            <UButton
              icon="i-lucide-folder-open"
              size="xs"
              color="neutral"
              variant="ghost"
              title="Show in folder"
            />
          </div>
        </div>
      </UCard>
    </div>
  </div>
</template>
