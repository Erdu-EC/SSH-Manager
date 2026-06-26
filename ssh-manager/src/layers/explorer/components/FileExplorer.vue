<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import type { FileEntry } from '~/types'

const UIcon = resolveComponent('UIcon')

const props = defineProps<{
  sessionId: string
}>()

const explorerStore = useExplorerStore()

const currentSession = computed(() => explorerStore.perSession[props.sessionId])
const entries = computed(() => currentSession.value?.entries ?? [])
const loading = computed(() => currentSession.value?.loading ?? false)
const currentPath = computed(() => currentSession.value?.currentPath ?? '/')

const selectedPath = ref<string | null>(null)
const rowSelection = ref<Record<string, boolean>>({})
const treeItems = ref<TreeFileItem[]>([])
const expandedKeys = ref<string[]>([])

const { formatSize, formatPermissions, formatDate, getFileIcon, getIconName } = useFileFormatter()

const columns = [
  {
    accessorKey: 'name',
    header: 'Name',
    cell: ({ row }: { row: { original: FileEntry } }) => {
      const entry = row.original
      const iconName = entry.is_directory ? 'lucide:folder' : getIconName(entry.name)
      return h('div', { 'data-path': entry.path, 'class': 'flex items-center gap-2' }, [
        h(UIcon, {
          name: iconName,
          class: [
            'text-sm shrink-0',
            entry.is_directory ? 'text-warning' : ''
          ]
        }),
        h('span', null, entry.name)
      ])
    }
  },
  {
    accessorKey: 'size',
    header: 'Size',
    meta: { class: { th: 'text-right', td: 'text-right tabular-nums' } },
    cell: ({ row }: { row: { original: FileEntry } }) => {
      const entry = row.original
      return h('span', { 'data-path': entry.path }, entry.is_directory ? '\u2014' : formatSize(entry.size ?? 0))
    }
  },
  {
    accessorKey: 'modified_at',
    header: 'Modified',
    cell: ({ row }: { row: { original: FileEntry } }) => {
      const entry = row.original
      return h('span', { 'data-path': entry.path }, formatDate(entry.modified_at))
    }
  },
  {
    accessorKey: 'permissions',
    header: 'Permissions',
    meta: { class: { td: 'font-mono text-xs' } },
    cell: ({ row }: { row: { original: FileEntry } }) => {
      const entry = row.original
      return h('span', { 'data-path': entry.path }, formatPermissions(entry.permissions, entry.is_directory))
    }
  }
]

interface TreeFileItem {
  label: string
  icon?: string
  defaultExpanded?: boolean
  disabled?: boolean
  data?: FileEntry & { path: string, childrenLoaded?: boolean }
  children?: TreeFileItem[]
}

function joinPath(parent: string, child: string): string {
  if (parent.endsWith('/')) return parent + child
  return parent + '/' + child
}

function makeTreeItem(entry: FileEntry, fullPath: string): TreeFileItem {
  return {
    label: entry.name,
    icon: entry.is_directory ? 'i-lucide-folder' : getFileIcon(entry.name),
    children: entry.is_directory ? [] : undefined,
    data: { ...entry, path: fullPath }
  }
}

function updateTreeForPath(path: string, dirEntries: FileEntry[]) {
  const segments = path.split('/').filter(Boolean)

  if (segments.length === 0) {
    treeItems.value = dirEntries.filter(e => e.is_directory).map(e => makeTreeItem(e, '/' + e.name)) as TreeFileItem[]
    expandedKeys.value = []
    return
  }

  let currentLevel = treeItems.value
  let builtPath = ''
  let targetNode: TreeFileItem | null = null
  const keysToExpand: string[] = []

  for (let i = 0; i < segments.length; i++) {
    builtPath = builtPath ? builtPath + '/' + segments[i] : '/' + segments[i]
    const isLeaf = i === segments.length - 1

    let existing = currentLevel.find(n => n.label === segments[i])

    if (!existing) {
      existing = {
        label: segments[i]!,
        icon: 'i-lucide-folder',
        children: [],
        data: {
          name: segments[i]!,
          path: builtPath,
          is_directory: true,
          is_symlink: false,
          size: null,
          permissions: null,
          modified_at: null,
          childrenLoaded: true
        }
      }
      currentLevel.push(existing)
    } else if (existing.data) {
      existing.data.childrenLoaded = true
    }

    if (isLeaf) {
      targetNode = existing
    } else {
      if (!existing.children) {
        existing.children = []
      }
      currentLevel = existing.children
    }

    keysToExpand.push(builtPath)
  }

  if (targetNode) {
    targetNode.children = dirEntries.filter(e => e.is_directory).map(e => makeTreeItem(e, path + '/' + e.name)) as TreeFileItem[]
    targetNode.icon = 'i-lucide-folder-open'
  }

  expandedKeys.value = keysToExpand
}

