use crate::workspace::data::{Interaction, WorkData, WorkMeta};
use std::path::PathBuf;

/// ============================================================
/// WorkStorage — 作品数据的磁盘读写
/// 职责单一：文件 I/O，不管理状态
/// 每个作品一个文件夹，含三个文件:
///   settings.json      → WorkMeta
///   graph.json         → Graph
///   interactions.jsonl → Vec<Interaction>
/// ============================================================
pub struct WorkStorage {
    pub base_dir: PathBuf,
}

impl WorkStorage {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// 作品文件夹路径
    pub fn work_dir(&self, id: &str) -> PathBuf {
        self.base_dir.join(id)
    }

    // ─── 元数据 ───

    pub fn meta_path(&self, id: &str) -> PathBuf {
        self.work_dir(id).join("settings.json")
    }

    pub async fn save_meta(&self, id: &str, meta: &WorkMeta) -> Result<(), StorageError> {
        let dir = self.work_dir(id);
        tokio::fs::create_dir_all(&dir).await?;
        let json = serde_json::to_string_pretty(meta)?;
        tokio::fs::write(self.meta_path(id), json).await?;
        Ok(())
    }

    pub async fn load_meta(&self, id: &str) -> Result<WorkMeta, StorageError> {
        let content = tokio::fs::read_to_string(self.meta_path(id)).await?;
        Ok(serde_json::from_str(&content)?)
    }

    // ─── 图谱 ───

    pub fn graph_path(&self, id: &str) -> PathBuf {
        self.work_dir(id).join("graph.json")
    }

    pub async fn save_graph(&self, id: &str, graph: &crate::graph::graph::Graph) -> Result<(), StorageError> {
        let dir = self.work_dir(id);
        tokio::fs::create_dir_all(&dir).await?;
        let json = serde_json::to_string_pretty(graph)?;
        tokio::fs::write(self.graph_path(id), json).await?;
        Ok(())
    }

    pub async fn load_graph(&self, id: &str) -> Result<crate::graph::graph::Graph, StorageError> {
        let content = tokio::fs::read_to_string(self.graph_path(id)).await?;
        Ok(serde_json::from_str(&content)?)
    }

    // ─── 对话历史 ───

    pub fn interactions_path(&self, id: &str) -> PathBuf {
        self.work_dir(id).join("interactions.jsonl")
    }

    pub async fn save_interactions(&self, id: &str, interactions: &[Interaction]) -> Result<(), StorageError> {
        let dir = self.work_dir(id);
        tokio::fs::create_dir_all(&dir).await?;
        let mut lines = String::new();
        for i in interactions {
            lines.push_str(&serde_json::to_string(i)?);
            lines.push('\n');
        }
        tokio::fs::write(self.interactions_path(id), lines).await?;
        Ok(())
    }

    pub async fn load_interactions(&self, id: &str) -> Result<Vec<Interaction>, StorageError> {
        let path = self.interactions_path(id);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = tokio::fs::read_to_string(&path).await?;
        let mut result = Vec::new();
        for line in content.lines() {
            if !line.is_empty() {
                result.push(serde_json::from_str(line)?);
            }
        }
        Ok(result)
    }

    // ─── 完整作品 ───

    pub async fn save_work(&self, id: &str, data: &WorkData) -> Result<(), StorageError> {
        self.save_meta(id, &data.meta).await?;
        self.save_graph(id, &data.graph).await?;
        self.save_interactions(id, &data.interactions).await?;
        Ok(())
    }

    pub async fn load_work(&self, id: &str) -> Result<WorkData, StorageError> {
        let meta = self.load_meta(id).await?;
        let graph = self.load_graph(id).await?;
        let interactions = self.load_interactions(id).await?;
        Ok(WorkData { meta, graph, interactions })
    }

    // ─── 作品列表 ───

