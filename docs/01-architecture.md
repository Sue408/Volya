# 架构概览

## 技术选型

| 层 | 技术 | 选型理由 |
|-----|------|---------|
| 桌面框架 | **Tauri v2** | 轻量、安全、Rust 原生性能 |
| 前端框架 | **Vue 3 + TypeScript** | 组合式 API、类型安全、轻量 |
| 构建工具 | **Vite 6** | 快速 HMR、Tauri 原生支持 |
| 后端语言 | **Rust (edition 2021)** | 高性能、内存安全、Tauri 原生 |
| LLM API | **Anthropic Messages API** | 原生 Tool Use 支持、流式 SSE |
| 配置文件 | **JSON / JSONL** | 人类可读、易于调试 |

## 架构分层

```
┌─────────────────────────────────────────────────┐
│                   前端 (Vue 3)                    │
│  ┌───────────────────────────────────────────┐  │
│  │  ChatPanel  │  MessageBubble  │  ChatInput │  │
│  │  SettingsDialog     │    useAgent (IPC)   │  │
│  └──────────────────────┬────────────────────┘  │
│                         │ invoke / event        │
├─────────────────────────┼───────────────────────┤
│              Tauri IPC 桥接层                    │
├─────────────────────────┼───────────────────────┤
│             后端 (Rust)                         │
│  ┌──────────────────────┴────────────────────┐  │
│  │  ipc/commands.rs    — Tauri Commands      │  │
│  │  ipc/events.rs      — 事件定义             │  │
│  ├───────────────────────────────────────────┤  │
│  │  agent/              — Agent 系统          │  │
│  │  ├── novel_agent.rs  — 核心创作代理        │  │
│  │  ├── session.rs      — 会话循环管理        │  │
│  │  ├── gate.rs         — 权限中间件          │  │
│  │  ├── tool.rs         — 工具系统            │  │
│  │  └── llm/            — LLM 集成           │  │
│  ├───────────────────────────────────────────┤  │
│  │  graph/              — 故事图谱引擎        │  │
│  │  ├── node.rs         — 节点类型定义        │  │
│  │  ├── edge.rs         — 关系边定义          │  │
│  │  └── graph.rs        — 图容器 + DAG 校验   │  │
│  ├───────────────────────────────────────────┤  │
│  │  workspace/          — 作品数据管理        │  │
│  │  ├── data.rs         — 数据结构定义        │  │
│  │  └── storage.rs      — 磁盘持久化          │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## 核心数据流

### 对话循环 (Session Loop)

```
用户输入
  → invoke("send_message", { message })
  → NovelAgent.process_message()
    → 1. 将用户消息写入 conversation_history
    → 2. 循环: LLM API 调用 → 流式返回
        ├─ thinking → agent:block_start + delta → 前端渲染
        ├─ text    → agent:block_start + delta → 前端渲染
        └─ tool_use → agent:tool_call
      → 3. 执行工具 (Tool)
        └─ Gate 权限校验 → Allowed / NeedsApproval / Denied
      → 4. 结果写回 conversation_history
      → 5. 循环继续 / 完成
  → emit("agent:done")
```

### 权限审批流程

```
LLM 返回 tool_use → Gate 检查
  ├─ Allowed → 自动执行
  ├─ NeedsApproval → 暂停循环，存 pending_approvals
  │   → 前端显示审批卡片
  │   → 用户批准 / 拒绝
  │   → handle_approval → continue_after_approval
  │   → 合并自动+审批结果 → 继续 LLM 循环
  └─ Denied → 跳过
```

## 模块依赖关系

```
ipc (最上层, 依赖所有模块)
  ├── agent (依赖 workspace, graph)
  │   ├── llm (独立)
  │   ├── gate (独立)
  │   └── tool (依赖 workspace, graph)
  ├── workspace (依赖 graph)
  └── graph (独立, 最底层)
```
