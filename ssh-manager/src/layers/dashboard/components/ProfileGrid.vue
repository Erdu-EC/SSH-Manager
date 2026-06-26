<script setup lang="ts">
import type { AppProfile } from '~/types'

const profileStore = useProfileStore()
const { connectingProfile, handleConnect, handleDelete } = useProfileActions()

const emit = defineEmits<{
  edit: [profile: AppProfile]
}>()

const profiles = computed(() => profileStore.combinedProfiles)
const loading = computed(() => profileStore.loading)
</script>

<template>
  <div>
    <div
      v-if="loading"
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3"
    >
      <div
        v-for="i in 6"
        :key="i"
        class="h-28 rounded-lg bg-elevated animate-pulse"
      />
    </div>

    <div v-else-if="profiles.length === 0">
      <CommonEmptyState
        icon="i-lucide-server"
        :title="$t('profiles.noProfiles')"
        :description="$t('profiles.addProfileHint')"
      />
    </div>

    <div
      v-else
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3"
    >
      <ProfileCard
        v-for="profile in profiles"
        :key="profile.id"
        :profile="profile"
        :connecting="connectingProfile === profile.id"
        @connect="handleConnect"
        @edit="emit('edit', $event)"
        @delete="handleDelete"
      />
    </div>
  </div>
</template>
