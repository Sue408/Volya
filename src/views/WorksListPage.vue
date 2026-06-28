<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Plus, ArrowRight, Trash2, Book, Clock } from '@lucide/vue'
import { useWorks } from '../composables/useWorks'

const router = useRouter()
const { works, loading, listWorks, createWork, deleteWork } = useWorks()

const selectedId = ref<string | null>(null)
const creating = ref(false)
const showNewDialog = ref(false)
const newTitle = ref('')
const showDeleteConfirm = ref(false)
const deleteTarget = ref<{ id: string; title: string } | null>(null)
const deletingId = ref<string | null>(null)

onMounted(async () => {
  await listWorks()
  if (works.value.length > 0) {
    selectedId.value = works.value[0].id
  }
})

const sidebarListRef = ref<HTMLElement | null>(null)
const scrolled = ref(false)
const scrolledToBottom = ref(false)

function onSidebarScroll() {
  const el = sidebarListRef.value
  if (!el) return
  scrolled.value = el.scrollTop > 0
  scrolledToBottom.value = el.scrollHeight - el.scrollTop - el.clientHeight < 2
}

const selectedWork = computed(() =>
  works.value.find(w => w.id === selectedId.value) || null
)

function selectWork(id: string) { selectedId.value = id }

function openWork(id: string) { router.push(`/works/${id}`) }

async function handleCreate() {
  const title = newTitle.value.trim()
  if (!title) return
  creating.value = true
  const id = await createWork(title)
  creating.value = false
  showNewDialog.value = false
  newTitle.value = ''
  if (id) { selectedId.value = id }
}

function confirmDelete(work: { id: string; title: string }) {
  deleteTarget.value = work
  showDeleteConfirm.value = true
}

async function handleDelete() {
  if (!deleteTarget.value) return
  deletingId.value = deleteTarget.value.id
  const ok = await deleteWork(deleteTarget.value.id)
  deletingId.value = null
  showDeleteConfirm.value = false
  if (ok && selectedId.value === deleteTarget.value.id) {
    selectedId.value = works.value[0]?.id || null
  }
  deleteTarget.value = null
}

function statusLabel(status: string): string {
  const map: Record<string, string> = { Draft: '草稿', InProgress: '进行中', Completed: '已完成' }
  return map[status] || status
}

function timeAgo(iso: string): string {
  const d = new Date(iso); const n = new Date()
  const m = Math.floor((n.getTime() - d.getTime()) / 60000)
  if (m < 1) return '刚刚'
  if (m < 60) return `${m} 分钟前`
  const h = Math.floor(m / 60)
  if (h < 24) return `${h} 小时前`
  const day = Math.floor(h / 24)
  return day < 7 ? `${day} 天前` : d.toLocaleDateString('zh-CN')
}
</script>

