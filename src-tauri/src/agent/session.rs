use crate::agent::novel_agent::NovelAgent;
use crate::workspace::data::WorkData;

/// ============================================================
/// SessionLoop — 管理 Agent 的会话状态
/// 一个 WorkSpace = WorkData + SessionLoop
/// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Idle,
    Processing,
    AwaitingApproval { tool_name: String },
}

pub struct SessionLoop {
    pub agent: NovelAgent,
    pub state: SessionState,
}

impl SessionLoop {
    pub fn new(work_data: WorkData, permission_level: u8) -> Self {
        Self {
            agent: NovelAgent::new(work_data, permission_level),
            state: SessionState::Idle,
        }
    }

    pub fn is_processing(&self) -> bool {
        self.state != SessionState::Idle
    }
}
