<script setup lang="ts">
</script>

<template>
  <div class="works-list">
    <div class="greeting">
      <h1 class="greeting-title">✦ 你好呀，今天想创作什么故事？</h1>
      <p class="greeting-desc">还没有作品，点击下方开始你的第一部小说吧 🍊</p>
    </div>

    <div class="works-grid">
      <!-- 新建作品卡片 -->
      <div class="new-work-card">
        <div class="new-work-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
        </div>
        <span class="new-work-label">新建作品</span>
        <span class="new-work-hint">或拖拽文件夹到此处</span>
      </div>
    </div>

    <!-- Doro 小尾巴 -->
    <div class="doro-footer">
      <span>🍊 Doro 在陪着你哦~</span>
    </div>
  </div>
</template>

<style scoped>
.works-list {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100%;
  padding: var(--space-10) var(--space-6);
  overflow-y: auto;
}

.greeting {
  text-align: center;
  margin-bottom: var(--space-10);
}

.greeting-title {
  font-family: var(--font-display);
  font-size: var(--font-size-2xl);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--space-2);
  letter-spacing: -0.01em;
}

.greeting-desc {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
}

.works-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--space-4);
  width: 100%;
  max-width: 680px;
}

/* ─── 新建卡片 ─── */
.new-work-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: 200px;
  border: 2px dashed var(--border-color);
  border-radius: var(--radius-lg);
  background: transparent;
  cursor: pointer;
  transition: all var(--transition-fast);
  color: var(--text-tertiary);
}

.new-work-card:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
  background: var(--bg-tertiary);
}

.new-work-icon {
  opacity: 0.6;
}

.new-work-label {
  font-size: var(--font-size-base);
  font-weight: 600;
}

.new-work-hint {
  font-size: var(--font-size-xs);
  opacity: 0.6;
}

/* ─── Doro ─── */
.doro-footer {
  margin-top: auto;
  padding-top: var(--space-8);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  opacity: 0.6;
}
</style>
