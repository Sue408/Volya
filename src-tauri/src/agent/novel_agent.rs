use crate::agent::gate::{Gate, GateDecision};
use crate::agent::llm::anthropic::{AnthropicClient, AnthropicMessage, ContentBlock};
use crate::agent::llm::config::GlobalConfig;
use crate::agent::tool::{get_tool_defs, Tool, ToolResult};
use crate::workspace::data::{Interaction, ToolCallRecord, WorkData};
use serde::{Deserialize, Serialize};

/// ============================================================
/// NovelAgent — 核心创作代理
/// 使用真实 LLM API（Anthropic）进行思考与工具调用
/// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Thinking,
    CallingTool { tool_name: String },
    Responding,
}

/// Agent 输出的流式事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentEvent {
    Thinking { content: String },
    ToolCall { tool_name: String, args: serde_json::Value },
    ToolResult { tool_name: String, result: serde_json::Value },
    Content { content: String },
    GateRequest { tool_name: String, tool_use_id: String, reason: String },
    Done,
    Error { message: String },
}

/// 系统提示词模板
const SYSTEM_PROMPT_TEMPLATE: &str = r#"你是 Doro，一个活泼又细心的 AI 创作助手，栖息在「Volya」小说创作平台中。

你的核心职责是协助作者进行小说创作。你拥有以下能力：

1. **作品管理** — 查看和修改作品元数据（标题、字数、风格等）
2. **角色创作** — 设计角色、地点、势力、物品、核心设定
3. **故事图谱管理** — 创建和管理故事节点及其关系
4. **正文创作** — 辅助生成和修改章节内容

## 工作方式

你可以使用工具来操作作品数据。当用户提出需求时：
- 如果需要查询信息，直接使用对应的读取工具
- 如果需要创建/修改内容，先确认用户意图再使用工具
- 完成工具调用后，向用户清晰地汇报结果

## 可用工具

{tool_descriptions}

请开始协助用户创作吧！
"#;

/// 待审批的工具调用项
#[derive(Debug, Clone)]
pub struct PendingApprovalItem {
    pub tool_use_id: String,
    pub tool_name: String,
    pub tool_args: serde_json::Value,
    pub _reason: String,
}

/// LLM 循环模式：控制遇到待审批项时自动执行结果的处理方式
enum LlmLoopMode {
    /// 首次循环（process_message）：暂存结果，等审批后与审批结果合并写入
    StashOnPending,
    /// 后续循环（continue_after_approval）：直接写入历史
    WriteOnPending,
}

pub struct NovelAgent {
    pub state: AgentState,
    pub tools: Vec<Box<dyn Tool>>,
    pub gate: Gate,
    pub work_data: WorkData,
    pub conversation_history: Vec<AnthropicMessage>,
    /// 等待用户审批的工具调用
    pub pending_approvals: Vec<PendingApprovalItem>,
    /// 审批暂停时暂存的自动执行结果，待审批后合并写入
    pub pending_auto_results: Vec<ContentBlock>,
    max_tool_rounds: u32,
}

impl NovelAgent {
    pub fn new(work_data: WorkData, permission_level: u8) -> Self {
        let gate = Gate::new(crate::agent::gate::PermissionLevel::from_u8(permission_level));
        let tools = crate::agent::tool::get_real_tools();
        Self {
            state: AgentState::Idle,
            tools,
            gate,
            work_data,
            conversation_history: Vec::new(),
            pending_approvals: Vec::new(),
            pending_auto_results: Vec::new(),
            max_tool_rounds: 10,
        }
    }

    /// 检查 LLM 是否已配置
    pub fn is_llm_ready(&self) -> bool {
        let config = GlobalConfig::load();
        config.is_valid()
    }

    /// 生成系统提示词
    fn get_system_prompt(&self) -> String {
        let defs = get_tool_defs(&self.tools);
        let tool_desc = serde_json::to_string_pretty(&defs).unwrap_or_default();
        SYSTEM_PROMPT_TEMPLATE.replace("{tool_descriptions}", &tool_desc)
    }

    fn add_assistant_response(&mut self, content_blocks: Vec<ContentBlock>) {
        self.conversation_history.push(AnthropicMessage {
            role: "assistant".to_string(),
            content: content_blocks,
        });
    }


    fn extract_tool_uses(blocks: &[ContentBlock]) -> Vec<(String, String, serde_json::Value)> {
        blocks.iter().filter_map(|block| match block {
            ContentBlock::ToolUse { id, name, input } => {
                Some((id.clone(), name.clone(), input.clone()))
            }
            _ => None,
        }).collect()
    }

    /// 发射事件到前端
    fn emit(app: &tauri::AppHandle, event: AgentEvent) {
        use tauri::Emitter;
        let name = match &event {
            AgentEvent::Thinking { .. } => "agent:thinking",
            AgentEvent::ToolCall { .. } => "agent:tool_call",
            AgentEvent::ToolResult { .. } => "agent:tool_result",
            AgentEvent::Content { .. } => "agent:content",
            AgentEvent::GateRequest { .. } => "agent:gate_request",
            AgentEvent::Done => "agent:done",
            AgentEvent::Error { .. } => "agent:error",
        };
        let _ = app.emit(name, event);
    }

