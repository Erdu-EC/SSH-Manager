import type { Terminal } from '@xterm/xterm'

export function useTerminalSync() {
  const { call, onEvent } = useTauri()
  const cleanupFns: (() => void)[] = []

  function setup(terminalId: string, terminal: Terminal) {
    terminal.onData((data: string) => {
      call('write_stdin', { terminalId, data }).catch(console.error)
    })

    const unlistenOutput = onEvent<{ terminalId: string, data: string }>('terminal-output', (payload) => {
      if (payload.terminalId === terminalId) {
        const binary = atob(payload.data)
        const bytes = new Uint8Array(binary.length)
        for (let i = 0; i < binary.length; i++) {
          bytes[i] = binary.charCodeAt(i)
        }
        terminal.write(bytes)
      }
    })
    cleanupFns.push(unlistenOutput)

    const unlistenError = onEvent<{ terminalId: string, message: string }>('terminal-error', (payload) => {
      if (payload.terminalId === terminalId) {
        terminal.writeln(`\r\n\x1b[31mError: ${payload.message}\x1b[0m`)
      }
    })
    cleanupFns.push(unlistenError)

    const unlistenExit = onEvent<{ terminalId: string, exitCode: number }>('terminal-exit', (payload) => {
      if (payload.terminalId === terminalId) {
        terminal.writeln(`\r\n\x1b[33mProcess exited with code ${payload.exitCode}\x1b[0m`)
      }
    })
    cleanupFns.push(unlistenExit)
  }

  function cleanup() {
    cleanupFns.forEach(fn => fn())
    cleanupFns.length = 0
  }

  return { setup, cleanup }
}
