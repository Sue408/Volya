use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ============================================================
/// Tool trait — Agent 可使用的工具接口
/// 当前 MVP 阶段为 Mock 实现，后续可对接真实的 Graph CRUD
/// ============================================================

/// 工具调用的请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub args: Value,
}

/// 工具调用的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub data: Value,
    pub message: String,
}

/// 工具的定义描述（用于 Agent 的 system prompt）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value, // JSON Schema
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Value;
    async fn execute(&self, args: Value) -> ToolResult;
}

// ─── Mock Tools ───

/// Mock: 获取作品元数据
pub struct GetWorkMeta;

#[async_trait]
impl Tool for GetWorkMeta {
    fn name(&self) -> &str {
        "get_work_meta"
    }
    fn description(&self) -> &str {
        "获取当前作品的元数据信息，包括标题、字数、状态等"
    }
    fn parameters(&self) -> Value {
        serde_json::json!({ "type": "object", "properties": {} })
    }
    async fn execute(&self, _args: Value) -> ToolResult {
        ToolResult {
            tool_name: self.name().to_string(),
            success: true,
            data: serde_json::json!({
                "title": "未命名作品",
                "completed_words": 0,
                "status": "draft"
            }),
            message: "成功读取作品元数据".to_string(),
        }
    }
}

/// Mock: 更新作品元数据
pub struct UpdateWorkMeta;

#[async_trait]
impl Tool for UpdateWorkMeta {
    fn name(&self) -> &str {
        "update_work_meta"
    }
    fn description(&self) -> &str {
        "更新作品的元数据字段，如标题、文风要求等"
    }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string", "description": "作品标题" },
                "style_guide": { "type": "string", "description": "文风要求" }
            }
        })
    }
    async fn execute(&self, args: Value) -> ToolResult {
        ToolResult {
            tool_name: self.name().to_string(),
            success: true,
            data: args,
            message: "元数据已更新（Mock）".to_string(),
        }
    }
}

/// Mock: 创建节点
pub struct CreateNode;

#[async_trait]
impl Tool for CreateNode {
    fn name(&self) -> &str {
        "create_node"
    }
    fn description(&self) -> &str {
        "在图谱中创建一个新节点（角色/地点/事件/章节等）"
    }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string", "description": "节点名称" },
                "category": {
                    "type": "string",
                    "enum": ["character", "location", "faction", "item", "concept", "arc", "event", "chapter"]
                }
            },
            "required": ["name", "category"]
        })
    }
    async fn execute(&self, args: Value) -> ToolResult {
        ToolResult {
            tool_name: self.name().to_string(),
            success: true,
            data: serde_json::json!({
                "id": uuid::Uuid::new_v4(),
                "name": args.get("name"),
                "category": args.get("category"),
            }),
            message: format!("节点「{}」已创建（Mock）", args.get("name").and_then(|v| v.as_str()).unwrap_or("未知")),
        }
    }
}

/// Mock: 获取图谱数据
pub struct GetGraph;

#[async_trait]
impl Tool for GetGraph {
    fn name(&self) -> &str {
        "get_graph"
    }
    fn description(&self) -> &str {
        "获取当前作品的完整图谱数据"
    }
    fn parameters(&self) -> Value {
        serde_json::json!({ "type": "object", "properties": {} })
    }
    async fn execute(&self, _args: Value) -> ToolResult {
        ToolResult {
            tool_name: self.name().to_string(),
            success: true,
            data: serde_json::json!({
                "nodes": [],
                "edges": [],
                "node_count": 0,
                "edge_count": 0,
            }),
            message: "图谱数据为空（Mock）".to_string(),
        }
    }
}

/// 获取所有可用工具
pub fn get_mock_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(GetWorkMeta),
        Box::new(UpdateWorkMeta),
        Box::new(CreateNode),
        Box::new(GetGraph),
    ]
}

/// 获取工具定义列表（用于构建 Agent 的 system prompt）
pub fn get_tool_defs(tools: &[Box<dyn Tool>]) -> Vec<ToolDef> {
    tools
        .iter()
        .map(|t| ToolDef {
            name: t.name().to_string(),
            description: t.description().to_string(),
            parameters: t.parameters(),
        })
        .collect()
}
