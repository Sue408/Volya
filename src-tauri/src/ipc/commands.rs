use crate::agent::session::{SessionLoop, SessionState};
use crate::workspace::manager::WorkManager;
use tauri::State;
use tokio::sync::Mutex;

/// ============================================================
/// Tauri Commands — 前端可以通过 invoke() 调用的函数
/// ============================================================

/// 全局状态
pub struct AppState {
    pub session: Mutex<Option<SessionLoop>>,
    pub manager: Mutex<WorkManager>,
}

/// 创建新作品并初始化会话
#[tauri::command]
pub async fn create_work(app_handle: tauri::AppHandle, state: State<'_, AppState>, title: String) -> Result<String, String> {
    use tauri::Emitter;

    let mut manager_lock = state.manager.lock().await;
    let ws = manager_lock.create(&title).await.map_err(|e| format!("创建失败: {}", e))?;
    let work_data = ws.data.clone();
    let session = SessionLoop::new(work_data, 1);

    let mut session_lock = state.session.lock().await;
    *session_lock = Some(session);

    let _ = app_handle.emit("agent:state", serde_json::json!({
        "state": "idle", "tool_name": None::<String>
    }));

    Ok(serde_json::json!({ "id": ws.id, "title": title }).to_string())
}

/// 列出所有已有作品
#[tauri::command]
pub async fn list_works(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let manager_lock = state.manager.lock().await;
    let summaries = manager_lock.list_works().await.map_err(|e| format!("列出作品失败: {}", e))?;
    Ok(summaries.into_iter().map(|s| {
        serde_json::json!({
            "id": s.id,
            "title": s.title,
            "status": s.status,
            "completed_words": s.completed_words,
            "updated_at": s.updated_at,
        })
    }).collect())
}

/// 打开一个已有作品（安全卸载旧的 → 加载 → 初始化会话）
#[tauri::command]
pub async fn open_work(app_handle: tauri::AppHandle, state: State<'_, AppState>, id: String) -> Result<serde_json::Value, String> {
    use tauri::Emitter;

    let mut manager_lock = state.manager.lock().await;
    let ws = manager_lock.open(&id).await.map_err(|e| format!("打开作品失败: {}", e))?;

    // 用 WorkManager 中的实际数据初始化会话
    let work_data = ws.data.clone();
    let session = SessionLoop::new(work_data, ws.data.meta.permission_level);

    let mut session_lock = state.session.lock().await;
    *session_lock = Some(session);

    let _ = app_handle.emit("agent:state", serde_json::json!({
        "state": "idle", "tool_name": None::<String>
    }));

    Ok(serde_json::json!({
        "id": ws.id,
        "title": ws.data.meta.title,
        "status": ws.data.meta.status,
        "completed_words": ws.data.meta.completed_words,
        "interactions": ws.data.interactions,
    }))
}

/// 获取 Agent 的工具描述（用于前端展示）
#[tauri::command]
pub async fn get_tool_descriptions(state: State<'_, AppState>) -> Result<String, String> {
    let session_lock = state.session.lock().await;
    let session = session_lock.as_ref().ok_or("会话未初始化")?;
    Ok(session.agent.get_tool_descriptions())
}

/// 发送消息给 Agent（核心交互入口）
/// Agent 通过 AppHandle 直接向前端发射流式事件
#[tauri::command]
pub async fn send_message(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    message: String,
) -> Result<(), String> {
    let mut manager_lock = state.manager.lock().await;
    let mut session_lock = state.session.lock().await;
    let session = session_lock.as_mut().ok_or("会话未初始化")?;

    if session.is_processing() {
        return Err("Agent 正在处理中，请稍候".to_string());
    }

    session.state = SessionState::Processing;
    session.agent.process_message(&message, &app_handle, &mut *manager_lock).await;

    // 将 Agent 的 work_data 同步回 WorkManager（对话持久化）
    if let Some(ref mut ws) = manager_lock.current_mut() {
        ws.data.interactions = session.agent.work_data.interactions.clone();
        ws.data.meta = session.agent.work_data.meta.clone();
    }

    // 如果没有待审批项，直接保存
    if session.agent.pending_approvals.is_empty() {
        let _ = manager_lock.save_current().await;
        session.state = SessionState::Idle;
    } else {
        session.state = SessionState::AwaitingApproval {
            tool_name: session.agent.pending_approvals.first()
                .map(|p| p.tool_name.clone())
                .unwrap_or_default(),
        };
    }

    Ok(())
}

