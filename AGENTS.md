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
