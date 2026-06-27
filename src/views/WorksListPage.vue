<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Plus, Book, FileText, Sparkles } from '@lucide/vue'
import { useWorks, workStatusLabels, type WorkStatus } from '../composables/useWorks'

const router = useRouter()
const { works, loading, listWorks, createWork, timeAgo } = useWorks()

const creating = ref(false)
const showNewDialog = ref(false)
const newTitle = ref('')

onMounted(() => {
  listWorks()
})

/** 新建作品 */
async function handleCreate() {
  const title = newTitle.value.trim()
  if (!title) return
  creating.value = true
  const id = await createWork(title)
  creating.value = false
  showNewDialog.value = false
  newTitle.value = ''
  if (id) {
    router.push(`/works/${id}`)
  }
}

/** 点击作品卡片 */
function openWork(id: string) {
  router.push(`/works/${id}`)
}
</script>

<template>
  <div class="works-list">
    <!-- 问候语 -->
    <div class="greeting">
      <h1 class="greeting-title">✦ 你好呀，今天想创作什么故事？</h1>
      <p class="greeting-desc" v-if="works.length > 0">
        你有 {{ works.length }} 个作品，继续你的创作之旅吧 🍊
      </p>
      <p class="greeting-desc" v-else>
        还没有作品，开始你的第一部小说吧 🍊
      </p>
    </div>

    <!-- 加载中 -->
    <div v-if="loading" class="loading-state">
      <span class="loading-dot"></span>
      <span>加载中...</span>
    </div>

    <!-- 作品网格 -->
    <div v-else class="works-grid">
      <div
        v-for="work in works"
        :key="work.id"
        class="work-card"
        @click="openWork(work.id)"
      >
        <div class="work-card-cover" :class="`cover-${work.status.toLowerCase()}`">
          <Book :size="24" class="work-card-cover-icon" />
        </div>
        <div class="work-card-body">
          <h3 class="work-card-title">{{ work.title }}</h3>
          <div class="work-card-meta">
            <span class="work-card-status" :class="`status-${work.status.toLowerCase()}`">
              {{ workStatusLabels[work.status as WorkStatus]?.icon }}
              {{ workStatusLabels[work.status as WorkStatus]?.label }}
            </span>
            <span class="work-card-words">{{ work.completed_words.toLocaleString() }} 字</span>
          </div>
          <div class="work-card-time">{{ timeAgo(work.updated_at) }}</div>
        </div>
      </div>

      <!-- 新建作品卡片 -->
      <div class="new-work-card" @click="showNewDialog = true">
        <Plus :size="28" class="new-work-icon" />
        <span class="new-work-label">新建作品</span>
        <span class="new-work-hint">或拖拽文件夹到此处</span>
      </div>
    </div>

    <!-- Doro 小尾巴 -->
    <div class="doro-footer">
      <Sparkles :size="14" />
      <span>Doro 在陪着你哦~</span>
    </div>

    <!-- 新建作品对话框 -->
    <Teleport to="body">
      <div v-if="showNewDialog" class="dialog-overlay" @click.self="showNewDialog = false">
        <div class="dialog-card">
          <h2 class="dialog-title">新建作品</h2>
          <input
            v-model="newTitle"
            class="dialog-input"
            placeholder="输入作品名称..."
            @keydown.enter="handleCreate"
            autofocus
          />
          <div class="dialog-actions">
            <button class="btn btn-secondary" @click="showNewDialog = false">取消</button>
            <button
              class="btn btn-primary"
              :disabled="creating || !newTitle.trim()"
              @click="handleCreate"
            >
              {{ creating ? '创建中...' : '开始创作 ✨' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
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

/* ─── 问候 ─── */
.greeting {
  text-align: center;
  margin-bottom: var(--space-8);
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

/* ─── 加载 ─── */
.loading-state {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
  padding: var(--space-8) 0;
}

.loading-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent-primary);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

/* ─── 作品网格 ─── */
.works-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--space-4);
  width: 100%;
  max-width: 720px;
}

/* ─── 作品卡片 ─── */
.work-card {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  overflow: hidden;
  transition: all var(--transition-fast);
  border: 1px solid var(--border-light);
}

.work-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
  border-color: var(--border-color);
}

.work-card-cover {
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
}

.work-card-cover.cover-draft {
  background: linear-gradient(135deg, var(--sage-100), var(--sage-200));
}

.work-card-cover.cover-inprogress {
  background: linear-gradient(135deg, var(--sage-200), var(--sage-300));
}

.work-card-cover.cover-completed {
  background: linear-gradient(135deg, var(--apricot-100), var(--apricot-200));
}

.work-card-emoji {
  font-size: 28px;
}

.work-card-body {
  padding: var(--space-3);
}

.work-card-title {
  font-family: var(--font-display);
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--space-2);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.work-card-meta {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-1);
}

.work-card-status {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.work-card-status.status-inprogress {
  color: var(--accent-primary);
}

.work-card-status.status-completed {
  color: var(--success);
}

.work-card-words {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.work-card-time {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

/* ─── 新建卡片 ─── */
.new-work-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: 100%;
  min-height: 180px;
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
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  margin-top: auto;
  padding-top: var(--space-8);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  opacity: 0.6;
}

.work-card-cover-icon {
  color: var(--accent-primary);
  opacity: 0.6;
}

/* ─── 新建对话框 ─── */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.dialog-card {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  padding: var(--space-6);
  width: 380px;
  max-width: 90vw;
  animation: slideUp 0.25s ease;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.dialog-title {
  font-family: var(--font-display);
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--space-4);
}

.dialog-input {
  width: 100%;
  padding: var(--space-3) var(--space-4);
  border: 1.5px solid var(--border-color);
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  color: var(--text-primary);
  background: var(--bg-input);
  outline: none;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  margin-bottom: var(--space-5);
}

.dialog-input:focus {
  border-color: var(--accent-primary);
  box-shadow: var(--shadow-focus);
}

.dialog-input::placeholder {
  color: var(--text-tertiary);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
}

.btn {
  padding: var(--space-2) var(--space-5);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-secondary {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

.btn-primary {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
