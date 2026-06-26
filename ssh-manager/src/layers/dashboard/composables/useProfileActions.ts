import type { AppProfile } from '~/types'

export function useProfileActions() {
  const profileStore = useProfileStore()
  const connectionStore = useConnectionStore()
  const { t } = useI18n()
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

  return {
    connectingProfile,
    handleConnect,
    handleDelete
  }
}
