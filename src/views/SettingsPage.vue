<script setup lang="ts">
import { ref, onMounted } from 'vue'

// 预留：后续从 composable 读取配置
const activeTab = ref<'llm' | 'preferences' | 'about'>('llm')
const llmReady = ref(false)

onMounted(() => {
  // 预留：加载 LLM 配置
})
</script>

<template>
  <div class="settings-page">
    <!-- 左侧导航 -->
    <nav class="settings-nav">
      <button
        class="nav-item"
        :class="{ active: activeTab === 'llm' }"
        @click="activeTab = 'llm'"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M12 2L2 7l10 5 10-5-10-5z" />
          <path d="M2 17l10 5 10-5" />
          <path d="M2 12l10 5 10-5" />
        </svg>
        <span>LLM 配置</span>
      </button>
      <button
        class="nav-item"
        :class="{ active: activeTab === 'preferences' }"
        @click="activeTab = 'preferences'"
        disabled
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
        <span>偏好</span>
      </button>
      <button
        class="nav-item"
        :class="{ active: activeTab === 'about' }"
        @click="activeTab = 'about'"
        disabled
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="16" x2="12" y2="12" />
          <line x1="12" y1="8" x2="12.01" y2="8" />
        </svg>
        <span>关于</span>
      </button>
    </nav>

    <!-- 右侧内容 -->
    <div class="settings-content">
      <!-- LLM 配置 -->
      <div v-if="activeTab === 'llm'" class="settings-section">
        <div class="status-bar" :class="{ connected: llmReady }">
          <span class="status-dot"></span>
          <span>{{ llmReady ? 'LLM 已连接' : 'LLM 未配置' }}</span>
        </div>

        <p class="section-hint">在此配置 AI 创作引擎的连接信息</p>

        <!-- 占位：后续实现完整配置表单 -->
        <div class="placeholder-card">
          <p>配置表单将在后续完善 ✨</p>
          <p class="hint">当前可点击右上角齿轮图标在弹出的对话框中配置</p>
        </div>
      </div>

      <!-- 偏好 -->
      <div v-else-if="activeTab === 'preferences'" class="settings-section">
        <div class="placeholder-card">
          <p>偏好设置即将到来 🎨</p>
        </div>
      </div>

      <!-- 关于 -->
      <div v-else class="settings-section">
        <div class="placeholder-card">
          <h3>Volya v0.1.0</h3>
          <p>一款温暖的 AI 小说创作工具 🍊</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  height: 100%;
  overflow: hidden;
}

/* ─── 左侧导航 ─── */
.settings-nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  width: 160px;
  padding: var(--space-4) var(--space-2);
  border-right: 1px solid var(--border-light);
  flex-shrink: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.nav-item:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--bg-tertiary);
  color: var(--accent-primary);
  font-weight: 600;
}

.nav-item:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* ─── 右侧内容 ─── */
.settings-content {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}

.settings-section {
  max-width: 520px;
}

.section-hint {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  margin-bottom: var(--space-6);
}

/* ─── 状态栏 ─── */
.status-bar {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  margin-bottom: var(--space-4);
}

.status-bar.connected {
  color: var(--success);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

/* ─── 占位卡片 ─── */
.placeholder-card {
  padding: var(--space-6);
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  border: 1px solid var(--border-light);
  text-align: center;
  color: var(--text-secondary);
}

.placeholder-card .hint {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  margin-top: var(--space-2);
}

.placeholder-card h3 {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
  margin-bottom: var(--space-2);
  color: var(--text-primary);
}
</style>
