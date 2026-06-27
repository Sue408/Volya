<script setup lang="ts">
import ChatPanel from '../components/ChatPanel.vue'
import WorkSidebar from '../components/WorkSidebar.vue'
import WordCountBar from '../components/WordCountBar.vue'
import { ref } from 'vue'

const sidebarOpen = ref(true)
</script>

<template>
  <div class="work-page">
    <WorkSidebar v-model:open="sidebarOpen" />
    <div class="work-main">
      <ChatPanel />
      <WordCountBar />
    </div>
  </div>
</template>

<style scoped>
.work-page {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.work-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}
</style>
