mod models;
mod database;
mod repository;
mod services;
mod commands;
mod config;
mod utils;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let exe_dir = std::env::current_exe()
                .expect("Failed to get executable path")
                .parent()
                .expect("Failed to get executable directory")
                .to_path_buf();
            let db = database::init(&exe_dir).expect("Failed to initialize database");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::project_cmds::get_projects,
            commands::project_cmds::create_project,
            commands::project_cmds::rename_project,
            commands::project_cmds::update_project_description,
            commands::project_cmds::update_project_color,
            commands::project_cmds::delete_project,
            commands::project_cmds::reorder_projects,
            commands::task_cmds::get_tasks,
            commands::task_cmds::create_task,
            commands::task_cmds::update_task,
            commands::task_cmds::delete_task,
            commands::task_cmds::search_tasks,
            commands::task_cmds::get_task_content,
            commands::task_cmds::update_task_content,
            commands::task_cmds::get_task_logs,
            commands::task_cmds::add_task_log,
            commands::task_cmds::delete_task_log,
            commands::task_cmds::delete_tasks,
            commands::task_cmds::move_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