    fn emit_state(app: &tauri::AppHandle, state: &str, tool_name: Option<String>) {
        use tauri::Emitter;
        let _ = app.emit("agent:state", serde_json::json!({
            "state": state,
            "tool_name": tool_name,
        }));
    }

    /// 公共 LLM 轮询循环 — 被 process_message 和 continue_after_approval 共用
    async fn run_llm_loop(
        &mut self,
        client: &AnthropicClient,
        system_prompt: &str,
        tool_defs: &[crate::agent::tool::ToolDef],
        app: &tauri::AppHandle,
        manager: &mut crate::workspace::manager::WorkManager,
        mode: LlmLoopMode,
    ) {
        let mut messages = self.conversation_history.clone();
        let mut tool_round = 0u32;

        loop {
            if tool_round >= self.max_tool_rounds {
                Self::emit(app, AgentEvent::Error {
                    message: "⚠️ 工具调用次数过多，已自动停止。".to_string(),
                });
                break;
            }
            tool_round += 1;

            let result = match client
                .stream_completion(system_prompt, &messages, tool_defs, app)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    Self::emit(app, AgentEvent::Error {
                        message: format!("❌ LLM 调用失败: {}", e),
                    });
                    break;
                }
            };

            let tool_uses = Self::extract_tool_uses(&result.content_blocks);

            if tool_uses.is_empty() {
                self.add_assistant_response(result.content_blocks);
                break;
            }

            self.add_assistant_response(result.content_blocks);

            let mut result_blocks: Vec<ContentBlock> = Vec::new();
            let mut has_pending = false;
            for (tool_use_id, tool_name, tool_args) in &tool_uses {
                let sensitivity = Gate::classify_tool(tool_name);
                match self.gate.check(tool_name, sensitivity) {
                    GateDecision::Allowed => {
                        self.state = AgentState::CallingTool {
                            tool_name: tool_name.clone(),
                        };
                        let r = self.execute_tool(tool_name, tool_args.clone(), manager).await;
                        Self::emit(app, AgentEvent::ToolResult {
                            tool_name: tool_name.clone(),
                            result: r.data.clone(),
                        });
                        result_blocks.push(ContentBlock::ToolResult {
                            tool_use_id: tool_use_id.clone(),
                            content: serde_json::to_string(&r.data).unwrap_or_default(),
                        });
                    }
                    GateDecision::NeedsApproval { tool, reason } => {
                        has_pending = true;
                        Self::emit(app, AgentEvent::GateRequest {
                            tool_name: tool.clone(),
                            tool_use_id: tool_use_id.clone(),
                            reason: reason.clone(),
                        });
                        self.pending_approvals.push(PendingApprovalItem {
                            tool_use_id: tool_use_id.clone(),
                            tool_name: tool.clone(),
                            tool_args: tool_args.clone(),
                            _reason: reason,
                        });
                    }
                    GateDecision::Denied { reason } => {
                        Self::emit(app, AgentEvent::Error { message: reason });
                    }
                }
            }

            if has_pending {
                match mode {
                    LlmLoopMode::StashOnPending => {
                        self.pending_auto_results = result_blocks;
                    }
                    LlmLoopMode::WriteOnPending => {
                        if !result_blocks.is_empty() {
                            self.conversation_history.push(AnthropicMessage {
                                role: "user".to_string(),
                                content: result_blocks,
                            });
                        }
                    }
                }
                Self::emit_state(app, "awaiting_approval",
                    self.pending_approvals.first().map(|p| p.tool_name.clone())
                );
                self.state = AgentState::Idle;
                return;
            }

