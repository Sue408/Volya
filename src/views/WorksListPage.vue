<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Plus, ArrowRight, Trash2, Book, Clock, FileText, AlignLeft, Sigma, Feather, BookOpen, Users, Tags, ChevronRight } from '@lucide/vue'
import { useWorks } from '../composables/useWorks'

const router = useRouter()
const { works, loading, listWorks, createWork, deleteWork } = useWorks()

const selectedId = ref<string | null>(null)
const creating = ref(false)
const showNewDialog = ref(false)
const newTitle = ref('')
const newDesc = ref('')
const showMore = ref(false)
const newTargetWords = ref<string>('')
const createError = ref('')
const newStyleGuide = ref('')
const newGenre = ref('')
const newAudience = ref('')
const newTagInput = ref('')
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
  createError.value = ''

  // 校验目标字数
  let targetWordsVal: number | undefined
  if (newTargetWords.value) {
    const num = parseInt(newTargetWords.value, 10)
    if (isNaN(num) || num <= 0) {
      createError.value = '目标字数请输入大于 0 的数字'
      return
    }
    if (num > 1000) {
      createError.value = '目标字数范围 1~1000 万'
      return
    }
    targetWordsVal = num
  }

  creating.value = true
  const id = await createWork(
    title,
    newDesc.value.trim() || undefined,
    targetWordsVal,
    newStyleGuide.value.trim() || undefined,
    newGenre.value.trim() || undefined,
    newAudience.value.trim() || undefined,
    parseTags(newTagInput.value),
  )
  creating.value = false
  showNewDialog.value = false
  newTitle.value = ''
  newDesc.value = ''
  showMore.value = false
  newTargetWords.value = ''
  newStyleGuide.value = ''
  newGenre.value = ''
  newAudience.value = ''
  newTagInput.value = ''
  if (id) { selectedId.value = id }
}

