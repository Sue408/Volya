# LLM 集成

## 全局配置

配置文件存储在平台对应的配置目录：

| 平台 | 路径 |
|------|------|
| Windows | `%APPDATA%\volya\config.json` |
| macOS | `~/Library/Application Support/volya/config.json` |
| Linux | `~/.config/volya/config.json` |

```json
{
  "llm": {
    "provider": "anthropic",
    "api_key": "sk-ant-...",
    "api_base": "https://api.anthropic.com",
    "model": "claude-sonnet-4-20250514",
    "max_tokens": 4096,
    "temperature": 0.7
  }
}
```

- 支持自定义 `api_base`（兼容第三方 Anthropic 接口，如 DeepSeek）
- API Key 仅保存在本地，前端配置时不回传已存储的 Key

## Anthropic Messages API 集成

### 请求

```rust
POST {api_base}/v1/messages
Headers:
  x-api-key: {api_key}
  anthropic-version: 2023-06-01  (仅官方端点发送)
  content-type: application/json
Body:
  model, max_tokens, temperature, system, messages, tools, stream: true
```

### 流式 SSE 处理

使用 `Accumulator` 结构体在后端累积：

```rust
struct Accumulator {
    current_text: String,       // 当前 text 块累积
    current_thinking: String,   // 当前 thinking 块累积
    current_block_type: String, // "text" | "thinking" | "tool_use"
}
```

每个 `block_id` 用 `block_{index}` 格式标记（index 来自 Anthropic SSE 的 index 字段）：

```
Anthropic SSE                   后端处理                         前端渲染
─────────────────      ──────────────────────      ────────────────────
content_block_start    → block_start { id, type }  → 创建流式卡片
content_block_delta    → delta { id, text }        → 按 id 追加文本
                       + Accumulator 累积           (纯映射，无逻辑)
content_block_stop     → block_stop { id }         → 完成
                       + 完整文本写入 content_blocks (供历史使用)
```

## 流式事件体系

| 事件名 | 载荷 | 说明 |
|--------|------|------|
| `agent:block_start` | `{ id, type }` | 内容块开始，type: "text" / "thinking" |
| `agent:delta` | `{ id, text }` | 流式文本块 |
| `agent:block_stop` | `{ id }` | 内容块完成 |
| `agent:tool_call` | `{ tool_name, args }` | 工具调用请求 |
| `agent:tool_result` | `{ tool_name, result }` | 工具执行结果 |
| `agent:gate_request` | `{ tool_name, tool_use_id, reason }` | 权限审批请求 |
| `agent:state` | `{ state, tool_name? }` | 会话状态变更 |
| `agent:done` | `{}` | 本轮处理完成 |
| `agent:error` | `{ message }` | 错误信息 |
