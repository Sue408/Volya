<script setup lang="ts">
import { ref } from 'vue'
import { ArrowUp } from '@lucide/vue'

const props = defineProps<{
  disabled: boolean
  placeholder?: string
}>()

const emit = defineEmits<{
  send: [message: string]
}>()

const input = ref('')
const isComposing = ref(false)

function handleSubmit() {
  const text = input.value.trim()
  if (!text || props.disabled || isComposing.value) return
  emit('send', text)
  input.value = ''
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    handleSubmit()
  }
}

function autoResize(event: Event) {
  const el = event.target as HTMLTextAreaElement
  el.style.height = 'auto'
  el.style.height = el.scrollHeight + 'px'
}
</script>

<template>
  <div class="chat-input-wrapper">
    <div class="chat-input-container">
      <textarea
        v-model="input"
        :placeholder="placeholder || '给 Doro 发送消息...'"
        :disabled="disabled"
        class="chat-input"
        rows="1"
        @keydown="handleKeydown"
        @compositionstart="isComposing = true"
        @compositionend="isComposing = false"
        @input="autoResize"
      />
      <button
        class="send-button"
        :class="{ active: input.trim() && !disabled }"
        :disabled="disabled || !input.trim()"
        @click="handleSubmit"
      >
        <ArrowUp :size="18" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.chat-input-wrapper {
  padding: var(--space-3) var(--space-4) var(--space-4);
  background: var(--bg-chat);
  border-top: 1px solid var(--border-light);
}

.chat-input-container {
  display: flex;
  align-items: flex-end;
  gap: var(--space-2);
  background: var(--bg-input);
  border: 1.5px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--space-2);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.chat-input-container:focus-within {
  border-color: var(--accent-primary);
  box-shadow: var(--shadow-focus);
}

.chat-input {
  flex: 1;
  border: none;
  outline: none;
  resize: none;
  padding: var(--space-2) var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  color: var(--text-primary);
  background: transparent;
  line-height: 1.5;
  max-height: 200px;
  min-height: 24px;
}

.chat-input::placeholder {
  color: var(--text-tertiary);
}

.chat-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.send-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: var(--radius-md);
  background: var(--sage-200);
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.send-button.active {
  background: var(--accent-primary);
  color: var(--text-on-color);
}

.send-button.active:hover {
  background: var(--accent-hover);
  transform: scale(1.05);
}

.send-button:disabled {
  cursor: not-allowed;
}
</style>
