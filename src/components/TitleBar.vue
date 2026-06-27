<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { ChevronLeft, Sun, Moon, Settings, Minus, Maximize2, Minimize2, X, FileText } from '@lucide/vue'
import { useAgent } from '../composables/useAgent'
import { useTheme } from '../composables/useTheme'

const router = useRouter()
const route = useRoute()
const { currentWorkTitle } = useAgent()
const { effectiveTheme, toggleTheme } = useTheme()
const isMaximized = ref(false)

const emit = defineEmits<{
  openSettings: []
}>()

onMounted(async () => {
  try {
    const appWindow = getCurrentWindow()
    isMaximized.value = await appWindow.isMaximized()
  } catch { /* 非 Tauri 环境 */ }
})

/** 页面主标题 */
const pageTitle = computed(() => {
  if (route.name === 'settings') return '设置'
  return 'Volya'
})

/** 子标题（工作页显示作品名） */
const pageSubtitle = computed(() => {
  if (route.name === 'work') {
    return currentWorkTitle.value || '未命名作品'
  }
  return ''
})

/** 是否显示返回按钮 */
const showBack = computed(() => route.name !== 'works')

function goBack() {
  router.push('/')
}

// ─── 窗口控制 ───
async function handleMinimize() {
  try { await getCurrentWindow().minimize() } catch {}
}
async function handleMaximize() {
  try {
    const win = getCurrentWindow()
    await win.toggleMaximize()
    isMaximized.value = await win.isMaximized()
  } catch {}
}
async function handleClose() {
  try { await getCurrentWindow().close() } catch {}
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <!-- 左侧 — 返回 / 图标 + 标题 -->
    <div class="titlebar-left">
      <button v-if="showBack" class="tb-btn" @click="goBack" title="返回">
        <ChevronLeft :size="16" />
      </button>
      <FileText v-else class="tb-icon" :size="18" />
      <span class="tb-title">{{ pageTitle }}</span>
      <span v-if="pageSubtitle" class="tb-sep">/</span>
      <span v-if="pageSubtitle" class="tb-subtitle">{{ pageSubtitle }}</span>
    </div>

    <!-- 中间 — 拖拽区 -->
    <div class="titlebar-center"></div>

    <!-- 右侧 — 主题切换 / 设置 / 窗口控制 -->
    <div class="titlebar-right">
      <button class="tb-btn" @click="toggleTheme" :title="effectiveTheme === 'dark' ? '切换亮色模式' : '切换暗色模式'">
        <Sun v-if="effectiveTheme === 'light'" :size="16" />
        <Moon v-else :size="16" />
      </button>
      <button class="tb-btn" @click="emit('openSettings')" title="设置">
        <Settings :size="16" />
      </button>

      <span class="tb-divider"></span>

      <button class="tb-btn ctrl" @click="handleMinimize" title="最小化">
        <Minus :size="14" />
      </button>
      <button class="tb-btn ctrl" @click="handleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <Maximize2 v-if="!isMaximized" :size="13" />
        <Minimize2 v-else :size="13" />
      </button>
      <button class="tb-btn ctrl close" @click="handleClose" title="关闭">
        <X :size="14" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  height: 40px;
  padding: 0 var(--space-3);
  background: var(--bg-titlebar);
  user-select: none;
  flex-shrink: 0;
  /* 无 border-bottom，与内容区完全融合 */
}

/* ─── 左侧 ─── */
.titlebar-left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
}

.tb-icon {
  color: var(--accent-primary);
  flex-shrink: 0;
}

.tb-title {
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
}

.tb-sep {
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
}

.tb-subtitle {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ─── 中间拖拽区 ─── */
.titlebar-center {
  flex: 1;
  min-width: 0;
}

/* ─── 右侧 ─── */
.titlebar-right {
  display: flex;
  align-items: center;
  gap: 2px;
}

.tb-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 30px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tb-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tb-btn.ctrl.close:hover {
  background: var(--error);
  color: white;
}

.tb-divider {
  width: 1px;
  height: 18px;
  background: var(--border-light);
  margin: 0 var(--space-1);
}

/* ─── 窗口控制按钮在 macOS 上靠左，Windows 上靠右 ─── */
/* 当前默认 Windows 风格：右侧 */
</style>