            if !result_blocks.is_empty() {
                self.conversation_history.push(AnthropicMessage {
                    role: "user".to_string(),
                    content: result_blocks,
                });
            }
            messages = self.conversation_history.clone();
        }

        self.state = AgentState::Idle;
        Self::emit_state(app, "idle", None);
        Self::emit(app, AgentEvent::Done);
        self.sync_interactions();
    }

    /// 处理用户消息 — 真实 LLM 流式处理
    pub async fn process_message(
        &mut self,
        user_message: &str,
        app: &tauri::AppHandle,
        manager: &mut crate::workspace::manager::WorkManager,
    ) {
        let config = GlobalConfig::load();
        if !config.is_valid() {
            Self::emit(app, AgentEvent::Error {
                message: "⚠️ LLM 尚未配置！请先在设置中配置 API Key 和模型参数。".to_string(),
            });
            Self::emit(app, AgentEvent::Done);
            return;
        }
        let client = AnthropicClient::new(&config.llm);

        self.state = AgentState::Thinking;
        Self::emit_state(app, "processing", None);

        // 将用户消息写入对话历史（确保 LLM 上下文完整）
        self.conversation_history.push(AnthropicMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: user_message.to_string(),
            }],
        });
        let system_prompt = self.get_system_prompt();
        let tool_defs = get_tool_defs(&self.tools);

        self.run_llm_loop(&client, &system_prompt, &tool_defs, app, manager, LlmLoopMode::StashOnPending).await;
    }

    /// 用户做出审批决策后，继续 LLM 循环
    pub async fn continue_after_approval(
        &mut self,
        decisions: Vec<(String, bool)>, // (tool_use_id, approved)
        app: &tauri::AppHandle,
        manager: &mut crate::workspace::manager::WorkManager,
    ) {
        let config = GlobalConfig::load();
        if !config.is_valid() {
            Self::emit(app, AgentEvent::Error {
                message: "LLM 配置已失效".to_string(),
            });
            Self::emit(app, AgentEvent::Done);
            return;
        }
        let client = AnthropicClient::new(&config.llm);

        Self::emit_state(app, "processing", None);

        // 根据用户决策生成 tool_result 块
        let mut result_blocks: Vec<ContentBlock> = Vec::new();
        for (tool_use_id, approved) in &decisions {
            if let Some(item) = self.pending_approvals.iter().find(|p| p.tool_use_id == *tool_use_id) {
                if *approved {
                    let result = self.execute_tool(&item.tool_name, item.tool_args.clone(), manager).await;
                    Self::emit(app, AgentEvent::ToolResult {
                        tool_name: item.tool_name.clone(),
                        result: result.data.clone(),
                    });
                    result_blocks.push(ContentBlock::ToolResult {
                        tool_use_id: tool_use_id.clone(),
                        content: serde_json::to_string(&result.data).unwrap_or_default(),
                    });
                } else {
                    result_blocks.push(ContentBlock::ToolResult {
                        tool_use_id: tool_use_id.clone(),
                        content: "用户取消了此操作".to_string(),
                    });
                }
            }
        }
        self.pending_approvals.clear();

        // 合并自动执行结果 + 审批结果 → 同一条 user 消息（Anthropic API 要求）
        let mut all_results = std::mem::take(&mut self.pending_auto_results);
        all_results.extend(result_blocks);
        if !all_results.is_empty() {
            self.conversation_history.push(AnthropicMessage {
                role: "user".to_string(),
                content: all_results,
            });
        }

        // 继续 LLM 循环
        let system_prompt = self.get_system_prompt();
        let tool_defs = get_tool_defs(&self.tools);

        self.run_llm_loop(&client, &system_prompt, &tool_defs, app, manager, LlmLoopMode::WriteOnPending).await;
    }

    /// 将 conversation_history 同步到 work_data.interactions（对话持久化）
    /// 两遍扫描：先收集所有 ToolResult，再配对 ToolUse 和 ToolResult
    fn sync_interactions(&mut self) {
        use std::collections::HashMap;

        // Pass 1: 收集所有 tool_use_id → result 映射
        let mut result_map: HashMap<String, serde_json::Value> = HashMap::new();
        for msg in &self.conversation_history {
            for block in &msg.content {
                if let ContentBlock::ToolResult { tool_use_id, content } = block {
                    if let Ok(val) = serde_json::from_str(content) {
                        result_map.insert(tool_use_id.clone(), val);
                    }
                }
            }
        }

        // Pass 2: 构建 Interaction，将 ToolUse 与对应的结果配对
        for msg in &self.conversation_history {
            let text: String = msg.content.iter()
                .filter_map(|b| match b {
                    ContentBlock::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");

            let tool_calls: Vec<ToolCallRecord> = msg.content.iter()
                .filter_map(|b| match b {
                    ContentBlock::ToolUse { id, name, input } => {
                        Some(ToolCallRecord {
                            tool_name: name.clone(),
                            args: input.clone(),
                            result: result_map.remove(id).unwrap_or(serde_json::Value::Null),
                        })
                    }
                    _ => None,
                })
                .collect();

            // 避免重复
            let exists = self.work_data.interactions.iter().any(|i| {
                i.role == msg.role && i.content == text
            });

            if !exists {
                self.work_data.interactions.push(Interaction {
                    role: msg.role.clone(),
                    content: text,
                    timestamp: chrono::Utc::now(),
                    tool_calls: if tool_calls.is_empty() { None } else { Some(tool_calls) },
                });
            }
        }
    }

    pub async fn execute_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
        manager: &mut crate::workspace::manager::WorkManager,
    ) -> ToolResult {
        for tool in &self.tools {
            if tool.name() == tool_name {
                return tool.execute(args, manager).await;
            }
        }
        ToolResult {
            tool_name: tool_name.to_string(),
            success: false,
            data: serde_json::json!(null),
            message: format!("未知工具: {}", tool_name),
        }
    }

    pub fn get_tool_descriptions(&self) -> String {
        let defs = get_tool_defs(&self.tools);
        serde_json::to_string_pretty(&defs).unwrap_or_default()
    }
}
