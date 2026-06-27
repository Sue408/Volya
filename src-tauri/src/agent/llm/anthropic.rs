use crate::agent::llm::config::LlmConfig;
use crate::agent::tool::ToolDef;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ============================================================
/// Anthropic Messages API 客户端
/// 后端累积流式块，块完成时一次性发送完整内容给前端
/// 前端只需渲染，无需任何累积逻辑
/// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse { id: String, name: String, input: Value },
    #[serde(rename = "tool_result")]
    ToolResult { tool_use_id: String, content: String },
}

#[derive(Debug, Serialize)]
struct MessagesRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    system: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: Value,
}

// ─── SSE 流事件类型 ───

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum StreamEvent {
    #[serde(rename = "message_start")]
    MessageStart,
    #[serde(rename = "content_block_start")]
    ContentBlockStart { index: usize, content_block: ContentBlockStartData },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: usize, delta: ContentDelta },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta { delta: MessageDeltaData },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Deserialize)]
struct ContentBlockStartData {
    #[serde(rename = "type")]
    type_: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    input: Value,
}

#[derive(Debug, Deserialize)]
struct ContentDelta {
    #[serde(rename = "type")]
    #[serde(default)]
    type_: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    thinking: String,
}

#[derive(Debug, Deserialize)]
struct MessageDeltaData {
    #[serde(default)]
    stop_reason: Option<String>,
}

pub struct CompletionResult {
    pub content_blocks: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
}

/// 流式累积器 — 后端负责累积，前端只管渲染
struct Accumulator {
    current_text: String,       // 当前 text 块的累积
    current_thinking: String,   // 当前 thinking 块的累积
    current_block_type: String, // "text" | "thinking" | "tool_use"
}

impl Accumulator {
    fn new() -> Self {
        Self {
            current_text: String::new(),
            current_thinking: String::new(),
            current_block_type: String::new(),
        }
    }
}

pub struct AnthropicClient {
    api_key: String,
    api_base: String,
    model: String,
    max_tokens: u32,
    temperature: f64,
    http_client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(config: &LlmConfig) -> Self {
        Self {
            api_key: config.api_key.clone(),
            api_base: config.api_base.trim_end_matches('/').to_string(),
            model: config.model.clone(),
            max_tokens: config.max_tokens,
            temperature: config.temperature,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn stream_completion(
        &self,
        system: &str,
        messages: &[AnthropicMessage],
        tools: &[ToolDef],
        app: &tauri::AppHandle,
    ) -> Result<CompletionResult, AnthropicError> {
        let anthropic_tools: Vec<AnthropicTool> = tools
            .iter()
            .map(|t| AnthropicTool {
                name: t.name.clone(),
                description: t.description.clone(),
                input_schema: t.parameters.clone(),
            })
            .collect();

        let request_body = MessagesRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            temperature: Some(self.temperature),
            system: system.to_string(),
            messages: messages.to_vec(),
            tools: if anthropic_tools.is_empty() { None } else { Some(anthropic_tools) },
            stream: true,
        };

        let url = format!("{}/v1/messages", self.api_base);
        let mut req = self.http_client.post(&url)
            .header("x-api-key", &self.api_key)
            .header("content-type", "application/json");
        if self.api_base.contains("anthropic.com") {
            req = req.header("anthropic-version", "2023-06-01");
        }
        let response = req.json(&request_body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AnthropicError::ApiError(status.as_u16(), body));
        }

        let mut stream = response.bytes_stream();
        let mut buf = String::new();
        let mut accum = Accumulator::new();
        let mut content_blocks: Vec<ContentBlock> = Vec::new();
        let mut stop_reason: Option<String> = None;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buf.push_str(&String::from_utf8_lossy(&chunk));
            loop {
                if let Some(pos) = buf.find("\n\n") {
                    let block = buf[..pos].to_string();
                    buf = buf[pos + 2..].to_string();
                    Self::process_line(&block, &mut accum, app, &mut content_blocks, &mut stop_reason);
                } else {
                    break;
                }
            }
        }
        if !buf.trim().is_empty() {
            Self::process_line(&buf, &mut accum, app, &mut content_blocks, &mut stop_reason);
        }

