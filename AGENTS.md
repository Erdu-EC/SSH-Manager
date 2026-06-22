# AGENTS.md — SSH Explorer

## Structure

```
ssh-manager/
├── package.json          # Tauri CLI only — `pnpm tauri <cmd>`
├── pnpm-workspace.yaml   # workspace: `src/`
├── src/                  # Nuxt 4 app (frontend)
│   ├── package.json      # actual app scripts
│   ├── nuxt.config.ts
│   └── app/              # Nuxt 4 app dir: app.vue, pages/, components/
└── src-tauri/            # Tauri v2 Rust backend
    ├── src/lib.rs         # `ssh_manager_lib` crate (entry: main.rs → ssh_manager_lib::run)
    └── capabilities/      # Tauri v2 capability permissions
```

## Commands

Run from **`ssh-manager/`** (root):
- `pnpm tauri dev` — Tauri dev server (spawns `cd src && pnpm dev`)
- `pnpm tauri build` — production build (runs `cd src && pnpm generate` first)

Run from **`ssh-manager/src/`**:
- `pnpm lint` — ESLint (via `@nuxt/eslint`)
- `pnpm typecheck` — `nuxt typecheck` (uses vue-tsc)

CI order (src/.github/workflows/ci.yml): `pnpm install` → `pnpm run lint` → `pnpm run typecheck`

## Key config

- **Nuxt**: `ssr: false`, Vite `envPrefix: ['VITE_', 'TAURI_']`, `strictPort: true`, `clearScreen: false`
- **ESLint stylistic**: `commaDangle: 'never'`, `braceStyle: '1tbs'`
- **CSS**: Tailwind v4 (`@import "tailwindcss"`), `@nuxt/ui`, custom green palette in `app/assets/css/main.css`
- **Tauri v2**: `withGlobalTauri: true`, CSP null, capabilities in `src-tauri/capabilities/`
- **Rust**: `edition 2021`, crate-type `["staticlib", "cdylib", "rlib"]`, release profile: `lto=true`, `opt-level=3`, `panic="abort"`, `strip=true`
- **Env vars**: prefix with `VITE_` or `TAURI_` for Vite exposure

## Notes

- `src/main.ts` is **leftover Tauri vanilla template files** — the Nuxt app lives in `src/app/`
- Nuxt auto-imports components from `app/components/`, pages from `app/pages/`
- TypeScript v6.0.3, Nuxt 4, Nuxt UI 4, Vue 3, vue-tsc v3.3.5

## Nuxt guidelines
The frontend app is built with Nuxt 4 using TypeScript, Nuxt UI v4, and Pinia for state management.

1. Use strict TypeScript for all files.
2. Follow the Nuxt 4 directory structure and conventions.
3. Use auto-imports for components, composables, and stores to simplify imports.
4. Use the Nuxt 4 layers feature to organize code by feature and maintain separation of concerns.
5. Use the Composition API for logic encapsulation and reusability.
6. Use the defined API conventions for creating DTOs and API services to maintain consistency.
7. Use the defined test structure and run tests regularly to ensure code quality and catch issues early.
8. Use i18n for localization and keep translation files organized in the `lang/locales/` directory.
9. Apply the correct separation of responsibilities.
10. Use Pinia for state management and keep stores focused on api calls, state and actions related to that state, without UI logic.
11. Stores must return all reactive states and getters for correct SSR hydration.
12. Composables should orchestrate ui communication with store, contain reusable logic that can be shared across components and stores, and should not contain GLOBAL state management, but can use local reactive state.
13. Create components with single responsibility and encapsulation of non shared behaviour, and place them in the appropriate directory.
14. Use defineModel for defining one or more component v-model bindings, instead of emitting custom events for two-way data binding. This simplifies component usage and improves readability.
15. Pages should only contain UI logic and should call composables.