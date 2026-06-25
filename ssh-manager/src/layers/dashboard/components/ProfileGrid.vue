<script setup lang="ts">
import type { AppProfile } from '~/types'

const profileStore = useProfileStore()
const connectionStore = useConnectionStore()
const { t } = useI18n()

const emit = defineEmits<{
  edit: [profile: AppProfile]
}>()

const profiles = computed(() => profileStore.combinedProfiles)
const loading = computed(() => profileStore.loading)

const toast = useToast()

const connectingProfile = ref<string | null>(null)

async function handleConnect(profile: AppProfile) {
  connectingProfile.value = profile.id
  try {
    const sessionId = await connectionStore.connect({
      host: profile.host,
      port: profile.port,
      username: profile.username,
      auth_type: profile.auth_type,
      password: null,
      identity_file: profile.identity_file
    })
    navigateTo(`/session/${sessionId}`)
  } catch (e) {
    const msg = String(e)
    const short = msg.length > 120 ? msg.slice(0, 120) + '…' : msg
    toast.add({
      title: t('profiles.connectionFailed'),
      description: short,
      color: 'error',
      icon: 'i-lucide-plug',
      duration: 8000
    })
  } finally {
    connectingProfile.value = null
  }
}

async function handleDelete(id: string) {
  try {
    await profileStore.deleteProfile(id)
  } catch (e) {
    toast.add({
      title: t('profiles.deleteFailed'),
      description: String(e),
      color: 'error',
      icon: 'i-lucide-trash',
      duration: 5000
    })
  }
}
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
