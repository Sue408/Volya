<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ChatPanel from '../components/ChatPanel.vue'
import WorkSidebar from '../components/WorkSidebar.vue'
import WordCountBar from '../components/WordCountBar.vue'
import { useAgent } from '../composables/useAgent'

const route = useRoute()
const router = useRouter()
const { openWork, resetMessages, loadLlmConfig, llmReady, cleanup } = useAgent()

const sidebarOpen = ref(true)
const loading = ref(true)
const error = ref<string | null>(null)

/** 初始化工作页面 */
async function initWork(id: string) {
  loading.value = true
  error.value = null
  try {
    // 清理旧会话事件监听
    cleanup()
    // 重置消息
    resetMessages()
    // 打开作品
    await openWork(id)
    // 加载 LLM 配置
    await loadLlmConfig()
    // 添加欢迎消息
    await addGreeting()
  } catch (e) {
    error.value = `打开作品失败: ${e}`
  } finally {
    loading.value = false
  }
}

/** 添加 Doro 的欢迎消息 */
async function addGreeting() {
  const { messages } = useAgent()
  // 防止重复添加
  if (messages.value.length > 0) return

  // 从 useAgent 内部获取 llmReady
  const greeting = llmReady.value
    ? '你好呀！我是 **Doro** 🍊，你的专属创作助手！\n\n我可以帮你：\n- 📖 **管理作品** — 查看和修改作品元数据\n- 👤 **创建角色** — 设计人物、地点、势力等\n- 📝 **生成正文** — 辅助创作章节内容\n- 🔗 **管理图谱** — 构建故事的关系网络\n\n当前权限模式：**半自动 (Lv 1)**\n现在开始我们的创作之旅吧！✨'
    : '你好呀！我是 **Doro** 🍊～\n\n欢迎来到 Volya！在开始之前，需要你配置一下 LLM：\n1. 点击右上角的 ⚙️ 齿轮图标\n2. 输入你的 Anthropic API Key\n3. 选择模型，然后保存配置\n\n配置完成后，我们就可以开始创作啦！🎉'

  messages.value.push({
    id: Date.now(),
    role: 'assistant',
    content: greeting,
    timestamp: new Date(),
  })
}

onMounted(() => {
  const id = route.params.id as string
  if (id) {
    initWork(id)
  }
})

// 路由参数变化时重新加载（从首页点不同作品）
watch(() => route.params.id, (newId) => {
  if (newId && typeof newId === 'string') {
    initWork(newId)
  }
})
</script>

<template>
  <div class="work-page">
    <!-- 加载状态 -->
    <div v-if="loading" class="work-loading">
      <span class="loading-dot"></span>
      <span>正在加载作品...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="work-error">
      <p class="error-text">{{ error }}</p>
      <button class="back-btn" @click="router.push('/')">← 返回首页</button>
    </div>

    <!-- 正常显示 -->
    <template v-else>
      <WorkSidebar v-model:open="sidebarOpen" />
      <div class="work-main">
        <ChatPanel />
        <WordCountBar />
      </div>
    </template>
  </div>
</template>

<style scoped>
.work-page {
  display: flex;
  height: 100%;
  overflow: hidden;
  background: var(--bg-primary);
}

.work-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  background: var(--bg-chat);
  margin: var(--space-2);
  margin-left: 0;
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-light);
}

/* ─── 加载 ─── */
.work-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  width: 100%;
  color: var(--text-tertiary);
  font-size: var(--font-size-sm);
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

/* ─── 错误 ─── */
.work-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-4);
  width: 100%;
  padding: var(--space-8);
}

.error-text {
  color: var(--error);
  font-size: var(--font-size-base);
}

.back-btn {
  padding: var(--space-2) var(--space-4);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.back-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
</style>
