<script setup lang="ts">
const colorMode = useColorMode()
const searchMode = ref(false)

function toggleColorMode() {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

function openSearch() {
  searchMode.value = true
}

const { t } = useI18n()

const searchGroups = computed(() => [
  {
    id: 'navigation',
    label: t('layout.navigation'),
    items: [
      { id: 'dashboard', label: t('layout.dashboard'), icon: 'i-lucide-layout-dashboard', to: '/' }
    ]
  }
])
</script>

<template>
  <UDashboardGroup>
    <UDashboardPanel>
      <template #header>
        <UDashboardNavbar
          title="SSH Explorer"
          icon="i-lucide-terminal"
        >
          <template #right>
            <UButton
              icon="i-lucide-search"
              size="sm"
              color="neutral"
              variant="ghost"
              :title="$t('layout.search')"
              @click="openSearch"
            />
            <UButton
              :icon="colorMode.value === 'dark' ? 'i-lucide-moon' : 'i-lucide-sun'"
              size="sm"
              color="neutral"
              variant="ghost"
              @click="toggleColorMode"
            />
            <CommonLanguageSwitcher />
          </template>
        </UDashboardNavbar>
      </template>
      <template #body>
        <div class="p-4 h-full">
          <slot />
        </div>
      </template>
    </UDashboardPanel>

    <UDashboardSearch
      v-model:open="searchMode"
      :groups="searchGroups"
    />
  </UDashboardGroup>
</template>
