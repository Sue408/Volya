# 前端架构

## 组件树

```
App.vue
└── ChatPanel.vue          ← 主工作区
    ├── MessageBubble.vue   ← 消息气泡（6种角色）
    ├── ChatInput.vue        ← 输入框
    └── SettingsDialog.vue   ← 配置弹窗
```

### 组件职责

| 组件 | 职责 | 状态 |
|------|------|------|
| `ChatPanel` | 主面板布局、顶栏、消息列表、权限选择、审批栏 | `status`, `showSettings`, `showPermissionMenu` |
| `MessageBubble` | 按角色渲染消息卡片（纯展示） | 无内部状态 |
| `ChatInput` | 文本输入、Enter 发送、Shift+Enter 换行 | `input` |
| `SettingsDialog` | LLM 配置表单（API 地址/Key/模型/温度） | `apiKey`, `model`, 等 |

## 消息角色

| 角色 | 渲染样式 | 说明 |
|------|---------|------|
| `user` | 右对齐绿色气泡 | 用户消息，无头像 |
| `assistant` | 左对齐，带 Doro 头像 | Agent 回复 |
| `thinking` | 左对齐，左侧绿色竖线 | Agent 思考过程 |
| `tool` | 灰色卡片，带扳手图标 | 工具调用与结果 |
| `gate_request` | 黄色边框，带锁图标 | 审批请求 |

## 状态管理

使用 Vue 3 Composition API + `useAgent` composable：

```typescript
function useAgent() {
  // 响应式状态
  messages: Ref<AgentMessage[]>
  status: Ref<'idle' | 'processing' | 'awaiting_approval'>
  isProcessing: Ref<boolean>
  llmConfig: Ref<{ ... }>
  pendingApproval: Ref<{ toolName, toolUseId, reason } | null>
  
  // 方法
  init, sendMessage, respondApproval, setPermissionLevel,
  loadLlmConfig, saveLlmConfig, checkConnection
}
```

## 主题系统

使用 CSS 自定义变量实现，主色调为鼠尾草绿（Sage Green）：

```css
:root {
  --sage-50:  #f6f8f4;   /* 最浅背景 */
  --sage-100: #e8ede3;   /* 卡片/面板背景 */
  --sage-200: #d1dbca;   /* 边框/分割线 */
  --sage-300: #b3c2a8;   /* —— */
  --sage-400: #8a9f7b;   /* 次要文字 */
  --sage-500: #6c8360;   /* 主色 */
  --sage-600: #566b4c;   /* 强调色 */
  --sage-700: #43543b;
  --sage-800: #36442f;
  --sage-900: #283623;   /* 最深 */
  
  --apricot-400: #e58f53; /* 暖杏点缀色 🍊 */
}
```

支持 **prefers-color-scheme: dark** 自动切换深色模式。
