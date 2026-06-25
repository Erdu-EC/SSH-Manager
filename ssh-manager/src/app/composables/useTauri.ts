import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import * as connection from './tauri/commands/connection'
import * as explorer from './tauri/commands/explorer'
import * as profiles from './tauri/commands/profiles'
import * as sftp from './tauri/commands/sftp'
import * as shell from './tauri/commands/shell'
import * as sshConfig from './tauri/commands/sshConfig'
import * as sshKeys from './tauri/commands/sshKeys'
import * as terminal from './tauri/commands/terminal'

export function useTauri() {
  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

  async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    return invoke<T>(cmd, args)
  }

  function onEvent<T>(event: string, handler: (payload: T) => void) {
    if (!isTauri) return () => {}
    const unlisten = listen<T>(event, e => handler(e.payload))
    return () => {
      unlisten.then(fn => fn())
    }
  }

  const commands = {
    connection,
    explorer,
    profiles,
    sftp,
    shell,
    sshConfig,
    sshKeys,
    terminal
  }

  return { isTauri, call, onEvent, commands }
}
