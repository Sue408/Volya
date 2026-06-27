use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 节点唯一标识
pub type NodeId = Uuid;

/// ============================================================
/// Node — 图中的核心节点
/// 四大类: Entity(5子类), Arc, Event, Chapter
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    /// 显示名称（所有节点共有的基础字段）
    pub name: String,
    /// 节点大类
    pub category: NodeCategory,
    /// 节点具体数据（根据 category 不同而不同）
    pub data: NodeData,
}

/// 节点大类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeCategory {
    Entity(EntityKind),
    Arc,
    Event,
    Chapter,
}

/// Entity 的子类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityKind {
    Character,
    Location,
    Faction,
    Item,
    Concept,
}

/// ============================================================
/// NodeData — 各类型节点的具体字段
/// 使用 serde tag 保证 JSON 序列化时清晰可辨
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeData {
    Character {
        name: String,
        age: Option<u32>,
        gender: Option<String>,
        appearance: Option<String>,
        personality: Option<String>,
        background: Option<String>,
        motivation: Option<String>,
    },
    Location {
        name: String,
        description: Option<String>,
        climate: Option<String>,
        significance: Option<String>,
        history: Option<String>,
    },
    Faction {
        name: String,
        description: Option<String>,
        ideology: Option<String>,
        goals: Option<String>,
        members: Vec<String>,
    },
    Item {
        name: String,
        description: Option<String>,
        item_type: Option<String>,
        properties: Option<String>,
        owner: Option<String>,
    },
    Concept {
        name: String,
        description: Option<String>,
        category: Option<String>,
        rules: Option<String>,
    },
    Arc {
        title: String,
        summary: Option<String>,
        status: ArcStatus,
    },
    Event {
        title: String,
        summary: Option<String>,
        chapter_id: Option<NodeId>,
    },
    Chapter {
        title: String,
        content: String,
        word_count: u64,
        status: ChapterStatus,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ArcStatus {
    #[default]
    Planned,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ChapterStatus {
    #[default]
    Draft,
    InReview,
    Finalized,
}

impl Node {
    pub fn new(name: &str, category: NodeCategory, data: NodeData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            category,
            data,
        }
    }
}
