<script setup lang="ts">
const emit = defineEmits<{
  close: [name: string]
}>()

const folderName = ref('')
const inputRef = ref<HTMLInputElement>()

onMounted(() => {
  nextTick(() => inputRef.value?.focus())
})

function submit() {
  if (folderName.value.trim()) {
    emit('close', folderName.value.trim())
  }
}
</script>

<template>
  <UModal
    title="New Folder"
    @close:prevent="emit('close', '')"
  >
    <template #body>
      <UForm
        :state="{ name: folderName }"
        @submit="submit"
      >
        <UFormField label="Folder name">
          <UInput
            ref="inputRef"
            v-model="folderName"
            placeholder="My Folder"
            class="w-full"
          />
        </UFormField>
      </UForm>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton
          color="neutral"
          variant="outline"
          @click="emit('close', '')"
        >
          Cancel
        </UButton>
        <UButton @click="submit">
          Create
        </UButton>
      </div>
    </template>
  </UModal>
</template>
