use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ============================================================
/// 全局 LLM 配置
/// 存储在 ~/.volya/config.json (或平台对应的目录)
/// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// 供应商: "anthropic" | "openai"
    pub provider: String,
    /// API 密钥
    pub api_key: String,
    /// API 请求地址（支持自定义端点，如代理/镜像）
    pub api_base: String,
    /// 模型名称
    pub model: String,
    /// 最大输出 token
    pub max_tokens: u32,
    /// 温度参数
    pub temperature: f64,
    /// 思考预算 token（启用扩展思考时）
    pub thinking_budget: Option<u32>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "anthropic".to_string(),
            api_key: String::new(),
            api_base: "https://api.anthropic.com".to_string(),
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            thinking_budget: None,
        }
    }
}

/// ============================================================
/// GlobalConfig — 全局应用配置
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub llm: LlmConfig,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            llm: LlmConfig::default(),
        }
    }
}

impl GlobalConfig {
    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        let base = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("volya");
        base.join("config.json")
    }

    /// 从磁盘加载配置，不存在则返回默认值
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => {
                            eprintln!("配置解析失败，使用默认配置: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("读取配置文件失败: {}", e);
                }
            }
        }
        Self::default()
    }

    /// 保存配置到磁盘
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// 检查配置是否有效（是否有 API key）
    pub fn is_valid(&self) -> bool {
        !self.llm.api_key.is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
