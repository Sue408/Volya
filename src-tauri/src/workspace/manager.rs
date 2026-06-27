use crate::graph::graph::Graph;
use crate::workspace::data::{WorkData, WorkMeta, WorkSummary};
use crate::workspace::storage::WorkStorage;
use uuid::Uuid;

/// ============================================================
/// WorkSpace — 作品在内存中的运行时上下文
/// 一个 WorkSpace = WorkData + 运行时标记
/// ============================================================
pub struct WorkSpace {
    /// 作品唯一标识（UUID）
    pub id: String,
    /// 作品数据
    pub data: WorkData,
    /// 是否有未保存到磁盘的修改
    pub dirty: bool,
}

impl WorkSpace {
    pub fn new(id: String, data: WorkData) -> Self {
        Self {
            id,
            data,
            dirty: false,
        }
    }

    pub fn summary(&self) -> WorkSummary {
        WorkSummary {
            id: self.id.clone(),
            title: self.data.meta.title.clone(),
            status: self.data.meta.status.clone(),
            completed_words: self.data.meta.completed_words,
            updated_at: self.data.meta.updated_at,
        }
    }
}

/// ============================================================
/// WorkManager — 管理当前作品的生命周期
/// 职责:
///   1. 新建作品（创建目录 + 初始化 WorkData + 落盘）
///   2. 打开作品（安全卸载旧的 → 从磁盘加载 → 设为当前）
///   3. 列出已有作品
///   4. 标记 dirty + 触发保存
/// ============================================================
pub struct WorkManager {
    storage: WorkStorage,
    current: Option<WorkSpace>,
}

