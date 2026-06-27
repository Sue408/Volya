import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

/**
 * Agent 会话状态
 */
export type SessionStatus = 'idle' | 'processing' | 'awaiting_approval'

export interface AgentMessage {
  id: number
  role: 'user' | 'assistant' | 'tool' | 'thinking' | 'gate_request'
  content: string
  toolName?: string
  toolUseId?: string
  timestamp: Date
}

/**
 * useAgent — 封装与 Rust 后端 Agent 的所有通信
 * 管理消息列表、会话状态、事件监听
 */
export function useAgent() {
  const messages = ref<AgentMessage[]>([])
  const status = ref<SessionStatus>('idle')
  const isProcessing = ref(false)
  const currentPermissionLevel = ref(1)
  const pendingApproval = ref<{ toolName: string; toolUseId: string; reason: string } | null>(null)

  let msgCounter = 0
  let unlisteners: UnlistenFn[] = []
  // block_id → messages 数组索引，用于流式 delta 追加
  const blockIndex = ref<Record<string, number>>({})

  /**
   * 初始化：创建默认作品 + 注册事件监听
   */
  async function init(title: string = '未命名作品') {
    try {
      await invoke('create_work', { title })
      // 获取当前状态
      const state = await invoke<{ permission_level: number }>('get_session_state')
      currentPermissionLevel.value = state.permission_level
      setupListeners()
    } catch (e) {
      console.error('初始化失败:', e)
    }
  }

  /**
   * 注册 Tauri 事件监听 — 纯渲染映射
   */
  function setupListeners() {
    // 块开始 → 创建新消息卡片
    listen<{ id: string; type: string; tool_name?: string; args?: any }>('agent:block_start', (event) => {
      const idx = messages.value.length
      if (event.payload.type === 'thinking') {
        messages.value.push({
          id: ++msgCounter, role: 'thinking', content: '', timestamp: new Date(),
        })
      } else {
        messages.value.push({
          id: ++msgCounter, role: 'assistant', content: '', timestamp: new Date(),
        })
      }
      blockIndex.value[event.payload.id] = idx
    }).then(fn => unlisteners.push(fn))

    // 流式 delta → 按 block_id 追加到对应卡片
    listen<{ id: string; text: string }>('agent:delta', (event) => {
      const idx = blockIndex.value[event.payload.id]
      if (idx !== undefined && messages.value[idx]) {
        messages.value[idx].content += event.payload.text
      }
    }).then(fn => unlisteners.push(fn))

    // 块结束 → 清理 blockIndex，防止内存泄漏
    listen<{ id: string }>('agent:block_stop', (event) => {
      delete blockIndex.value[event.payload.id]
    }).then(fn => unlisteners.push(fn))

    // 工具调用事件（无流式，一次性）
    listen<{ tool_name: string; args: any }>('agent:tool_call', (event) => {
      messages.value.push({
        id: ++msgCounter,
        role: 'tool',
        content: `**调用工具**：${event.payload.tool_name}\n参数：${JSON.stringify(event.payload.args, null, 2)}`,
        toolName: event.payload.tool_name,
        timestamp: new Date(),
      })
    }).then(fn => unlisteners.push(fn))

    // 工具结果事件
    listen<{ tool_name: string; result: any }>('agent:tool_result', (event) => {
      const lastMsg = messages.value[messages.value.length - 1]
      if (lastMsg && lastMsg.role === 'tool' && lastMsg.toolName === event.payload.tool_name) {
        lastMsg.content += `\n\n✅ **执行完成**：\n\`\`\`json\n${JSON.stringify(event.payload.result, null, 2)}\n\`\`\``
      } else {
        messages.value.push({
          id: ++msgCounter,
          role: 'tool',
          content: `工具「${event.payload.tool_name}」结果：\n\`\`\`json\n${JSON.stringify(event.payload.result, null, 2)}\n\`\`\``,
          toolName: event.payload.tool_name,
          timestamp: new Date(),
        })
      }
    }).then(fn => unlisteners.push(fn))

    // 审批请求事件
    listen<{ tool_name: string; tool_use_id: string; reason: string }>('agent:gate_request', (event) => {
      pendingApproval.value = {
        toolName: event.payload.tool_name,
        toolUseId: event.payload.tool_use_id,
        reason: event.payload.reason,
      }
      messages.value.push({
        id: ++msgCounter,
        role: 'gate_request',
        content: event.payload.reason,
        toolName: event.payload.tool_name,
        toolUseId: event.payload.tool_use_id,
        timestamp: new Date(),
      })
    }).then(fn => unlisteners.push(fn))

    // 完成事件
    listen('agent:done', () => {
      isProcessing.value = false
      status.value = 'idle'
      pendingApproval.value = null
    }).then(fn => unlisteners.push(fn))

    // 错误事件
    listen<{ message: string }>('agent:error', (event) => {
      messages.value.push({
        id: ++msgCounter,
        role: 'assistant',
        content: `❌ 错误：${event.payload.message}`,
        timestamp: new Date(),
      })
      isProcessing.value = false
      status.value = 'idle'
    }).then(fn => unlisteners.push(fn))

    // 状态变更事件
    listen<{ state: string; tool_name?: string }>('agent:state', (event) => {
      const s = event.payload.state
      status.value = s as SessionStatus
      isProcessing.value = s === 'processing' || s === 'awaiting_approval'
    }).then(fn => unlisteners.push(fn))
  }

  /**
   * 发送消息给 Agent
   */
  async function sendMessage(message: string) {
    if (!message.trim() || isProcessing.value) return

    // 添加用户消息
    messages.value.push({
      id: ++msgCounter,
      role: 'user',
      content: message,
      timestamp: new Date(),
    })

    try {
      await invoke('send_message', { message })
    } catch (e) {
      messages.value.push({
        id: ++msgCounter,
        role: 'assistant',
        content: `❌ 发送消息失败：${e}`,
        timestamp: new Date(),
      })
      isProcessing.value = false
      status.value = 'idle'
    }
  }

  /**
   * 回应审批请求
   */
  async function respondApproval(toolUseId: string, approved: boolean) {
    try {
      await invoke('handle_approval', { toolUseId, approved })
      if (!approved) {
        pendingApproval.value = null
      }
    } catch (e) {
      console.error('审批响应失败:', e)
    }
  }

  /**
   * 设置权限等级
   */
  async function setPermissionLevel(level: number) {
    try {
      const result = await invoke<string>('set_permission_level', { level })
      currentPermissionLevel.value = level
      messages.value.push({
        id: ++msgCounter,
        role: 'assistant',
        content: `⚙️ ${result}`,
        timestamp: new Date(),
      })
    } catch (e) {
      console.error('设置权限失败:', e)
    }
  }

  /**
   * 清理事件监听
   */
  function cleanup() {
    unlisteners.forEach(fn => fn())
    unlisteners = []
  }

  /**
   * LLM 配置
   */
  const llmConfig = ref({
    configured: false,
    provider: 'anthropic',
    apiBase: 'https://api.anthropic.com',
    model: 'claude-sonnet-4-20250514',
    maxTokens: 4096,
    temperature: 0.7,
    hasApiKey: false,
  })

  const llmReady = ref(false)

  async function loadLlmConfig() {
    try {
      const config = await invoke<any>('get_llm_config')
      llmConfig.value = {
        configured: config.configured,
        provider: config.provider,
        apiBase: config.api_base,
        model: config.model,
        maxTokens: config.max_tokens,
        temperature: config.temperature,
        hasApiKey: config.has_api_key,
      }
      llmReady.value = config.has_api_key
    } catch (e) {
      console.error('加载 LLM 配置失败:', e)
    }
  }

  async function saveLlmConfig(opts: {
    apiKey: string
    apiBase: string
    model: string
    maxTokens: number
    temperature: number
  }) {
    try {
      const result = await invoke<string>('save_llm_config', {
        apiKey: opts.apiKey,
        apiBase: opts.apiBase,
        model: opts.model,
        maxTokens: opts.maxTokens,
        temperature: opts.temperature,
      })
      // 回写所有字段，确保前端状态与磁盘一致
      llmConfig.value.apiBase = opts.apiBase
      llmConfig.value.model = opts.model
      llmConfig.value.maxTokens = opts.maxTokens
      llmConfig.value.temperature = opts.temperature
      if (opts.apiKey) {
        llmConfig.value.hasApiKey = true
        llmConfig.value.configured = true
      }
      llmReady.value = llmConfig.value.hasApiKey
      return result
    } catch (e) {
      throw new Error(`保存配置失败: ${e}`)
    }
  }

  async function checkConnection() {
    try {
      const status = await invoke<any>('check_llm_connection')
      llmReady.value = status.ready
      return status
    } catch (e) {
      return { ready: false, message: `检查失败: ${e}` }
    }
  }

  return {
    // 状态
    messages,
    status,
    isProcessing,
    currentPermissionLevel,
    pendingApproval,
    llmConfig,
    llmReady,
    // 方法
    init,
    sendMessage,
    respondApproval,
    setPermissionLevel,
    loadLlmConfig,
    saveLlmConfig,
    checkConnection,
    cleanup,
  }
}
