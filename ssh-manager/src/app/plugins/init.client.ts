export default defineNuxtPlugin({
  name: 'init',
  setup: async () => {
    const profileStore = useProfileStore()
    await profileStore.fetchAll()
  }
})
