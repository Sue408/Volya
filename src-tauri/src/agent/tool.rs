use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::graph::edge::{Edge, EdgeType};
use crate::graph::node::{EntityKind, Node, NodeCategory, NodeData};
use crate::workspace::manager::WorkManager;

/// ============================================================
/// Tool trait — Agent 可使用的工具接口
/// execute 接受 &mut WorkManager 用于操作真实数据
/// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub data: Value,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Value;
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult;
}

// ─── 工具函数 ───

fn err(tool_name: &str, message: impl Into<String>) -> ToolResult {
    ToolResult {
        tool_name: tool_name.to_string(),
        success: false,
        data: Value::Null,
        message: message.into(),
    }
}

fn ok(tool_name: &str, data: Value, message: impl Into<String>) -> ToolResult {
    ToolResult {
        tool_name: tool_name.to_string(),
        success: true,
        data,
        message: message.into(),
    }
}

/// 解析 NodeCategory 和 EntityKind 字符串
fn parse_category(s: &str) -> Option<(NodeCategory, NodeData)> {
    let empty_data = |name: &str| match name {
        "character" => Some(NodeData::Character {
            name: String::new(), age: None, gender: None,
            appearance: None, personality: None, background: None, motivation: None,
        }),
        "location" => Some(NodeData::Location {
            name: String::new(), description: None, climate: None,
            significance: None, history: None,
        }),
        "faction" => Some(NodeData::Faction {
            name: String::new(), description: None, ideology: None,
            goals: None, members: vec![],
        }),
        "item" => Some(NodeData::Item {
            name: String::new(), description: None, item_type: None,
            properties: None, owner: None,
        }),
        "concept" => Some(NodeData::Concept {
            name: String::new(), description: None, category: None, rules: None,
        }),
        _ => None,
    };
    match s {
        "character" => Some((NodeCategory::Entity(EntityKind::Character), empty_data("character").unwrap())),
        "location"  => Some((NodeCategory::Entity(EntityKind::Location), empty_data("location").unwrap())),
        "faction"   => Some((NodeCategory::Entity(EntityKind::Faction), empty_data("faction").unwrap())),
        "item"      => Some((NodeCategory::Entity(EntityKind::Item), empty_data("item").unwrap())),
        "concept"   => Some((NodeCategory::Entity(EntityKind::Concept), empty_data("concept").unwrap())),
        "arc"       => Some((NodeCategory::Arc, NodeData::Arc {
            title: String::new(), summary: None, status: Default::default(),
        })),
        "event"     => Some((NodeCategory::Event, NodeData::Event {
            title: String::new(), summary: None, chapter_id: None,
        })),
        "chapter"   => Some((NodeCategory::Chapter, NodeData::Chapter {
            title: String::new(), content: String::new(), word_count: 0, status: Default::default(),
        })),
        _ => None,
    }
}

// ============================================================
// 真实工具实现 — 操作 WorkManager
// ============================================================

pub struct GetWorkMeta;
pub struct UpdateWorkMeta;
pub struct CreateNode;
pub struct UpdateNode;
pub struct RemoveNode;
pub struct GetGraph;
pub struct AddEdge;
pub struct RemoveEdge;
pub struct SearchNodes;

#[async_trait]
impl Tool for GetWorkMeta {
    fn name(&self) -> &str { "get_work_meta" }
    fn description(&self) -> &str { "获取当前作品的元数据信息，包括标题、字数、状态等" }
    fn parameters(&self) -> Value { serde_json::json!({ "type": "object", "properties": {} }) }
    async fn execute(&self, _args: Value, manager: &mut WorkManager) -> ToolResult {
        let ws = match manager.current() {
            Some(ws) => ws,
            None => return err("get_work_meta", "当前没有打开的作品"),
        };
        ok("get_work_meta", serde_json::to_value(&ws.data.meta).unwrap_or_default(), "成功读取作品元数据")
    }
}

