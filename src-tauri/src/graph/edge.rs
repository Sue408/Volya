use crate::graph::node::NodeId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 边唯一标识
pub type EdgeId = Uuid;

/// ============================================================
/// Edge — 节点间的关系边
/// 核心思想：用「基础类型 + 自然语言描述」来定义关系
/// 基础类型用于结构化查询，自然语言描述让 LLM 理解语义
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub source_id: NodeId,
    pub target_id: NodeId,
    /// 基础关系类型
    pub edge_type: EdgeType,
    /// 自然语言描述（如 "Alice 深爱着 Bob，但 Bob 浑然不知"）
    pub description: String,
}

/// 基础关系类型 — 保持精简，通过 description 补充语义细节
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EdgeType {
    /// 包含关系（如 arc 包含 event, arc 包含 chapter）
    Contains,
    /// 顺序关系（如 event → event 的先后顺序）
    Sequence,
    /// 创建/产生关系
    Creates,
    /// 影响/作用关系
    Influences,
    /// 引用关系
    References,
    /// 通用关联（由 description 定义具体语义）
    RelatesTo,
}

impl Edge {
    pub fn new(source_id: NodeId, target_id: NodeId, edge_type: EdgeType, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            edge_type,
            description: description.to_string(),
        }
    }
}