        Ok(CompletionResult { content_blocks, stop_reason })
    }

    fn process_line(
        line: &str,
        accum: &mut Accumulator,
        app: &tauri::AppHandle,
        content_blocks: &mut Vec<ContentBlock>,
        stop_reason: &mut Option<String>,
    ) {
        for l in line.lines() {
            let l = l.trim();
            if let Some(data) = l.strip_prefix("data: ") {
                if let Ok(event) = serde_json::from_str::<StreamEvent>(data) {
                    Self::handle_event(event, accum, app, content_blocks, stop_reason);
                }
            }
        }
    }

    fn emit(app: &tauri::AppHandle, name: &str, payload: impl Serialize + Clone) {
        use tauri::Emitter;
        let _ = app.emit(name, payload);
    }

    fn handle_event(
        event: StreamEvent,
        accum: &mut Accumulator,
        app: &tauri::AppHandle,
        content_blocks: &mut Vec<ContentBlock>,
        stop_reason: &mut Option<String>,
    ) {
        match event {
            StreamEvent::ContentBlockStart { index, content_block } => {
                let block_id = format!("block_{}", index);
                accum.current_block_type = content_block.type_.clone();
                match content_block.type_.as_str() {
                    "text" => {
                        accum.current_text.push_str(&content_block.text);
                        Self::emit(app, "agent:block_start", serde_json::json!({
                            "id": &block_id,
                            "type": "text",
                        }));
                        // 首段 text 也作为 delta 推送（保证流式体验）
                        if !content_block.text.is_empty() {
                            Self::emit(app, "agent:delta", serde_json::json!({
                                "id": &block_id,
                                "text": &content_block.text,
                            }));
                        }
                    }
                    "thinking" => {
                        accum.current_thinking.push_str(&content_block.text);
                        Self::emit(app, "agent:block_start", serde_json::json!({
                            "id": &block_id,
                            "type": "thinking",
                        }));
                        if !content_block.text.is_empty() {
                            Self::emit(app, "agent:delta", serde_json::json!({
                                "id": &block_id,
                                "text": &content_block.text,
                            }));
                        }
                    }
                    "tool_use" => {
                        // tool_use 无流式，直接发送完整信息
                        Self::emit(app, "agent:tool_call", serde_json::json!({
                            "tool_name": content_block.name,
                            "args": content_block.input,
                        }));
                        content_blocks.push(ContentBlock::ToolUse {
                            id: content_block.id,
                            name: content_block.name,
                            input: content_block.input,
                        });
                    }
                    _ => {}
                }
            }
            StreamEvent::ContentBlockDelta { index, delta } => {
                let block_id = format!("block_{}", index);
                match accum.current_block_type.as_str() {
                    "text" => {
                        accum.current_text.push_str(&delta.text);
                        if !delta.text.is_empty() {
                            Self::emit(app, "agent:delta", serde_json::json!({
                                "id": &block_id,
                                "text": &delta.text,
                            }));
                        }
                    }
                    "thinking" => {
                        accum.current_thinking.push_str(&delta.thinking);
                        if !delta.thinking.is_empty() {
                            Self::emit(app, "agent:delta", serde_json::json!({
                                "id": &block_id,
                                "text": &delta.thinking,
                            }));
                        }
                    }
                    _ => {}
                }
            }
            StreamEvent::ContentBlockStop { index } => {
                let block_id = format!("block_{}", index);
                match accum.current_block_type.as_str() {
                    "text" if !accum.current_text.is_empty() => {
                        let text = std::mem::take(&mut accum.current_text);
                        content_blocks.push(ContentBlock::Text { text });
                    }
                    "thinking" if !accum.current_thinking.is_empty() => {
                        let _ = std::mem::take(&mut accum.current_thinking);
                    }
                    _ => {}
                }
                Self::emit(app, "agent:block_stop", serde_json::json!({
                    "id": &block_id,
                }));
                accum.current_block_type.clear();
            }
            StreamEvent::MessageDelta { delta: d } => {
                *stop_reason = d.stop_reason;
            }
            StreamEvent::MessageStop => {
                // 处理残留（LLM 被截断时）
                if !accum.current_thinking.is_empty() {
                    let _ = std::mem::take(&mut accum.current_thinking);
                }
                if !accum.current_text.is_empty() {
                    let text = std::mem::take(&mut accum.current_text);
                    content_blocks.push(ContentBlock::Text { text });
                }
            }
            StreamEvent::MessageStart | StreamEvent::Ping => {}
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnthropicError {
    #[error("HTTP 请求失败: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API 返回错误 [{0}]: {1}")]
    ApiError(u16, String),
}
