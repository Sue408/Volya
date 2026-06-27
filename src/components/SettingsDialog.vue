<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  visible: boolean
  config: {
    provider: string
    apiBase: string
    model: string
    maxTokens: number
    temperature: number
    hasApiKey: boolean
  }
  llmReady: boolean
}>()

const emit = defineEmits<{
  close: []
  save: [config: { apiKey: string; apiBase: string; model: string; maxTokens: number; temperature: number }]
}>()

const apiKey = ref('')
const apiBase = ref('')
const model = ref('')
const maxTokens = ref(4096)
const temperature = ref(0.7)
const showKey = ref(false)
const saving = ref(false)
const saved = ref(false)

watch(() => props.visible, (v) => {
  if (v) {
    apiKey.value = ''
    apiBase.value = props.config.apiBase
    model.value = props.config.model
    maxTokens.value = props.config.maxTokens
    temperature.value = props.config.temperature
    saved.value = false
  }
})

async function handleSave() {
  saving.value = true
  try {
    emit('save', {
      apiKey: apiKey.value || '',
      apiBase: apiBase.value,
      model: model.value,
      maxTokens: maxTokens.value,
      temperature: temperature.value,
    })
    saved.value = true
    setTimeout(() => emit('close'), 800)
  } finally {
    saving.value = false
  }
}

function handleOverlayClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('overlay')) {
    emit('close')
  }
}
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="overlay" @click="handleOverlayClick">
      <div class="dialog" role="dialog" aria-label="设置">
        <div class="dialog-header">
          <h2 class="dialog-title">⚙️ 设置</h2>
          <button class="close-btn" @click="$emit('close')">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        <div class="dialog-body">
          <!-- 连接状态 -->
          <div class="status-bar" :class="{ connected: llmReady }">
            <span class="status-dot"></span>
            <span>{{ llmReady ? 'LLM 已连接' : 'LLM 未配置' }}</span>
          </div>

          <!-- Provider -->
          <div class="field">
            <label class="field-label">供应商</label>
            <div class="field-note">当前仅支持 Anthropic</div>
            <div class="input-wrapper">
              <input type="text" :value="config.provider" disabled class="input disabled" />
            </div>
          </div>

          <!-- API Base URL -->
          <div class="field">
            <label class="field-label">API 地址</label>
            <div class="field-note">自定义 API 端点（代理/镜像），留空使用默认</div>
            <div class="input-wrapper">
              <input type="text" v-model="apiBase" placeholder="https://api.anthropic.com" class="input" />
            </div>
          </div>

          <!-- API Key -->
          <div class="field">
            <label class="field-label">API Key</label>
            <div class="field-note">输入你的 Anthropic API 密钥</div>
            <div class="input-wrapper">
              <input
                :type="showKey ? 'text' : 'password'"
                v-model="apiKey"
                :placeholder="config.hasApiKey ? '••••••••（已配置，留空则保持不变）' : 'sk-ant-...'"
                class="input"
              />
              <button class="toggle-vis" @click="showKey = !showKey" tabindex="-1">
                <svg v-if="showKey" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94" />
                  <path d="M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19" />
                  <line x1="1" y1="1" x2="23" y2="23" />
                </svg>
                <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                  <circle cx="12" cy="12" r="3" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Model -->
          <div class="field">
            <label class="field-label">模型</label>
            <div class="field-note">Anthropic 模型名称</div>
            <div class="input-wrapper">
              <input type="text" v-model="model" placeholder="claude-sonnet-4-20250514" class="input" />
            </div>
          </div>

          <!-- Max Tokens -->
          <div class="field">
            <label class="field-label">最大输出 Token</label>
            <div class="field-note">{{ maxTokens.toLocaleString() }}</div>
            <input type="range" v-model.number="maxTokens" min="1024" max="16384" step="1024" class="slider" />
            <div class="slider-labels">
              <span>1K</span>
              <span>16K</span>
            </div>
          </div>

          <!-- Temperature -->
          <div class="field">
            <label class="field-label">温度</label>
            <div class="field-note">{{ temperature.toFixed(1) }} — {{ temperature < 0.4 ? '精确' : temperature < 0.7 ? '平衡' : '创意' }}</div>
            <input type="range" v-model.number="temperature" min="0" max="1" step="0.1" class="slider" />
            <div class="slider-labels">
              <span>0 精确</span>
              <span>1 创意</span>
            </div>
          </div>
        </div>

        <div class="dialog-footer">
          <button class="btn secondary" @click="$emit('close')">取消</button>
          <button class="btn primary" :disabled="saving" @click="handleSave">
            {{ saved ? '✅ 已保存' : saving ? '保存中...' : '保存配置' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.overlay {
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

.dialog {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  width: 440px;
  max-width: 90vw;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.25s ease;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(20px) scale(0.97); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-5) var(--space-6) var(--space-3);
}

.dialog-title {
  font-size: var(--font-size-lg);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
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

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.dialog-body {
  padding: var(--space-3) var(--space-6);
  overflow-y: auto;
  flex: 1;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-4) var(--space-6);
  border-top: 1px solid var(--border-light);
}

/* ─── 状态栏 ─── */
.status-bar {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  background: #fef3e2;
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  color: var(--warning);
  margin-bottom: var(--space-5);
}

.status-bar.connected {
  background: #eaf7ed;
  color: var(--success);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

/* ─── 字段 ─── */
.field {
  margin-bottom: var(--space-5);
}

.field-label {
  display: block;
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.field-note {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  margin-bottom: var(--space-2);
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.input {
  width: 100%;
  padding: var(--space-2) var(--space-3);
  border: 1.5px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-family: var(--font-mono);
  color: var(--text-primary);
  background: var(--bg-input);
  outline: none;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.input:focus {
  border-color: var(--accent-primary);
  box-shadow: var(--shadow-focus);
}

.input.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle-vis {
  position: absolute;
  right: 6px;
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-vis:hover {
  color: var(--text-secondary);
  background: var(--bg-hover);
}

/* ─── 滑块 ─── */
.slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--sage-200);
  outline: none;
  margin: var(--space-2) 0 var(--space-1);
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  transition: transform var(--transition-fast);
  border: 2px solid white;
  box-shadow: var(--shadow-sm);
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.15);
}

.slider-labels {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

/* ─── 按钮 ─── */
.btn {
  padding: var(--space-2) var(--space-5);
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn.primary {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.btn.primary:hover {
  background: var(--accent-hover);
}

.btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.secondary {
  background: transparent;
  border: 1.5px solid var(--border-color);
  color: var(--text-secondary);
}

.btn.secondary:hover {
  background: var(--bg-hover);
  border-color: var(--sage-300);
}
</style>
