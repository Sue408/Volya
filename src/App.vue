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
      <RouterView />
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
</style>