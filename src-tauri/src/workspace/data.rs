use serde::{Deserialize, Serialize};

/// ============================================================
/// WorkMeta — 作品的字段型数据，存储在 settings.json
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkMeta {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub completed_words: u64,
    pub target_words: Option<u64>,
    pub status: WorkStatus,
    pub style_guide: Option<String>,
    pub genre: Option<String>,
    pub audience: Option<String>,
    pub permission_level: u8,
    pub total_tokens: u64,
    pub tags: Vec<String>,
    pub version: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl WorkMeta {
    pub fn new(title: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            title: title.to_string(),
            author: None,
            description: None,
            completed_words: 0,
            target_words: None,
            status: WorkStatus::Draft,
            style_guide: None,
            genre: None,
            audience: None,
            permission_level: 1,
            total_tokens: 0,
            tags: Vec::new(),
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    pub graph: crate::graph::graph::Graph,
    pub interactions: Vec<Interaction>,
}

impl WorkData {
    pub fn new(title: &str) -> Self {
        Self {
            meta: WorkMeta::new(title),
            graph: crate::graph::graph::Graph::new(),
            interactions: Vec::new(),
        }
    }
}

/// ============================================================
/// Interaction — 单条 AI 对话记录
/// JSONL 格式，每行一条
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interaction {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallRecord {
    pub tool_name: String,
    pub args: serde_json::Value,
    pub result: serde_json::Value,
}

/// ============================================================
/// WorkSummary — 作品摘要，用于列表展示
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSummary {
    pub id: String,
    pub title: String,
    pub status: WorkStatus,
    pub completed_words: u64,
    pub target_words: Option<u64>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub tags: Vec<String>,
    pub total_tokens: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_meta_new() {
        let meta = WorkMeta::new("测试作品");
        assert_eq!(meta.title, "测试作品");
        assert_eq!(meta.status, WorkStatus::Draft);
        assert_eq!(meta.version, 1);
        assert!(meta.tags.is_empty());
    }

    #[test]
    fn test_work_data_new() {
        let data = WorkData::new("测试");
        assert_eq!(data.meta.title, "测试");
        assert!(data.interactions.is_empty());
        assert!(data.graph.all_nodes().is_empty());
    }

    #[test]
    fn test_meta_serialize_roundtrip() {
        let meta = WorkMeta::new("序列化测试");
        let json = serde_json::to_string_pretty(&meta).unwrap();
        let deserialized: WorkMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(meta.title, deserialized.title);
        assert_eq!(meta.version, deserialized.version);
    }

    #[test]
    fn test_interaction_with_tool_calls() {
        let i = Interaction {
            role: "assistant".to_string(),
            content: String::new(),
            timestamp: chrono::Utc::now(),
            tool_calls: Some(vec![ToolCallRecord {
                tool_name: "create_node".to_string(),
                args: serde_json::json!({"name": "艾琳"}),
                result: serde_json::json!({"id": "abc"}),
            }]),
        };
        let json = serde_json::to_string_pretty(&i).unwrap();
        let d: Interaction = serde_json::from_str(&json).unwrap();
        assert_eq!(d.tool_calls.unwrap().len(), 1);
    }

    #[test]
    fn test_work_status_default() {
        assert_eq!(WorkStatus::default(), WorkStatus::Draft);
    }
}