function parseTags(input: string): string[] {
  return input.split(';').map(t => t.trim()).filter(t => t.length > 0)
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
              <div class="empty-illustration">
                <svg viewBox="340 240 360 520" fill="currentColor" color="var(--accent-primary)" xmlns="http://www.w3.org/2000/svg">
                  <path d="M0 0 C17.69 8.48 26.17 26.94 32.46 44.61 C35.12 52.32 37.25 60.14 39.21 68.05 C39.54 68.05 39.87 68.05 40.21 68.05 C40.5 66.77 40.79 65.49 41.09 64.17 C47.13 38.34 57.9 16.31 81.08 1.89 C87.1 -1.2 94.35 -0.83 100.64 1.11 C104.59 4.1 106.7 7.31 107.59 12.12 C108.24 24.3 101.19 34.45 93.48 43.17 C83.26 54.51 72.34 64.15 59.72 72.69 C56.07 75.15 56.07 75.15 54.21 79.05 C54.99 78.99 55.77 78.93 56.57 78.88 C79.3 77.99 104.27 83.63 122.21 98.05 C122.97 98.64 123.73 99.22 124.52 99.82 C128.44 103.55 129.65 106.67 129.93 112.13 C129.94 113.2 129.94 113.2 129.96 114.3 C129.98 115.35 129.98 115.35 130 116.42 C129.89 120.14 129.49 121.76 126.86 124.46 C120.43 128.32 113.53 129.03 106.21 128.05 C87.5 123.12 68.99 111.06 55.51 97.29 C53.34 94.85 53.34 94.85 50.21 94.05 C50.61 95.61 50.61 95.61 51.01 97.2 C51.37 98.61 51.73 100.02 52.08 101.43 C52.34 102.44 52.34 102.44 52.61 103.48 C53.16 105.67 53.69 107.86 54.21 110.05 C54.39 110.84 54.58 111.63 54.78 112.45 C56.12 118.57 56.56 124.42 56.52 130.68 C56.51 131.63 56.51 132.59 56.5 133.57 C56.28 146.48 54.64 159.09 47.21 170.05 C43.81 173.05 43.81 173.05 41.21 173.05 C41.57 188.27 42.69 203.04 45.21 218.05 C45.41 219.26 45.61 220.46 45.82 221.71 C46.04 223.01 46.26 224.31 46.48 225.64 C46.59 226.31 46.7 226.97 46.82 227.65 C47.37 230.93 47.92 234.21 48.49 237.48 C49.58 243.83 50.44 250.09 50.94 256.51 C51.2 258.96 51.65 261.27 52.2 263.67 C53.78 271.06 53.94 278.64 54.33 286.18 C54.42 287.72 54.51 289.26 54.59 290.81 C54.81 294.56 55.01 298.3 55.21 302.05 C57.08 296.79 58.59 291.58 59.74 286.11 C64.17 265.77 74.23 248.2 91.86 236.52 C95.2 234.47 98.66 232.73 102.21 231.05 C103.17 230.59 104.13 230.12 105.12 229.64 C111.12 226.94 117.33 225 123.64 223.18 C124.41 222.95 125.18 222.72 125.98 222.49 C130.06 221.36 133.04 220.71 137.21 222.05 C137.21 222.71 137.21 223.37 137.21 224.05 C135.67 225.31 135.67 225.31 133.58 226.74 C128.04 230.78 124.11 235.21 120.82 241.23 C120.39 242.02 119.95 242.81 119.51 243.63 C118.29 245.89 117.1 248.17 115.92 250.45 C110.75 260.3 105.23 268.34 97.21 276.05 C96.59 276.68 95.97 277.3 95.33 277.95 C90.31 282.72 84.39 286.3 78.55 289.97 C67.97 296.22 67.97 296.22 60.96 305.93 C60.24 307.51 60.24 307.51 59.52 309.12 C59.08 310.09 58.65 311.05 58.21 312.05 C57.48 313.32 56.75 314.6 56.02 315.86 C52.5 322.92 51.82 331.37 50.53 339.09 C48.98 348.13 46.65 356.52 43.21 365.05 C42.95 365.68 42.7 366.32 42.44 366.97 C38.3 377.28 33.15 386.67 27.21 396.05 C21.46 395.3 21.46 395.3 19.21 393.05 C19.76 392.24 20.32 391.42 20.89 390.59 C31.87 374.24 39.21 356.78 43.46 337.55 C43.62 336.83 43.78 336.11 43.95 335.36 C46.59 322.58 46.48 309.19 46.52 296.18 C46.52 295.41 46.52 294.64 46.53 293.84 C46.53 277.02 44.71 260.66 42.21 244.05 C42.03 242.84 41.84 241.63 41.66 240.38 C40.82 234.84 39.96 229.31 39.09 223.77 C36.45 206.76 35.96 190.28 36.21 173.05 C34.82 172.65 34.82 172.65 33.41 172.25 C30.29 171.08 29.07 170.38 27.02 167.86 C26.55 167.31 26.08 166.76 25.6 166.2 C21.68 160.17 20.68 155.17 20.77 148.11 C20.78 147.28 20.78 146.45 20.79 145.59 C21.27 127.48 29.18 110.46 36.21 94.05 C32.25 96.3 28.77 98.9 25.21 101.74 C7.93 115.21 -11.51 126.27 -34.04 123.8 C-40.48 122.58 -45.27 118.57 -49.79 114.05 C-50.6 107.41 -50.7 100.62 -47.18 94.76 C-37.81 83.22 -21.68 80.7 -7.79 79.05 C-4.85 78.89 -1.92 78.86 1.02 78.86 C1.83 78.86 2.65 78.86 3.49 78.86 C12.14 78.93 20.64 79.9 29.21 81.05 C28.15 80.58 28.15 80.58 27.07 80.09 C4.09 69.23 -15.77 51.47 -25.17 27.55 C-26.78 18.51 -26.6 11.98 -21.59 4.11 C-15.94 -2.36 -7.82 -2.66 0 0 Z " fill="currentColor" transform="translate(450.79296875,355.94921875)"/>
                  <path d="M0 0 C2.3 2.3 2.3 2.95 2.56 6.06 C2.56 9.77 2.56 9.77 4 11 C4.33 10.34 4.66 9.68 5 9 C5.66 9 6.32 9 7 9 C3.13 36.43 -4.05 59.54 -19 83 C-24.75 82.25 -24.75 82.25 -27 80 C-26.16 78.78 -26.16 78.78 -25.31 77.54 C-9.74 54.34 -0.84 27.98 0 0 Z " fill="currentColor" transform="translate(497,669)"/>
                  <path d="M0 0 C0.33 0 0.66 0 1 0 C1 7.92 1 15.84 1 24 C1.66 24 2.32 24 3 24 C3.33 24.99 3.66 25.98 4 27 C4.66 26.01 5.32 25.02 6 24 C6.24 25.75 6.24 25.75 6.48 27.54 C8.19 39.88 10.15 52.16 12.29 64.43 C13.4 70.92 14.31 77.33 14.77 83.9 C14.88 86.06 14.88 86.06 16 88 C16.13 90.67 16.04 93.32 16 96 C15.67 94.68 15.34 93.36 15 92 C14.34 92 13.68 92 13 92 C12.34 95.96 11.68 99.92 11 104 C10.67 104 10.34 104 10 104 C9.9 103.14 9.8 102.27 9.7 101.38 C7.97 86.58 6.22 71.79 3.88 57.06 C0.86 37.82 -0.28 19.52 0 0 Z " fill="currentColor" opacity="0.7" transform="translate(487,529)"/>
                  <path d="M0 0 C3.7 2.03 5.65 3.48 8 7 C8.43 12.3 8.52 15.75 5 20 C2.12 22.32 0.24 22.96 -3.44 23.5 C-8.17 22.84 -9.66 21.34 -13 18 C-14.74 14.52 -14.59 10.8 -14 7 C-10.6 0.86 -6.8 -0.57 0 0 Z " fill="currentColor" opacity="0.5" transform="translate(406,617)"/>
                  <path d="M0 0 C2.66 0.64 3.88 1.16 5.76 3.11 C7.88 7.21 8.72 10.1 7.57 14.61 C5.8 17.63 3.75 20.03 0.57 21.61 C-8.31 22.04 -8.31 22.04 -11.43 19.61 C-13.69 16.79 -14.4 14.81 -14.99 11.24 C-14.35 7.08 -13.35 4.97 -10.48 1.95 C-7.08 -0.27 -3.97 -0.28 0 0 Z " fill="currentColor" opacity="0.55" transform="translate(648.4296875,514.38671875)"/>
                  <path d="M0 0 C5.58 3.16 5.58 3.16 7 6 C7.68 11.86 7.68 11.86 5.95 14.76 C3.22 18.09 1.49 19.83 -2.81 20.31 C-6.73 19.93 -8.21 18.71 -11 16 C-12.75 12.5 -12.69 8.81 -12 5 C-9.1 0.62 -5.11 -0.96 0 0 Z " fill="currentColor" opacity="0.45" transform="translate(383,520)"/>
                  <path d="M0 0 C3.77 0.71 5.44 1.7 8 4.56 C8.72 8.33 8.3 10.02 6.44 13.38 C3.21 16.27 1.31 16.71 -3 16.56 C-6.42 14.16 -7.77 12.68 -8.62 8.56 C-7.58 3.56 -5.04 0.94 0 0 Z " fill="currentColor" opacity="0.5" transform="translate(376,335.4375)"/>
                  <path d="M0 0 C2.38 1.56 2.38 1.56 4 4 C4.56 7.5 4.56 7.5 4 11 C1.58 13.7 -0.01 14.81 -3.56 15.5 C-6.92 14.81 -8.05 13.82 -10 11 C-10.57 7.03 -10.62 4.94 -8.38 1.56 C-5.28 -0.47 -3.64 -0.61 0 0 Z " fill="currentColor" opacity="0.5" transform="translate(658,286)"/>
                </svg>

                <p class="empty-text">开始你的第一个故事吧</p>
                <button class="empty-btn" @click="showNewDialog = true">
                  ✦ 开始创作
                </button>
              </div>
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
                    <span v-if="selectedWork.target_words" class="stat-item">/ {{ (selectedWork.target_words / 10000).toLocaleString() }} 万</span>
                    <span class="stat-sep">·</span>
                    <span class="stat-item" :class="`s-${selectedWork.status.toLowerCase()}`">{{ statusLabel(selectedWork.status) }}</span>
                    <span class="stat-sep">·</span>
                    <Clock :size="13" />
                    <span class="stat-item">{{ timeAgo(selectedWork.updated_at) }}</span>
                  </div>
                </div>
              </div>
              <div class="detail-divider"></div>
              <div class="detail-tags">
                <span v-if="selectedWork.genre" class="detail-genre">{{ selectedWork.genre }}</span>
                <span v-for="tag in selectedWork.tags" :key="tag" class="detail-tag">{{ tag }}</span>
              </div>
              <div class="detail-body">
                <p v-if="selectedWork.description" class="detail-desc">{{ selectedWork.description }}</p>
                <p v-else class="detail-snippet">从这里继续你的故事...</p>
              </div>
              <div class="detail-footer">
                <span class="footer-tokens">{{ selectedWork.total_tokens.toLocaleString() }} tokens</span>
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
        <div class="dialog-card create-card">
          <h2 class="dialog-title">新建作品</h2>

          <div class="dialog-body">
            <!-- 核心区 -->
            <div class="field-group">
              <label class="field-label"><FileText :size="14" /> 作品名称</label>
              <input
                v-model="newTitle"
                class="dialog-input"
                placeholder="给故事取个名字..."
                @keydown.enter="handleCreate"
                autofocus
              />
            </div>
            <div class="field-group">
              <label class="field-label"><AlignLeft :size="14" /> 简介</label>
              <textarea
                v-model="newDesc"
                class="dialog-textarea desc-input"
                placeholder="用一句话说说这是个什么故事（可选）"
                rows="3"
              ></textarea>
            </div>

            <!-- 更多设置 -->
            <div class="more-toggle" @click="showMore = !showMore">
              <span class="more-line"></span>
              <span class="more-label">{{ showMore ? '收起设置' : '更多设置' }}</span>
              <ChevronRight :size="12" class="more-arrow" :class="{ open: showMore }" />
            </div>

            <div class="more-fields-wrap" :class="{ open: showMore }">
              <div class="more-fields">
                <div class="field-group">
                  <label class="field-label"><Sigma :size="14" /> 目标字数</label>
                  <div class="input-suffix">
                    <input v-model="newTargetWords" class="dialog-input field-input" type="text" inputmode="numeric" placeholder="10" />
                    <span class="suffix">万</span>
                  </div>
                </div>
                <div class="field-group">
                  <label class="field-label"><Feather :size="14" /> 风格指南</label>
                  <textarea v-model="newStyleGuide" class="dialog-textarea" rows="3" placeholder="例：文风参考村上春树，冷峻细腻"></textarea>
                </div>
                <div class="field-group">
                  <label class="field-label"><BookOpen :size="14" /> 体裁</label>
                  <input v-model="newGenre" class="dialog-input field-input" placeholder="玄幻 / 科幻 / 言情 / 悬疑 ..." />
                </div>
                <div class="field-group">
                  <label class="field-label"><Users :size="14" /> 目标读者</label>
                  <input v-model="newAudience" class="dialog-input field-input" placeholder="例：18-25 岁女性" />
                </div>
                <div class="field-group">
                  <label class="field-label"><Tags :size="14" /> 标签</label>
                  <input v-model="newTagInput" class="dialog-input" placeholder="用 ; 分隔多个标签" />

                </div>
              </div>
            </div>
          </div>

          <p v-if="createError" class="create-error">{{ createError }}</p>

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

.detail-metrics {
  display: flex;
  gap: var(--space-6);
}

.metric-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-1);
}

