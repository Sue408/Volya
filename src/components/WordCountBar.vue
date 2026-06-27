<script setup lang="ts">
// 预留：后续从 useAgent 或 useWorks 读取实际数据
const currentWords = 0
const targetWords = 0
const progress = 0
</script>

<template>
  <footer class="wordcount-bar" v-if="targetWords > 0">
    <div class="wordcount-info">
      <span class="wordcount-text">
        {{ currentWords.toLocaleString() }} / {{ targetWords.toLocaleString() }} 字
      </span>
      <span class="wordcount-pct">{{ Math.round(progress * 100) }}%</span>
    </div>
    <div class="wordcount-track">
      <div class="wordcount-fill" :style="{ width: `${Math.min(progress * 100, 100)}%` }"></div>
    </div>
  </footer>

  <footer v-else class="wordcount-bar minimal">
    <span class="wordcount-text">{{ currentWords.toLocaleString() }} 字</span>
  </footer>
</template>

<style scoped>
.wordcount-bar {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: var(--space-1) var(--space-4);
  border-top: 1px solid var(--border-light);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.wordcount-bar.minimal {
  align-items: center;
  padding: var(--space-1) var(--space-4);
}

.wordcount-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.wordcount-text {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.wordcount-pct {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.wordcount-track {
  height: 3px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.wordcount-fill {
  height: 100%;
  background: var(--accent-primary);
  border-radius: var(--radius-full);
  transition: width var(--transition-slow);
}
</style>
