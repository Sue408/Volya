<script setup lang="ts">
import { useRouter } from 'vue-router'
import TitleBar from './components/TitleBar.vue'

const router = useRouter()

function handleOpenSettings() {
  router.push('/settings')
}
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