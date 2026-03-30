mod commands;
mod desktop_manager;
mod models;
mod persistence;
mod state;
mod window_enum;

use state::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            let data_path = data_dir.join("projects.json");

            let data = persistence::load_data(&data_path);

            app.manage(Mutex::new(AppState {
                projects: data.projects,
                assignments: data.assignments,
                data_path,
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::projects::list_projects,
            commands::projects::create_project,
            commands::projects::delete_project,
            commands::projects::rename_project,
            commands::projects::switch_project,
            commands::windows::list_open_windows,
            commands::windows::assign_window_to_project,
            commands::windows::unassign_window,
            commands::windows::get_project_windows,
            commands::desktops::list_desktops,
            commands::desktops::get_current_desktop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