#[async_trait]
impl Tool for UpdateWorkMeta {
    fn name(&self) -> &str { "update_work_meta" }
    fn description(&self) -> &str { "更新作品的元数据字段，如标题、文风要求等" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "title":       { "type": "string", "description": "作品标题" },
                "author":      { "type": "string", "description": "作者" },
                "description": { "type": "string", "description": "作品简介" },
                "style_guide": { "type": "string", "description": "文风要求" },
                "genre":       { "type": "string", "description": "题材（奇幻/科幻/言情等）" },
                "audience":    { "type": "string", "description": "目标受众" },
                "target_words": { "type": "integer", "description": "目标字数" },
            }
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let meta = match manager.meta_mut() {
            Some(m) => m,
            None => return err("update_work_meta", "当前没有打开的作品"),
        };
        if let Some(v) = args.get("title").and_then(|v| v.as_str()) { meta.title = v.to_string(); }
        if let Some(v) = args.get("author").and_then(|v| v.as_str()) { meta.author = Some(v.to_string()); }
        if let Some(v) = args.get("description").and_then(|v| v.as_str()) { meta.description = Some(v.to_string()); }
        if let Some(v) = args.get("style_guide").and_then(|v| v.as_str()) { meta.style_guide = Some(v.to_string()); }
        if let Some(v) = args.get("genre").and_then(|v| v.as_str()) { meta.genre = Some(v.to_string()); }
        if let Some(v) = args.get("audience").and_then(|v| v.as_str()) { meta.audience = Some(v.to_string()); }
        if let Some(v) = args.get("target_words").and_then(|v| v.as_u64()) { meta.target_words = Some(v); }
        meta.updated_at = chrono::Utc::now();
        manager.mark_dirty();
        ok("update_work_meta", args, "元数据已更新")
    }
}

#[async_trait]
impl Tool for CreateNode {
    fn name(&self) -> &str { "create_node" }
    fn description(&self) -> &str { "在图谱中创建一个新节点（角色/地点/事件/章节等）" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "name":     { "type": "string", "description": "节点显示名称" },
                "category": { "type": "string", "enum": ["character","location","faction","item","concept","arc","event","chapter"], "description": "节点类型" },
            }, "required": ["name", "category"]
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let name = match args.get("name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return err("create_node", "缺少 name 参数"),
        };
        let cat_str = match args.get("category").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return err("create_node", "缺少 category 参数"),
        };
        let (category, data) = match parse_category(cat_str) {
            Some(p) => p,
            None => return err("create_node", format!("未知的节点类型: {}", cat_str).as_str()),
        };
        let graph = match manager.graph_mut() {
            Some(g) => g,
            None => return err("create_node", "当前没有打开的作品"),
        };
        let node = Node::new(name, category, data);
        let id = node.id;
        graph.add_node(node);
        manager.mark_dirty();
        ok("create_node", serde_json::json!({ "id": id, "name": name, "category": cat_str }), format!("节点「{}」已创建", name))
    }
}

#[async_trait]
impl Tool for UpdateNode {
    fn name(&self) -> &str { "update_node" }
    fn description(&self) -> &str { "更新图谱中已有节点的字段" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "node_id": { "type": "string", "description": "节点 UUID" },
                "name":    { "type": "string", "description": "新的显示名称" },
            }, "required": ["node_id"]
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let id_str = match args.get("node_id").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return err("update_node", "缺少 node_id 参数"),
        };
        let node_id = match uuid::Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return err("update_node", "无效的 node_id 格式"),
        };
        let graph = match manager.graph_mut() {
            Some(g) => g,
            None => return err("update_node", "当前没有打开的作品"),
        };
        let node = match graph.get_node_mut(&node_id) {
            Some(n) => n,
            None => return err("update_node", format!("未找到节点: {}", id_str).as_str()),
        };
        if let Some(v) = args.get("name").and_then(|v| v.as_str()) {
            node.name = v.to_string();
        }
        manager.mark_dirty();
        ok("update_node", serde_json::json!({ "id": id_str }), "节点已更新")
    }
}

#[async_trait]
impl Tool for RemoveNode {
    fn name(&self) -> &str { "remove_node" }
    fn description(&self) -> &str { "删除图谱中的一个节点及其关联的边" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "node_id": { "type": "string", "description": "节点 UUID" },
            }, "required": ["node_id"]
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let id_str = match args.get("node_id").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return err("remove_node", "缺少 node_id 参数"),
        };
        let node_id = match uuid::Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return err("remove_node", "无效的 node_id 格式"),
        };
        let graph = match manager.graph_mut() {
            Some(g) => g,
            None => return err("remove_node", "当前没有打开的作品"),
        };
        if graph.remove_node(&node_id).is_some() {
            manager.mark_dirty();
            ok("remove_node", serde_json::json!({}), format!("节点 {} 已删除", id_str))
        } else {
            err("remove_node", format!("未找到节点: {}", id_str))
        }
    }
}