watch(
  () => currentSession.value?.entries,
  (newEntries) => {
    if (newEntries && currentPath.value) {
      updateTreeForPath(currentPath.value, newEntries)
    }
  },
  { immediate: true }
)

// eslint-disable-next-line @typescript-eslint/no-explicit-any
async function handleToggle(_event: any, item: TreeFileItem) {
  if (item.data?.path && !item.data?.childrenLoaded) {
    item.data.childrenLoaded = true
    const childEntries = await explorerStore.loadChildren(props.sessionId, item.data.path)
    if (item.children) {
      item.children = childEntries.filter((entry: FileEntry) => entry.is_directory).map((entry: FileEntry) => {
        const fullPath = joinPath(item.data!.path, entry.name)
        return makeTreeItem(entry, fullPath)
      })
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleSelect(_event: any, item: TreeFileItem) {
  if (item.data?.is_directory) {
    explorerStore.navigateTo(props.sessionId, item.data.path)
  } else if (item.data) {
    selectedPath.value = item.data.path
    explorerStore.selectFile(item.data.path)
  }
}

function getKey(item: TreeFileItem): string {
  return item.data?.path ?? item.label ?? ''
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleRowSelect(_event: any, row: any) {
  const entry = row.original as FileEntry
  rowSelection.value = { [entry.path]: true }
  selectedPath.value = entry.path
}

function handleRowDoubleClick(e: MouseEvent) {
  const el = (e.target as HTMLElement).closest('[data-path]')
  if (!el) return
  const path = el.getAttribute('data-path')
  const entry = entries.value.find(e => e.path === path)
  if (entry?.is_directory) {
    explorerStore.navigateTo(props.sessionId, entry.path)
  } else if (entry) {
    explorerStore.selectFile(entry.path)
  }
}

const folderCount = computed(() => entries.value.filter(e => e.is_directory).length)
const fileCount = computed(() => entries.value.length - folderCount.value)

const totalSize = computed(() => {
  return entries.value.reduce((sum, e) => sum + (e.is_directory ? 0 : (e.size ?? 0)), 0)
})

const canGoBack = computed(() => currentPath.value !== '/')

function goToParent() {
  if (!canGoBack.value) return
  const parent = currentPath.value.replace(/\/+$/, '')
  const idx = parent.lastIndexOf('/')
  const target = idx <= 0 ? '/' : parent.slice(0, idx) || '/'
  explorerStore.navigateTo(props.sessionId, target)
}

onMounted(() => {
  explorerStore.navigateTo(props.sessionId, '/')
})

const contextEntry = ref<FileEntry | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const contextVisible = ref(false)

function closeContextMenu() {
  contextVisible.value = false
}

const {
  deleteSelected,
  handleRename,
  handleDeleteAction,
  handleCopyPath,
  handleProperties,
  openNewFolder
} = useExplorerActions(
  props.sessionId,
  entries,
  currentPath,
  selectedPath,
  rowSelection,
  contextEntry,
  closeContextMenu,
  formatSize
)

useEventListener(window, 'keydown', (e: KeyboardEvent) => {
  if ((e.key === 'Delete' || e.key === 'Del') && selectedPath.value) {
    e.preventDefault()
    deleteSelected()
  }
  if (e.key === 'Escape' && contextVisible.value) {
    closeContextMenu()
  }
})

function onTableContextMenu(e: MouseEvent) {
  e.preventDefault()
  const el = (e.target as HTMLElement).closest('[data-path]')
  if (!el) return
  const path = el.getAttribute('data-path')
  const entry = entries.value.find(e => e.path === path)
  if (!entry) return

  contextEntry.value = entry
  selectedPath.value = entry.path
  contextPos.value = { x: e.clientX, y: e.clientY }
  contextVisible.value = true
}
</script>

<template>
  <div class="flex flex-col h-full">
    <FileExplorerToolbar
      :current-path="currentPath"
      :loading="loading"
      :can-go-back="canGoBack"
      :selected-path="selectedPath"
      @navigate="explorerStore.navigateTo(props.sessionId, $event)"
      @refresh="explorerStore.refresh(props.sessionId)"
      @new-folder="openNewFolder"
      @parent-dir="goToParent"
      @delete="deleteSelected"
    />

    <div class="flex-1 flex overflow-hidden">
      <div class="w-56 border-r border-(--ui-border)/75 overflow-y-auto bg-elevated/10">
        <UTree
          v-model:expanded="expandedKeys"
          :items="treeItems"
          color="neutral"
          size="sm"
          :get-key="getKey"
          @select="handleSelect"
          @toggle="handleToggle"
        />
      </div>

      <div class="flex-1 flex flex-col overflow-hidden">
        <div
          class="flex-1 overflow-auto select-none"
          @contextmenu="onTableContextMenu"
          @dblclick="handleRowDoubleClick"
        >
          <UTable
            v-if="entries.length"
            v-model:row-selection="rowSelection"
            :columns="columns"
            :data="entries"
            class="flex-1 w-full"
            sticky="header"
            :get-row-id="(row: FileEntry) => row.path"
            :ui="{ root: 'overflow-visible' }"
            @select="handleRowSelect"
          />

          <div
            v-else-if="!loading"
            class="flex items-center justify-center h-full"
          >
            <CommonEmptyState
              icon="i-lucide-folder-open"
              title="Empty directory"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-2 px-4 py-1.5 shrink-0 border-t border-(--ui-border)/75 text-xs text-muted">
      <span class="text-foreground/70">{{ fileCount + folderCount }} item{{ fileCount + folderCount !== 1 ? 's' : '' }}</span>
      <span
        v-if="folderCount > 0"
        class="text-muted"
      >({{ folderCount }} dir{{ folderCount !== 1 ? 's' : '' }})</span>
      <span
        v-if="totalSize > 0"
        class="ml-auto font-mono tabular-nums text-muted"
      >{{ formatSize(totalSize) }}</span>
    </div>

    <Teleport to="body">
      <div
        v-if="contextVisible"
        class="fixed inset-0 z-50"
        @click="closeContextMenu"
        @contextmenu.prevent="closeContextMenu"
      >
        <div
          :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }"
          class="absolute min-w-40 bg-default rounded-lg shadow-lg ring ring-default py-1 overflow-hidden"
          @click.stop
        >
          <button
            class="w-full flex items-center gap-2 px-3 py-1.5 text-sm text-default hover:bg-elevated transition-colors"
            @click="handleRename"
          >
            <span class="i-lucide-pencil text-dimmed shrink-0" />
            Rename
          </button>
          <button
            class="w-full flex items-center gap-2 px-3 py-1.5 text-sm text-error hover:bg-elevated transition-colors"
            @click="handleDeleteAction"
          >
            <span class="i-lucide-trash-2 shrink-0" />
            Delete
          </button>
          <div class="h-px bg-(--ui-border)/50 my-1" />
          <button
            class="w-full flex items-center gap-2 px-3 py-1.5 text-sm text-default hover:bg-elevated transition-colors"
            @click="handleCopyPath"
          >
            <span class="i-lucide-copy text-dimmed shrink-0" />
            Copy Path
          </button>
          <button
            class="w-full flex items-center gap-2 px-3 py-1.5 text-sm text-default hover:bg-elevated transition-colors"
            @click="handleProperties"
          >
            <span class="i-lucide-info text-dimmed shrink-0" />
            Properties
          </button>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
:deep(thead th) {
  position: sticky;
  top: 0;
  z-index: 1;
}
:deep(tr[aria-selected="true"]) {
  background-color: color-mix(in srgb, var(--ui-primary) 10%, transparent) !important;
}
:deep(tr[aria-selected="true"]) td:first-child {
  box-shadow: inset 3px 0 0 var(--ui-primary);
}
</style>
