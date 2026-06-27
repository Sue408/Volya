# Agent 系统

## 架构概览

```
用户
  │  交互
  ▼
NovelAgent ─── Tool 模块 ─── Graph CRUD / Meta CRUD
  │                              │
  │  协调 / 审阅                 │ 实际数据操作
  ▼                              ▼
Crew (规划中)              WorkData / 磁盘
  ├── WritingAgent (写作代理)
  ├── CharacterDesigner (角色设计)
  └── ...
```

## NovelAgent (核心创作代理)

用户与 AI 的唯一交互入口。工作流程：

```
process_message(user_input, app_handle)
  │
  ├─ 1. 写入用户消息到 conversation_history
  ├─ 2. 加载 LLM 配置 → 创建 AnthropicClient
  ├─ 3. 进入 LLM 循环:
  │    ├─ stream_completion → 流式接收 SSE 事件
  │    ├─ 事件处理:
  │    │   ├─ thinking → agent:block_start + delta → 前端
  │    │   ├─ text     → agent:block_start + delta → 前端
  │    │   └─ tool_use → agent:tool_call → 前端
  │    ├─ 无 tool_use → 写入 assistant 回复 → break
  │    └─ 有 tool_use → 执行工具 → 结果写回 history → 继续循环
  └─ 4. 发射 agent:done
```

### 系统提示词

Agent 使用模板化的系统提示词，包含：
- 角色定义（Doro 的身份与个性）
- 能力描述（作品管理、角色创作、图谱管理、正文创作）
- 工作方式说明
- 可用工具列表（动态生成）

## Gate (权限中间件)

三级权限体系，类似 Claude Code 的 Plan/Auto 模式：

| 等级 | 模式 | 行为 |
|------|------|------|
| **Lv 0** | 仅建议 | 只读操作自动执行，写入/删除需审批 |
| **Lv 1** | 半自动 (默认) | 只读+写入自动，危险操作需审批 |
| **Lv 2** | 全自动 | 全部自动执行 |

### 工具敏感度分类

```rust
pub enum ToolSensitivity {
    ReadOnly,   // 读取操作，始终安全
    Write,      // 写入操作，Lv0 需审批
    Dangerous,  // 删除操作，Lv0/Lv1 需审批
}
```

### 审批流程

```
LLM 返回 tool_use
  → Gate::check(tool_name, sensitivity)
    → Allowed → 执行 → 结果写入 history
    → NeedsApproval → 暂停循环 → 存储到 pending_approvals
      → 前端显示审批卡片
      → 用户点击 允许/拒绝
      → handle_approval(tool_use_id, approved)
        → continue_after_approval()
          → 合并自动+审批结果 → 继续 LLM 循环
    → Denied → 跳过
```

## Tool (工具系统)

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Value;  // JSON Schema
    async fn execute(&self, args: Value) -> ToolResult;
}
```

### 当前 Mock 工具 (MVP)

| 工具 | 说明 | 敏感度 |
|------|------|--------|
| `get_work_meta` | 获取作品元数据 | ReadOnly |
| `update_work_meta` | 更新作品元数据 | Write |
| `create_node` | 创建图谱节点 | Write |
| `get_graph` | 获取完整图谱 | ReadOnly |

## SessionLoop (会话管理)

管理一次会话的生命周期：

```rust
pub struct SessionLoop {
    pub agent: NovelAgent,
    pub state: SessionState,  // Idle / Processing / AwaitingApproval
}
```

## Crew (规划中)

子代理体系。NovelAgent 协调多个专业子代理：
- **WritingAgent**: 专注正文创作
- **CharacterDesigner**: 专注角色设计
- **PlotPlanner**: 专注剧情规划

NovelAgent 在此架构中承担**协调、审阅、把控**的核心角色。
