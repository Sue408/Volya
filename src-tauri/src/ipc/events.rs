use crate::agent::novel_agent::AgentEvent;
use serde::Serialize;
use tauri::Emitter;

/// ============================================================
/// IPC 事件定义
/// Rust 后端通过 Tauri event 系统向前端推送流式数据
/// ============================================================

/// 前端监听的事件名称
pub const EVENT_AGENT_STREAM: &str = "agent:stream";
pub const EVENT_AGENT_ERROR: &str = "agent:error";
pub const EVENT_AGENT_STATE: &str = "agent:state";

/// 发送流式事件到前端
pub fn emit_agent_event(app_handle: &tauri::AppHandle, event: AgentEvent) {
    let event_name = match &event {
        AgentEvent::Thinking { .. } => "agent:thinking",
        AgentEvent::ToolCall { .. } => "agent:tool_call",
        AgentEvent::ToolResult { .. } => "agent:tool_result",
        AgentEvent::Content { .. } => "agent:content",
        AgentEvent::GateRequest { .. } => "agent:gate_request",
        AgentEvent::Done => "agent:done",
        AgentEvent::Error { .. } => "agent:error",
    };

    let _ = app_handle.emit(event_name, event);
}

/// 会话状态（同步给前端）
#[derive(Debug, Clone, Serialize)]
pub struct SessionStatePayload {
    pub state: String, // "idle" | "processing" | "awaiting_approval"
    pub tool_name: Option<String>,
}

pub fn emit_session_state(app_handle: &tauri::AppHandle, state: SessionStatePayload) {
    let _ = app_handle.emit(EVENT_AGENT_STATE, state);
}
