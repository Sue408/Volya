use crate::graph::graph::Graph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 作品唯一标识
pub type WorkId = Uuid;

/// ============================================================
/// WorkMeta — 作品的字段型数据
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeta {
    /// 作品名称
    pub title: String,
    /// 已完成字数
    pub completed_words: u64,
    /// 目标字数
    pub target_words: Option<u64>,
    /// 作品权限等级 (0 = 仅建议, 1 = 半自动, 2 = 全自动)
    pub permission_level: u8,
    /// 累计 token 花费（估算）
    pub total_tokens: u64,
    /// 文风要求
    pub style_guide: Option<String>,
    /// 作品类型（小说/短篇/连载等）
    pub work_type: Option<String>,
    /// 作品受众
    pub audience: Option<String>,
    /// 作品状态
    pub status: WorkStatus,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后修改时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorkStatus {
    #[default]
    Draft,
    InProgress,
    Completed,
}

/// ============================================================
/// WorkData — 作品在内存中的核心数据
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkData {
    pub meta: WorkMeta,
    pub graph: Graph,
    /// AI 对话历史 (interactions.jsonl 的内存表示)
    pub interactions: Vec<Interaction>,
}

impl WorkData {
    pub fn new(title: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            meta: WorkMeta {
                title: title.to_string(),
                completed_words: 0,
                target_words: None,
                permission_level: 1,
                total_tokens: 0,
                style_guide: None,
                work_type: None,
                audience: None,
                status: WorkStatus::Draft,
                created_at: now,
                updated_at: now,
            },
            graph: Graph::new(),
            interactions: Vec::new(),
        }
    }
}

/// ============================================================
/// Interaction — 单条 AI 对话记录
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub role: String, // "user" | "assistant"
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tool_calls: Option<Vec<ToolCallRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub tool_name: String,
    pub args: serde_json::Value,
    pub result: serde_json::Value,
}
