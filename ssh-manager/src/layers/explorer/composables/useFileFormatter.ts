export function useFileFormatter() {
  function formatSize(bytes: number): string {
    if (bytes === 0) return '0 B'
    const units = ['B', 'KB', 'MB', 'GB', 'TB']
    const i = Math.floor(Math.log(bytes) / Math.log(1024))
    return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`
  }

  function formatPermissions(perms: string | null, isDirectory: boolean): string {
    if (!perms) return ''
    const octal = perms.length > 3 ? perms.slice(-3) : perms
    const typeChar = isDirectory ? 'd' : '-'
    const n = parseInt(octal, 8)
    const rwx = (bits: number) => {
      const r = (bits & 4) ? 'r' : '-'
      const w = (bits & 2) ? 'w' : '-'
      const x = (bits & 1) ? 'x' : '-'
      return r + w + x
    }
    const owner = rwx(Math.floor(n / 64))
    const group = rwx(Math.floor((n % 64) / 8))
    const other = rwx(n % 8)
    return `${typeChar}${owner}${group}${other} (${octal})`
  }

  function formatDate(dateStr: string | null): string {
    if (!dateStr) return '-'
    const ts = Number(dateStr)
    const d = Number.isFinite(ts) ? new Date(ts * 1000) : new Date(dateStr)
    if (isNaN(d.getTime())) return '-'
    return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })
  }

  function getFileIcon(name: string): string {
    const ext = name.split('.').pop()?.toLowerCase()
    const icons: Record<string, string> = {
      'ts': 'i-lucide-file-code', 'tsx': 'i-lucide-file-code',
      'js': 'i-lucide-file-code', 'jsx': 'i-lucide-file-code',
      'rs': 'i-lucide-file-code', 'vue': 'i-lucide-file-code',
      'json': 'i-lucide-file-json',
      'md': 'i-lucide-file-text', 'txt': 'i-lucide-file-text',
      'yml': 'i-lucide-file-text', 'yaml': 'i-lucide-file-text',
      'toml': 'i-lucide-file-text',
      'sh': 'i-lucide-terminal', 'bash': 'i-lucide-terminal',
      'png': 'i-lucide-image', 'jpg': 'i-lucide-image',
      'jpeg': 'i-lucide-image', 'gif': 'i-lucide-image',
      'svg': 'i-lucide-image', 'ico': 'i-lucide-image',
      'zip': 'i-lucide-archive', 'tar': 'i-lucide-archive',
      'gz': 'i-lucide-archive', 'bz2': 'i-lucide-archive',
      '7z': 'i-lucide-archive'
    }
    return ext && icons[ext] ? icons[ext] : 'i-lucide-file'
  }

  function getIconName(name: string): string {
    const ext = name.split('.').pop()?.toLowerCase()
    const icons: Record<string, string> = {
      'ts': 'vscode-icons:file-type-typescript',
      'tsx': 'vscode-icons:file-type-typescript',
      'js': 'vscode-icons:file-type-js',
      'jsx': 'vscode-icons:file-type-reactjs',
      'rs': 'vscode-icons:file-type-rust',
      'vue': 'vscode-icons:file-type-vue',
      'json': 'vscode-icons:file-type-json',
      'md': 'vscode-icons:file-type-markdown',
      'txt': 'lucide:file-text',
      'yml': 'vscode-icons:file-type-yaml',
      'yaml': 'vscode-icons:file-type-yaml',
      'toml': 'vscode-icons:file-type-toml',
      'sh': 'vscode-icons:file-type-shell',
      'bash': 'vscode-icons:file-type-shell',
      'zsh': 'vscode-icons:file-type-shell',
      'png': 'lucide:image',
      'jpg': 'lucide:image',
      'jpeg': 'lucide:image',
      'gif': 'lucide:image',
      'svg': 'lucide:image',
      'ico': 'lucide:image',
      'zip': 'lucide:archive',
      'tar': 'lucide:archive',
      'gz': 'lucide:archive',
      'bz2': 'lucide:archive',
      '7z': 'lucide:archive'
    }
    return ext && icons[ext] ? icons[ext] : 'lucide:file'
  }

  return {
    formatSize,
    formatPermissions,
    formatDate,
    getFileIcon,
    getIconName
  }
}
