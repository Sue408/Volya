<script setup lang="ts">
import { Menu, FileText, Users, Map, BarChart3 } from '@lucide/vue'

defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
}>()
</script>

<template>
  <aside class="work-sidebar" :class="{ collapsed: !open }">
    <div class="sidebar-header">
      <button class="toggle-btn" @click="emit('update:open', !open)" title="切换侧栏">
        <Menu :size="16" />
      </button>
      <span v-if="open" class="sidebar-title">导航</span>
    </div>

    <nav v-if="open" class="sidebar-nav">
      <button class="nav-btn active">
        <FileText :size="16" class="nav-icon" />
        <span class="nav-label">大纲</span>
      </button>
      <button class="nav-btn" disabled>
        <Users :size="16" class="nav-icon" />
        <span class="nav-label">角色</span>
      </button>
      <button class="nav-btn" disabled>
        <Map :size="16" class="nav-icon" />
        <span class="nav-label">图谱</span>
      </button>
      <button class="nav-btn" disabled>
        <BarChart3 :size="16" class="nav-icon" />
        <span class="nav-label">统计</span>
      </button>
    </nav>
  </aside>
</template>

<style scoped>
.work-sidebar {
  display: flex;
  flex-direction: column;
  width: 180px;
  background: var(--bg-primary);
  transition: width var(--transition-normal);
  overflow: hidden;
  flex-shrink: 0;
}

.work-sidebar.collapsed {
  width: 44px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-2);
  min-height: 40px;
}

.toggle-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.toggle-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.sidebar-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-secondary);
  white-space: nowrap;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: var(--space-2);
}

.nav-btn {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-2);
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

.nav-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.nav-btn.active {
  background: var(--bg-tertiary);
  color: var(--accent-primary);
  font-weight: 600;
}

.nav-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.nav-icon {
  font-size: 16px;
  line-height: 1;
  flex-shrink: 0;
}

.nav-label {
  white-space: nowrap;
}
</style>
