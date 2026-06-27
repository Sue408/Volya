use serde::{Deserialize, Serialize};

/// ============================================================
/// Gate — 权限控制中间件
/// 类似 Claude Code 的 Plan/Auto 模式
/// Lv 0: 仅建议 — 所有工具调用都需要用户确认
/// Lv 1: 半自动 — 非敏感工具自动执行，敏感工具需确认
/// Lv 2: 全自动 — 所有工具自动执行
/// ============================================================

/// 权限等级
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PermissionLevel {
    /// 仅建议 — Agent 只能给出建议，所有修改需用户确认
    Lv0Suggestion,
    /// 半自动 — 非敏感操作自动执行，敏感操作需审批
    #[default]
    Lv1SemiAuto,
    /// 全自动 — Agent 可以自主操作
    Lv2FullAuto,
}

impl PermissionLevel {
    pub fn from_u8(level: u8) -> Self {
        match level {
            0 => PermissionLevel::Lv0Suggestion,
            1 => PermissionLevel::Lv1SemiAuto,
            2 => PermissionLevel::Lv2FullAuto,
            _ => PermissionLevel::Lv1SemiAuto,
        }
    }
}

/// 工具的敏感度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolSensitivity {
    /// 只读操作（读取数据），总是安全
    ReadOnly,
    /// 写入操作（修改已有数据），需要关注
    Write,
    /// 危险操作（删除数据等），必须审批
    Dangerous,
}

/// Gate 的决策结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateDecision {
    /// 允许执行
    Allowed,
    /// 需要用户审批
    NeedsApproval { tool: String, reason: String },
    /// 被拒绝
    Denied { reason: String },
}

/// ============================================================
/// Gate — 权限中间件
/// ============================================================
pub struct Gate {
    level: PermissionLevel,
}

impl Gate {
    pub fn new(level: PermissionLevel) -> Self {
        Self { level }
    }

    pub fn set_level(&mut self, level: PermissionLevel) {
        self.level = level;
    }

    pub fn current_level(&self) -> &PermissionLevel {
        &self.level
    }

    /// 检查某个工具调用是否被允许
    pub fn check(&self, tool_name: &str, sensitivity: ToolSensitivity) -> GateDecision {
        match (&self.level, sensitivity) {
            // Lv 0: 只读操作可以执行，其他都需要确认
            (PermissionLevel::Lv0Suggestion, ToolSensitivity::ReadOnly) => GateDecision::Allowed,
            (PermissionLevel::Lv0Suggestion, _) => GateDecision::NeedsApproval {
                tool: tool_name.to_string(),
                reason: "当前为「仅建议」模式，需要您的确认后才能执行此操作".to_string(),
            },
            // Lv 1: 只读和写入自动执行，危险操作需要确认
            (PermissionLevel::Lv1SemiAuto, ToolSensitivity::Dangerous) => GateDecision::NeedsApproval {
                tool: tool_name.to_string(),
                reason: "此操作有潜在风险，需要您的确认".to_string(),
            },
            (PermissionLevel::Lv1SemiAuto, _) => GateDecision::Allowed,
            // Lv 2: 全部自动执行
            (PermissionLevel::Lv2FullAuto, _) => GateDecision::Allowed,
        }
    }

    /// 判断某个工具是否只读（根据名称判断）
    pub fn classify_tool(tool_name: &str) -> ToolSensitivity {
        match tool_name {
            "get_work_meta" | "get_graph" | "search_nodes" => ToolSensitivity::ReadOnly,
            "create_node" | "update_work_meta" | "update_node" | "add_edge" => ToolSensitivity::Write,
            "remove_node" | "remove_edge" | "clear_graph" => ToolSensitivity::Dangerous,
            _ => ToolSensitivity::Write, // 默认保守
        }
    }
}

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_level_from_u8() {
        assert!(matches!(PermissionLevel::from_u8(0), PermissionLevel::Lv0Suggestion));
        assert!(matches!(PermissionLevel::from_u8(1), PermissionLevel::Lv1SemiAuto));
        assert!(matches!(PermissionLevel::from_u8(2), PermissionLevel::Lv2FullAuto));
        assert!(matches!(PermissionLevel::from_u8(99), PermissionLevel::Lv1SemiAuto)); // 默认
    }

    #[test]
    fn test_lv0_readonly_allowed() {
        let gate = Gate::new(PermissionLevel::Lv0Suggestion);
        match gate.check("get_graph", ToolSensitivity::ReadOnly) {
            GateDecision::Allowed => {},
            other => panic!("Lv0 只读应该允许，得到: {:?}", other),
        }
    }

    #[test]
    fn test_lv0_write_needs_approval() {
        let gate = Gate::new(PermissionLevel::Lv0Suggestion);
        match gate.check("create_node", ToolSensitivity::Write) {
            GateDecision::NeedsApproval { .. } => {},
            other => panic!("Lv0 写入需要审批，得到: {:?}", other),
        }
    }

    #[test]
    fn test_lv1_readonly_allowed() {
        let gate = Gate::new(PermissionLevel::Lv1SemiAuto);
        assert!(matches!(gate.check("get_graph", ToolSensitivity::ReadOnly), GateDecision::Allowed));
    }

    #[test]
    fn test_lv1_write_allowed() {
        let gate = Gate::new(PermissionLevel::Lv1SemiAuto);
        assert!(matches!(gate.check("create_node", ToolSensitivity::Write), GateDecision::Allowed));
    }

    #[test]
    fn test_lv1_dangerous_needs_approval() {
        let gate = Gate::new(PermissionLevel::Lv1SemiAuto);
        match gate.check("remove_node", ToolSensitivity::Dangerous) {
            GateDecision::NeedsApproval { .. } => {},
            other => panic!("Lv1 危险操作需要审批，得到: {:?}", other),
        }
    }

    #[test]
    fn test_lv2_always_allowed() {
        let gate = Gate::new(PermissionLevel::Lv2FullAuto);
        assert!(matches!(gate.check("remove_node", ToolSensitivity::Dangerous), GateDecision::Allowed));
        assert!(matches!(gate.check("create_node", ToolSensitivity::Write), GateDecision::Allowed));
        assert!(matches!(gate.check("get_graph", ToolSensitivity::ReadOnly), GateDecision::Allowed));
    }

    #[test]
    fn test_classify_tool() {
        assert!(matches!(Gate::classify_tool("get_work_meta"), ToolSensitivity::ReadOnly));
        assert!(matches!(Gate::classify_tool("get_graph"), ToolSensitivity::ReadOnly));
        assert!(matches!(Gate::classify_tool("search_nodes"), ToolSensitivity::ReadOnly));
        assert!(matches!(Gate::classify_tool("create_node"), ToolSensitivity::Write));
        assert!(matches!(Gate::classify_tool("update_work_meta"), ToolSensitivity::Write));
        assert!(matches!(Gate::classify_tool("remove_node"), ToolSensitivity::Dangerous));
        assert!(matches!(Gate::classify_tool("remove_edge"), ToolSensitivity::Dangerous));
        assert!(matches!(Gate::classify_tool("unknown_tool"), ToolSensitivity::Write)); // 默认保守
    }

    #[test]
    fn test_set_level() {
        let mut gate = Gate::new(PermissionLevel::Lv0Suggestion);
        assert!(matches!(gate.current_level(), PermissionLevel::Lv0Suggestion));
        gate.set_level(PermissionLevel::Lv2FullAuto);
        assert!(matches!(gate.current_level(), PermissionLevel::Lv2FullAuto));
    }
}
