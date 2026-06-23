export default defineNuxtConfig({

  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@vueuse/nuxt',
    '@pinia/nuxt'
  ],

  ssr: false,

  devtools: {
    enabled: true
  },

  css: ['~/assets/css/main.css'],

  ignore: ['**/src-tauri/**'],

  routeRules: {
    '/': { prerender: true }
  },
  compatibilityDate: '2026-06-22',

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true
    },
    optimizeDeps: {
      include: [
        '@tauri-apps/api/core',
        '@tauri-apps/api/webview',
        'pinia'
      ]
    }
  },
  telemetry: false,

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  }
})