.metric-value {
  font-family: var(--font-display);
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
}

.metric-label {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
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
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.detail-desc {
  font-family: var(--font-display);
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  line-height: 1.7;
  text-align: left;
  width: 100%;
  white-space: pre-wrap;
  word-break: break-word;
  padding-top: var(--space-1);
}

.detail-snippet {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
  color: var(--text-tertiary);
  font-style: italic;
  text-align: center;
  margin: auto;
}

.detail-snippet {
  font-family: var(--font-display);
  font-size: var(--font-size-lg);
  color: var(--text-tertiary);
  font-style: italic;
  text-align: center;
}

/* ─── 体裁 + 标签 ─── */
.detail-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1);
  max-height: 60px;
  overflow-y: auto;
}

.detail-genre {
  padding: 2px 10px;
  border-radius: var(--radius-full);
  background: color-mix(in srgb, var(--accent-primary) 15%, transparent);
  color: var(--accent-primary);
  font-size: var(--font-size-xs);
  font-weight: 500;
}

.detail-tag {
  padding: 2px 8px;
  border-radius: var(--radius-full);
  background: var(--bg-tertiary);
  color: var(--text-tertiary);
  font-size: var(--font-size-xs);
}

/* ─── 底部 ─── */
.detail-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.footer-tokens {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-family: var(--font-mono);
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
  padding: var(--space-10) var(--space-8) var(--space-8);
}