<template>
  <div class="works-list">
    <div class="workspace">
      <!-- ═══ Title Wrapper ═══ -->
      <div class="title-wrapper">
        <h1 class="page-title">Volya</h1>
        <p class="page-subtitle">落笔生花 · 万物成书</p>
      </div>

      <!-- ═══ Main Wrapper ═══ -->
      <div class="main-wrapper">
        <!-- 左侧：作品选择栏 (1) -->
        <aside class="works-sidebar card">
          <div class="sidebar-header">
            <Book :size="16" />
            <span class="sidebar-label">作品</span>
            <button class="sidebar-add" @click="showNewDialog = true" title="新建作品">
              <Plus :size="14" />
            </button>
          </div>

          <div v-if="loading" class="sidebar-loading">
            <span class="loading-dot"></span>
          </div>

          <div v-else-if="works.length === 0" class="sidebar-empty">
            <p>还没有作品</p>
          </div>

          <div v-else ref="sidebarListRef" class="sidebar-list" :class="{ 'scrolled': scrolled, 'at-bottom': scrolledToBottom }" @scroll="onSidebarScroll">
            <div
              v-for="work in works"
              :key="work.id"
              class="sidebar-item"
              :class="{ active: selectedId === work.id }"
              @click="selectWork(work.id)"
            >
              <div class="item-top">
                <span class="item-title">{{ work.title }}</span>
                <button class="item-delete" @click.stop="confirmDelete(work)" title="删除">
                  <Trash2 :size="12" />
                </button>
              </div>
              <div class="item-meta">
                <span class="item-status" :class="`s-${work.status.toLowerCase()}`">{{ statusLabel(work.status) }}</span>
                <span class="item-words">{{ work.completed_words.toLocaleString() }} 字</span>
              </div>
            </div>
          </div>
        </aside>

        <!-- 右侧：详情预览 (9) -->
        <main class="works-preview">
          <Transition name="preview-fade">
            <div v-if="works.length === 0" key="empty" class="preview-empty card">
              <p>还没有作品，开始创作吧</p>
            </div>
            <div v-else-if="loading" key="loading" class="preview-loading">
              <span class="loading-dot"></span>
            </div>
            <div v-else-if="selectedWork" :key="selectedWork.id" class="preview-detail card">
              <div class="detail-top">
                <div class="detail-cover" :class="`cover-${selectedWork.status.toLowerCase()}`">
                  <Book :size="32" />
                </div>
                <div class="detail-info">
                  <h2 class="detail-title">{{ selectedWork.title }}</h2>
                  <div class="detail-stats">
                    <span class="stat-item">{{ selectedWork.completed_words.toLocaleString() }} 字</span>
                    <span class="stat-sep">·</span>
                    <span class="stat-item" :class="`s-${selectedWork.status.toLowerCase()}`">{{ statusLabel(selectedWork.status) }}</span>
                    <span class="stat-sep">·</span>
                    <Clock :size="13" />
                    <span class="stat-item">{{ timeAgo(selectedWork.updated_at) }}</span>
                  </div>
                </div>
              </div>
              <div class="detail-divider"></div>
              <div class="detail-body">
                <p class="detail-snippet">从这里继续你的故事...</p>
              </div>
              <div class="detail-actions">
                <button class="detail-btn primary" @click="openWork(selectedWork.id)">
                  <span>继续创作</span>
                  <ArrowRight :size="16" />
                </button>
              </div>
            </div>
          </Transition>
        </main>
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
              {{ creating ? '创建中...' : '开始创作' }}
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
  padding: var(--space-8) var(--space-6);
  overflow-y: auto;
}

.workspace {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
  width: 86%;
  max-width: 960px;
  min-width: 0;
  flex: 1;
  min-height: 0;
}

/* ═══ Title Wrapper ═══ */
.title-wrapper {
  padding-bottom: var(--space-1);
}

.page-title {
  font-family: var(--font-logo);
  font-size: var(--font-size-3xl);
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 0.06em;
  line-height: 1.1;
  margin-bottom: var(--space-2);
  transition: font-size var(--transition-normal);
}

.page-subtitle {
  font-family: var(--font-display);
  font-size: var(--font-size-base);
  font-weight: 500;
  color: var(--text-tertiary);
  transition: font-size var(--transition-normal);
}

/* 窗口较宽时标题放大 */
@media (min-width: 1200px) {
  .page-title { font-size: var(--font-size-4xl); }
  .page-subtitle { font-size: var(--font-size-lg); }
}

@media (min-width: 1600px) {
  .page-title { font-size: 3rem; }
  .page-subtitle { font-size: var(--font-size-xl); }
}

/* ═══ Main Wrapper: 1:9 卡片布局 ═══ */
.main-wrapper {
  display: flex;
  gap: var(--space-5);
  flex: 1;
  min-height: 0;
  max-height: 460px;
}

/* ─── 左侧：作品选择栏 (1) ─── */
.works-sidebar {
  width: 28%;
  min-width: 160px;
  max-width: 240px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  padding: var(--space-4);
  min-height: 0;
}

.sidebar-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  font-weight: 600;
}

.sidebar-add {
  margin-left: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.sidebar-add:hover {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.sidebar-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  overflow-y: auto;
  flex: 1;
  min-height: 0;
  padding-right: var(--space-1);
  margin-right: calc(var(--space-1) * -1);
  /* 完全隐藏滚动条，保留滑动功能 */
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.sidebar-list::-webkit-scrollbar {
  display: none;
}

/* ─── 动态上下边缘淡出 ───
   scrollTop=0:  仅底部淡化
   scrollTop>0:  上下都淡化
   scrolled 到底: 仅顶部淡化 */
.sidebar-list {
  -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
  mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
  -webkit-mask-size: 100% 100%;
  mask-size: 100% 100%;
  -webkit-mask-repeat: no-repeat;
  mask-repeat: no-repeat;
  transition: -webkit-mask-image var(--transition-normal), mask-image var(--transition-normal);
}

.sidebar-list.scrolled {
  -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
  mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
}

.sidebar-list.at-bottom {
  -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 100%);
  mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 100%);
}

