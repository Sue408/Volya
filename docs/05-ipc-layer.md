# IPC 通信层

## Tauri Commands

前端通过 `invoke()` 调用的后端函数：

| Command | 参数 | 返回 | 说明 |
|---------|------|------|------|
| `create_work` | `title: String` | `String` | 创建作品并初始化会话 |
| `get_tool_descriptions` | — | `String` | 获取工具定义 |
| `send_message` | `message: String` | `()` | 发送消息给 Agent（核心入口） |
| `handle_approval` | `tool_use_id, approved` | `()` | 回应审批请求 |
| `set_permission_level` | `level: u8` | `String` | 修改权限等级 |
| `get_session_state` | — | `Value` | 获取当前会话状态 |
| `get_llm_config` | — | `Value` | 获取 LLM 配置 |
| `save_llm_config` | `api_key, api_base, model, max_tokens, temperature` | `String` | 保存 LLM 配置 |
| `check_llm_connection` | — | `Value` | 检查 LLM 连接状态 |

## 事件系统 (后端 → 前端)

Agent 在 `process_message` 和 `continue_after_approval` 中通过 `tauri::AppHandle::emit()` 直接推送事件。

### 事件与前端映射

```
agent:block_start ──────→ map block_id → 创建消息卡片
agent:delta       ──────→ 按 block_id 找到卡片 → 追加文本
agent:block_stop  ──────→ (可选)
agent:tool_call   ──────→ 创建 tool 消息卡片
agent:tool_result ──────→ 更新 tool 卡片（追加结果）
agent:gate_request──────→ 显示审批 UI
agent:state       ──────→ 更新状态指示器
agent:done        ──────→ 重置处理状态
agent:error       ──────→ 显示错误消息
```

### 前端事件处理（极简设计）

前端不包含任何业务累积逻辑，只做**事件 → DOM 映射**：

```typescript
const blockIndex: Record<string, number> = {}

listen('agent:block_start', (e) => {
  messages.push({ role: e.type, content: '' })
  blockIndex[e.id] = messages.length - 1
})

listen('agent:delta', (e) => {
  const idx = blockIndex[e.id]
  messages[idx].content += e.text  // 唯一逻辑：追加文本
})
```
