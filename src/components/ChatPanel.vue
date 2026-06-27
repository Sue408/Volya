<script setup lang="ts">
import { ref, nextTick, watch, onUnmounted } from 'vue'
import MessageBubble from './MessageBubble.vue'
import ChatInput from './ChatInput.vue'
import SettingsDialog from './SettingsDialog.vue'
import { useAgent } from '../composables/useAgent'

const emit = defineEmits<{
  workCreated: []
}>()

const {
  messages,
  status,
  isProcessing,
  currentPermissionLevel,
  pendingApproval,
  llmConfig,
  llmReady,
  init,
  sendMessage,
  respondApproval,
  setPermissionLevel,
  loadLlmConfig,
  saveLlmConfig,
  cleanup,
} = useAgent()

const messagesContainer = ref<HTMLElement | null>(null)

// 组件卸载时清理 Tauri 事件监听，避免热更新重复注册
onUnmounted(() => cleanup())
const showPermissionMenu = ref(false)
const showSettings = ref(false)

// 初始化会话
init('我的新作品').then(async () => {
  emit('workCreated')
  await loadLlmConfig()
  const greeting = llmReady.value
    ? '你好呀！我是 **Doro** 🍊，你的专属创作助手！\n\n我可以帮你：\n- 📖 **管理作品** — 查看和修改作品元数据\n- 👤 **创建角色** — 设计人物、地点、势力等\n- 📝 **生成正文** — 辅助创作章节内容\n- 🔗 **管理图谱** — 构建故事的关系网络\n\n当前权限模式：**半自动 (Lv 1)**\n现在开始我们的创作之旅吧！✨'
    : '你好呀！我是 **Doro** 🍊～\n\n欢迎来到 Volya！在开始之前，需要你配置一下 LLM：\n1. 点击右上角的 ⚙️ 齿轮图标\n2. 输入你的 Anthropic API Key\n3. 选择模型，然后保存配置\n\n配置完成后，我们就可以开始创作啦！🎉'
  messages.value.push({
    id: 0,
    role: 'assistant',
    content: greeting,
    timestamp: new Date(),
  })
})

// 自动滚动到底部
watch(
  () => messages.value.length,
  async () => {
    await nextTick()
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  }
)

// 权限等级标签
const permissionLabels = ['仅建议 (Lv 0)', '半自动 (Lv 1)', '全自动 (Lv 2)']
const permissionColors = ['var(--apricot-400)', 'var(--sage-500)', 'var(--success)']

function handleSend(text: string) {
  sendMessage(text)
}

function handleApprove() {
  if (pendingApproval.value) {
    respondApproval(pendingApproval.value.toolUseId, true)
  }
}

function handleReject() {
  if (pendingApproval.value) {
    respondApproval(pendingApproval.value.toolUseId, false)
  }
}

function handlePermissionChange(level: number) {
  setPermissionLevel(level)
  showPermissionMenu.value = false
}

async function handleSaveConfig(config: { apiKey: string; apiBase: string; model: string; maxTokens: number; temperature: number }) {
  try {
    const result = await saveLlmConfig(config)
    messages.value.push({
      id: Date.now(),
      role: 'assistant',
      content: `✅ ${result}`,
      timestamp: new Date(),
    })
  } catch (e: any) {
    messages.value.push({
      id: Date.now(),
      role: 'assistant',
      content: `❌ ${e.message || '保存配置失败'}`,
      timestamp: new Date(),
    })
  }
}
</script>

