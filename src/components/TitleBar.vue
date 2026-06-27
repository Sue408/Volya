<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'

const router = useRouter()
const route = useRoute()
const isMaximized = ref(false)

onMounted(async () => {
  try {
    const appWindow = getCurrentWindow()
    isMaximized.value = await appWindow.isMaximized()
  } catch {
    // 非 Tauri 环境（如浏览器开发）忽略
  }
})

/** 页面标题映射 */
const pageTitle = () => {
  switch (route.name) {
    case 'works':
      return 'Volya'
    case 'work':
      return 'Volya'
    case 'settings':
      return '设置'
    default:
      return 'Volya'
  }
}

/** 子标题（工作页显示作品名） */
const pageSubtitle = () => {
  if (route.name === 'work') {
    return String(route.params.id || '')
  }
  return ''
}

/** 是否显示返回按钮 */
const showBack = computed(() => route.name !== 'works')

function goBack() {
  router.push('/')
}

// ─── 窗口控制 ───
async function handleMinimize() {
  try {
    await getCurrentWindow().minimize()
  } catch {}
}

async function handleMaximize() {
  try {
    const appWindow = getCurrentWindow()
    await appWindow.toggleMaximize()
    isMaximized.value = await appWindow.isMaximized()
  } catch {}
}

async function handleClose() {
  try {
    await getCurrentWindow().close()
  } catch {}
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <!-- 左侧区 -->
    <div class="titlebar-left">
      <button
        v-if="showBack"
        class="titlebar-btn back-btn"
        @click="goBack"
        title="返回"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 18 9 12 15 6" />
        </svg>
      </button>
      <span v-else class="titlebar-icon">🍊</span>
    </div>

    <!-- 中间区（可拖拽） -->
    <div class="titlebar-center" data-tauri-drag-region>
      <span class="titlebar-title">{{ pageTitle() }}</span>
      <span v-if="pageSubtitle()" class="titlebar-sep">/</span>
      <span v-if="pageSubtitle()" class="titlebar-subtitle">{{ pageSubtitle() }}</span>
    </div>

    <!-- 右侧区 -->
    <div class="titlebar-right">
      <div class="titlebar-controls">
        <button class="ctrl-btn minimize" @click="handleMinimize" title="最小化">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <line x1="2" y1="6" x2="10" y2="6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
        </button>
        <button class="ctrl-btn maximize" @click="handleMaximize" :title="isMaximized ? '还原' : '最大化'">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect v-if="!isMaximized" x="1.5" y="1.5" width="9" height="9" rx="1" fill="none" stroke="currentColor" stroke-width="1.5" />
            <template v-else>
              <rect x="2" y="4" width="6" height="6" rx="1" fill="none" stroke="currentColor" stroke-width="1.5" />
              <path d="M2 4V3a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1H8" fill="none" stroke="currentColor" stroke-width="1.5" />
            </template>
          </svg>
        </button>
        <button class="ctrl-btn close" @click="handleClose" title="关闭">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <line x1="2" y1="2" x2="10" y2="10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
            <line x1="10" y1="2" x2="2" y2="10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 38px;
  padding: 0 var(--space-3);
  background: var(--bg-titlebar);
  user-select: none;
  flex-shrink: 0;
  -webkit-app-region: drag;
}

/* ─── 左区 ─── */
.titlebar-left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: 100px;
  -webkit-app-region: no-drag;
}

.titlebar-icon {
  font-size: 18px;
  line-height: 1;
  padding: 0 var(--space-1);
}

.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.back-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ─── 中间区 ─── */
.titlebar-center {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex: 1;
  justify-content: center;
  min-width: 0;
  -webkit-app-region: drag;
}

.titlebar-title {
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
}

.titlebar-sep {
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
}

.titlebar-subtitle {
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ─── 右区 ─── */
.titlebar-right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: 100px;
  justify-content: flex-end;
  -webkit-app-region: no-drag;
}

.titlebar-controls {
  display: flex;
  align-items: center;
  gap: 2px;
}

.ctrl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 28px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.ctrl-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.ctrl-btn.close:hover {
  background: var(--error);
  color: white;
}
</style>