#[async_trait]
impl Tool for GetGraph {
    fn name(&self) -> &str { "get_graph" }
    fn description(&self) -> &str { "获取当前作品的完整图谱数据" }
    fn parameters(&self) -> Value { serde_json::json!({ "type": "object", "properties": {} }) }
    async fn execute(&self, _args: Value, manager: &mut WorkManager) -> ToolResult {
        let ws = match manager.current() {
            Some(ws) => ws,
            None => return err("get_graph", "当前没有打开的作品"),
        };
        let nodes: Vec<Value> = ws.data.graph.all_nodes().iter().map(|n| {
            serde_json::json!({ "id": n.id, "name": n.name, "category": &n.category })
        }).collect();
        let edges: Vec<Value> = ws.data.graph.all_edges().iter().map(|e| {
            serde_json::json!({ "id": e.id, "source": e.source_id, "target": e.target_id, "type": &e.edge_type, "description": e.description })
        }).collect();
        ok("get_graph", serde_json::json!({ "nodes": nodes, "edges": edges, "node_count": nodes.len(), "edge_count": edges.len() }), "图谱数据获取成功")
    }
}

#[async_trait]
impl Tool for AddEdge {
    fn name(&self) -> &str { "add_edge" }
    fn description(&self) -> &str { "在两个节点之间添加一条关系边" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "source_id":   { "type": "string", "description": "源节点 UUID" },
                "target_id":   { "type": "string", "description": "目标节点 UUID" },
                "edge_type":   { "type": "string", "enum": ["contains","sequence","creates","influences","references","relates_to"], "description": "关系类型" },
                "description": { "type": "string", "description": "自然语言描述此关系" },
            }, "required": ["source_id", "target_id", "description"]
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let parse_edge_type = |s: &str| -> Option<EdgeType> {
            match s {
                "contains"    => Some(EdgeType::Contains),
                "sequence"    => Some(EdgeType::Sequence),
                "creates"     => Some(EdgeType::Creates),
                "influences"  => Some(EdgeType::Influences),
                "references"  => Some(EdgeType::References),
                "relates_to"  => Some(EdgeType::RelatesTo),
                _ => None,
            }
        };
        let src = args.get("source_id").and_then(|v| v.as_str()).and_then(|s| uuid::Uuid::parse_str(s).ok());
        let dst = args.get("target_id").and_then(|v| v.as_str()).and_then(|s| uuid::Uuid::parse_str(s).ok());
        let type_str = args.get("edge_type").and_then(|v| v.as_str()).unwrap_or("relates_to");
        let desc = args.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let (src_id, dst_id) = match (src, dst) {
            (Some(s), Some(d)) => (s, d),
            _ => return err("add_edge", "source_id 或 target_id 格式无效"),
        };
        let edge_type = parse_edge_type(type_str).unwrap_or(EdgeType::RelatesTo);
        let graph = match manager.graph_mut() {
            Some(g) => g,
            None => return err("add_edge", "当前没有打开的作品"),
        };
        let edge = Edge::new(src_id, dst_id, edge_type, desc);
        let edge_id = edge.id;
        match graph.add_edge(edge) {
            Ok(_) => {
                manager.mark_dirty();
                ok("add_edge", serde_json::json!({ "edge_id": edge_id }), "关系边已添加")
            }
            Err(e) => err("add_edge", format!("添加边失败: {}", e).as_str()),
        }
    }
}

#[async_trait]
impl Tool for RemoveEdge {
    fn name(&self) -> &str { "remove_edge" }
    fn description(&self) -> &str { "删除一条关系边" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "edge_id": { "type": "string", "description": "边的 UUID" },
            }, "required": ["edge_id"]
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let id_str = match args.get("edge_id").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return err("remove_edge", "缺少 edge_id 参数"),
        };
        let edge_id = match uuid::Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return err("remove_edge", "无效的 edge_id 格式"),
        };
        let graph = match manager.graph_mut() {
            Some(g) => g,
            None => return err("remove_edge", "当前没有打开的作品"),
        };
        if graph.remove_edge(&edge_id).is_some() {
            manager.mark_dirty();
            ok("remove_edge", serde_json::json!({}), "边已删除")
        } else {
            err("remove_edge", format!("未找到边: {}", id_str))
        }
    }
}

