mod agent;
mod graph;
mod ipc;
mod workspace;

use ipc::commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            session: tokio::sync::Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::create_work,
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
