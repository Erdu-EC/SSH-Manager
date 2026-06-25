<script setup lang="ts">
import type { AppProfile } from '~/types'

const props = defineProps<{
  profile: AppProfile & { source?: 'app' | 'ssh-config' }
  connecting?: boolean
}>()

const emit = defineEmits<{
  connect: [profile: AppProfile]
  edit: [profile: AppProfile]
  delete: [id: string]
}>()

const authIcon = computed(() => {
  switch (props.profile.auth_type) {
    case 'key': return 'i-lucide-key-round'
    case 'password': return 'i-lucide-lock'
    default: return 'i-lucide-shield'
  }
})

const sourceLabel = computed(() => {
  return props.profile.source === 'ssh-config' ? 'SSH Config' : 'App'
})
</script>

<template>
  <UCard
    variant="outline"
    class="group relative transition-all duration-200"
    :class="[connecting ? 'opacity-60 pointer-events-none' : 'cursor-pointer hover:shadow-sm hover:bg-elevated hover:border-primary/30']"
    @click="!connecting && emit('connect', profile)"
  >
    <div class="flex items-start justify-between">
      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-2 mb-1">
          <p class="font-semibold text-sm truncate">
            {{ profile.name }}
          </p>
          <UBadge
            size="xs"
            :color="profile.source === 'ssh-config' ? 'neutral' : 'primary'"
            variant="subtle"
          >
            {{ sourceLabel }}
          </UBadge>
        </div>
        <p class="text-xs text-muted font-mono truncate">
          {{ profile.username }}@{{ profile.host }}:{{ profile.port }}
        </p>
      </div>
      <UBadge
        v-if="connecting"
        color="primary"
        variant="subtle"
        size="sm"
        class="shrink-0 ml-2"
      >
        <UIcon
          name="i-lucide-loader"
          class="animate-spin mr-1"
        />
        {{ $t('profiles.connecting') }}
      </UBadge>
    </div>

    <div class="flex items-center gap-2 text-xs text-muted mt-2">
      <div :class="[authIcon, 'text-sm shrink-0']" />
      <span class="capitalize">{{ profile.auth_type }}</span>
      <span
        v-if="profile.identity_file"
        class="truncate ml-1 font-mono"
      >{{ profile.identity_file }}</span>
    </div>

    <div class="absolute top-3 right-3 hidden group-hover:flex items-center gap-1">
      <UTooltip :text="connecting ? $t('profiles.connecting') : $t('profiles.connect')">
        <UButton
          :icon="connecting ? 'i-lucide-loader' : 'i-lucide-plug'"
          size="xs"
          color="primary"
          variant="solid"
          :disabled="connecting"
          :class="{ 'animate-spin': connecting }"
          @click.stop="!connecting && emit('connect', profile)"
        />
      </UTooltip>
      <UTooltip :text="$t('profiles.edit')">
        <UButton
          v-if="profile.source === 'app'"
          icon="i-lucide-pencil"
          size="xs"
          color="neutral"
          variant="ghost"
          @click.stop="emit('edit', profile)"
        />
      </UTooltip>
      <UTooltip :text="$t('profiles.delete')">
        <UButton
          v-if="profile.source === 'app'"
          icon="i-lucide-trash"
          size="xs"
          color="error"
          variant="ghost"
          @click.stop="emit('delete', profile.id)"
        />
      </UTooltip>
    </div>
  </UCard>
</template>
