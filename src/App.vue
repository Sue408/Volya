<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import TitleBar from './components/TitleBar.vue'

const router = useRouter()

function handleOpenSettings() {
  router.push('/settings')
}

/* ─── 视口自适应字号缩放 ───
   根据窗口宽度计算 --font-scale，注入到 :root
   参考宽度 1000px → scale = 1.0
   范围：0.85 ~ 1.08 */
const SCALE_REF = 1000
const SCALE_MIN = 0.85
const SCALE_MAX = 1.08

let resizeTimer: number | null = null

function updateFontScale() {
  const w = window.innerWidth
  const scale = Math.min(SCALE_MAX, Math.max(SCALE_MIN, w / SCALE_REF))
  document.documentElement.style.setProperty('--font-scale', scale.toString())
}

onMounted(() => {
  updateFontScale()
  window.addEventListener('resize', () => {
    if (resizeTimer) cancelAnimationFrame(resizeTimer)
    resizeTimer = requestAnimationFrame(updateFontScale)
  })
})

onUnmounted(() => {
  window.removeEventListener('resize', updateFontScale)
  if (resizeTimer) cancelAnimationFrame(resizeTimer)
})
</script>

<template>
  <div class="app-shell">
    <TitleBar @open-settings="handleOpenSettings" />
    <main class="app-main">
      <RouterView v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  width: 100%;
  overflow: hidden;
}

html {
  font-size: 16px;  /* rem 基准 */
}

body {
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  color: var(--text-primary);
  background: var(--bg-primary);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

::selection {
  background: var(--sage-200);
  color: var(--sage-900);
}

/* ─── 字号缩放平滑过渡 ─── */
* {
  transition: font-size 0.2s ease;
}

.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.app-main {
  flex: 1;
  overflow: hidden;
}

/* ═══ 全局卡片基础样式 ═══ */
.card {
  background: var(--bg-card);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  transition: box-shadow var(--transition-fast), border-color var(--transition-fast);
}

.card-interactive {
  cursor: pointer;
}

.card-interactive:hover {
  border-color: var(--border-color);
  box-shadow: var(--shadow-md);
}

.card-interactive:active {
  transform: scale(0.99);
}

/* ─── 页面过渡动画 ─── */
.page-enter-active,
.page-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.page-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>