<script setup lang="ts">
import type { AppProfile, AuthType } from '~/types'

const props = withDefaults(defineProps<{
  profile?: AppProfile | null
}>(), {
  profile: null
})

const emit = defineEmits<{
  close: [profile?: AppProfile]
}>()

const profileStore = useProfileStore()

const editingId = ref<string | null>(null)
const name = ref('')
const host = ref('')
const port = ref(22)
const username = ref('')
const authType = ref<AuthType>('key')
const identityFile = ref('')
const password = ref('')
const saving = ref(false)
const error = ref('')

onMounted(() => {
  if (props.profile) {
    editingId.value = props.profile.id
    name.value = props.profile.name
    host.value = props.profile.host
    port.value = props.profile.port
    username.value = props.profile.username
    authType.value = props.profile.auth_type
    identityFile.value = props.profile.identity_file || ''
  }
})

async function save() {
  if (!name.value || !host.value || !username.value) {
    const { t } = useI18n()
    error.value = t('connectionForm.errors.required')
    return
  }

  saving.value = true
  error.value = ''

  try {
    const profile: AppProfile = {
      id: editingId.value || crypto.randomUUID(),
      name: name.value,
      host: host.value,
      port: port.value,
      username: username.value,
      auth_type: authType.value,
      identity_file: identityFile.value || null,
      use_ssh_config: false,
      ssh_config_host: null,
      created_at: new Date().toISOString(),
      last_used: null
    }
    await profileStore.saveProfile(profile)
    emit('close', profile)
  } catch (e) {
    error.value = String(e)
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <UModal
    :title="editingId ? $t('connectionForm.titleEdit') : $t('connectionForm.titleNew')"
    class="min-w-96"
    @close:prevent="emit('close')"
  >
    <template #body>
      <UForm
        :state="{ name, host, port, username }"
        class="space-y-3"
      >
        <UFormField
          :label="$t('connectionForm.name')"
          required
        >
          <UInput
            v-model="name"
            :placeholder="$t('connectionForm.namePlaceholder')"
            class="w-full"
          />
        </UFormField>

        <div class="grid grid-cols-3 gap-3">
          <UFormField
            :label="$t('connectionForm.host')"
            required
            class="col-span-2"
          >
            <UInput
              v-model="host"
              :placeholder="$t('connectionForm.hostPlaceholder')"
              class="w-full"
            />
          </UFormField>
          <UFormField :label="$t('connectionForm.port')">
            <UInput
              v-model.number="port"
              type="number"
              min="1"
              max="65535"
              class="w-full"
            />
          </UFormField>
        </div>

        <UFormField
          :label="$t('connectionForm.username')"
          required
        >
          <UInput
            v-model="username"
            :placeholder="$t('connectionForm.usernamePlaceholder')"
            class="w-full"
          />
        </UFormField>

        <UFormField :label="$t('connectionForm.auth')">
          <USelect
            v-model="authType"
            :items="[
              { label: $t('connectionForm.authKey'), value: 'key' },
              { label: $t('connectionForm.authPassword'), value: 'password' }
            ]"
            class="w-full"
          />
        </UFormField>

        <UFormField
          v-if="authType === 'key'"
          :label="$t('connectionForm.identityFile')"
        >
          <UInput
            v-model="identityFile"
            placeholder="~/.ssh/id_rsa"
            class="w-full"
          />
        </UFormField>

        <UFormField
          v-if="authType === 'password'"
          :label="$t('connectionForm.password')"
        >
          <UInput
            v-model="password"
            type="password"
            class="w-full"
          />
        </UFormField>

        <p
          v-if="error"
          class="text-sm text-error-500"
        >
          {{ error }}
        </p>
      </UForm>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton
          color="neutral"
          variant="outline"
          @click="emit('close')"
        >
          {{ $t('connectionForm.cancel') }}
        </UButton>
        <UButton
          :loading="saving"
          @click="save"
        >
          {{ editingId ? $t('connectionForm.save') : $t('connectionForm.create') }}
        </UButton>
      </div>
    </template>
  </UModal>
</template>
