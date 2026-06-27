<script setup lang="ts">
import type { AgentMessage } from '../composables/useAgent'

defineProps<{
  message: AgentMessage
}>()

function formatTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
  })
}
</script>

<template>
  <div class="message-row" :class="[`role-${message.role}`]">
    <!-- 用户消息 -->
    <template v-if="message.role === 'user'">
      <div class="message user-message">
        <div class="message-content">{{ message.content }}</div>
        <div class="message-time">{{ formatTime(message.timestamp) }}</div>
      </div>
    </template>

    <!-- Agent 思考中 -->
    <template v-else-if="message.role === 'thinking'">
      <div class="message thinking-message">
        <div class="thinking-header">
          <span class="thinking-dot"></span>
          <span class="thinking-label">Doro 正在思考...</span>
        </div>
        <div class="thinking-content">{{ message.content }}</div>
      </div>
    </template>

    <!-- 工具调用 -->
    <template v-else-if="message.role === 'tool'">
      <div class="message tool-message">
        <div class="tool-header">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
          </svg>
          <span class="tool-name">{{ message.toolName || '工具调用' }}</span>
        </div>
        <div class="tool-content"><!-- eslint-disable-next-line vue/no-v-html -->
          <div v-if="message.content" v-html="message.content.replace(/```json\n?/g, '<code>').replace(/\n```/g, '</code>').replace(/\n/g, '<br>')"></div>
        </div>
      </div>
    </template>

    <!-- 审批请求 -->
    <template v-else-if="message.role === 'gate_request'">
      <div class="message gate-message">
        <div class="gate-header">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
            <path d="M7 11V7a5 5 0 0 1 10 0v4" />
          </svg>
          <span>需要确认</span>
          <span class="gate-tool">{{ message.toolName }}</span>
        </div>
        <div class="gate-content">{{ message.content }}</div>
      </div>
    </template>

    <!-- 助手回复 -->
    <template v-else>
      <div class="message assistant-message">
        <div class="message-avatar">
          <svg width="18" height="18" viewBox="0 0 32 32" fill="none">
            <circle cx="16" cy="16" r="14" fill="var(--sage-300)" />
            <circle cx="12" cy="14" r="2.5" fill="white" />
            <circle cx="20" cy="14" r="2.5" fill="white" />
            <path d="M10 21c2 3 10 3 12 0" stroke="white" stroke-width="1.5" stroke-linecap="round" fill="none" />
          </svg>
        </div>
        <div class="message-body">
          <div class="message-sender">Doro</div>
          <div class="message-content" style="white-space: pre-wrap;">{{ message.content }}</div>
          <div class="message-time">{{ formatTime(message.timestamp) }}</div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.message-row {
  padding: var(--space-2) var(--space-4);
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}

/* ─── 用户消息（右对齐气泡） ─── */
.message-row.role-user {
  display: flex;
  justify-content: flex-end;
}

.user-message {
  max-width: 75%;
  background: var(--sage-600);
  color: white;
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-lg) var(--radius-lg) var(--radius-sm) var(--radius-lg);
}

.user-message .message-time {
  color: rgba(255, 255, 255, 0.6);
  font-size: var(--font-size-xs);
  margin-top: var(--space-1);
  text-align: right;
}

/* ─── 思维过程 ─── */
.thinking-message {
  background: var(--sage-100);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  border-left: 3px solid var(--sage-400);
}

.thinking-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}

.thinking-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--sage-400);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.thinking-label {
  font-size: var(--font-size-sm);
  color: var(--sage-600);
  font-weight: 600;
}

.thinking-content {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.6;
  white-space: pre-wrap;
}

/* ─── 工具调用 ─── */
.tool-message {
  background: var(--neutral-100);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  border: 1px solid var(--border-light);
}

.tool-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--neutral-500);
  font-size: var(--font-size-sm);
  font-weight: 500;
  margin-bottom: var(--space-2);
}

.tool-content {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  font-family: var(--font-mono);
}

/* ─── 审批请求 ─── */
.gate-message {
  background: #fff8e6;
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  border: 1px solid var(--warning);
}

.gate-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--warning);
  font-size: var(--font-size-sm);
  font-weight: 600;
  margin-bottom: var(--space-2);
}

.gate-tool {
  background: var(--warning);
  color: white;
  padding: 1px 8px;
  border-radius: var(--radius-full);
  font-size: var(--font-size-xs);
}

.gate-content {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

/* ─── 助手回复 ─── */
.assistant-message {
  display: flex;
  gap: var(--space-3);
  align-items: flex-start;
}

.message-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--sage-200);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-top: 2px;
}

.message-body {
  flex: 1;
  min-width: 0;
}

.message-sender {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.assistant-message .message-content {
  font-size: var(--font-size-base);
  line-height: 1.7;
  color: var(--text-primary);
}

.assistant-message .message-time {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  margin-top: var(--space-2);
}
</style>
