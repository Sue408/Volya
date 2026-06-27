<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Plus, Trash2 } from '@lucide/vue'
import { useWorks } from '../composables/useWorks'

const router = useRouter()
const { works, loading, listWorks, createWork, deleteWork } = useWorks()

const hoverNew = ref(false)
const creating = ref(false)
const showNewDialog = ref(false)
const newTitle = ref('')

const deletingId = ref<string | null>(null)
const showDeleteConfirm = ref(false)
const deleteTarget = ref<{ id: string; title: string } | null>(null)

onMounted(() => { listWorks() })

async function handleCreate() {
  const title = newTitle.value.trim()
  if (!title) return
  creating.value = true
  const id = await createWork(title)
  creating.value = false
  showNewDialog.value = false
  newTitle.value = ''
  if (id) router.push(`/works/${id}`)
}

function openWork(id: string) { router.push(`/works/${id}`) }

function confirmDelete(work: { id: string; title: string }) {
  deleteTarget.value = work
  showDeleteConfirm.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deletingId.value = deleteTarget.value.id
  await deleteWork(deleteTarget.value.id)
  deletingId.value = null
  showDeleteConfirm.value = false
  deleteTarget.value = null
}

function statusLabel(status: string): string {
  const map: Record<string, string> = { Draft: '草稿', InProgress: '进行中', Completed: '已完成' }
  return map[status] || status
}
</script>

<template>
  <div class="works-list">
    <!-- 上区：标题 + 副标题 -->
    <div class="top-section">
      <h1 class="page-title">Volya</h1>
      <p class="page-subtitle">落笔生花 · 万物成书</p>
    </div>

    <!-- 下区：控制栏 + 作品列表（占剩余高度） -->
    <div class="content-section">
      <!-- Control bar -->
      <div class="control-bar">
        <span class="control-label">我的作品</span>
        <button
          class="btn-new"
          :class="{ expanded: hoverNew }"
          @mouseenter="hoverNew = true"
          @mouseleave="hoverNew = false"
          @click="showNewDialog = true"
        >
          <template v-if="hoverNew">
            <Plus :size="16" />
            <span class="btn-new-label">新建作品</span>
          </template>
          <Plus v-else :size="16" />
        </button>
      </div>

      <!-- Works list -->
      <div v-if="!loading" class="works-list-vertical">
        <div v-if="works.length === 0" class="works-empty">
          <p class="empty-hint">还没有作品，点击右上角 + 开始创作吧</p>
        </div>

        <div
          v-for="work in works"
          :key="work.id"
          class="work-row"
          @click="openWork(work.id)"
        >
          <div class="row-left">
            <h3 class="row-title">{{ work.title }}</h3>
            <span class="row-words">{{ work.completed_words.toLocaleString() }} 字</span>
          </div>
          <div class="row-right">
            <span class="row-status">{{ statusLabel(work.status) }}</span>
            <button
              class="row-delete"
              :class="{ loading: deletingId === work.id }"
              @click.stop="confirmDelete(work)"
              title="删除作品"
            >
              <Trash2 :size="14" />
            </button>
            <span class="row-arrow">&rarr;</span>
          </div>
        </div>
      </div>

      <!-- 加载 -->
      <div v-if="loading" class="loading-state">
        <span class="loading-dot"></span>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <Teleport to="body">
      <div v-if="showDeleteConfirm" class="dialog-overlay" @click.self="showDeleteConfirm = false">
        <div class="dialog-card dialog-warn">
          <h2 class="dialog-title">删除作品</h2>
          <p class="dialog-desc">确定要删除「{{ deleteTarget?.title }}」吗？此操作不可撤销。</p>
          <div class="dialog-actions">
            <button class="btn btn-ghost" @click="showDeleteConfirm = false">取消</button>
            <button class="btn btn-danger" @click="handleDelete">
              {{ deletingId ? '删除中...' : '删除' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>

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
  padding: 5vh 10vw;
  overflow-y: auto;
}

/* ─── 上区：标题 ─── */
.top-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 4vh;
  margin-bottom: 5vh;
}

.page-title {
  font-family: var(--font-logo);
  font-size: var(--font-size-5xl);
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 0.06em;
  line-height: 1.1;
  margin-bottom: var(--space-3);
}

.page-subtitle {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
  font-weight: 500;
  color: var(--text-tertiary);
}

/* ─── 下区：内容 ─── */
.content-section {
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 520px;
  flex: 1;
}

/* ─── 控制栏 ─── */
.control-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: var(--space-3);
  margin-bottom: var(--space-4);
  border-bottom: 1px solid var(--border-light);
}

.control-label {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 0.04em;
  text-transform: uppercase;
}

/* ─── 新建按钮：v-if 切换两种状态 ─── */
.btn-new {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  width: 36px;
  height: 36px;
  padding: 0;
  border: none;
  border-radius: var(--radius-full);
  background: var(--accent-primary);
  color: var(--text-on-color);
  cursor: pointer;
  transition: width 0.25s ease, box-shadow 0.25s ease, padding 0.25s ease;
  white-space: nowrap;
  flex-shrink: 0;
  overflow: hidden;
}

.btn-new.expanded {
  width: 130px;
  padding: 0 var(--space-4);
  box-shadow: 0 0 12px rgba(201,96,58,0.35);
}

.btn-new-label {
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-on-color);
}

/* ─── 纵向作品列表 ─── */
.works-list-vertical {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.work-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.work-row:hover {
  background: var(--bg-tertiary);
}

.work-row:active {
  transform: scale(0.99);
}

.row-delete {
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
  opacity: 0;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.work-row:hover .row-delete {
  opacity: 1;
}

.row-delete:hover {
  background: var(--error);
  color: white;
  opacity: 0.85;
}

.row-delete.loading {
  opacity: 1;
  animation: pulse 1.5s ease-in-out infinite;
}

.row-left {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  min-width: 0;
}

.row-title {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-words {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  white-space: nowrap;
}

.row-right {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-shrink: 0;
}

.row-status {
  font-size: var(--font-size-xs);
  color: var(--accent-primary);
}

.row-arrow {
  font-size: var(--font-size-lg);
  color: var(--text-tertiary);
  transition: transform var(--transition-fast);
}

.work-row:hover .row-arrow {
  transform: translateX(4px);
  color: var(--accent-primary);
}

/* ─── 空状态 ─── */
.works-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-10) var(--space-4);
}

.empty-hint {
  font-size: var(--font-size-base);
  color: var(--text-tertiary);
}

/* ─── 加载 ─── */
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.loading-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-primary);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

/* ─── 对话框通用 ─── */
.dialog-desc {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin-bottom: var(--space-5);
  line-height: 1.5;
}

.dialog-warn {
  border-top: 3px solid var(--error);
}

.btn-ghost {
  background: transparent;
  color: var(--text-tertiary);
  padding: var(--space-2) var(--space-5);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: color var(--transition-fast);
}

.btn-ghost:hover {
  color: var(--text-secondary);
}

.btn-danger {
  background: var(--error);
  color: white;
  padding: var(--space-2) var(--space-5);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: opacity var(--transition-fast);
}

.btn-danger:hover {
  background: #B05A5A;
}

/* ===========================================
   新建对话框
   =========================================== */
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
  width: 360px;
  max-width: 90vw;
  animation: slideUp 0.25s ease;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.dialog-title {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
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
