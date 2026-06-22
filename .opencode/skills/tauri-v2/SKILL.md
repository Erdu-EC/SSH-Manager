---
name: tauri-v2
description: Complete skill for building Tauri v2 applications — architecture, IPC, security, Rust commands, plugins, modular core APIs, bundling, and distribution. Based on official Tauri v2 docs at https://tauri.app.
license: MIT
metadata:
  author: Tauri Contributors / opencode (Improved)
  version: "2.1"
compatibility: Requires Rust, Node.js, and platform-specific webview dependencies
---

# Tauri v2 Development

This skill provides comprehensive guidance for building, configuring, and distributing Tauri v2 applications. It covers the full stack: project setup, core concepts, inter-process communication (IPC), security (capabilities/permissions), Rust backend development (powered by Tokio), frontend integration, plugin architecture, and distribution.

## When to Apply

- Apply this skill whenever the task involves Tauri v2 development.
- Covers both desktop (Windows, macOS, Linux) and mobile (Android, iOS) targets.
- Refer to the official docs at https://tauri.app for the latest updates.

## Table of Contents

1. [Project Setup & Structure](#1-project-setup--structure)
2. [Core Concepts](#2-core-concepts)
3. [Security: Permissions & Capabilities](#3-security-permissions--capabilities)
4. [Calling Rust from the Frontend](#4-calling-rust-from-the-frontend)
5. [Calling the Frontend from Rust](#5-calling-the-frontend-from-rust)
6. [State Management](#6-state-management)
7. [Configuration Files](#7-configuration-files)
8. [Plugin Development & Core APIs](#8-plugin-development--core-apis)
9. [Distribution & Bundling](#9-distribution--bundling)

---

## 1. Project Setup & Structure

### Prerequisites

**Windows:** Microsoft C++ Build Tools ("Desktop development with C++"), WebView2 (built-in on Win 10 1803+), optional VBSCRIPT for MSI builds.

**macOS:** Xcode (or `xcode-select --install` for CLI tools only).

**Linux:** System packages for WebKit2GTK 4.1, e.g. on Debian/Ubuntu:
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Rust:** Install via `rustup`:
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

**Node.js:** LTS version recommended (only needed for JS/TS frontends).

### Create a Project

Use `create-tauri-app`:
```bash
pnpm create tauri-app
# or: npm create tauri-app@latest, yarn create tauri-app, etc.
```

Or add Tauri to an existing project:
```bash
pnpm add -D @tauri-apps/cli@latest
pnpm tauri init
```

### Project Structure

```
my-app/
├── package.json
├── index.html
├── src/                  # Frontend source
├── src-tauri/            # Rust backend
│   ├── Cargo.toml
│   ├── build.rs          # tauri_build::build()
│   ├── tauri.conf.json   # Main Tauri config
│   ├── capabilities/     # Capability files (JSON/TOML)
│   ├── icons/            # App icons
│   └── src/
│       ├── main.rs       # Desktop entry (calls lib::run())
│       └── lib.rs        # Core logic + #[cfg_attr(mobile, tauri::mobile_entry_point)]
```

### Dev Commands

```bash
pnpm tauri dev      # Development server
pnpm tauri build    # Production build + bundling
pnpm tauri icon     # Generate icons from source image
```

---

## 2. Core Concepts

### Architecture

Tauri is composed of several Rust crates:
- **`tauri`** — Main crate: runtime, macros, config, IPC
- **`tauri-runtime`** — Glue between Tauri and webview libraries
- **`tauri-macros`** / **`tauri-codegen`** — Compile-time code generation
- **`tauri-build`** — Build-time macro rigging
- **`tauri-runtime-wry`** — OS-level windowing (printing, monitors)
- **`tauri-utils`** — Config parsing, CSP, asset management
- **`tauri-bundler`** — Platform-specific bundling

Upstream crates maintained by Tauri:
- **TAO** — Cross-platform window creation (fork of winit)
- **WRY** — Cross-platform WebView rendering

### Process Model

- **Core Process** (Rust): Entry point, full OS access, window orchestration, global state, IPC routing. Uses the Tokio asynchronous runtime under the hood
- **WebView Process** (OS-native): Renders HTML/CSS/JS using the system's WebView (WebView2 on Windows, WKWebView on macOS, webkitgtk on Linux)

WebView libraries are **dynamically linked** at runtime — not bundled in the executable — keeping app sizes small (~600KB minimal).

### Inter-Process Communication (IPC)

Tauri uses asynchronous message passing between processes:

**Events** — fire-and-forget, one-way, suitable for lifecycle/state changes:
```ts
import { emit, listen } from '@tauri-apps/api/event';
emit('file-selected', '/path/to/file');
const unlisten = await listen('download-started', (event) => {});
```

**Commands** — FFI-like invoke calls, type-safe, supports args, return values, errors, async:
```ts
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('my_command', { argName: 'value' });
```

---

## 3. Security: Permissions & Capabilities

### Capabilities

Capability files in `src-tauri/capabilities/` (JSON or TOML) define which permissions are granted to which windows/webviews:

```json
{
  "identifier": "main-capability",
  "windows": ["main"],
  "permissions": [
    "core:path:default",
    "core:event:default",
    "core:window:default",
    "core:app:default",
    "core:resources:default",
    "core:menu:default",
    "core:tray:default",
    "core:window:allow-set-title"
  ]
}
```

Platform-specific capabilities with `platforms` array (`linux`, `macOS`, `windows`, `iOS`, `android`):
```json
{
  "identifier": "mobile-capability",
  "windows": ["main"],
  "platforms": ["iOS", "android"],
  "permissions": ["nfc:allow-scan", "biometric:allow-authenticate"]
}
```

Remote API access via `remote.urls`:
```json
{
  "remote": { "urls": ["https://*.tauri.app"] }
}
```

### Permissions

Permissions describe command privileges. Defined as TOML in `permissions/`:

```toml
[[permission]]
identifier = "allow-read-file"
description = "Enables the read_file command."
commands.allow = ["read_file"]

[[permission.scope.allow]]
path = "$HOME/*"
```

Permission identifiers follow the pattern `<name>:<permission>` (e.g., `fs:read-files`).
Plugin prefix `tauri-plugin-` is auto-prepended at compile time.

Permission sets group permissions:
```toml
[[set]]
identifier = "allow-websocket"
permissions = ["allow-connect", "allow-send"]
```

Default permission (auto-enabled when plugin is added):
```toml
[default]
description = "Allows making HTTP requests"
permissions = ["allow-request"]
```

### Restricting App Commands

In `src-tauri/build.rs`:
```rust
fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&["your_command"])),
    ).unwrap();
}
```

---

## 4. Calling Rust from the Frontend

### Commands

Define commands in `src-tauri/src/lib.rs` (or a separate module):

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Note:** Commands in `lib.rs` cannot be `pub`. Commands in separate modules SHALL be `pub`.

**Arguments:** Use camelCase in JS, snake_case in Rust by default, or use `rename_all`:
```rust
#[tauri::command(rename_all = "snake_case")]
fn my_cmd(invoke_message: String) {}
```

**Return data:** Any `serde::Serialize` type. For large binary data, use `tauri::ipc::Response`.

**Error handling:** Return `Result<T, E>` where `E: Serialize`:
```rust
#[tauri::command]
fn login(user: String, password: String) -> Result<String, String> {
    if user == "admin" && password == "secret" {
        Ok("logged_in".into())
    } else {
        Err("invalid credentials".into())
    }
}
```

For rich error types, use `thiserror` + manual `Serialize` impl.

**Async commands:** Declare as `async fn`. Borrowed types (`&str`, `State<'_, _>`) are unsupported — use owned types or wrap return in `Result`:
```rust
#[tauri::command]
async fn my_cmd(value: String) -> Result<String, ()> {
    some_async_fn().await;
    Ok(value)
}
```

**Channels (streaming):** Use `tauri::ipc::Channel` for streaming data:
```rust
#[tauri::command]
async fn stream_data(path: PathBuf, reader: tauri::ipc::Channel<&[u8]>) {
    let mut file = tokio::fs::File::open(path).await.unwrap();
    let mut chunk = vec![0; 4096];
    loop {
        let len = file.read(&mut chunk).await.unwrap();
        if len == 0 { break; }
        reader.send(&chunk).unwrap();
    }
}
```

**Built-in injections** (command function parameters):
- `tauri::WebviewWindow` — access the invoking window
- `tauri::AppHandle` — access the app handle
- `tauri::State<'_, T>` — access managed state
- `tauri::ipc::Request` — raw request body + headers
- `tauri::ipc::CommandScope<'_, T>` — command-specific scope
- `tauri::ipc::GlobalScope<'_, T>` — global scope

### Event System (Frontend emitting)

```ts
import { emit, listen, once } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

// Global event
emit('file-selected', '/path/to/file');

// Webview-specific
const webview = getCurrentWebviewWindow();
webview.emitTo('editor', 'file-changed', { path: '/path/to/file' });

// Listen
const unlisten = await listen<DownloadStarted>('download-started', (event) => {
    console.log(event.payload.url);
});

// One-time
once('ready', (event) => {});
```

**Always clean up listeners on component unmount** using the returned `unlisten` function.

### Frontend Invocation

```ts
import { invoke, Channel } from '@tauri-apps/api/core';

// Simple invoke
invoke('my_custom_command', { invokeMessage: 'Hello!' });

// With channels
const onEvent = new Channel<DownloadEvent>();
onEvent.onmessage = (msg) => console.log(msg);
await invoke('download', { url: '...', onEvent });
```

---

## 5. Calling the Frontend from Rust

### Event System (Rust emitting)

```rust
use tauri::{AppHandle, Emitter};

// Global event
app.emit("download-started", &url).unwrap();

// Webview-specific
app.emit_to("login", "login-result", "loggedIn").unwrap();

// Filtered to multiple webviews
app.emit_filter("open-file", path, |target| {
    matches!(target, EventTarget::WebviewWindow { label } if label == "main" || label == "file-viewer")
}).unwrap();
```

### Listening to Events in Rust

```rust
use tauri::Listener;

app.listen("download-started", |event| {
    if let Ok(payload) = serde_json::from_str::<DownloadStarted>(&event.payload()) {
        println!("downloading {}", payload.url);
    }
});

app.once("ready", |event| {
    println!("app is ready");
});

// Manual unlisten
let id = app.listen("status-changed", |_| {});
app.unlisten(id);
```

### Channels (from Rust)

Channels are optimized for streaming large/ordered data:

```rust
use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    Started { url: String, download_id: usize },
    Progress { download_id: usize, chunk_length: usize },
    Finished { download_id: usize },
}

#[tauri::command]
fn download(url: String, on_event: Channel<DownloadEvent>) {
    on_event.send(DownloadEvent::Started { url, download_id: 1 }).unwrap();
}
```

### Evaluating JavaScript

```rust
use tauri::Manager;

let webview = app.get_webview_window("main").unwrap();
webview.eval("console.log('hello from Rust')")?;
```

For complex scripts with Rust data, use the `serialize-to-javascript` crate.

---

## 6. State Management

### Managing State

Use `app.manage()` to register state, then access via `tauri::State` in commands:

```rust
use std::sync::Mutex;

struct AppState {
    counter: u32,
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState { counter: 0 }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![increment])
        .run(tauri::generate_context!())
        .expect("error");
}

#[tauri::command]
fn increment(state: tauri::State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    state.counter
}
```

**Key rules:**
- Use `Mutex<T>` (or `tokio::sync::Mutex` for async + hold across await points)
- **Do NOT** wrap in `Arc` — Tauri does this internally
- For async commands, return type MUST be `Result`
- Access state outside commands via `app.state::<T>()` using the `Manager` trait

---

## 7. Configuration Files

### `tauri.conf.json` (or `Tauri.toml` with `config-toml` feature)

Key sections:
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "bundle": {
    "active": true,
    "icon": ["icons/app.png"],
    "targets": "all"
  },
  "app": {
    "windows": [{ "title": "My App", "width": 1024, "height": 768 }],
    "security": {
      "capabilities": ["main-capability"]
    }
  },
  "plugins": {
    "updater": {
      "pubkey": "...",
      "endpoints": ["https://my.app.updater/{{target}}/{{current_version}}"]
    }
  }
}
```

### Platform-specific config files

- `tauri.linux.conf.json` / `tauri.windows.conf.json` / `tauri.macos.conf.json`
- `tauri.android.conf.json` / `tauri.ios.conf.json`
- Merged via JSON Merge Patch (RFC 7396)

### Extending config at build time

```bash
pnpm tauri build --config src-tauri/tauri.beta.conf.json
```

### `Cargo.toml`

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[build-dependencies]
tauri-build = { version = "2" }
```

---

## 8. Plugin Development

### Initialize a plugin

```bash
pnpm tauri plugin new <name>        # with JS API bindings
pnpm tauri plugin new <name> --no-api  # Rust-only
pnpm tauri plugin new <name> --android --ios  # with mobile support
```

### Plugin structure

```
tauri-plugin-<name>/
├── src/
│   ├── lib.rs          # Builder, permissions, state
│   ├── commands.rs     # Command definitions
│   ├── desktop.rs      # Desktop-specific impl
│   ├── mobile.rs       # Mobile-specific impl
│   ├── error.rs        # Error type
│   └── models.rs       # Shared structs
├── permissions/        # Permission files (TOML)
├── android/            # Kotlin/Java library
├── ios/                # Swift package
├── guest-js/           # TypeScript API source
├── build.rs
├── Cargo.toml
└── package.json
```

### Plugin configuration

```json
{
  "plugins": {
    "plugin-name": { "timeout": 30 }
  }
}
```

```rust
#[derive(Deserialize)]
pub struct Config {
    timeout: usize,
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("plugin-name")
        .setup(|app, api| {
            let timeout = api.config().timeout;
            Ok(())
        })
        .build()
}
```

### Plugin lifecycle hooks

```rust
Builder::new("plugin-name")
    .setup(|app, _api| { /* init state, spawn tasks */ Ok(()) })
    .on_navigation(|window, url| { /* validate navigation */ true })
    .on_webview_ready(|window| { /* init scripts per window */ })
    .on_event(|app, event| { /* handle RunEvent */ })
    .on_drop(|app| { /* cleanup */ })
    .invoke_handler(tauri::generate_handler![commands::my_cmd])
```

### Autogenerated permissions

In `build.rs`:
```rust
const COMMANDS: &[&str] = &["upload"];
fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
```

This auto-generates `allow-upload` and `deny-upload` permissions.

### Plugin JS API bindings

In `guest-js/index.ts`:
```ts
import { invoke, Channel } from '@tauri-apps/api/core';

export async function upload(url: string, onProgress: (p: number) => void) {
    const channel = new Channel<number>();
    channel.onmessage = onProgress;
    await invoke('plugin:<plugin-name>|upload', { url, onEvent: channel });
}
```

---

## 9. Distribution & Bundling

### Build Commands

```bash
pnpm tauri build                    # Build + bundle
pnpm tauri build --no-bundle        # Build only
pnpm tauri bundle --bundles app,dmg # Bundle only
```

### Supported Formats

| Platform | Formats |
|----------|---------|
| Windows  | MSI, NSIS |
| macOS    | DMG, App Bundle, App Store |
| Linux    | AppImage, Debian, RPM, Snap, AUR |
| Android  | APK, AAB (Google Play) |
| iOS      | IPA (App Store) |

### Code Signing

Most platforms require signing:
- **macOS:** `codesign` + notarization
- **Windows:** Authenticode certificate
- **Linux:** GPG for Debian/RPM
- **Android:** Keystore
- **iOS:** Apple Developer certificate

### CI/CD

GitHub Actions: use `tauri-apps/tauri-action` for cross-platform builds.

---

## Quick Reference — Common Patterns

### Minimal command
```rust
#[tauri::command]
fn hello() -> String { "world".into() }
```

### Command with args + error
```rust
#[tauri::command]
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("division by zero".into()) }
    else { Ok(a / b) }
}
```

### Stateful command
```rust
struct Counter(Mutex<u32>);

#[tauri::command]
fn inc(state: tauri::State<'_, Counter>) -> u32 {
    let mut c = state.0.lock().unwrap();
    *c += 1;
    *c
}
```

### Async command with channel
```rust
#[tauri::command]
async fn count(to: u32, on_event: tauri::ipc::Channel<u32>) {
    for i in 0..to {
        on_event.send(i).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
```

### Default capability
```json
{
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open"
  ]
}
```

---

## Official Resources

- **Docs:** https://tauri.app
- **GitHub:** https://github.com/tauri-apps/tauri
- **CLI Reference:** https://tauri.app/reference/cli/
- **Config Reference:** https://tauri.app/reference/config/
- **Plugins:** https://tauri.app/plugin/
- **Discord:** https://discord.com/invite/tauri
- **Tauri Book:** https://tauri.app/about/book/
