<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeft, Plug, Save } from '@lucide/vue'
import { useAgent } from '../composables/useAgent'

const router = useRouter()
const { llmConfig, loadLlmConfig, saveLlmConfig, checkConnection } = useAgent()

const providerUrl = ref('')
const apiKey = ref('')
const modelName = ref('')
const testing = ref(false)
const saving = ref(false)
const connectionMsg = ref('')

function goBack() {
  router.push('/')
}

/** 从后端加载配置并回填表单 */
async function loadConfig() {
  await loadLlmConfig()
  providerUrl.value = llmConfig.value.apiBase
  modelName.value = llmConfig.value.model
  // apiKey 不渲染到前端，仅通过 hasApiKey 告知是否已设置
}

onMounted(() => {
  loadConfig()
})

/** 测试连接：先保存配置，再发真实测试消息 */
async function testConnection() {
  testing.value = true
  connectionMsg.value = ''
  try {
    // 1. 先保存当前表单数据到后端
    await saveLlmConfig({
      apiKey: apiKey.value,
      apiBase: providerUrl.value,
      model: modelName.value,
      maxTokens: 4096,
      temperature: 0.7,
    })
    // 保存后清空 key 输入（不留明文）
    apiKey.value = ''

    // 2. 发送真实测试消息
    const status = await checkConnection()
    if (status.ready) {
      connectionMsg.value = '连接成功'
    } else {
      const msg = status.message || ''
      if (msg.includes('未配置')) {
        connectionMsg.value = '请先在设置中填写 API Key'
      } else if (msg.includes('401') || msg.includes('403') || msg.includes('unauthorized') || msg.includes('invalid') || msg.includes('Invalid')) {
        connectionMsg.value = 'API Key 无效，请检查后重试'
      } else if (msg.includes('timeout') || msg.includes('timed out')) {
        connectionMsg.value = '连接超时，请检查网络和服务商地址'
      } else if (msg.includes('refused') || msg.includes('resolve') || msg.includes('dns')) {
        connectionMsg.value = '无法连接到服务商，请检查地址是否正确'
      } else {
        connectionMsg.value = msg
      }
    }
  } catch (e) {
    const msg = String(e)
    if (msg.includes('timeout') || msg.includes('ETIMEDOUT')) {
      connectionMsg.value = '连接超时，请检查网络'
    } else {
      connectionMsg.value = '连接失败，请检查配置'
    }
  }
  // 3 秒后自动消失
  setTimeout(() => { connectionMsg.value = '' }, 3000)
  testing.value = false
}

const saveMsg = ref('')

/** 保存设置 */
async function saveSettings() {
  saving.value = true
  saveMsg.value = ''
  try {
    await saveLlmConfig({
      apiKey: apiKey.value,
      apiBase: providerUrl.value,
      model: modelName.value,
      maxTokens: 4096,
      temperature: 0.7,
    })
    // 保存成功后清除 key 输入框（不保留明文）
    apiKey.value = ''
    saveMsg.value = '配置已保存'
  } catch (e) {
    saveMsg.value = '保存失败，请重试'
  }
  saving.value = false
  setTimeout(() => { saveMsg.value = '' }, 3000)
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-header">
      <h1 class="settings-title">设置</h1>
    </div>
    <div class="settings-card card">
      <button class="back-btn" @click="goBack">
        <ArrowLeft :size="16" />
        <span>返回</span>
      </button>

      <!-- LLM 配置表单 -->
      <div class="form-body">
        <div class="form-group">
          <label class="form-label">服务商地址</label>
          <input
            v-model="providerUrl"
            class="form-input"
            type="text"
            placeholder="https://api.openai.com"
          />
        </div>

        <div class="form-group">
          <label class="form-label">API Key</label>
          <input
            v-model="apiKey"
            class="form-input"
            type="password"
            :placeholder="llmConfig.maskedApiKey || 'sk-...'"
          />
        </div>

        <div class="form-group">
          <label class="form-label">模型名称</label>
          <input
            v-model="modelName"
            class="form-input"
            type="text"
            placeholder="gpt-4o"
          />
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="form-actions">
        <button class="form-btn secondary" :disabled="testing" @click="testConnection">
          <Plug :size="16" />
          <span>{{ testing ? '测试中...' : '测试连接' }}</span>
        </button>
        <button class="form-btn primary" :disabled="saving" @click="saveSettings">
          <Save :size="16" />
          <span>{{ saving ? '保存中...' : '保存设置' }}</span>
        </button>
      </div>

      <div class="msg-row">
        <p v-if="saveMsg" class="feedback-msg save">{{ saveMsg }}</p>
        <p v-if="connectionMsg" class="feedback-msg">{{ connectionMsg }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100%;
  padding: var(--space-8);
  padding-top: 12vh;
}

.settings-header {
  width: 42%;
  min-width: 320px;
  max-width: 640px;
  margin-bottom: var(--space-3);
  padding: 0 var(--space-6);
}

.settings-title {
  font-family: var(--font-display);
  font-size: var(--font-size-2xl);
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.2;
}

.settings-card {
  width: 42%;
  min-width: 320px;
  max-width: 640px;
  padding: var(--space-4) var(--space-6);
  display: flex;
  flex-direction: column;
}

/* ─── 返回按钮 ─── */
.back-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-2);
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  cursor: pointer;
  align-self: flex-start;
  flex-shrink: 0;
  transition: color var(--transition-fast);
}

.back-btn:hover {
  color: var(--accent-primary);
}

/* ─── 表单 ─── */
.form-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
  padding: var(--space-4) 0;
  min-height: 0;
  overflow-y: auto;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.form-label {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-secondary);
}

.form-input {
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background: var(--bg-input);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  outline: none;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.form-input:focus {
  border-color: var(--accent-primary);
  box-shadow: var(--shadow-focus);
}

.form-input::placeholder {
  color: var(--text-tertiary);
}

/* ─── 操作按钮 ─── */
.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  flex-shrink: 0;
  padding-top: var(--space-8);
}

.form-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.form-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.form-btn.primary {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.form-btn.primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.form-btn.secondary {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.form-btn.secondary:hover:not(:disabled) {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.msg-row {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  margin-top: var(--space-2);
  min-height: 1.5em;
}

.feedback-msg {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.feedback-msg.save {
  color: var(--success);
}

</style>