/// 用户回应审批请求 — 处理后继续 LLM 循环
#[tauri::command]
pub async fn handle_approval(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    tool_use_id: String,
    approved: bool,
) -> Result<(), String> {
    let mut session_lock = state.session.lock().await;
    let session = session_lock.as_mut().ok_or("会话未初始化")?;

    match &session.state {
        SessionState::AwaitingApproval { .. } => { /* 正常 */ }
        _ => return Err("当前没有待审批的操作".to_string()),
    }

    session.state = SessionState::Processing;

    // 构建决策列表（当前只处理单个，后续可扩展批量）
    let decisions = vec![(tool_use_id, approved)];

    // 继续 LLM 循环
    let mut manager_lock = state.manager.lock().await;
    session.agent.continue_after_approval(decisions, &app_handle, &mut *manager_lock).await;

    // 同步 Agent 数据回 WorkManager 并保存
    if let Some(ref mut ws) = manager_lock.current_mut() {
        ws.data.interactions = session.agent.work_data.interactions.clone();
        ws.data.meta = session.agent.work_data.meta.clone();
    }

    if session.agent.pending_approvals.is_empty() {
        let _ = manager_lock.save_current().await;
        session.state = SessionState::Idle;
    } else {
        session.state = SessionState::AwaitingApproval {
            tool_name: session.agent.pending_approvals.first()
                .map(|p| p.tool_name.clone())
                .unwrap_or_default(),
        };
    }

    Ok(())
}

/// 修改权限等级
#[tauri::command]
pub async fn set_permission_level(
    state: State<'_, AppState>,
    level: u8,
) -> Result<String, String> {
    let mut session_lock = state.session.lock().await;
    let session = session_lock.as_mut().ok_or("会话未初始化")?;

    let level_name = match level {
        0 => "仅建议 (Lv 0)",
        1 => "半自动 (Lv 1)",
        2 => "全自动 (Lv 2)",
        _ => return Err("无效的权限等级，请输入 0、1 或 2".to_string()),
    };

    session.agent.gate.set_level(crate::agent::gate::PermissionLevel::from_u8(level));
    Ok(format!("权限等级已切换至：{}", level_name))
}

/// 获取当前会话状态
#[tauri::command]
pub async fn get_session_state(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let session_lock = state.session.lock().await;
    let session = session_lock.as_ref().ok_or("会话未初始化")?;

    Ok(serde_json::json!({
        "state": match &session.state {
            SessionState::Idle => "idle",
            SessionState::Processing => "processing",
            SessionState::AwaitingApproval { .. } => "awaiting_approval",
        },
        "permission_level": match session.agent.gate.current_level() {
            crate::agent::gate::PermissionLevel::Lv0Suggestion => 0,
            crate::agent::gate::PermissionLevel::Lv1SemiAuto => 1,
            crate::agent::gate::PermissionLevel::Lv2FullAuto => 2,
        },
        "agent_state": format!("{:?}", session.agent.state),
        "llm_ready": session.agent.is_llm_ready(),
    }))
}

// ─── LLM 配置管理 ───

/// 获取当前 LLM 配置（不返回 API Key，只返回是否存在）
#[tauri::command]
pub async fn get_llm_config() -> Result<serde_json::Value, String> {
    let config = crate::agent::llm::config::GlobalConfig::load();
    Ok(serde_json::json!({
        "configured": !config.llm.api_key.is_empty(),
        "provider": config.llm.provider,
        "api_base": config.llm.api_base,
        "model": config.llm.model,
        "max_tokens": config.llm.max_tokens,
        "temperature": config.llm.temperature,
        "has_api_key": !config.llm.api_key.is_empty(),
    }))
}

/// 保存 LLM 配置
#[tauri::command]
pub async fn save_llm_config(
    state: State<'_, AppState>,
    api_key: String,
    api_base: String,
    model: String,
    max_tokens: u32,
    temperature: f64,
) -> Result<String, String> {
    let mut config = crate::agent::llm::config::GlobalConfig::load();
    // 仅当传入了非空的 api_key 时才覆盖（安全保护：前端看不到已保存的 Key）
    if !api_key.is_empty() {
        config.llm.api_key = api_key;
    }
    config.llm.api_base = api_base;
    config.llm.model = model;
    config.llm.max_tokens = max_tokens;
    config.llm.temperature = temperature;

    config.save().map_err(|e| format!("保存配置失败: {}", e))?;

    Ok("配置已保存 ✅\n请开始新的对话以使用新配置。".to_string())
}

/// 检查 LLM 连接状态
#[tauri::command]
pub async fn check_llm_connection(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let session_lock = state.session.lock().await;
    let session = session_lock.as_ref().ok_or("会话未初始化")?;

    let ready = session.agent.is_llm_ready();
    let config = crate::agent::llm::config::GlobalConfig::load();

    Ok(serde_json::json!({
        "ready": ready,
        "provider": config.llm.provider,
        "model": config.llm.model,
        "message": if ready {
            "LLM 已就绪，可以开始对话 🚀"
        } else {
            "⚠️ 尚未配置 API Key，请在设置中配置"
        }
    }))
}