.preview-loading {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* ─── 笔尖生花插图 ─── */
.empty-illustration {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
}

.empty-illustration svg {
  width: 130px;
  height: 130px;
}

.empty-text {
  font-family: var(--font-display);
  font-size: var(--font-size-base);
  color: var(--text-tertiary);
  margin: 0;
}

.empty-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-5);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.empty-btn:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
  background: color-mix(in srgb, var(--accent-primary) 8%, transparent);
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

.create-card {
  width: 500px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  margin-bottom: var(--space-5);
  padding-right: var(--space-1);
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.dialog-body::-webkit-scrollbar {
  display: none;
}

.dialog-textarea {
  width: 100%;
  padding: var(--space-3) var(--space-4);
  border: 1.5px solid var(--border-color);
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  background: var(--bg-input);
  outline: none;
  resize: none;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  margin-bottom: var(--space-4);
}

.dialog-textarea:focus {
  border-color: var(--accent-primary);
  box-shadow: var(--shadow-focus);
}

.dialog-textarea::placeholder {
  color: var(--text-tertiary);
}

.desc-input {
  max-height: 5.6em;
  overflow-y: auto;
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

/* ─── 更多设置切换 ─── */
.more-toggle {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  user-select: none;
  padding: var(--space-1) 0;
  margin-bottom: var(--space-3);
}

.more-line {
  flex: 1;
  height: 1px;
  background: var(--divider-color);
}

.more-label {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  letter-spacing: 0.04em;
}

.more-arrow {
  color: var(--text-tertiary);
  transition: transform var(--transition-fast);
}

.more-arrow.open {
  transform: rotate(90deg);
}

/* ─── 展开区（带动画） ─── */
.more-fields-wrap {
  max-height: 0;
  opacity: 0;
  overflow: hidden;
  transition: max-height 0.35s ease, opacity 0.3s ease;
}

.more-fields-wrap.open {
  max-height: 600px;
  opacity: 1;
}

.more-fields {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
  padding-top: var(--space-2);
}

.field-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.field-label {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: 500;
}

.field-input {
  margin-bottom: 0 !important;
}

.input-suffix {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

.input-suffix .field-input {
  flex: 1;
}

.suffix {
  font-size: var(--font-size-sm);
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.create-error {
  font-size: var(--font-size-xs);
  color: var(--error);
  margin-bottom: var(--space-3);
  text-align: right;
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
