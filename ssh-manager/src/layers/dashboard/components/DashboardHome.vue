<script setup lang="ts">
import type { AppProfile } from '~/types'
import { LazyConnectionForm } from '#components'

const { t } = useI18n()

const tabs = computed(() => [
  { label: t('dashboard.profiles'), icon: 'i-lucide-server', id: 'profiles' },
  { label: t('dashboard.sshConfig'), icon: 'i-lucide-file-text', id: 'config' },
  { label: t('dashboard.sshKeys'), icon: 'i-lucide-key-round', id: 'keys' }
])

const activeTab = ref('profiles')

const overlay = useOverlay()
const connectionForm = overlay.create(LazyConnectionForm)

async function openNewProfile() {
  await connectionForm.open({ profile: null })
}

async function openEditProfile(profile: AppProfile) {
  await connectionForm.open({ profile })
}
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex items-center justify-between mb-6">
      <UTabs
        v-model="activeTab"
        :items="tabs"
        size="sm"
        value-key="id"
      />
      <UButton
        v-if="activeTab === 'profiles'"
        icon="i-lucide-plus"
        size="sm"
        color="primary"
        variant="solid"
        @click="openNewProfile"
      >
        {{ $t('dashboard.newProfile') }}
      </UButton>
    </div>

    <div class="flex-1">
      <ProfileGrid
        v-if="activeTab === 'profiles'"
        @edit="openEditProfile"
      />
      <SshConfigView v-else-if="activeTab === 'config'" />
      <SshKeysSshKeyList v-else-if="activeTab === 'keys'" />
    </div>
  </div>
</template>
