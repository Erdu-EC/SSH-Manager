import type { AppProfile, SshConfigEntry, SshKeyInfo } from '~/types'

export const useProfileStore = defineStore('profile', () => {
  const appProfiles = ref<AppProfile[]>([])
  const sshConfig = ref<SshConfigEntry[]>([])
  const sshKeys = ref<SshKeyInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const combinedProfiles = computed(() => {
    const app = appProfiles.value.map(p => ({
      ...p,
      source: 'app' as const
    }))
    const config = sshConfig.value
      .filter(entry => entry.host !== '*' && entry.host_name)
      .map(entry => ({
        id: `config-${entry.host}`,
        name: entry.host,
        host: entry.host_name || entry.host,
        port: entry.port || 22,
        username: entry.user || '',
        auth_type: 'key' as const,
        identity_file: entry.identity_file,
        use_ssh_config: true,
        ssh_config_host: entry.host,
        created_at: '',
        last_used: null,
        source: 'ssh-config' as const
      }))
    return [...app, ...config]
  })

  async function fetchAll() {
    loading.value = true
    error.value = null
    try {
      const tauri = useTauri()
      const [profiles, configs, keys] = await Promise.all([
        tauri.commands.profiles.getAppProfiles(),
        tauri.commands.sshConfig.getSshConfig(),
        tauri.commands.sshKeys.listSshKeys()
      ])
      appProfiles.value = profiles
      sshConfig.value = configs
      sshKeys.value = keys
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function saveProfile(profile: AppProfile) {
    try {
      const tauri = useTauri()
      await tauri.commands.profiles.saveAppProfile(profile)
      await fetchAll()
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function deleteProfile(id: string) {
    try {
      const tauri = useTauri()
      await tauri.commands.profiles.deleteAppProfile(id)
      await fetchAll()
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  return {
    appProfiles,
    sshConfig,
    sshKeys,
    loading,
    error,
    combinedProfiles,
    fetchAll,
    saveProfile,
    deleteProfile
  }
})
