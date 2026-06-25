<script setup lang="ts">
const { locale, locales, setLocale } = useI18n()

const flagIcons: Record<string, string> = {
  en: 'i-circle-flags-us',
  es: 'i-circle-flags-es'
}

const languageOptions = computed(() => {
  return locales.value.map((l) => {
    const code = typeof l === 'string' ? l : l.code
    return {
      label: code.toUpperCase(),
      value: code,
      icon: flagIcons[code] || 'i-lucide-globe'
    }
  })
})

const currentIcon = computed(() => flagIcons[locale.value] || 'i-lucide-globe')
</script>

<template>
  <USelect
    :model-value="locale"
    :items="languageOptions"
    :icon="currentIcon"
    size="sm"
    variant="ghost"
    class="w-24"
    @update:model-value="setLocale"
  />
</template>
