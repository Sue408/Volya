<script setup lang="ts">
import ChatPanel from './components/ChatPanel.vue'
</script>

<template>
  <ChatPanel />
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
</style>