.sidebar-item {
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  transition: background var(--transition-fast), transform var(--transition-fast);
  will-change: transform;
}

.sidebar-item:hover {
  background: var(--bg-hover);
  transform: scale(1.03);
}

.sidebar-item.active {
  background: var(--bg-tertiary);
}

.sidebar-item.active .item-title {
  color: var(--accent-primary);
}

.item-top {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.item-title {
  flex: 1;
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-delete {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  opacity: 0;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.sidebar-item:hover .item-delete {
  opacity: 1;
}

.item-delete:hover {
  background: var(--error);
  color: white;
}

.item-meta {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--font-size-xs);
}

.item-status.s-inprogress { color: var(--accent-primary); }
.item-status.s-completed { color: var(--success); }
.item-status.s-draft { color: var(--text-tertiary); }

.item-words {
  color: var(--text-tertiary);
}

/* ─── 右侧：详情预览 (9) ─── */
.works-preview {
  flex: 1;
  min-width: 0;
  min-height: 0;
  position: relative;
}

.preview-detail {
  padding: var(--space-8);
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  height: 100%;
  box-sizing: border-box;
}

.detail-top {
  display: flex;
  gap: var(--space-5);
  align-items: flex-start;
}

.detail-cover {
  width: 80px;
  height: 100px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: white;
}

.cover-draft { background: linear-gradient(145deg, var(--bg-tertiary), var(--border-color)); color: var(--text-tertiary); }
.cover-inprogress { background: linear-gradient(145deg, var(--accent-primary), var(--accent-hover)); }
.cover-completed { background: linear-gradient(145deg, var(--success), #3d7a52); }

.detail-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding-top: var(--space-1);
}

.detail-title {
  font-family: var(--font-display);
  font-size: var(--font-size-2xl);
  font-weight: 600;
  color: var(--text-primary);
}

.detail-stats {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  flex-wrap: wrap;
}

.stat-sep { opacity: 0.3; }

.stat-item.s-inprogress { color: var(--accent-primary); }
.stat-item.s-completed { color: var(--success); }

.detail-divider {
  height: 1px;
  background: var(--divider-color);
}

.detail-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.detail-snippet {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
  color: var(--text-tertiary);
  font-style: italic;
  text-align: center;
}

.detail-actions {
  display: flex;
  justify-content: flex-end;
}

.detail-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-5);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.detail-btn.primary {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.detail-btn.primary:hover {
  background: var(--accent-hover);
  gap: var(--space-3);
}

/* ─── 预览卡片淡入淡出 ───
     绝对定位叠放，避免离开元素移除后父容器高度抖动 */
.preview-fade-enter-active,
.preview-fade-leave-active {
  transition: opacity 200ms ease;
  position: absolute;
  inset: 0;
}

.preview-fade-enter-from,
.preview-fade-leave-to {
  opacity: 0;
}

/* ─── 空/加载 ─── */
.sidebar-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  flex: 1;
}

.sidebar-empty {
  display: flex;
  flex: 1;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
}

.preview-empty {
  height: 100%;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-loading {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-empty,
.preview-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: var(--space-8);
  font-size: var(--font-size-base);
  color: var(--text-tertiary);
}

.preview-empty {
  min-height: 200px;
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

/* ═══ 对话框 ═══ */
.dialog-desc { font-size: var(--font-size-base); color: var(--text-secondary); margin-bottom: var(--space-5); line-height: 1.5; }
.dialog-warn { border-top: 3px solid var(--error); }
.btn-ghost { background: transparent; color: var(--text-tertiary); padding: var(--space-2) var(--space-5); border: none; border-radius: var(--radius-md); font-family: var(--font-sans); font-size: var(--font-size-sm); cursor: pointer; transition: color var(--transition-fast); }
.btn-ghost:hover { color: var(--text-secondary); }
.btn-danger { background: var(--error); color: white; padding: var(--space-2) var(--space-5); border: none; border-radius: var(--radius-md); font-family: var(--font-sans); font-size: var(--font-size-sm); font-weight: 500; cursor: pointer; }
.btn-danger:hover { background: #B05A5A; }

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
