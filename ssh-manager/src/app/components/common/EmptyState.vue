<script setup lang="ts">
defineProps<{
  icon?: string
  title: string
  description?: string
}>()
</script>

<template>
  <div class="flex flex-col items-center justify-center py-16 text-center">
    <div
      v-if="icon"
      :class="[icon, 'text-4xl text-muted mb-4 opacity-50']"
    />
    <p class="text-base font-semibold text-foreground">
      {{ title }}
    </p>
    <p
      v-if="description"
      class="text-sm text-muted mt-1.5 max-w-sm"
    >
      {{ description }}
    </p>
  </div>
</template>
