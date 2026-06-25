<script setup lang="ts">
const profileStore = useProfileStore()

const entries = computed(() => profileStore.sshConfig)
const loading = computed(() => profileStore.loading)
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
        <div class="h-20 animate-pulse bg-muted/20 rounded" />
      </UCard>
    </div>

    <div v-else-if="entries.length === 0">
      <CommonEmptyState
        icon="i-lucide-file-text"
        title="No SSH Config Found"
        description="No ~/.ssh/config file found or it's empty."
      />
    </div>

    <div
      v-else
      class="grid grid-cols-1 md:grid-cols-2 gap-3"
    >
      <UCard
        v-for="entry in entries"
        :key="entry.host"
        variant="outline"
      >
        <div class="flex items-center justify-between mb-2">
          <div class="flex items-center gap-2">
            <span class="i-lucide-file-text text-sm text-muted" />
            <p class="font-semibold text-sm">
              {{ entry.host }}
            </p>
          </div>
          <UBadge
            v-if="entry.host === '*'"
            size="xs"
            color="neutral"
            variant="subtle"
          >
            Default
          </UBadge>
        </div>
        <div class="text-xs text-muted space-y-0.5 font-mono">
          <p v-if="entry.host_name">
            HostName {{ entry.host_name }}
          </p>
          <p v-if="entry.user">
            User {{ entry.user }}
          </p>
          <p v-if="entry.port">
            Port {{ entry.port }}
          </p>
          <p
            v-if="entry.identity_file"
            class="truncate"
          >
            IdentityFile {{ entry.identity_file }}
          </p>
          <p v-if="entry.proxy_jump">
            ProxyJump {{ entry.proxy_jump }}
          </p>
          <p
            v-if="entry.proxy_command"
            class="truncate"
          >
            ProxyCommand {{ entry.proxy_command }}
          </p>
        </div>
      </UCard>
    </div>
  </div>
</template>
