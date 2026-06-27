mod agent;
mod graph;
mod ipc;
mod workspace;

use ipc::commands::AppState;
use workspace::manager::WorkManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 默认作品目录：~/.volya/works/
    let base_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("volya")
        .join("works");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            session: tokio::sync::Mutex::new(None),
            manager: tokio::sync::Mutex::new(WorkManager::new(base_dir)),
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::create_work,
            ipc::commands::list_works,
            ipc::commands::open_work,
            ipc::commands::delete_work,
            ipc::commands::get_tool_descriptions,
            ipc::commands::send_message,
            ipc::commands::handle_approval,
            ipc::commands::set_permission_level,
            ipc::commands::get_session_state,
            ipc::commands::get_llm_config,
            ipc::commands::save_llm_config,
            ipc::commands::check_llm_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
