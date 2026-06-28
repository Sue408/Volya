<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Sun, Moon, Settings, Minus, Maximize2, Minimize2, X } from '@lucide/vue'
import { useTheme } from '../composables/useTheme'

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

/** 标题始终为 Volya，不随页面变化 */

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
    <!-- 左侧 — Logo 标题，始终不变 -->
    <div class="titlebar-left">
      <span class="tb-title">Volya</span>
    </div>

    <!-- 中间 — 拖拽区 -->
    <div class="titlebar-center"></div>

    <!-- 右侧 — 主题切换 / 设置 / 窗口控制（放大图标） -->
    <div class="titlebar-right">
      <button class="tb-btn" @click="toggleTheme" :title="effectiveTheme === 'dark' ? '切换亮色模式' : '切换暗色模式'">
        <Sun v-if="effectiveTheme === 'light'" :size="20" />
        <Moon v-else :size="20" />
      </button>
      <button class="tb-btn" @click="emit('openSettings')" title="设置">
        <Settings :size="20" />
      </button>

      <span class="tb-divider"></span>

      <button class="tb-btn ctrl" @click="handleMinimize" title="最小化">
        <Minus :size="18" />
      </button>
      <button class="tb-btn ctrl" @click="handleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <Maximize2 v-if="!isMaximized" :size="16" />
        <Minimize2 v-else :size="16" />
      </button>
      <button class="tb-btn ctrl close" @click="handleClose" title="关闭">
        <X :size="18" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  height: 44px;
  padding: 0 var(--space-4);
  background: var(--bg-titlebar);
  user-select: none;
  flex-shrink: 0;
}

/* ─── 左侧 ─── */
.titlebar-left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
}

.tb-title {
  font-family: var(--font-logo);
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  line-height: 1;
  letter-spacing: 0.06em;
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
  width: 36px;
  height: 34px;
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
  height: 22px;
  background: var(--border-light);
  margin: 0 var(--space-1);
}

/* ─── 窗口控制按钮在 macOS 上靠左，Windows 上靠右 ─── */
/* 当前默认 Windows 风格：右侧 */
</style>

