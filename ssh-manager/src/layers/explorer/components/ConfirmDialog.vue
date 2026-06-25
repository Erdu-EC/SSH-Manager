<script setup lang="ts">
defineProps<{
  title?: string
  description?: string
  confirmLabel?: string
  confirmColor?: 'error' | 'primary' | 'neutral' | 'success' | 'warning'
}>()

const emit = defineEmits<{
  close: [value: boolean]
}>()
</script>

<template>
  <UModal
    :dismissible="false"
    :ui="{ footer: 'justify-end' }"
  >
    <template #title>
      <div class="flex items-center gap-2">
        <span class="i-lucide-triangle-alert text-lg text-error shrink-0" />
        <span>{{ title }}</span>
      </div>
    </template>
    <template #description>
      {{ description }}
    </template>
    <template #footer>
      <UButton
        color="neutral"
        variant="outline"
        @click="emit('close', false)"
      >
        Cancel
      </UButton>
      <UButton
        :color="confirmColor || 'error'"
        @click="emit('close', true)"
      >
        {{ confirmLabel || 'Delete' }}
      </UButton>
    </template>
  </UModal>
</template>
