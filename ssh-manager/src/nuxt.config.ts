export default defineNuxtConfig({
  extends: [
    './layers/core',
    './layers/dashboard',
    './layers/explorer',
    './layers/terminal'
  ],

  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@vueuse/nuxt',
    '@pinia/nuxt',
    '@nuxtjs/i18n',
    '@nuxt/test-utils/module'
  ],

  ssr: false,

  imports: {
    dirs: [
      'layers/core/stores',
      'layers/explorer/stores',
      'layers/terminal/stores',
      'layers/dashboard/stores'
    ]
  },

  devtools: {
    enabled: true
  },

  css: ['~/assets/css/main.css'],

  ignore: ['**/src-tauri/**'],

  routeRules: {
    '/': { prerender: true }
  },
  compatibilityDate: '2026-06-25',

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true
    },
    optimizeDeps: {
      include: [
        '@tauri-apps/api/core',
        '@tauri-apps/api/event',
        '@tauri-apps/api/webview',
        '@xterm/addon-fit',
        '@xterm/xterm',
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
  },

  i18n: {
    lazy: true,
    langDir: 'locales/',
    restructureDir: 'lang',
    locales: [
      { code: 'en', file: 'en.json' },
      { code: 'es', file: 'es.json' }
    ],
    defaultLocale: 'en',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      alwaysRedirect: true,
      fallbackLocale: 'en'
    },
    bundle: {
      optimizeTranslationDirective: false
    }
  },

  pinia: {
    storesDirs: [
      './layers/core/stores',
      './layers/explorer/stores',
      './layers/terminal/stores',
      './layers/dashboard/stores'
    ]
  }
})