<template>
  <div class="chat-panel">
    <!-- 顶部栏 -->
    <header class="chat-header">
      <div class="header-left">
        <div class="app-logo">
          <svg width="22" height="22" viewBox="0 0 32 32" fill="none">
            <circle cx="16" cy="16" r="14" fill="var(--sage-300)" />
            <circle cx="12" cy="14" r="2.5" fill="white" />
            <circle cx="20" cy="14" r="2.5" fill="white" />
            <path d="M10 21c2 3 10 3 12 0" stroke="white" stroke-width="1.5" stroke-linecap="round" fill="none" />
          </svg>
        </div>
        <div class="app-info">
          <span class="app-name">Volya</span>
          <span class="app-status" :class="status">
            {{ status === 'idle' ? '在线' : status === 'processing' ? '处理中...' : '等待确认' }}
          </span>
        </div>
      </div>
      <div class="header-right">
        <!-- 设置按钮 -->
        <button class="settings-btn" :class="{ warned: !llmReady }" @click="showSettings = true" title="设置">
          <svg v-if="llmReady" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
          <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
            <line x1="12" y1="9" x2="12" y2="13" />
            <line x1="12" y1="17" x2="12.01" y2="17" />
          </svg>
        </button>
        <!-- 权限选择器 -->
        <div class="permission-selector" @click="showPermissionMenu = !showPermissionMenu">
          <span class="permission-dot" :style="{ background: permissionColors[currentPermissionLevel] }"></span>
          <span class="permission-label">{{ permissionLabels[currentPermissionLevel] }}</span>
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <polyline points="6 9 12 15 18 9" />
          </svg>
          <!-- 下拉菜单 -->
          <div v-if="showPermissionMenu" class="permission-dropdown">
            <button
              v-for="(label, idx) in permissionLabels"
              :key="idx"
              class="permission-option"
              :class="{ active: idx === currentPermissionLevel }"
              @click.stop="handlePermissionChange(idx)"
            >
              <span class="option-dot" :style="{ background: permissionColors[idx] }"></span>
              <span>{{ label }}</span>
            </button>
          </div>
        </div>
      </div>
    </header>

    <!-- 消息列表 -->
    <div ref="messagesContainer" class="messages-area">
      <div v-if="messages.length === 0" class="welcome-placeholder">
        <div class="welcome-icon">
          <svg width="48" height="48" viewBox="0 0 32 32" fill="none">
            <circle cx="16" cy="16" r="14" fill="var(--sage-200)" />
            <circle cx="12" cy="14" r="2.5" fill="var(--sage-500)" />
            <circle cx="20" cy="14" r="2.5" fill="var(--sage-500)" />
            <path d="M10 21c2 3 10 3 12 0" stroke="var(--sage-500)" stroke-width="1.5" stroke-linecap="round" fill="none" />
          </svg>
        </div>
        <h2 class="welcome-title">欢迎来到 Volya</h2>
        <p class="welcome-desc">你的 AI 创作助手已就绪，开始写作吧！</p>
      </div>
      <MessageBubble
        v-for="msg in messages"
        :key="msg.id"
        :message="msg"
      />
      <!-- 审批按钮 -->
      <div v-if="status === 'awaiting_approval' && pendingApproval" class="approval-bar">
        <span class="approval-text">允许执行「{{ pendingApproval.toolName }}」吗？</span>
        <div class="approval-actions">
          <button class="approval-btn reject" @click="handleReject">拒绝</button>
          <button class="approval-btn approve" @click="handleApprove">允许</button>
        </div>
      </div>
    </div>

    <!-- 输入区 -->
    <ChatInput
      :disabled="isProcessing"
      @send="handleSend"
    />

    <!-- 设置对话框 -->
    <SettingsDialog
      :visible="showSettings"
      :config="llmConfig"
      :llm-ready="llmReady"
      @close="showSettings = false"
      @save="handleSaveConfig"
    />
  </div>
</template>

<style scoped>
.chat-panel {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-chat);
}

/* ─── 顶部栏 ─── */
.chat-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-light);
  background: var(--bg-secondary);
  -webkit-app-region: drag;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.app-logo {
  width: 34px;
  height: 34px;
  border-radius: var(--radius-md);
  background: var(--sage-100);
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-info {
  display: flex;
  flex-direction: column;
}

.app-name {
  font-size: var(--font-size-base);
  font-weight: 700;
  color: var(--text-primary);
}

.app-status {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.app-status.processing {
  color: var(--accent-primary);
}

.app-status.awaiting_approval {
  color: var(--warning);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  -webkit-app-region: no-drag;
}

/* ─── 设置按钮 ─── */
.settings-btn {
  width: 34px;
  height: 34px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  color: var(--text-tertiary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.settings-btn:hover {
  background: var(--bg-hover);
  color: var(--text-secondary);
}

.settings-btn.warned {
  color: var(--warning);
  animation: gentlePulse 2s ease-in-out infinite;
}

@keyframes gentlePulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

/* ─── 权限选择器 ─── */
.permission-selector {
  position: relative;
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-1) var(--space-3);
  background: var(--bg-tertiary);
  border-radius: var(--radius-full);
  cursor: pointer;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  transition: background var(--transition-fast);
  user-select: none;
}

.permission-selector:hover {
  background: var(--bg-hover);
}

.permission-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.permission-label {
  font-weight: 500;
}

.permission-dropdown {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  padding: var(--space-1);
  z-index: 100;
  min-width: 160px;
}

.permission-option {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  width: 100%;
  padding: var(--space-2) var(--space-3);
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.permission-option:hover {
  background: var(--bg-hover);
}

.permission-option.active {
  background: var(--sage-100);
  font-weight: 600;
}

.option-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

/* ─── 消息区域 ─── */
.messages-area {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) 0;
  scroll-behavior: smooth;
}

.messages-area::-webkit-scrollbar {
  width: 6px;
}

.messages-area::-webkit-scrollbar-track {
  background: transparent;
}

.messages-area::-webkit-scrollbar-thumb {
  background: var(--sage-200);
  border-radius: 3px;
}

.messages-area::-webkit-scrollbar-thumb:hover {
  background: var(--sage-300);
}

/* ─── 欢迎占位 ─── */
.welcome-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-12) var(--space-4);
  text-align: center;
}

.welcome-icon {
  margin-bottom: var(--space-4);
  opacity: 0.6;
}

.welcome-title {
  font-size: var(--font-size-xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 var(--space-2);
}

.welcome-desc {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin: 0;
}

/* ─── 审批栏 ─── */
.approval-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: var(--space-3) var(--space-4);
  padding: var(--space-3) var(--space-4);
  background: #fff8e6;
  border: 1px solid var(--warning);
  border-radius: var(--radius-lg);
}

.approval-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
}

.approval-actions {
  display: flex;
  gap: var(--space-2);
}

.approval-btn {
  padding: var(--space-1) var(--space-4);
  border: 1.5px solid transparent;
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.approval-btn.approve {
  background: var(--success);
  color: white;
}

.approval-btn.approve:hover {
  background: #4a8e5f;
}

.approval-btn.reject {
  background: transparent;
  border-color: var(--error);
  color: var(--error);
}

.approval-btn.reject:hover {
  background: var(--error);
  color: white;
}
</style>