impl WorkManager {
    pub fn new(base_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            storage: WorkStorage::new(base_dir.into()),
            current: None,
        }
    }

    /// 当前是否有已载入的作品
    pub fn has_current(&self) -> bool {
        self.current.is_some()
    }

    /// 获取当前作品的只读引用
    pub fn current(&self) -> Option<&WorkSpace> {
        self.current.as_ref()
    }

    /// 获取当前作品的可变引用
    pub fn current_mut(&mut self) -> Option<&mut WorkSpace> {
        self.current.as_mut()
    }

    /// 标记当前作品有未保存的修改
    pub fn mark_dirty(&mut self) {
        if let Some(ref mut ws) = self.current {
            ws.dirty = true;
        }
    }

    // ─── 新建作品 ───

    /// 创建一个新作品：生成 UUID → 初始化 WorkData → 落盘 → 设为当前
    pub async fn create(&mut self, title: &str) -> Result<&mut WorkSpace, ManagerError> {
        let id = Uuid::new_v4().to_string();
        let data = WorkData::new(title);

        // 先落盘
        self.storage.save_work(&id, &data).await?;

        // 设为当前（自动卸掉旧的）
        self.unload_current().await?;
        self.current = Some(WorkSpace::new(id, data));

        Ok(self.current.as_mut().unwrap())
    }

    // ─── 打开作品 ───

    /// 打开一个已有作品：安全卸载旧的 → 从磁盘加载 → 设为当前
    pub async fn open(&mut self, id: &str) -> Result<&mut WorkSpace, ManagerError> {
        if !self.storage.work_exists(id) {
            return Err(ManagerError::NotFound(id.to_string()));
        }

        // 如果已经是当前打开的作品，直接返回
        if let Some(ref ws) = self.current {
            if ws.id == id {
                return Ok(self.current.as_mut().unwrap());
            }
        }

        // 安全卸载旧作品
        self.unload_current().await?;

        // 从磁盘加载
        let data = self.storage.load_work(id).await?;
        self.current = Some(WorkSpace::new(id.to_string(), data));

        Ok(self.current.as_mut().unwrap())
    }

    // ─── 安全卸载 ───

    /// 如果当前有作品且 dirty，先保存再卸载
    pub async fn unload_current(&mut self) -> Result<(), ManagerError> {
        if let Some(ws) = self.current.take() {
            if ws.dirty {
                self.storage.save_work(&ws.id, &ws.data).await?;
            }
        }
        Ok(())
    }

    // ─── 保存 ───

    /// 强制保存当前作品
    pub async fn save_current(&mut self) -> Result<(), ManagerError> {
        if let Some(ref ws) = self.current {
            self.storage.save_work(&ws.id, &ws.data).await?;
            // 保存成功后清除 dirty 标记
            if let Some(ref mut inner) = self.current {
                inner.dirty = false;
            }
        }
        Ok(())
    }

    // ─── 列作 ───

    /// 列出所有已有作品
    pub async fn list_works(&self) -> Result<Vec<WorkSummary>, ManagerError> {
        let ids = self.storage.list_work_ids()?;
        let mut summaries = Vec::new();
        for id in ids {
            match self.storage.load_meta(&id).await {
                Ok(meta) => {
                    summaries.push(WorkSummary {
                        id,
                        title: meta.title,
                        status: meta.status,
                        completed_words: meta.completed_words,
                        updated_at: meta.updated_at,
                    });
                }
                Err(e) => {
                    eprintln!("跳过损坏的作品目录: {}", e);
                }
            }
        }
        // 按更新时间降序排列
        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(summaries)
    }

    // ─── 工具方法 ───

    /// 获取当前作品的 Graph 可变引用
    pub fn graph_mut(&mut self) -> Option<&mut Graph> {
        self.current.as_mut().map(|ws| &mut ws.data.graph)
    }

    /// 获取当前作品的 Meta 可变引用
    pub fn meta_mut(&mut self) -> Option<&mut WorkMeta> {
        self.current.as_mut().map(|ws| &mut ws.data.meta)
    }

    /// 获取 base_dir 引用
    pub fn base_dir(&self) -> &std::path::Path {
        self.storage.base_dir.as_path()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("存储错误: {0}")]
    Storage(#[from] crate::workspace::storage::StorageError),
    #[error("作品不存在: {0}")]
    NotFound(String),
}

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    async fn setup() -> WorkManager {
        let dir = tempdir().unwrap();
        WorkManager::new(dir.path().to_path_buf())
    }

    #[tokio::test]
    async fn test_create_work() {
        let mut manager = setup().await;
        let ws = manager.create("我的小说").await.unwrap();
        assert_eq!(ws.data.meta.title, "我的小说");
        assert!(!ws.id.is_empty());
        assert!(!ws.dirty); // 新建后已落盘，dirty = false
    }

    #[tokio::test]
    async fn test_create_then_list() {
        let mut manager = setup().await;
        manager.create("作品A").await.unwrap();
        manager.create("作品B").await.unwrap();

        let list = manager.list_works().await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_create_then_open() {
        let mut manager = setup().await;
        let ws = manager.create("打开测试").await.unwrap();
        let id = ws.id.clone();
        drop(ws); // 释放可变借用

        // 重新打开
        let reopened = manager.open(&id).await.unwrap();
        assert_eq!(reopened.data.meta.title, "打开测试");
    }

    #[tokio::test]
    async fn test_dirty_mark_and_save() {
        let mut manager = setup().await;
        let id;
        {
            let ws = manager.create("脏标记测试").await.unwrap();
            ws.data.meta.completed_words = 1000;
            id = ws.id.clone();
        }
        manager.mark_dirty();
        assert!(manager.current().unwrap().dirty);

        // 保存并验证 dirty 清除
        manager.save_current().await.unwrap();
        assert!(!manager.current().unwrap().dirty);

        // 重新打开验证数据持久化
        let reopened = manager.open(&id).await.unwrap();
        assert_eq!(reopened.data.meta.completed_words, 1000);
    }

    #[tokio::test]
    async fn test_open_auto_saves_previous() {
        let mut manager = setup().await;
        let id_a;
        {
            let ws = manager.create("作品A").await.unwrap();
            ws.data.meta.completed_words = 500;
            id_a = ws.id.clone();
        }
        manager.mark_dirty();

        // 打开作品B → 应该自动保存作品A
        manager.create("作品B").await.unwrap();

        // 重新打开作品A，验证数据还在
        let ws_a = manager.open(&id_a).await.unwrap();
        assert_eq!(ws_a.data.meta.completed_words, 500);
    }

    #[tokio::test]
    async fn test_list_empty() {
        let dir = tempdir().unwrap();
        let manager = WorkManager::new(dir.path().to_path_buf());
        let list = manager.list_works().await.unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_open_nonexistent() {
        let mut manager = setup().await;
        let result = manager.open("nonexistent-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_none_on_init() {
        let manager = setup().await;
        assert!(!manager.has_current());
        assert!(manager.current().is_none());
    }

    #[tokio::test]
    async fn test_open_same_id_twice() {
        let mut manager = setup().await;
        let id = {
            let ws = manager.create("重复打开").await.unwrap();
            ws.id.clone()
        };

        // 第二次打开同一作品应直接返回
        let ws = manager.open(&id).await.unwrap();
        assert_eq!(ws.data.meta.title, "重复打开");
    }

    #[tokio::test]
    async fn test_graph_mut() {
        let mut manager = setup().await;
        manager.create("图谱测试").await.unwrap();

        let graph = manager.graph_mut().unwrap();
        assert!(graph.all_nodes().is_empty());
    }

    #[tokio::test]
    async fn test_meta_mut() {
        let mut manager = setup().await;
        manager.create("Meta测试").await.unwrap();

        let meta = manager.meta_mut().unwrap();
        meta.genre = Some("科幻".to_string());
        manager.mark_dirty();

        assert_eq!(
            manager.current().unwrap().data.meta.genre.as_deref(),
            Some("科幻")
        );
    }
}