    /// 扫描 base_dir，返回所有作品 ID（文件夹名）
    pub fn list_work_ids(&self) -> Result<Vec<String>, StorageError> {
        let mut ids = Vec::new();
        if !self.base_dir.exists() {
            return Ok(ids);
        }
        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                // 检查是否为有效的作品文件夹（有 settings.json）
                let meta_path = entry.path().join("settings.json");
                if meta_path.exists() {
                    if let Some(name) = entry.file_name().to_str() {
                        ids.push(name.to_string());
                    }
                }
            }
        }
        Ok(ids)
    }

    /// 检查作品文件夹是否存在
    pub fn work_exists(&self, id: &str) -> bool {
        self.meta_path(id).exists()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("作品不存在: {0}")]
    NotFound(String),
}

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::data::WorkData;
    use tempfile::tempdir;

    /// 辅助函数：创建临时 WorkStorage，返回 (storage, id, _dir)
    /// _dir 必须保持存活，否则临时目录会被删除
    async fn setup() -> (WorkStorage, String, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let storage = WorkStorage::new(dir.path().to_path_buf());
        let id = "test-work-1234".to_string();
        (storage, id, dir)
    }

    #[tokio::test]
    async fn test_save_and_load_meta() {
        let (storage, id, _d) = setup().await;
        let meta = crate::workspace::data::WorkMeta::new("测试作品");
        storage.save_meta(&id, &meta).await.unwrap();
        let loaded = storage.load_meta(&id).await.unwrap();
        assert_eq!(loaded.title, "测试作品");
    }

    #[tokio::test]
    async fn test_save_and_load_graph() {
        let (storage, id, _d) = setup().await;
        let graph = crate::graph::graph::Graph::new();
        storage.save_graph(&id, &graph).await.unwrap();
        let loaded = storage.load_graph(&id).await.unwrap();
        assert!(loaded.all_nodes().is_empty());
    }

    #[tokio::test]
    async fn test_save_and_load_interactions() {
        let (storage, id, _d) = setup().await;
        let interactions = vec![
            Interaction {
                role: "user".to_string(),
                content: "你好".to_string(),
                timestamp: chrono::Utc::now(),
                tool_calls: None,
            },
            Interaction {
                role: "assistant".to_string(),
                content: "你好！".to_string(),
                timestamp: chrono::Utc::now(),
                tool_calls: None,
            },
        ];
        storage.save_interactions(&id, &interactions).await.unwrap();
        let loaded = storage.load_interactions(&id).await.unwrap();
        assert_eq!(loaded.len(), 2);
    }

    #[tokio::test]
    async fn test_save_and_load_full_work() {
        let (storage, id, _d) = setup().await;
        let mut data = WorkData::new("完整作品");
        data.meta.author = Some("作者".to_string());
        data.meta.genre = Some("奇幻".to_string());
        storage.save_work(&id, &data).await.unwrap();
        let loaded = storage.load_work(&id).await.unwrap();
        assert_eq!(loaded.meta.title, "完整作品");
        assert_eq!(loaded.meta.author.unwrap(), "作者");
    }

    #[tokio::test]
    async fn test_list_work_ids() {
        let (storage, _id, _d) = setup().await;
        storage.save_meta("work-a", &WorkMeta::new("作品A")).await.unwrap();
        storage.save_meta("work-b", &WorkMeta::new("作品B")).await.unwrap();
        let ids = storage.list_work_ids().unwrap();
        assert!(ids.contains(&"work-a".to_string()));
        assert!(ids.contains(&"work-b".to_string()));
    }

    #[tokio::test]
    async fn test_work_exists() {
        let (storage, id, _d) = setup().await;
        assert!(!storage.work_exists(&id));
        storage.save_meta(&id, &WorkMeta::new("测试")).await.unwrap();
        assert!(storage.work_exists(&id));
    }

    #[tokio::test]
    async fn test_load_nonexistent_meta() {
        let dir = tempdir().unwrap();
        let storage = WorkStorage::new(dir.path().to_path_buf());
        let result = storage.load_meta("nonexistent").await;
        assert!(result.is_err());
    }
}
