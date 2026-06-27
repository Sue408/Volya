use crate::workspace::data::WorkData;
use std::path::PathBuf;

/// WorkStorage 负责将 WorkData 读写到磁盘
/// 每个作品是一个文件夹，包含:
///   - settings.json   (作品配置 / meta)
///   - graph.json      (图数据)
///   - interactions.jsonl (对话历史)
pub struct WorkStorage {
    base_dir: PathBuf,
}

impl WorkStorage {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// 获取某个作品的文件夹路径
    pub fn work_dir(&self, work_id: &str) -> PathBuf {
        self.base_dir.join(work_id)
    }

    /// 保存作品到磁盘
    pub async fn save(&self, work_id: &str, data: &WorkData) -> Result<(), StorageError> {
        let dir = self.work_dir(work_id);
        tokio::fs::create_dir_all(&dir).await?;

        // 保存 settings.json (meta)
        let meta_path = dir.join("settings.json");
        let meta_json = serde_json::to_string_pretty(&data.meta)?;
        tokio::fs::write(&meta_path, meta_json).await?;

        // 保存 graph.json
        let graph_path = dir.join("graph.json");
        let graph_json = serde_json::to_string_pretty(&data.graph)?;
        tokio::fs::write(&graph_path, graph_json).await?;

        // 保存 interactions.jsonl
        let interactions_path = dir.join("interactions.jsonl");
        let mut lines = String::new();
        for interaction in &data.interactions {
            let line = serde_json::to_string(interaction)?;
            lines.push_str(&line);
            lines.push('\n');
        }
        tokio::fs::write(&interactions_path, lines).await?;

        Ok(())
    }

    /// 从磁盘加载作品
    pub async fn load(&self, work_id: &str) -> Result<WorkData, StorageError> {
        let dir = self.work_dir(work_id);
        if !dir.exists() {
            return Err(StorageError::NotFound(work_id.to_string()));
        }

        // 读取 settings.json
        let meta_path = dir.join("settings.json");
        let meta_json = tokio::fs::read_to_string(&meta_path).await?;
        let meta = serde_json::from_str(&meta_json)?;

        // 读取 graph.json
        let graph_path = dir.join("graph.json");
        let graph_json = tokio::fs::read_to_string(&graph_path).await?;
        let graph = serde_json::from_str(&graph_json)?;

        // 读取 interactions.jsonl
        let interactions_path = dir.join("interactions.jsonl");
        let mut interactions = Vec::new();
        if interactions_path.exists() {
            let content = tokio::fs::read_to_string(&interactions_path).await?;
            for line in content.lines() {
                if !line.is_empty() {
                    let interaction = serde_json::from_str(line)?;
                    interactions.push(interaction);
                }
            }
        }

        Ok(WorkData {
            meta,
            graph,
            interactions,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Workspace not found: {0}")]
    NotFound(String),
}