#[async_trait]
impl Tool for SearchNodes {
    fn name(&self) -> &str { "search_nodes" }
    fn description(&self) -> &str { "按名称或类型搜索图谱中的节点" }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object", "properties": {
                "keyword":  { "type": "string", "description": "搜索关键词（匹配节点名称）" },
                "category": { "type": "string", "enum": ["character","location","faction","item","concept","arc","event","chapter"], "description": "按类型筛选" },
            }
        })
    }
    async fn execute(&self, args: Value, manager: &mut WorkManager) -> ToolResult {
        let ws = match manager.current() {
            Some(ws) => ws,
            None => return err("search_nodes", "当前没有打开的作品"),
        };
        let keyword = args.get("keyword").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
        let cat_filter = args.get("category").and_then(|v| v.as_str());
        let matched: Vec<Value> = ws.data.graph.all_nodes().iter().filter(|n| {
            let name_match = keyword.is_empty() || n.name.to_lowercase().contains(&keyword);
            let cat_match = match cat_filter {
                Some("character") => matches!(n.category, NodeCategory::Entity(EntityKind::Character)),
                Some("location")  => matches!(n.category, NodeCategory::Entity(EntityKind::Location)),
                Some("faction")   => matches!(n.category, NodeCategory::Entity(EntityKind::Faction)),
                Some("item")      => matches!(n.category, NodeCategory::Entity(EntityKind::Item)),
                Some("concept")   => matches!(n.category, NodeCategory::Entity(EntityKind::Concept)),
                Some("arc")       => matches!(n.category, NodeCategory::Arc),
                Some("event")     => matches!(n.category, NodeCategory::Event),
                Some("chapter")   => matches!(n.category, NodeCategory::Chapter),
                _ => true,
            };
            name_match && cat_match
        }).map(|n| serde_json::json!({ "id": n.id, "name": n.name, "category": &n.category })).collect();
        ok("search_nodes", serde_json::json!({ "results": matched, "count": matched.len() }), format!("找到 {} 个节点", matched.len()))
    }
}

// ─── 工厂函数 ───

/// 获取所有真实工具
pub fn get_real_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(GetWorkMeta),
        Box::new(UpdateWorkMeta),
        Box::new(CreateNode),
        Box::new(UpdateNode),
        Box::new(RemoveNode),
        Box::new(GetGraph),
        Box::new(AddEdge),
        Box::new(RemoveEdge),
        Box::new(SearchNodes),
    ]
}

/// 获取工具定义列表（用于构建 Agent 的 system prompt）
pub fn get_tool_defs(tools: &[Box<dyn Tool>]) -> Vec<ToolDef> {
    tools.iter().map(|t| ToolDef {
        name: t.name().to_string(),
        description: t.description().to_string(),
        parameters: t.parameters(),
    }).collect()
}
// ─── 测试 ───
#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::manager::WorkManager;
    use tempfile::tempdir;

    async fn setup() -> (WorkManager, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let mut m = WorkManager::new(dir.path().to_path_buf());
        m.create("测试作品").await.unwrap();
        (m, dir)
    }

    #[tokio::test]
    async fn test_get_work_meta() {
        let (mut m, _d) = setup().await;
        let r = GetWorkMeta.execute(Value::Null, &mut m).await;
        assert!(r.success);
        assert_eq!(r.data.get("title").and_then(|v| v.as_str()), Some("测试作品"));
    }

    #[tokio::test]
    async fn test_create_node() {
        let (mut m, _d) = setup().await;
        let r = CreateNode.execute(serde_json::json!({"name":"艾琳","category":"character"}), &mut m).await;
        assert!(r.success);
        assert_eq!(m.current().unwrap().data.graph.all_nodes().len(), 1);
    }

    #[tokio::test]
    async fn test_search_nodes() {
        let (mut m, _d) = setup().await;
        CreateNode.execute(serde_json::json!({"name":"艾琳","category":"character"}), &mut m).await;
        CreateNode.execute(serde_json::json!({"name":"王城","category":"location"}), &mut m).await;
        let r = SearchNodes.execute(serde_json::json!({"keyword":"艾琳"}), &mut m).await;
        assert_eq!(r.data.get("count").and_then(|v| v.as_u64()).unwrap(), 1);
    }
}
